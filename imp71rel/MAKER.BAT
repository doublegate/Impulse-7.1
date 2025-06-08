copy /y *.exe c:\ir
copy /y *.ovr c:\ir
copy /y data\*.* c:\ir\data
copy /y *.txt c:\ir
cd\ir
del *.zip
pkzip -p -r %1
cd\imp


