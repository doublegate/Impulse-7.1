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
BUILD_BATCH="$TEMP_DIR/build_impulse_temp.bat"

echo "=== Impulse 7.1 Build Script ==="
echo "Repository: $REPO_DIR"
echo "DOSBox C: drive will be: /tmp/impulse_build"

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

# Create DOSBox configuration file
echo "⚙️ Creating DOSBox configuration..."
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
call c:\\build_impulse.bat

# Exit DOSBox automatically
exit
EOF

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
if not exist c:\bp\bin\bpc.exe (
    echo.
    echo ERROR: Borland Pascal not found at c:\bp\bin\bpc.exe
    echo Checking what's in c:\bp\:
    if exist c:\bp dir c:\bp
    if exist c:\bp\bin dir c:\bp\bin
    echo.
    pause
    exit 1
)

echo Found Borland Pascal compiler at c:\bp\bin\bpc.exe

REM Change to source directory (mounted as D:)
d:
echo Changed to source directory:
cd

REM Check if imp.pas exists
if not exist imp.pas (
    echo.
    echo ERROR: imp.pas not found in source directory
    echo Contents of source directory:
    dir
    echo.
    pause
    exit 1
)

echo Found imp.pas in source directory

REM Add Borland Pascal to PATH
set PATH=c:\bp\bin;%PATH%

REM Step 1: Build with -$G+ -B flags
echo.
echo Step 1: Building with -$G+ -B flags...
bpc -$G+ -B imp.pas
if errorlevel 1 (
    echo.
    echo ERROR: Build step failed
    pause
    exit 1
)
echo Build step completed successfully

REM Step 2: Compile normally  
echo.
echo Step 2: Final compilation...
bpc imp.pas
if errorlevel 1 (
    echo.
    echo ERROR: Final compilation failed
    pause
    exit 1
)
echo Compilation completed successfully

REM Copy output files to the output directory (mounted as E:)
echo.
echo Copying output files...
if exist imp.exe (
    copy imp.exe e:\
    echo Copied imp.exe to output directory
) else (
    echo WARNING: imp.exe not found!
)

if exist imp.ovr (
    copy imp.ovr e:\  
    echo Copied imp.ovr to output directory
) else (
    echo WARNING: imp.ovr not found!
)

REM Copy any TPU files that were created
echo Checking for TPU files...
for %%f in (*.tpu) do (
    if exist %%f (
        copy %%f e:\
        echo Copied %%f to output directory
    )
)

echo.
echo === Compilation Complete ===
echo Check the output directory for your files
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
cp "$BUILD_BATCH" "$TEMP_BUILD_DIR/"

echo "🚀 Starting DOSBox compilation..."
echo "💡 Using Borland Pascal from your repo's BP directory"
echo "💡 DOSBox C: drive: $TEMP_BUILD_DIR"
echo "💡 DOSBox will stay open for troubleshooting - close it manually when done"
echo ""

# Run DOSBox with our configuration (will stay open for troubleshooting)
dosbox -conf "$DOSBOX_CONF"

# Wait a moment for file operations to complete
sleep 2

# Check if compilation was successful
echo ""
echo "🔍 Checking compilation results..."

if [ -f "$OUTPUT_DIR/imp.exe" ] && [ -f "$OUTPUT_DIR/imp.ovr" ]; then
    echo "✅ Compilation successful!"
    
    # Copy files to build directory
    echo "📋 Copying files to build directory..."
    cp "$OUTPUT_DIR/imp.exe" "$BUILD_DIR/"
    cp "$OUTPUT_DIR/imp.ovr" "$BUILD_DIR/"
    
    # Copy any TPU files that were created
    if ls "$OUTPUT_DIR"/*.tpu 1> /dev/null 2>&1; then
        cp "$OUTPUT_DIR"/*.tpu "$BUILD_DIR/"
        echo "✅ Also copied TPU files"
    fi
    
    echo ""
    echo "🎉 Build complete! Files available in build/ directory:"
    ls -la "$BUILD_DIR/"
    
    # Show file sizes
    echo ""
    echo "📊 File sizes:"
    du -h "$BUILD_DIR"/*
    
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
    echo "   - Run DOSBox manually with: dosbox -conf dosbox_build.conf"
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

