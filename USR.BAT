@echo off
copy /y imp.exe c:\sr
copy /y imp.ovr c:\sr
cd\sr
imp %1 %2 %3 %4 %5
cd\imp\src
exit
