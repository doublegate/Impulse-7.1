@echo off
echo making an impulse beta distrobution - %1
echo savesrc
echo adding latest executables
copy /y src\imp.exe ..
copy /y src\imp.ovr ..
echo moving source code
mkdir c:\tempsrc
move src\*.* c:\tempsrc
deltree /y src
echo raring impulse beta - %1
rar a -r %1
echo restoring source code
mkdir src
move c:\tempsrc\*.* src
rmdir c:\tempsrc
echo done!

