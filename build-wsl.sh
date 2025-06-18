#!/bin/bash

# Impulse 7.1 Build Script for Ubuntu Linux
# Automates compilation using DOSBox

set -e  # Exit on any error

# Configuration
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$REPO_DIR/build"
SOURCE_DIR="$REPO_DIR/source"
OUTPUT_DIR="$REPO_DIR/output"
INCLUDE_DIR="$REPO_DIR/include"
BP_DIR="$REPO_DIR/BP"

# DOSBox configuration and batch files (temporary)
TEMP_DIR="$REPO_DIR/.tmp"
DOSBOX_CONF="$TEMP_DIR/dosbox_build_temp.conf"
BUILD_BATCH="$TEMP_DIR/build.bat"

# Detect if running under WSL
IS_WSL=false
if grep -qi "microsoft" /proc/version 2>/dev/null || [ -n "${WSL_DISTRO_NAME:-}" ] || [ -n "${WSLENV:-}" ]; then
    IS_WSL=true
fi

echo "=== Impulse 7.1 Build Script ==="
echo "Repository: $REPO_DIR"
echo "DOSBox C: drive will be: /tmp/impulse_build"

if [ "$IS_WSL" = true ]; then
    echo "🐧 Detected WSL environment - using text-only mode"
else
    echo "🖥️  Detected native Linux environment - using graphical mode"
fi

# Check if DOSBox is installed
if ! command -v dosbox &> /dev/null; then
    echo "❌ DOSBox is not installed!"
    echo "Install it with: sudo apt install dosbox"
    exit 1
fi

# Check if BP directory exists in repo
if [ ! -d "$BP_DIR" ]; then
    echo "❌ BP directory not found at: $BP_DIR"
    echo "Please ensure the Borland Pascal BP folder is in your repo"
    exit 1
fi

echo "✅ Found BP directory at: $BP_DIR"

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"
mkdir -p "$OUTPUT_DIR"
mkdir -p "$TEMP_DIR"

# Clean previous build artifacts from output directory
echo "🧹 Cleaning previous build artifacts..."
rm -f "$OUTPUT_DIR"/*.exe "$OUTPUT_DIR"/*.ovr "$OUTPUT_DIR"/*.tpu 2>/dev/null || true

# Create DOSBox configuration file with WSL-specific settings
echo "⚙️ Creating DOSBox configuration..."
if [ "$IS_WSL" = true ]; then
    # WSL configuration - dummy/headless mode
    cat > "$DOSBOX_CONF" << EOF
[sdl]
output=dummy
windowresolution=640x480
autolock=false

[dosbox]
machine=hercules
memsize=16

[cpu]
core=auto
cputype=auto
cycles=max

[render]
frameskip=10
aspect=false
scaler=none

[video]
vmemsize=1

[autoexec]
# Mount directories
mount c: /tmp/impulse_build
mount d: $SOURCE_DIR
mount e: $OUTPUT_DIR
mount f: $INCLUDE_DIR

# Change to source directory
d:

# Set up environment and run build
set PATH=c:\\bp\\bin;%PATH%
call c:\\build.bat

# Exit DOSBox automatically
exit
EOF
else
    # Native Linux configuration - with graphics
    cat > "$DOSBOX_CONF" << EOF
[sdl]
windowresolution=800x600
output=surface
autolock=false

[dosbox]
machine=svga_s3
memsize=16

[cpu]
core=auto
cputype=auto
cycles=10000

[autoexec]
# Mount directories
mount c: /tmp/impulse_build
mount d: $SOURCE_DIR
mount e: $OUTPUT_DIR
mount f: $INCLUDE_DIR

# Change to source directory
d:

# Set up environment and run build
set PATH=c:\\bp\\bin;%PATH%
call c:\\build.bat

# Exit DOSBox automatically
exit
EOF
fi

# Create build batch file for DOS
echo "📝 Creating DOS build batch file..."
cat > "$BUILD_BATCH" << 'EOF'
@echo off
echo === Impulse 7.1 Compilation in DOSBox ===

REM Check if we're on the C: drive
c:
echo Current drive and directory:
cd

REM Check if Borland Pascal is available
if exist c:\bp\bin\bpc.exe goto bpfound
echo.
echo ERROR: Borland Pascal not found at c:\bp\bin\bpc.exe
echo Checking what's in c:\bp\:
if exist c:\bp dir c:\bp
if exist c:\bp\bin dir c:\bp\bin
echo.
pause
goto end

:bpfound
echo Found Borland Pascal compiler at c:\bp\bin\bpc.exe

REM Change to source directory (mounted as D:)
d:
echo Changed to source directory:
cd

REM Check if imp.pas exists
if exist imp.pas goto impfound
echo.
echo ERROR: imp.pas not found in source directory
echo Contents of source directory:
dir
echo.
pause
goto end

:impfound
echo Found imp.pas in source directory

REM Add Borland Pascal to PATH
set PATH=c:\bp\bin;%PATH%

:step1
REM Step 1: Build with -$G+ -B flags and output to E:\
echo.
echo Step 1: Building with -$G+ -B flags...
bpc -$G+ -B -Uf:\ -Ee:\ imp.pas
if errorlevel 1 goto buildfail
echo Build step completed successfully
goto step2

:buildfail
echo.
echo ERROR: Build step failed
pause
goto end

:step2
REM Step 2: Compile normally with output to E:\
echo.
echo Step 2: Final compilation...
bpc -Uf:\ -Ee:\ imp.pas
if errorlevel 1 goto compfail
echo Compilation completed successfully
goto checkfiles

:compfail
echo.
echo ERROR: Final compilation failed
pause
goto end

:checkfiles
REM Check if output files were created in E:\
echo.
echo Checking for output files in E:\...
e:
if exist imp.exe goto foundexe
echo WARNING: imp.exe not found in E:\
goto checkovr

:foundexe
echo Found imp.exe in output directory

:checkovr
if exist imp.ovr goto foundovr
echo WARNING: imp.ovr not found in E:\
goto checktpu

:foundovr
echo Found imp.ovr in output directory

:checktpu
REM Check for any TPU files in E:\
echo Checking for TPU files in E:\...
for %%f in (*.tpu) do echo Found TPU file: %%f

echo.
echo === Compilation Complete ===
echo Files are located in the output directory (E:\)
goto end

:end
echo.
EOF

# Create temporary directory for DOSBox C: drive and set up BP
TEMP_BUILD_DIR="/tmp/impulse_build"
echo "🔧 Setting up DOSBox C: drive at: $TEMP_BUILD_DIR"
rm -rf "$TEMP_BUILD_DIR"  # Clean any previous setup
mkdir -p "$TEMP_BUILD_DIR"

# Copy Borland Pascal to the DOSBox C: drive
echo "📦 Copying Borland Pascal from repo to DOSBox environment..."
cp -r "$BP_DIR" "$TEMP_BUILD_DIR/bp"
echo "✅ Borland Pascal copied to: $TEMP_BUILD_DIR/bp"

# Copy build batch file to DOSBox C: drive
cp "$BUILD_BATCH" "$TEMP_BUILD_DIR/build.bat"
echo "✅ Build batch file copied to DOSBox C: drive"

echo "🚀 Starting DOSBox compilation..."
echo "💡 Using Borland Pascal from your repo's BP directory"
echo "💡 DOSBox C: drive: $TEMP_BUILD_DIR"

if [ "$IS_WSL" = true ]; then
    echo "💡 WSL detected - using headless DOSBox mode"
    echo ""
    # Try to run DOSBox headless in WSL
    # First, unset DISPLAY to prevent X11 attempts
    unset DISPLAY
    
    # Run DOSBox with minimal output and automatic exit
    timeout 60 dosbox -conf "$DOSBOX_CONF" -exit > /dev/null 2>&1 || {
        echo "⚠️  DOSBox may have had display issues in WSL, but compilation might have succeeded"
        echo "    This is normal in WSL environments"
    }
else
    echo "💡 DOSBox will stay open for troubleshooting - close it manually when done"
    echo ""
    # On native Linux, keep DOSBox open for troubleshooting
    dosbox -conf "$DOSBOX_CONF"
fi

# Wait a moment for file operations to complete
sleep 2

# Check if compilation was successful (DOS creates uppercase filenames)
echo ""
echo "🔍 Checking compilation results..."

if [ -f "$OUTPUT_DIR/IMP.EXE" ] && [ -f "$OUTPUT_DIR/IMP.OVR" ]; then
    echo "✅ Compilation successful!"
    
    # Copy files to build directory
    echo "📋 Copying files to build directory..."
    cp "$OUTPUT_DIR/IMP.EXE" "$BUILD_DIR/IMP.EXE"
    cp "$OUTPUT_DIR/IMP.OVR" "$BUILD_DIR/IMP.OVR"
    
    echo ""
    echo "🎉 Build complete! Files available in build/ directory:"
    ls -la "$BUILD_DIR/"
    
    # Show file sizes
    echo ""
    echo "📊 File sizes:"
    du -h "$BUILD_DIR"/IMP.*
    
else
    echo "❌ Compilation failed or output files not found"
    echo ""
    echo "🔍 Contents of output directory:"
    ls -la "$OUTPUT_DIR/" || echo "Output directory is empty or doesn't exist"
    echo ""
    echo "💡 Troubleshooting tips:"
    echo "   - Check that imp.pas exists in the source directory: $SOURCE_DIR"
    echo "   - Borland Pascal was copied from: $BP_DIR"
    echo "   - DOSBox C: drive location: $TEMP_BUILD_DIR"
    if [ "$IS_WSL" = true ]; then
        echo "   - For WSL debugging, run: dosbox -conf $DOSBOX_CONF"
    else
        echo "   - Run DOSBox manually with: dosbox -conf $DOSBOX_CONF"
    fi
    exit 1
fi

# Cleanup temporary files
echo ""
echo "🧹 Cleaning up temporary files..."
rm -rf "$TEMP_DIR"
rm -rf "$TEMP_BUILD_DIR"

echo ""
echo "=== ✅ Build Script Complete ==="
echo "Your compiled Impulse 7.1 files are ready in the build/ directory!"
