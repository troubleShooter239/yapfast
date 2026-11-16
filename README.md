# Yapfast — testing

This branch of my fork is used to publish my own test releases of Yapfast.
It combines changes from my PRs so I can test them together and check whether
they break each other.

Releases are built for Windows and Linux, including an AppImage and an Arch Linux
package. Builds can be started through GitHub Actions and are intended for testing.

Merged branches are recorded in [CHANGELOG.md](CHANGELOG.md).
See the [instructions](docs/fork-testing.md) for details on updating branches
and publishing builds.

## Update from upstream/main

```powershell
git fetch upstream
git switch testing
git merge upstream/main
git push
```
