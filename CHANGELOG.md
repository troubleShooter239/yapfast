# Changelog

This file records branches merged into `testing` to test PRs together.
For each new merge, add an entry with the date, branch name, and merge commit.

## 2026-11-16

Merged into `testing`:

| Branch                                | Changes                                                 | Merge commit |
| ------------------------------------- | ------------------------------------------------------- | ------------ |
| `chore/basic-ci`                      | CI, prek hooks, formatting and code style configuration | `a338c94d`   |
| `chore/deps-and-lint`                 | Dependency updates and lint fixes                       | `1e1aba91`   |
| `feat/check-microphone`               | Microphone testing and level meter                      | `768c991a`   |
| `feat/go-live-preview-and-resolution` | Go Live preview and resolution selection                | `602bb1ae`   |
| `feat/stream-audio-controls`          | Stream audio controls and window previews               | `bb6641f4`   |
| `fix/twemoji-font-smoothing`          | Twemoji and font smoothing settings                     | `dbab7713`   |
| `fix/video-codecs-and-orientation`    | HEVC support and video orientation fixes                | `d4693c30`   |
| `fix/high-cpu-usage-in-server`        | Scrolling and CPU usage optimizations                   | `e50e972b`   |

`fix/windows-restart-hang` is already included in the base `main` branch,
so it was not merged separately. The fork owner chose to exclude
`feat/window-transparency` and `feat/voice-hotkeys`
from the branch merges.

Configured builds of the selected commit for Windows and Linux, with optional
publication as a testing prerelease. Resolved merge conflicts and incompatibilities
between the merged branches; compilation checks for all targets passed on Windows.
Packaging in Actions and the combined changes at runtime have not yet been verified.

Merged `chore/basic-ci` while retaining the testing README, updated dependencies,
HEVC support and video orientation fixes. Enabled CI for pushes and PRs targeting
`testing`. Fixed the contribution guide's link after the README replacement.

Fixed Skia preparation scripts failing under Nix's Python interpreter: restored
parentheses around exception tuples and set Ruff's Python target to 3.13 to
prevent the formatter from introducing newer exception syntax.

Selected Python 3.14.3 (`python314` from the existing nixpkgs pin) for Linux
Skia preparation and both Nix development shells, without updating nixpkgs.
Kept scripts compatible with Python 3.13 for use outside Nix.

Updated the shared DeepFilterNet source hash to match revision `9b8790d` in
`Cargo.lock`, using the actual hash reported by the Linux Nix build. This fixes
the reported fixed-output hash mismatch during Cargo vendoring.

Fixed Linux CI selecting Clang 23 while installing Clang 18. Added the signed
LLVM APT repository, installed Clang/libclang/LLVM 23, and added a toolchain
check before Cargo runs.

Moved both Nix flakes to `nixos-26.05` and selected LLVM 23 consistently for
the application and development shells. The Rust package now uses LLVM's
Clang stdenv so native dependencies do not fall back to GCC. Documented how
to update both nixpkgs lock files.

Selected Python 3.15.2 from the updated nixpkgs in both Nix shells and Skia
preparation, and aligned Ruff's formatter target with Python 3.15.

Validated Nix evaluation of the application (Clang 23.1.0), AppImage and both
development shells. Full release builds and AppImage runtime compatibility
have not yet been validated with the updated nixpkgs.

Fixed Nix Cargo build/check hooks injecting GCC target compiler variables despite
the package using Clang stdenv. Both hooks now use Clang's Rust environment helper.
Verified the generated build hook selects Clang 23.1.0 for the target compiler.
Replaced constant-size `chunks_exact` calls in the Linux frame recorder with
`as_chunks` to satisfy Clippy; verified unchanged hashes for 16,388 inputs.
Full Linux compilation and release packaging remain to be verified.

Fixed the remaining GCC fallback through `HOST_CC`/`HOST_CXX` in Nix Rust
hooks and explicitly set cc-rs's lowercase target-specific compiler variables.
Updated constant-size chunk iteration in the tray recorder and Windows audio
tests for Clippy. Windows workspace Clippy with warnings denied passed; tray
pixel conversion matched the old implementation for 4,097 inputs.

A Nix smoke build verified cc-rs compiler-variable precedence selects Clang
23.1.0 for both C and C++, and successfully checked the compiler's Clang 23
preprocessor macros. Full Linux application/release builds remain unverified.

Updated the nightly workflow's cache action from v4 to v5 (Node.js 24 runtime)
to remove the Node.js 20 deprecation warning.

Updated Windows Skia preparation to read the pinned Clang version from the Nix
shell instead of requiring 21.1.7. Its GN configuration and resource-header
layout now use that compiler's major version, matching LLVM 23. Verified version
validation and generated paths with a mocked build; the full Skia build is pending.

Audited the first-party Python tools for Python 3.15. Enabled Ruff bug checks,
upgrade checks and explicit subprocess exit handling, including extensionless
scripts in directory scans. Preserved manually handled command failures.

Made Skia cache readers tolerate corrupt or non-object metadata, closed HTTP
responses and log files explicitly, bounded the emoji-data fetch, and reaped
measurement subprocesses after forced shutdown. Source snapshots now include
both root Nix flake files and the changelog, and timestamp handling works on
Windows without following unsupported symlinks. Preserved explicit CRLF when
writing the Visual Studio environment batch file.

Added six build-tool regression tests (including the updated font-notice fixture)
and run them in CI on Python 3.15. Ruff, syntax checks, documentation links and
the unit tests passed locally. Live Linux GUI/audio/network workloads and full
release builds were not run as part of this Python audit.

Replaced string concatenation with f-strings throughout first-party Python
scripts, including extensionless commands. Kept byte buffers, list concatenation
and arithmetic unchanged. Verified JSON payloads and Windows package text output
remain unchanged; Python lint, formatting and regression tests passed.

## 2026-11-17

Fixed Linux linking against LiveKit's prefixed FFmpeg symbols. The core build
script maps the Chromium FFmpeg calls in its native codec shims to `LK_*` on
Linux, matching the pinned `webrtc-a545de4` archive. Windows and the separate
`yf_*` FFmpeg build for HEVC/hardware codecs retain their existing linkage.

Verified the pinned Linux archive hash and a native Linux link/decode smoke
build using H264, AAC, MP4 and Opus/WebM fixtures. Windows all-target compilation
checks, Rust formatting and documentation link checks passed. The complete
application/AppImage release build has not yet been rerun.
