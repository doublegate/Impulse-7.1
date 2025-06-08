@echo off
echo updating superRAD to the latest build!
copy /Y imp.exe c:\superRAD
copy /Y imp.ovr c:\superRAD
copy /Y new.nfo c:\superRAD
echo now loading superRAD
pause
cd\superRAD
imp
