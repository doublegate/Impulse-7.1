@echo off
echo making an impulse beta distrobution - %1
echo adding latest executables
copy /y src\imp.exe
copy /y src\imp.ovr
copy /y src\new.nfo

echo making temp directory
mkdir betatmp
copy src\install\install.exe betatmp
copy src\install\unrar.exe betatmp
copy readme. betatmp
copy upgrade. betatmp
copy new.nfo betatmp
copy imp.doc betatmp
copy file_id.diz betatmp
echo moving source code
mkdir c:\tempsrc
move src\*.* c:\tempsrc
deltree /y src

echo making rar file impulse.rar
rar a -r -x@exclude.lst impulse.rar
move impulse.rar betatmp

echo zipping impulse beta - %1
chdir betatmp
pkzip %1
move %1 ..
chdir ..

echo removing temp files
deltree /y betatmp

echo restoring source code
mkdir src
move c:\tempsrc\*.* src
rmdir c:\tempsrc

echo done! created %1 !

