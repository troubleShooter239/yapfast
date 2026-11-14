# Run before native Windows Cargo commands. Environment changes are process-local.
# Usage: . ./scripts/windows-env.ps1
param([switch]$Prepare)

& {
    $ErrorActionPreference = 'Stop'
    $repoRoot = Split-Path $PSScriptRoot -Parent
    if ($env:OS -ne 'Windows_NT') { throw 'This setup is for native Windows builds.' }

    if (-not $env:SKIA_BINARIES_URL) {
        $archive = Get-ChildItem -Path "$repoRoot/.local/windows/skia/prepared/skia-binaries-*-x86_64-pc-windows-msvc-*-static.tar.gz" -ErrorAction SilentlyContinue |
            Select-Object -First 1
        if (-not $archive) {
            throw 'Prepare Skia first: python scripts/prepare-windows-skia.py'
        }
        # skia-bindings strips file:// and opens the remaining path directly.
        $env:SKIA_BINARIES_URL = 'file://' + $archive.FullName.Replace('\', '/')
    }

    if (-not $env:LK_CUSTOM_WEBRTC) {
        # The published build helper defaults to a newer, incompatible archive.
        # Nix supplies this same pinned native archive through LK_CUSTOM_WEBRTC.
        $locations = @(
            "$repoRoot/.local/windows/webrtc-51ef663",
            "$repoRoot/.local/windows/webrtc-51ef663/win-x64-release",
            "$repoRoot/target/*/build/scratch*/out/livekit_webrtc/livekit/win-x64-release-webrtc-51ef663/win-x64-release"
        )
        $archive = Get-Item -Path $locations -ErrorAction SilentlyContinue |
            Where-Object { $_.PSIsContainer -and (Test-Path -LiteralPath (Join-Path $_.FullName 'include')) } |
            Select-Object -First 1
        if (-not $archive -and $Prepare) {
            $destination = Join-Path $repoRoot '.local/windows/webrtc-51ef663'
            $download = Join-Path $repoRoot '.local/windows/webrtc-51ef663.zip'
            New-Item -ItemType Directory -Force (Split-Path $download -Parent) | Out-Null
            Invoke-WebRequest 'https://github.com/livekit/rust-sdks/releases/download/webrtc-51ef663/webrtc-win-x64-release.zip' -OutFile $download
            $expected = [Convert]::FromBase64String('ClalyRs7e0IiCCuKCTiNFqgCzeItZylb6yTFVwB1WzA=')
            $expectedHex = ([BitConverter]::ToString($expected)).Replace('-', '')
            if ((Get-FileHash -LiteralPath $download -Algorithm SHA256).Hash -ne $expectedHex) {
                throw 'The WebRTC archive does not match its pinned SHA-256.'
            }
            Expand-Archive -LiteralPath $download -DestinationPath $destination -Force
            $archive = Get-Item -LiteralPath (Join-Path $destination 'win-x64-release')
        }
        if (-not $archive) {
            throw 'Run . ./scripts/windows-env.ps1 -Prepare to fetch the pinned WebRTC archive, or set LK_CUSTOM_WEBRTC to its win-x64-release folder.'
        }
        $env:LK_CUSTOM_WEBRTC = $archive.FullName
    }

    $clang = Get-Command clang-cl -ErrorAction Stop
    # Use one compiler consistently across all native dependencies, even when
    # this shell previously selected MSVC or MinGW for another project.
    $env:CC = $clang.Source
    $env:CXX = $clang.Source
    if (-not $env:LIBCLANG_PATH) { $env:LIBCLANG_PATH = Split-Path $clang.Source -Parent }
    if (-not $env:NASM -and -not (Get-Command nasm -ErrorAction SilentlyContinue)) {
        $nasm = Get-ChildItem -Path "$repoRoot/.local/tools" -Filter nasm.exe -Recurse -ErrorAction SilentlyContinue |
            Select-Object -First 1
        if (-not $nasm) { throw 'Install NASM or set NASM to nasm.exe.' }
        $env:NASM = $nasm.FullName
    }
    Write-Host "Native Windows build environment ready (CC/CXX: $($clang.Source))."
}
