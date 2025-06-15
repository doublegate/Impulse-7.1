#!/bin/bash

# Docker-optimized build script for Impulse BBS
set -e

echo "=== Impulse BBS Docker Build ==="
echo "Pipeline ID: ${CI_PIPELINE_ID:-local}"
echo "Commit: ${CI_COMMIT_SHORT_SHA:-local}"

# Paths in the Docker container
BUILD_DIR="/impulse-build/build"
SOURCE_DIR="/impulse-build/source"
OUTPUT_DIR="/impulse-build/output"
INCLUDE_DIR="/impulse-build/include"
BP_DIR="/impulse-build/BP"

# Create DOSBox configuration for headless operation
cat > dosbox.conf << EOF
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

[mixer]
nosound=true

[midi]
mpu401=none

[sblaster]
sbtype=none

[gus]
gus=false

[speaker]
pcspeaker=false

[autoexec]
# Mount directories
mount c: /tmp/impulse_dosbox
mount d: $SOURCE_DIR
mount e: $OUTPUT_DIR
mount f: $INCLUDE_DIR

# Change to source directory and build
d:
set PATH=c:\\bp\\bin;%PATH%
call c:\\build.bat
exit
EOF

# Create the DOS build batch file
cat > build.bat << 'EOF'
@echo off
echo === Impulse BBS Compilation ===

REM Check if Borland Pascal is available
if exist c:\bp\bin\bpc.exe goto bpfound
echo ERROR: Borland Pascal not found
goto end

:bpfound
echo Found Borland Pascal compiler

REM Check if imp.pas exists
d:
if exist imp.pas goto impfound
echo ERROR: imp.pas not found in source directory
goto end

:impfound
echo Found imp.pas in source directory

REM Step 1: Build with flags
echo Step 1: Building with -$G+ -B flags...
bpc -$G+ -B -Uf:\ -Ee:\ imp.pas
if errorlevel 1 goto buildfail
echo Build step completed
goto step2

:buildfail
echo ERROR: Build step failed
goto end

:step2
echo Step 2: Final compilation...
bpc -Uf:\ -Ee:\ imp.pas
if errorlevel 1 goto compfail
echo Compilation completed
goto end

:compfail
echo ERROR: Final compilation failed

:end
echo Build process finished
EOF

# Clean previous builds
rm -rf "$BUILD_DIR"/*
rm -rf "$OUTPUT_DIR"/*

# Set up DOSBox environment
DOSBOX_DIR="/tmp/impulse_dosbox"
rm -rf "$DOSBOX_DIR"
mkdir -p "$DOSBOX_DIR"

# Copy Borland Pascal to DOSBox C: drive
cp -r "$BP_DIR" "$DOSBOX_DIR/bp"
cp build.bat "$DOSBOX_DIR/"

echo "🚀 Starting headless DOSBox compilation..."

# Use Xvfb for headless DOSBox execution
xvfb-run -a dosbox -conf dosbox.conf -noconsole

echo "🔍 Checking compilation results..."

# Check if compilation was successful
if [ -f "$OUTPUT_DIR/IMP.EXE" ] && [ -f "$OUTPUT_DIR/IMP.OVR" ]; then
    echo "✅ Compilation successful!"
    
    # Copy files to build directory
    cp "$OUTPUT_DIR/IMP.EXE" "$BUILD_DIR/"
    cp "$OUTPUT_DIR/IMP.OVR" "$BUILD_DIR/"
    
    # Create build info file
    cat > "$BUILD_DIR/build-info.txt" << EOF
Impulse BBS 7.1 Build Information
================================
Build Date: $(date)
Pipeline ID: ${CI_PIPELINE_ID:-local}
Commit SHA: ${CI_COMMIT_SHA:-local}
Commit Message: ${CI_COMMIT_MESSAGE:-local build}

Files:
EOF
    
    ls -la "$BUILD_DIR"/*.EXE "$BUILD_DIR"/*.OVR >> "$BUILD_DIR/build-info.txt"
    
    echo ""
    echo "🎉 Build complete! Files:"
    ls -la "$BUILD_DIR"/
    
    echo ""
    echo "📊 File sizes:"
    du -h "$BUILD_DIR"/IMP.*
    
else
    echo "❌ Compilation failed"
    echo "Output directory contents:"
    ls -la "$OUTPUT_DIR/"
    exit 1
fi

echo "=== Docker Build Complete ==="

