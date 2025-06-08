uses records;
procedure users;
var uRec : file of userRec;
    nRec : file of smalRec;
    u2   ,
    u    : userRec;
    n    : smalRec;
begin
   writeLn('Cleaning user.lst');
   assign(uRec,'user.lst');
   {$I-} reset(uRec); {$I+}
   if ioResult <> 0 then begin
      writeLn('You must already have a userdata file(user.lst)!');
      halt(ioResult);
   end;
   read(uRec,u);
   read(uRec,u2);
   rewrite(uRec);
   write(uRec,u);
   write(uRec,u2);
   close(uRec);
   writeLn('Done cleaning user.lst');
   writeLn('Cleaning names.lst');
   assign(nRec,'names.lst');
   {$I-} reset(nRec); {$I+}
   if ioResult <> 0 then begin
      writeLn('You must already have a short userdata file(names.lst)!');
      halt(ioResult);
   end;
   read(nRec,n);
   rewrite(nRec);
   write(nRec,n);
   close(nRec);
   writeLn('Done cleaning names.lst');
end;
procedure nuv;
var nF : file of nuvRec;
begin
   writeLn('Cleaning nuv.dat');
   assign(nf,'nuv.dat');
   rewrite(nf);
   close(nf);
   writeLn('Done cleaning nuv.dat');
   writeLn('You should now delete your infoform .ASW file also!');
end;
begin
  users;
  nuv;
end.