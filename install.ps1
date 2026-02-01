$Owner = "sonesuke"
$Repo = "docgraph"
$BinaryName = "docgraph"

$OS = "pc-windows-msvc"
$Arch = "x86_64"
$Target = "$Arch-$OS"

Write-Host "Detecting latest version..."
$Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Owner/$Repo/releases/latest"
$LatestTag = $Release.tag_name

if (-not $LatestTag) {
    Write-Error "Failed to fetch latest version."
    exit 1
}

Write-Host "Downloading $BinaryName $LatestTag for $Target..."
$AssetName = "${BinaryName}-${Target}.zip"
$Asset = $Release.assets | Where-Object { $_.name -eq $AssetName }

if (-not $Asset) {
    Write-Error "Asset not found: $AssetName"
    exit 1
}

$DownloadUrl = $Asset.browser_download_url
$TempPath = Join-Path $env:TEMP $AssetName
$ExtractPath = Join-Path $env:TEMP "$BinaryName-install"

if (Test-Path $ExtractPath) { Remove-Item -Recurse -Force $ExtractPath }
New-Item -ItemType Directory -Path $ExtractPath

Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempPath
Expand-Archive -Path $TempPath -DestinationPath $ExtractPath

$InstallDir = Join-Path $env:ProgramFiles $BinaryName
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir
}

Move-Item -Path (Join-Path $ExtractPath "$BinaryName.exe") -Destination (Join-Path $InstallDir "$BinaryName.exe") -Force

# Add to Path if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to User PATH. Please restart your terminal."
}

Remove-Item -Recurse -Force $ExtractPath
Remove-Item -Force $TempPath

Write-Host "Successfully installed $BinaryName $LatestTag"
