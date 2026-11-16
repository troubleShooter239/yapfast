"""Regression checks for build metadata, source packaging and process cleanup."""

import importlib.util
import io
import json
import subprocess
import tempfile
import unittest
import urllib.error
import zipfile
from pathlib import Path
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[1]


def load_module(name, relative):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"Cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


nix_skia = load_module("nix_skia", "nix/prepare-windows-skia.py")
windows_skia = load_module("windows_skia", "scripts/prepare-windows-skia.py")
release_package = load_module("release_package", "scripts/release_package.py")
check_stack = load_module("check_stack", "testing/check_stack.py")
# Linux measurements use sysconf at import time; no real /proc or processes are needed here.
with patch("os.sysconf", return_value=100, create=True):
    voice_harness = load_module("voice_harness", "testing/voice_harness.py")


class CacheMetadataTest(unittest.TestCase):
    def test_missing_or_damaged_metadata_is_a_cache_miss(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "stamp.json"
            for payload in (None, b"{", b"[]", b"null", b'"text"', b"\xff"):
                with self.subTest(payload=payload):
                    if payload is not None:
                        path.write_bytes(payload)
                    self.assertEqual(nix_skia.read_stamp(path), {})
                    self.assertIsNone(windows_skia.read_json(path))

    def test_valid_metadata_is_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "stamp.json"
            metadata = {"inputs": {"compiler": "Clang 23"}, "prepared_sha256": "hash"}
            path.write_text(json.dumps(metadata), encoding="utf-8")
            self.assertEqual(nix_skia.read_stamp(path), metadata)
            self.assertEqual(windows_skia.read_json(path), metadata)


class HttpResourcesTest(unittest.TestCase):
    def test_error_responses_are_closed_even_when_json_is_invalid(self):
        for payload in (b'{"errcode": "unauthorized"}', b"not JSON"):
            with self.subTest(payload=payload):
                response = io.BytesIO(payload)
                error = urllib.error.HTTPError(
                    "http://localhost", 401, "Unauthorized", {}, response
                )
                with patch.object(check_stack.urllib.request, "urlopen", side_effect=error):
                    status, _body = check_stack.request("http://localhost", "GET", "/test")
                self.assertEqual(status, 401)
                self.assertTrue(response.closed)


class SourceSnapshotTest(unittest.TestCase):
    def test_source_contains_flakes_and_is_reproducible(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            files = ("Cargo.toml", "Cargo.lock", "flake.nix", "flake.lock", "CHANGELOG.md")
            for name in files:
                (root / name).write_text(f"contents of {name}\n", encoding="utf-8")
            (root / "private.txt").write_text("untracked working material", encoding="utf-8")

            def git_output(command, **_kwargs):
                if command[1] == "ls-files":
                    # Model Git's path filtering rather than returning unrequested files.
                    selected = [name for name in files if name in command]
                    return "\0".join([*selected, ""]).encode()
                return b"1700000000\n"

            archives = []
            with (
                patch.object(release_package, "ROOT", root),
                patch.object(release_package.subprocess, "check_output", side_effect=git_output),
            ):
                for index in range(2):
                    work = root / f"work-{index}"
                    work.mkdir()
                    archive = root / f"source-{index}.zip"
                    self.assertFalse(
                        release_package.source_snapshot(
                            {"packages": []}, "Yapfast-test", work, archive
                        )
                    )
                    archives.append(archive)
            with zipfile.ZipFile(archives[0]) as archive:
                self.assertEqual(
                    set(archive.namelist()), {f"Yapfast-test-source/{name}" for name in files}
                )
            self.assertEqual(archives[0].read_bytes(), archives[1].read_bytes())


class ProcessCleanupTest(unittest.TestCase):
    def test_timed_out_participant_is_reaped_even_if_kill_races_with_exit(self):
        participant = object.__new__(voice_harness.Participant)
        participant.process = Mock()
        participant.process.poll.return_value = None
        participant.process.wait.side_effect = [subprocess.TimeoutExpired("participant", 60), 0]
        participant.log = Mock()
        participant.send = Mock()
        with (
            patch.object(voice_harness.signal, "SIGKILL", 9, create=True),
            patch.object(voice_harness.os, "killpg", side_effect=ProcessLookupError, create=True),
        ):
            participant.stop()
        participant.send.assert_called_once_with("exit")
        self.assertEqual(participant.process.wait.call_count, 2)
        participant.process.wait.assert_called_with(timeout=10)
        participant.log.close.assert_called_once_with()


if __name__ == "__main__":
    unittest.main()
