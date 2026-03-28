[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $true)]
    [string]$Tag,

    [string]$Remote = "origin",

    [string]$Message = "",

    [switch]$SkipBranchPush
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Git {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    & git @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

if ($Tag -notmatch "^v") {
    throw "Tag '$Tag' does not match the release workflow. Please use a tag starting with 'v'."
}

$branch = (& git branch --show-current).Trim()
if (-not $branch) {
    throw "Detached HEAD is not supported. Please checkout a branch first."
}

$status = (& git status --porcelain=v1).Trim()
if ($status) {
    throw "Working tree is not clean. Commit or stash changes before pushing a release tag."
}

$localTag = (& git tag --list $Tag).Trim()
if ($localTag) {
    throw "Local tag '$Tag' already exists."
}

$remoteTag = (& git ls-remote --tags $Remote "refs/tags/$Tag").Trim()
if ($remoteTag) {
    throw "Remote tag '$Tag' already exists on '$Remote'."
}

if (-not $Message) {
    $Message = "Release $Tag"
}

if (-not $SkipBranchPush -and $PSCmdlet.ShouldProcess("$Remote/$branch", "Push current branch")) {
    Invoke-Git -Arguments @("push", $Remote, $branch)
}

if ($PSCmdlet.ShouldProcess($Tag, "Create annotated tag")) {
    Invoke-Git -Arguments @("tag", "-a", $Tag, "-m", $Message)
}

try {
    if ($PSCmdlet.ShouldProcess("$Remote/$Tag", "Push release tag")) {
        Invoke-Git -Arguments @("push", $Remote, "refs/tags/$Tag")
    }
} catch {
    Invoke-Git -Arguments @("tag", "-d", $Tag)
    throw
}

Write-Host "Pushed tag '$Tag' to '$Remote'. GitHub Actions should now start the release workflow."
