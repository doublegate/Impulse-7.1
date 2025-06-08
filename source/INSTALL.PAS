{$M 30000,0,0}
program impulse_install;

uses records,crt,dos;

const SCREEN_SIZE=1440;
      INSTALL_VER='1.0';

procedure drawScreen;
var f : file;
begin
   assign(f,'install.exe');
   reset(f,1);
   seek(f,fileSize(f)-SCREEN_SIZE);
   blockread(f,mem[$B800:0],SCREEN_SIZE);
   close(f);
end;

procedure screenWrite(s : string);
begin
   gotoxy(29,8);
   clrEol;
   gotoxy(54-(length(s) div 2),8);
   write(s);
end;

function direxist(d:string): boolean;
var
   f   : file;
   attr: word;
begin
   d:= d + '\.';
   assign(f,d);
   getfattr(f,attr);
   direxist := ((attr and directory)=directory);
end;

procedure unPack(dir : string);
begin
   swapVectors;
   Exec(GetEnv('COMSPEC'), '/C unrar.exe x -r impulse.rar '+dir+'>nul');
   swapVectors;
end;

procedure update(dir : string);
var sr : file of systatrec;
    srec : systatrec;
begin
   assign(sr,dir+'\status.dat');
   reset(sr);
   read(sr,srec);
   with srec do begin
        gFilePath := dir + '\DATA\';
        aFilePath := dir + '\ANSI\';
        menuPath := dir + '\MENU\';
        trapPath := dir + '\TRAP\';
        msgPath := dir + '\MSGS\';
        tFilePath := dir + '\SCRIPT\';
        tempPath := dir + '\TEMP\';
        swapPath := dir + '\SWAP\';
   end;
   reset(sr);
   write(sr,srec);
   close(sr);
end;

var dir : string;

begin
   clrScr;
   drawScreen;
   textAttr := 15;
   gotoxy(40,1);
   Write(progName+' v.'+ver+' - Install v.'+INSTALL_VER);
   screenWrite('Getting directory...');
   gotoxy(31,4);
   writeLn('Enter the directory to install impulse to: ');
   gotoxy(31,5);
   readln(dir);
   if dir = '' then begin
      screenWrite('Error! No directory entered.');
      gotoxy(1,11);
      halt;
   end;
   if (dir[length(dir)] = '\') then begin
      dir[length(dir)] := #0;
      dec(dir[0]);
   end;
   if (not direxist(dir)) then begin
      screenWrite('Creating directory... '+dir);
      {$I-} mkdir(dir); {$I+}
      if ioResult <> 0 then begin
         screenWrite('Can not make directory '+dir);
         gotoxy(1,11);
         halt(ioResult);
      end;
   end;
   screenWrite('Unpacking impulse...');
   unpack(dir);
   screenWrite('Updating STATUS.DAT...');
   update(dir);
   screenWrite('Changing to impulse directory...');
   chdir(dir);
   screenWrite('Impulse has been succesfully installed!');
   gotoxy(1,11);
end.