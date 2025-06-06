{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
{ Rumor Section }
Unit Rumors;

Interface

Procedure RandomRumor;
Procedure AddRumor;
Procedure ListRumors;
procedure deleteRumor;

Implementation

uses
   common,   common1,  common2,  common3,  common5,  output,   records,
   strProc,  sys;

var r :rumorRec;
    rmFile: file of rumorRec;

function anyRumors : boolean;
var i : word;
    r2 : rumorRec;
begin
   anyRumors := false;
   reset(rmFile);
   for i := 0 to fileSize(rmFile)-1 do begin
      seek(rmFile,i);
      read(rmFile,r2);
      if r2 <> '' then anyRumors := true;
   end;
   reset(rmFile);
end;

procedure makeRumor;
begin
   rewrite(RmFile);
   r := 'Impulse rules in 98!';
   write(rmFile,r);
   reset(rmFile);
end;

Procedure RandomRumor;
Var max,n:word;
    stop : boolean;
Begin
   stop := false;
   Assign(RmFile, status^.gfilepath+'rumors.dat');
   {$I-} reset(RmFile); {$I+}
   if (ioresult<>0) then makeRumor;
   max := filesize(rmfile)-1;
   if anyRumors then begin
      randomize;
      repeat
         n:=Random(max);
         reset(rmfile);
         seek(rmfile,n);
         read(rmfile,r);
         if r <> '' then begin
            sysMciSet('RR',r);
            sysMciSet('RI',cStr(n));
            printf('rumors.ran');
            stop := true;
            sysResetMciList;
         end;
      until stop;
   end
   else sprint('No rumors exist!');
   Close(RmFile);
end;

Procedure AddRumor;
var i  : word;
    go ,
    added : boolean;
    r2 : rumorRec;

Begin
   added := false;
   go := false;
   Assign(RmFile, status^.gfilepath+'rumors.dat');
   {$I-} reset(RmFile); {$I+}
   if (ioresult<>0) then makeRumor;
   Sprompt(strings^.addrumor);
   inputl(r,70);
   if r <> '' then go := true;
   i := 0;
   while ((i < fileSize(rmFile)) AND (go)) do begin
      seek(rmFile,i);
      read(rmFile,r2);
      if i = fileSize(rmFile)-1 then begin
         seek(rmFile,fileSize(rmFile));
         write(rmFile,r);
         go := false;
         added := true;
      end
      else if r2 = '' then begin
         seek(rmFile,i);
         write(rmFile,r);
         go := false;
         added := true;
      end;
      inc(i);
   end;
   if added then sprint(strings^.rumorentered);
   CLose(RmFile);
   pauseScr;
End;

Procedure ListRumors;
var
   x : word;
Begin
   assign(rmfile,status^.gfilepath+'rumors.dat');
   {$I-} reset(RmFile); {$I+}
   if (ioresult<>0) then makeRumor;
   PrintF('rumors.top');
   if anyRumors then begin
      x := 1;
      while (not(eof(rmFile))) do begin
         read(Rmfile,r);
         if (r <> '') then begin
            sysMciSet('RR',r);
            sysMciSet('RI',cStr(x));
            printf('rumors.mid');
            sysResetMciList;
            inc(x);
         end;
      end;
   end;
   close(RmFile);
   printF('rumors.bot');
   pauseScr;
End;

procedure deleteRumor;
var   x      : word;
      s      : string;
begin
   assign(rmfile,status^.gfilepath+'rumors.dat');
   {$I-} reset(RmFile); {$I+}
   if (ioresult<>0) then makeRumor;
   sprompt('Rumor to delete: ');
   inputl(s,3);
   x := strStringToWord(s)-1;
   r := '';
   if (x >= 0) and (x <= filesize(rmfile)) then begin
      seek(rmfile,x);
      write(rmfile,r);
      sprint('Deleted rumor: '+cstr(x+1));
   end;
   close(rmFile);
   pauseScr;
end;
End.
