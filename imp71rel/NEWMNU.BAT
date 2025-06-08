@echo off
echo This file will update your .mnu files to the new binary format.
echo Please edit this file so that it updates all the mnu found in your
echo menus directory!.
pause
cls
echo updating menu files!
mnuconv auto.mnu
mnuconv batch.mnu
mnuconv bbslist.mnu
mnuconv email.mnu
mnuconv emailp.mnu
mnuconv file.mnu
mnuconv filep.mnu
mnuconv flogin.mnu
mnuconv matrix.mnu
mnuconv main.mnu
mnuconv msg.mnu
mnuconv online.mnu
mnuconv personal.mnu
mnuconv readp.mnu
mnuconv scanf.mnu
mnuconv scanm.mnu
mnuconv sysop.mnu
copy /y *.mnu *.bak
copy /y *.men *.mnu
echo done!



