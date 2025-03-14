# Post-installation script for hai on Windows
# This script installs man pages to the appropriate location

# Determine the appropriate documentation location
# Windows doesn't have a standard man page location, so we'll use a few options

# First try Program Files if we have write access
$ProgramDataDir = [System.Environment]::GetFolderPath('CommonApplicationData')
$ProgramFilesDir = [System.Environment]::GetFolderPath('ProgramFiles')

if (Test-Path -Path $ProgramFilesDir) {
    try {
        # Test if we can write to Program Files
        $testFile = Join-Path $ProgramFilesDir "hai-test.tmp"
        [System.IO.File]::WriteAllText($testFile, "test")
        Remove-Item $testFile
        
        # We can write to Program Files, use it
        $DocDir = Join-Path $ProgramFilesDir "hai\doc"
    } catch {
        # We can't write to Program Files, try ProgramData
        if (Test-Path -Path $ProgramDataDir) {
            try {
                $testFile = Join-Path $ProgramDataDir "hai-test.tmp"
                [System.IO.File]::WriteAllText($testFile, "test")
                Remove-Item $testFile
                
                # We can write to ProgramData, use it
                $DocDir = Join-Path $ProgramDataDir "hai\doc"
            } catch {
                # Fallback to user's documents folder
                $DocDir = Join-Path ([System.Environment]::GetFolderPath('MyDocuments')) "hai\doc"
            }
        } else {
            # Fallback to user's documents folder
            $DocDir = Join-Path ([System.Environment]::GetFolderPath('MyDocuments')) "hai\doc"
        }
    }
} else {
    # Fallback to user's documents folder
    $DocDir = Join-Path ([System.Environment]::GetFolderPath('MyDocuments')) "hai\doc"
}

# Create man directories
$Man1Dir = Join-Path $DocDir "man1"
$Man5Dir = Join-Path $DocDir "man5"

New-Item -ItemType Directory -Force -Path $Man1Dir | Out-Null
New-Item -ItemType Directory -Force -Path $Man5Dir | Out-Null

# Copy man pages
$Hai1 = "man\man1\hai.1"
$HaiConfig5 = "man\man5\hai-config.5"

if (Test-Path $Hai1) {
    Copy-Item $Hai1 -Destination $Man1Dir
    Write-Host "Installed man page: hai(1) to $Man1Dir"
}

if (Test-Path $HaiConfig5) {
    Copy-Item $HaiConfig5 -Destination $Man5Dir
    Write-Host "Installed man page: hai-config(5) to $Man5Dir"
}

Write-Host "Documentation installed to $DocDir"

# Note about viewing man pages on Windows
Write-Host ""
Write-Host "NOTE: To view man pages on Windows, you have several options:"
Write-Host "  1. Install Git Bash, which includes 'man'"
Write-Host "  2. Use Windows Subsystem for Linux (WSL)"
Write-Host "  3. Use a text editor to view the files directly at $DocDir"
Write-Host "  4. Install a third-party man page viewer" 