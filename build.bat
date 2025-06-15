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
