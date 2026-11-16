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
