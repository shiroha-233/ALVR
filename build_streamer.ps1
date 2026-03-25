$ErrorActionPreference = 'Stop'

function Invoke-WithRetry {
    param(
        [scriptblock] $Action,
        [int] $MaxAttempts = 3,
        [int] $DelaySeconds = 5
    )

    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        try {
            & $Action
            return
        }
        catch {
            if ($attempt -eq $MaxAttempts) {
                throw
            }

            Write-Host "Attempt $attempt failed: $($_.Exception.Message)"
            Start-Sleep -Seconds $DelaySeconds
        }
    }
}

$vsCandidates = @(
    'C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat',
    'C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat'
)

$vsDevCmd = $vsCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $vsDevCmd) {
    throw 'Could not find VsDevCmd.bat'
}

$setCommand = '@echo off && call "' + $vsDevCmd + '" -arch=x64 -host_arch=x64 >nul && set'

foreach ($line in (& cmd.exe /c $setCommand)) {
    if ($line -match '^(.*?)=(.*)$') {
        Set-Item -Path ("Env:{0}" -f $matches[1]) -Value $matches[2]
    }
}

$extraPaths = @(
    'C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin',
    'C:\Program Files\Microsoft Visual Studio\18\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin'
) | Where-Object { Test-Path $_ }

if ($extraPaths.Count -gt 0) {
    $env:PATH = (($extraPaths -join ';') + ';' + $env:PATH)
}

$atlIncludeCandidates = @(
    'C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\atlmfc\include'
) | Where-Object { Test-Path $_ }

if ($atlIncludeCandidates.Count -gt 0) {
    $env:INCLUDE = (($atlIncludeCandidates -join ';') + ';' + $env:INCLUDE)
}

$atlLibCandidates = @(
    'C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\atlmfc\lib\x64'
) | Where-Object { Test-Path $_ }

if ($atlLibCandidates.Count -gt 0) {
    $env:LIB = (($atlLibCandidates -join ';') + ';' + $env:LIB)
}

Write-Host "Using VS dev command: $vsDevCmd"

$depsRoot = 'D:\GITHUB\ALVR\deps\windows'
$libvplRoot = Join-Path $depsRoot 'libvpl'
$libvplInstall = Join-Path $libvplRoot 'alvr_build'
$libvplDll = Join-Path $libvplInstall 'bin\libvpl.dll'

$llvmRoot = 'D:\GITHUB\ALVR\deps\llvm'
$preferredLibclangPaths = @(
    (Join-Path $llvmRoot 'bin\libclang.dll'),
    'C:\Program Files\LLVM\bin\libclang.dll'
) | Where-Object { Test-Path $_ }

if ($preferredLibclangPaths.Count -gt 0) {
    $libclangPath = $preferredLibclangPaths[0]
}
else {
    Write-Host 'Preparing libclang dependency...'

    $llvmVersion = '22.1.1'
    $llvmInstaller = Join-Path $env:TEMP 'LLVM-22.1.1-win64.exe'
    $llvmUrl = "https://github.com/llvm/llvm-project/releases/download/llvmorg-$llvmVersion/LLVM-$llvmVersion-win64.exe"

    Remove-Item -Recurse -Force $llvmRoot -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $llvmInstaller -ErrorAction SilentlyContinue

    Invoke-WithRetry {
        Invoke-WebRequest -Uri $llvmUrl -OutFile $llvmInstaller
    } -MaxAttempts 5 -DelaySeconds 10

    Start-Process -Wait -FilePath $llvmInstaller -ArgumentList @('/S', "/D=$llvmRoot")

    if (-not (Test-Path $libclangPath)) {
        throw 'LLVM install completed but libclang.dll was not found'
    }
}

$env:LIBCLANG_PATH = Split-Path $libclangPath -Parent
$env:PATH = $env:LIBCLANG_PATH + ';' + $env:PATH

if (-not (Test-Path $libvplDll)) {
    Write-Host 'Preparing libvpl dependency...'

    New-Item -ItemType Directory -Force -Path $depsRoot | Out-Null

    $version = '2.15.0'
    $zipPath = Join-Path $env:TEMP 'libvpl-2.15.0.zip'
    $extractRoot = Join-Path $env:TEMP 'alvr-libvpl-extract'
    $downloadUrl = "https://github.com/intel/libvpl/archive/refs/tags/v$version.zip"

    Remove-Item -Recurse -Force $zipPath -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $extractRoot -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $libvplRoot -ErrorAction SilentlyContinue

    Invoke-WithRetry {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath
    }

    Expand-Archive -Path $zipPath -DestinationPath $extractRoot -Force

    Move-Item -Path (Join-Path $extractRoot "libvpl-$version") -Destination $libvplRoot

    Push-Location $libvplRoot
    try {
        cmake -B build -DUSE_MSVC_STATIC_RUNTIME=ON -DCMAKE_INSTALL_PREFIX="$libvplInstall"
        if ($LASTEXITCODE -ne 0) {
            throw "libvpl cmake configure failed with exit code $LASTEXITCODE"
        }

        cmake --build build --config Release
        if ($LASTEXITCODE -ne 0) {
            throw "libvpl cmake build failed with exit code $LASTEXITCODE"
        }

        cmake --install build --config Release
        if ($LASTEXITCODE -ne 0) {
            throw "libvpl cmake install failed with exit code $LASTEXITCODE"
        }
    }
    finally {
        Pop-Location
    }

    Remove-Item -Recurse -Force $extractRoot -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $zipPath -ErrorAction SilentlyContinue
}

cargo xtask build-streamer --release
if ($LASTEXITCODE -ne 0) {
    throw "build-streamer failed with exit code $LASTEXITCODE"
}
