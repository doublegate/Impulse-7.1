@echo off
copy /y imp.exe ..
copy /y imp.ovr ..
copy /y new.nfo ..
copy /y todo.nfo ..
cd ..
imp %1 %2 %3 %4 %5
cd src
exit
