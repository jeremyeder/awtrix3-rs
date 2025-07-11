# AWTRIX3 CLI Installation Script for Windows PowerShell
# This script installs the latest version of awtrix3 CLI

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\awtrix3",
    [string]$Version = "latest",
    [switch]$AddToPath = $true,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# Repository information
$REPO = "jeremyeder/awtrix3-rs"
$BIN_NAME = "awtrix3.exe"

# Colors for output
$RED = "Red"
$GREEN = "Green"
$YELLOW = "Yellow"
$BLUE = "Cyan"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $BLUE
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $GREEN
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor $YELLOW
}

function Write-ErrorMessage {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $RED
}

function Show-Help {
    Write-Host "AWTRIX3 CLI Installation Script"
    Write-Host "Usage: .\install.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -InstallDir <path>    Installation directory (default: $env:LOCALAPPDATA\Programs\awtrix3)"
    Write-Host "  -Version <version>    Specific version to install (default: latest)"
    Write-Host "  -AddToPath           Add installation directory to PATH (default: true)"
    Write-Host "  -Help                Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\install.ps1                           # Install latest version"
    Write-Host "  .\install.ps1 -Version 0.1.0           # Install specific version"
    Write-Host "  .\install.ps1 -InstallDir C:\Tools     # Custom install directory"
}

function Get-LatestVersion {
    Write-Info "Fetching latest release information..."
    
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest" -Method Get
        $version = $response.tag_name
        if ($version.StartsWith("v")) {
            $version = $version.Substring(1)
        }
        Write-Info "Latest version: $version"
        return $version
    }
    catch {
        Write-ErrorMessage "Failed to get latest version: $($_.Exception.Message)"
        exit 1
    }
}

function Test-Prerequisites {
    Write-Info "Checking prerequisites..."
    
    # Check if we have internet connectivity
    try {
        $null = Invoke-WebRequest -Uri "https://github.com" -Method Head -TimeoutSec 10
    }
    catch {
        Write-ErrorMessage "No internet connectivity detected"
        exit 1
    }
    
    # Check PowerShell version
    if ($PSVersionTable.PSVersion.Major -lt 5) {
        Write-ErrorMessage "PowerShell 5.0 or later is required"
        exit 1
    }
    
    Write-Info "Prerequisites check passed"
}

function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default {
            Write-ErrorMessage "Unsupported architecture: $arch"
            exit 1
        }
    }
}

function Download-And-Install {
    param([string]$Version)
    
    $arch = Get-Architecture
    $platform = "x86_64-pc-windows-msvc"
    $archive_name = "awtrix3-v$Version-$platform.zip"
    $download_url = "https://github.com/$REPO/releases/download/v$Version/$archive_name"
    
    Write-Info "Downloading $archive_name..."
    
    # Create temporary directory
    $temp_dir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    $archive_path = Join-Path $temp_dir $archive_name
    
    try {
        # Download archive
        Invoke-WebRequest -Uri $download_url -OutFile $archive_path -UseBasicParsing
        
        Write-Info "Extracting archive..."
        
        # Extract archive
        Expand-Archive -Path $archive_path -DestinationPath $temp_dir -Force
        
        # Create installation directory
        if (-not (Test-Path $InstallDir)) {
            Write-Info "Creating installation directory: $InstallDir"
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }
        
        # Copy binary
        $source_binary = Join-Path $temp_dir $BIN_NAME
        $dest_binary = Join-Path $InstallDir $BIN_NAME
        
        if (Test-Path $source_binary) {
            Copy-Item $source_binary $dest_binary -Force
            Write-Info "Installed $BIN_NAME to $InstallDir"
        } else {
            Write-ErrorMessage "Binary not found in archive"
            exit 1
        }
        
        # Install completions if available
        $completions_dir = Join-Path $temp_dir "completions"
        if (Test-Path $completions_dir) {
            Install-Completions $completions_dir
        }
        
    }
    catch {
        Write-ErrorMessage "Installation failed: $($_.Exception.Message)"
        exit 1
    }
    finally {
        # Cleanup
        if (Test-Path $temp_dir) {
            Remove-Item $temp_dir -Recurse -Force
        }
    }
}

function Install-Completions {
    param([string]$CompletionsDir)
    
    Write-Info "Installing shell completions..."
    
    # PowerShell completions
    $ps_completion = Join-Path $CompletionsDir "awtrix3.ps1"
    if (Test-Path $ps_completion) {
        $profile_dir = Split-Path $PROFILE -Parent
        if (-not (Test-Path $profile_dir)) {
            New-Item -ItemType Directory -Path $profile_dir -Force | Out-Null
        }
        
        $completion_dest = Join-Path $profile_dir "awtrix3-completion.ps1"
        Copy-Item $ps_completion $completion_dest -Force
        Write-Info "Installed PowerShell completions to $completion_dest"
        
        # Add to profile if not already there
        if (Test-Path $PROFILE) {
            $profile_content = Get-Content $PROFILE -Raw
            if (-not $profile_content.Contains("awtrix3-completion.ps1")) {
                Add-Content $PROFILE "`n# AWTRIX3 CLI completions`n. `"$completion_dest`""
                Write-Info "Added completion loading to PowerShell profile"
            }
        } else {
            Set-Content $PROFILE "# AWTRIX3 CLI completions`n. `"$completion_dest`""
            Write-Info "Created PowerShell profile with completions"
        }
    }
}

function Add-ToPath {
    if (-not $AddToPath) {
        return
    }
    
    Write-Info "Adding installation directory to PATH..."
    
    # Get current user PATH
    $current_path = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    # Check if already in PATH
    if ($current_path -split ";" -contains $InstallDir) {
        Write-Info "Installation directory already in PATH"
        return
    }
    
    # Add to PATH
    $new_path = if ($current_path) { "$current_path;$InstallDir" } else { $InstallDir }
    [Environment]::SetEnvironmentVariable("PATH", $new_path, "User")
    
    # Update current session PATH
    $env:PATH = "$env:PATH;$InstallDir"
    
    Write-Success "Added $InstallDir to PATH"
    Write-Info "Restart your terminal or run 'refreshenv' to use the new PATH"
}

function Test-Installation {
    Write-Info "Verifying installation..."
    
    $binary_path = Join-Path $InstallDir $BIN_NAME
    
    if (Test-Path $binary_path) {
        try {
            $version_output = & $binary_path --version 2>$null
            Write-Success "$BIN_NAME installed successfully!"
            Write-Info "Installed version: $version_output"
        }
        catch {
            Write-Warning "$BIN_NAME was installed but may not be working correctly"
        }
    } else {
        Write-ErrorMessage "Installation verification failed - binary not found"
        exit 1
    }
}

function Main {
    Write-Host "AWTRIX3 CLI Installer for Windows" -ForegroundColor $BLUE
    Write-Host "=================================" -ForegroundColor $BLUE
    Write-Host ""
    
    if ($Help) {
        Show-Help
        return
    }
    
    Test-Prerequisites
    
    # Resolve installation directory
    $InstallDir = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($InstallDir)
    
    # Get version
    if ($Version -eq "latest") {
        $Version = Get-LatestVersion
    } else {
        Write-Info "Installing version: $Version"
    }
    
    Download-And-Install $Version
    Add-ToPath
    Test-Installation
    
    Write-Host ""
    Write-Success "Installation complete!" -ForegroundColor $GREEN
    Write-Host ""
    Write-Host "Next steps:"
    Write-Host "1. Add a device: awtrix3 device add <name> <host>"
    Write-Host "2. Send a notification: awtrix3 notify `"Hello, World!`""
    Write-Host "3. View all commands: awtrix3 --help"
    Write-Host ""
    Write-Host "Documentation: https://github.com/$REPO"
}

# Run main function
Main