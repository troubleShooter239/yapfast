# Combined PR testing

`testing` is the fork's persistent branch for building and testing PRs together.
It starts at upstream `main` and merges these branches:

- `chore/deps-and-lint`
- `feat/check-microphone`
- `feat/go-live-preview-and-resolution`
- `feat/stream-audio-controls`
- `fix/twemoji-font-smoothing`
- `fix/video-codecs-and-orientation`
- `fix/high-cpu-usage-in-server`

`fix/windows-restart-hang` is already included through `main`.
Do not merge `chore/basic-ci`, `feat/window-transparency`, or
`feat/voice-hotkeys` into this branch. Features already merged upstream remain.

## Update the combined branch

Keep new PR work on separate branches created from `upstream/main`.
Do not create PR branches from `testing`, which contains all the other PRs.

```powershell
git fetch upstream
git fetch origin --prune
git switch testing
git merge upstream/main
git merge origin/chore/deps-and-lint
git merge origin/feat/check-microphone
git merge origin/feat/go-live-preview-and-resolution
git merge origin/feat/stream-audio-controls
git merge origin/fix/twemoji-font-smoothing
git merge origin/fix/video-codecs-and-orientation
git merge origin/fix/high-cpu-usage-in-server
git push origin testing
```

Record each newly merged branch in [CHANGELOG.md](../CHANGELOG.md), including the date and merge commit.

Run each merge separately. If it conflicts, resolve it and commit before
continuing, or use `git merge --abort` to cancel that merge.
Skip branch names that have been deleted after their PR was merged.
No rebase or force push is needed for normal updates. If a PR branch is rebased
or squash-merged upstream, merging it again can repeat changes or cause
conflicts; inspect its diff and resolve the overlap before continuing.
`main` and the individual PR branches are not modified by these commands.

## Build and publish

In the fork's Actions page, open the existing Nightly workflow
(`.github/workflows/nightly.yml`), choose **Run workflow**, and select
**testing**. The workflow on this branch is named **Testing release**.
It pins both builds to the selected commit, rather than always building main.

By default, download the Linux and Windows artifacts from the completed run.
Enable `publish` to also create a GitHub prerelease in this fork, named
`vMAJOR.NEXT_MINOR.0-testing.DATE.RUN_NUMBER`. This requires Actions to be
enabled in the fork; the release job uses its automatic `GITHUB_TOKEN`.
It needs no upstream signing secret or nightly environment.

These testing releases carry checksums but no minisign signature. Install them
manually. The app's existing updater and AppImage update metadata still refer
to upstream releases; they do not follow this fork's testing builds.

For local builds, follow [building.md](building.md). Native Windows builds are
for local development; use the workflow for distributable packages.
