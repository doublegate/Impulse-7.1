{$A+,O+,I-,R-,S-,L-,V-}
Unit File3;

Interface

Uses
   Common,   records;

function Searchfordups(completefn:astr):boolean;

Implementation

Uses
   Crt,       Dos,     File0,    File1;

function Searchfordups(completefn:astr):boolean;
var fn:astr;
    bn:integer;
    fbrd:integer;
    abort,next:boolean;

   function cansee(f:ulfrec):boolean;
   begin
      canSee:=((not (notval in f.filestat)) or (aacs(status^.seeunval)));
   end;

   procedure search2(b:integer; fn:astr; filestats:boolean; var abort,next:boolean);
   var f:ulfrec;
       oldboard,pl,rn:integer;
       done: boolean;

   begin
      oldboard:=fileboard;
      if (fileboard<>b) then changefileboard(b);
      loaduboard(b);
      if (fileboard=b) then begin
         recno(fn,pl,rn);
         if (baddlpath) then exit;
         while ((rn<=pl) and (not abort) and (not hangup) and (rn<>0)) do begin
            seek(ulff,rn);
            read(ulff,f);
            searchfordups:=true;
            nrecno(fn,pl,rn);
            done:=false;
            oldboard:=fileboard;
         end;
         close(ulff);
      end;
      fileboard:=oldboard;
      changefileboard(fileboard);
      loaduboard(fileboard);
   end;

begin
   fbrd:=fileboard;
   searchfordups:=false;
   fn:=completefn;
   bn:=1;
   abort:=FALSE;
   next:=FALSE;
   print('Searching for Duplicates.');
   while (not abort) and (bn<=maxulb) and (not hangup) do begin
      search2(bn,fn,FALSE,abort,next);
      inc(bn);
      if (next) then begin
         abort:=FALSE;
         next:=FALSE;
      end;
   end;
   changefileboard(fbrd);
   loaduboard(fbrd);
end;


end.

