{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mail9;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common5,  records,
   common,   timejunk, mail0,    mail4,    common4,  sys;

procedure mbaselist;
procedure mbasechange(var done:boolean; mstr:astr);
procedure readAutoMsg;
procedure writeAutoMsg;
procedure replyAutoMsg;
procedure mbasestats;

implementation

procedure mbaselist;
var b,b2 : integer;
begin
   b:=1;
   printf('msgbase.top');
   while (b <= numBoards) and (not hangup) do begin
      if (mbunhidden in memboard.mbstat) or (mbaseAc(b)) then begin
           b2:=ccboards[1][b];
           sysMciSet('ME',memboard.name);
           sysMciSet('MI',cstr(b2));
           if (b in zscanr.mzscan) then sysMciSet('MS','Yes')
           else sysMciSet('MS','NO');
           case memboard.mbtype of
              0 : sysMciSet('MT','Local');
              1 : sysMciSet('MT','Echo');
              2 : sysMciSet('MT','XMail');
           end;
           printf('msgbase.mid');
           sysResetMciList;
      end;
      inc(b);
   end;
  printf('msgbase.bot');
end;

procedure mbasechange(var done:boolean; mstr:astr);
var s:astr;
    i, oldBoard:integer;
    show:boolean;
begin
   oldBoard := board;
   calconf(1);
   if mstr<>'' then begin
      case mstr[1] of
         '+':begin
                i:=board;
                if (board>=numboards) then i:=0
                else begin
                   repeat
                      inc(i);
                      changeboard(i);
                   until (board=i) or (i>numboards);
                end;
                if (board=1) then lastCommandOvr := true;
                loadboard(i);
             end;
         '-':begin
                i:=board;
                if board<=0 then i:=numboards
                else begin
                   repeat
                      dec(i);
                      changeboard(i);
                   until (board=i) or (i<=0);
                end;
                if (board=1) then lastCommandOvr := true;
                loadboard(i);
             end;
         'L' : begin
            show:=true;
            mBaseList;
         end;
         else begin
            mbaselist;
            changeboard(value(mstr));
            if pos(';',mstr)>0 then begin
               s:=copy(mstr,pos(';',mstr)+1,length(mstr));
               curmenu:=status^.menupath+s+'.mnu';
               newmenutoload:=TRUE;
               done:=TRUE;
            end;
            lastcommandovr:=TRUE;
         end;
      end;
   end
   else begin
      s:='?';
      i:=board;
      while (s='?') and (not hangup) do begin
         sprompt(strings^.batchadd);
         input(s,3);
         i:=value(s);
         if (s=^M) then i:=1;
         if (s='?') then begin
            show:=true;
            mBaseList;
         end;
      end;
      if (i<1) or (i>numboards) then i:=board;
      changeboard(i);
      loadboard(i);
      lastcommandovr:=TRUE;
   end;
   if oldBoard <> board then sprint(strings^.mBaseJoined);
end;

procedure readAutoMsg;
var filv:text;
    s:astr;
    i,j:integer;
begin
   nl;
   assign(filv,status^.afilepath+'auto.msg');
   {$I-} reset(filv); {$I+}
   nofile:=(ioresult<>0);
   j:=0;
   if (nofile) then sprint('No AutoMessage available.')
   else begin
      readln(filv,s);
      case s[1] of
         '@': if (aacs(status^.anonpubread)) then s:=copy(s,2,length(s))+' (Posted Anonymously)'
              else s:='Anonymous';
         '!': if (cso) then s:=copy(s,2,length(s))+' (Posted Anonymously)'
              else s:='Anonymous';
      end;
      sysMciSet('AN',s);
      printf('AUTOREAD.TOP');
      while not(eof(filv)) do begin
         readLn(filv,s);
         sprint(s);
      end;
      printf('AUTOREAD.BOT');
      sysResetMciList;
    close(filv);
  end;
end;

procedure writeAutoMsg;
var filvar:text;
    i,j:integer;
    am:array[1..30] of astr;
    n:astr;
    c:char;
    option:char;
    abort,next:boolean;
begin
   if (ramsg in thisuser.ac) then print('You are restricted from writing automessages.')
   else begin
      abort:=FALSE;
      printf('AUTOWRT');
      i:=0;
      repeat
         cl(3);
         inc(i);
         inputwc(am[i],79);
         option:=upcase(am[i][2]);
      until (((am[i][1]='/') and ((option='S') or (option='A'))) or (i=30) or (hangup));
      if ((am[i][1]='/') and ((option='S') or (option='A'))) then dec(i);
      j:=i;
      if (j<>0) then begin
         n := thisUser.name;
         if (option='S') then begin
            if (aacs(status^.anonpubpost)) then
               if pynq(strings^.postanon,false) then
                  if (realsl=255) then n:='!'+n
                  else n:='@'+n;
            nl;
            assign(filvar,status^.afilepath+'auto.msg');
            {$I-} reset(filvar); {$I+}
            if (ioresult<>0) then assign(filvar,status^.afilepath+'auto.msg');
            rewrite(filvar);
            writeln(filvar,n);
            for i:=1 to j do writeln(filvar,am[i]);
            close(filvar);
            print('Auto-message saved.');
            sysoplog('Changed Auto-message');
         end
         else print(strings^.aborted);
      end
      else print(strings^.aborted);
   end;
end;

procedure replyAutoMsg;
var autof:text;
begin
   nl;
   nofile:=FALSE;
   assign(autof,status^.afilepath+'auto.msg');
   {$I-} reset(autof); {$I+}
   if (ioresult<>0) then print('Nothing to reply to.')
   else begin
      irt:='Your auto-message';
      readln(autof,lastname);
      close(autof);
      if (lastname[1]='@') then
         if (not aacs(status^.anonprivread)) then lastname:='';
      if (lastname[1]='!') and (so) then lastname:='';
      if (lastname='') then print('Can''t reply now.') else autoreply;
   end;
end;

procedure mbasestats;
begin
   with memBoard do begin
      sysMciSet('MI',cStr(ccboards[mConf][board]));
      if (password <> '') then sysMciSet('MW',password)
      else sysMciSet('MW','None');
      sysMciSet('MM',cStr(maxMsgs));
      case anonymous of
         atno : sysMciSet('MA','None');
         atyes : sysMciSet('MA','Allowed');
         atforced : sysMciSet('MA','All');
         atdearabby : sysMciSet('MA','Dear Abby');
         atanyname : sysMciSet('MA','Any Name');
      end;
      sysMciSet('MS',acs);
      sysMciSet('MP',postAcs);
      sysMciSet('MC',mciAcs);
      sysMciSet('MF',fileName+'.BRD');
      sysMciSet('ME',msgPath);
      case mbtype of
         0 : sysMciSet('MT','Local');
         1 : sysMciSet('MT','Echo');
         2 : sysMciSet('MT','XMail');
      end;
      printf('mbstats');
      sysResetMciList;
   end;
end;

end.
