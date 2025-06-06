{ Misc. Junk }
{$R-,S-,I-,F+,V-,B-,N-,L+,O+ }
unit misc2;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common,   ansidrv,
   common5,  records,  file2,    misc1,    sys;

procedure bulletins(par:astr);
procedure yourinfo;
procedure bbsListView;
procedure bbsListAdd;

implementation


procedure bulletins(par:astr);
var filv:text;
    main,subs,s:astr;
    i:integer;
begin
   nl;
   if (par='') then
      if (status^.bulletprefix='') then par:='bulletin;bullet'
      else par:='bulletin;'+status^.bulletprefix;
   if (pos(';',par)<>0) then begin
      main:=copy(par,1,pos(';',par)-1);
      subs:=copy(par,pos(';',par)+1,length(par)-pos(';',par));
   end
   else begin
      main:=par;
      subs:=par;
   end;
   printf(main);
   if (not nofile) then
      repeat
         i:=8-length(subs);
         if (i<1) then i:=1;
         sprompt(strings^.bulletinline);
         input(s,i);
         s:=sqoutsp(s);
         if (not hangup) then begin
            if (s='') then s:='Q';
            if (s='?') then printf(main);
            if ((s<>'Q') and (s<>'?') and (s<>'')) then printf(subs+s);
         end;
      until ((s='Q') or (hangup));
end;

procedure yourinfo;
begin
   cls;
   printF('userstat');
   pauseScr;
end;

procedure bbsListView;
Var pos:byte;
    RecF:File of bbsrec;
    RecFi:bbsRec;
Begin
   Assign(Recf,status^.gfilepath+'BBSLIST.DAT');
   {$I-} reset(recf); {$I+}
   if (ioResult <> 0) OR (fileSize(recf) = 0) then begin
      Rewrite(RecF);
      prompt('|U7No bbs list is empty!|NL');
      pauseScr;
      close(recF);
      exit;
   End;
   printf('BBSLIST.TOP');
   for pos:=1 to filesize(recf) do Begin
      seek(recf,pos-1);
      read(recf,recFi);
      sysMciSet('BN',recfi.name);
      sysMciSet('SP',recfi.speed);
      sysMciSet('CM',recfi.comment);
      sysMciSet('PH',recfi.phone);
      sysMciSet('SY',recfi.sysop);
      sysMciSet('TY',recfi.soft);
      printf('BBSLIST.MID');
      sysResetMciList;
   end; {eof(recf}
   printf('BBSLIST.BOT');
   pauseScr;
   close(recf);
End; {viewlist}

procedure bbsListAdd;
   Function InputInf(Q:String;Len:Byte):String;
   Var Tmp:String[30];
   Begin
      Sprompt(Q);
      Inputl(tmp,len);
      InputInf:=Tmp;
   End;

var recF : file of bbsRec;
    recFi : bbsrec;

Begin
   Assign(Recf,status^.gfilepath+'BBSLIST.DAT');
   {$I-} reset(recF); {$I+}
   if (ioResult <> 0) then Rewrite(RecF);
   recfi.name:=inputinf(strings^.bbsname,30);
   recfi.speed:=inputinf(strings^.bbsspeed,5);
   recfi.phone:=inputinf(strings^.bbsphone,12);
   recfi.soft:=inputinf(strings^.bbstype,8);
   recfi.sysop:=inputinf(strings^.nuzipcode,10);
   recfi.comment := inputinf(strings^.bbsadd,30);
   if pynq('|U1Save this? ',true) then Begin
      seek(recf,filesize(recf));
      write(recf,recfi);
   End;
   nl;
   close(recf);
End; {addlist}

end.
