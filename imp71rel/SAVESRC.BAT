@echo off
echo backing up impulse source code....
echo deleting old code
del c:\savesrc\impsrc.zip
cd src
echo zipping new code
pkzip c:\savesrc\impsrc.zip
cd ..
echo done!

