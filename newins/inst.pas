program makestat;

{ Minimal STATUS.DAT creator for Impulse BBS }
{ Only sets the directory paths - matches original installer exactly }

uses records, crt, dos;

var 
  sr: file of systatrec;
  srec: systatrec;
  dir: string;

begin
  clrscr;
  writeln('Impulse BBS STATUS.DAT Creator');
  writeln('===============================');
  writeln;
  
  write('Enter Impulse directory (e.g., C:\IMPULSE): ');
  readln(dir);
  
  if dir = '' then begin
    writeln('Error: No directory specified!');
    halt(1);
  end;
  
  { Remove trailing backslash if present }
  if (dir[length(dir)] = '\') then begin
    dir[length(dir)] := #0;
    dec(dir[0]);
  end;
  
  { Create the main directory if it doesn't exist }
  {$I-}
  chdir(dir);
  {$I+}
  if IOResult <> 0 then begin
    writeln('Directory ', dir, ' does not exist. Creating it...');
    {$I-}
    mkdir(dir);
    {$I+}
    if IOResult <> 0 then begin
      writeln('Error: Cannot create directory ', dir);
      halt(1);
    end;
  end;
  
  { Initialize record with zeros }
  fillchar(srec, sizeof(srec), 0);
  
  { Set only the directory paths (exactly like the original installer) }
  with srec do begin
    gFilePath := dir + '\DATA\';
    aFilePath := dir + '\ANSI\';
    menuPath := dir + '\MENU\';
    trapPath := dir + '\TRAP\';
    msgPath := dir + '\MSGS\';
    tFilePath := dir + '\SCRIPT\';
    tempPath := dir + '\TEMP\';
    swapPath := dir + '\SWAP\';
    
    { Set minimal required string fields to avoid crashes }
    bbsname := 'Impulse BBS';
    sysopname := 'SysOp';
    lastdate := '01/01/25';
  end;
  
  { Write the STATUS.DAT file }
  assign(sr, dir + '\STATUS.DAT');
  rewrite(sr);
  write(sr, srec);
  close(sr);
  
  writeln;
  writeln('STATUS.DAT created successfully!');
  writeln('Directory paths set to:');
  writeln('  DATA: ', srec.gFilePath);
  writeln('  ANSI: ', srec.aFilePath);
  writeln('  MENU: ', srec.menuPath);
  writeln('  TRAP: ', srec.trapPath);
  writeln('  MSGS: ', srec.msgPath);
  writeln('  SCRIPT: ', srec.tFilePath);
  writeln('  TEMP: ', srec.tempPath);
  writeln('  SWAP: ', srec.swapPath);
  writeln;
  writeln('Remember to create these directories before running IMP.EXE!');
end.

