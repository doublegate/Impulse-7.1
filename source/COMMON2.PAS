{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit common2;

interface

uses
  Crt,      Dos,      common5,  records,  myio,     common,   common4,
  scrlbk,   tmpcom,   cmd,    strProc;

Var
  f2key:byte;

procedure purgedir(s:astr);
procedure showudstats;
procedure sysopKeys(c:char);
procedure savesystat;
procedure remove_port;
procedure printf(fn:string);
procedure iport;
procedure sendcom1(c:char);
function recom1(var c:char):boolean;
procedure term_ready(ready_status:boolean);
function getwindysize(wind:integer):integer;
procedure inuserwindow;
procedure commandline(s:string);
procedure sclearwindow;
procedure schangewindow(needcreate:boolean; newwind:integer);
procedure topscr;
procedure noansi(s:string);
procedure showscr(fn:string);

implementation

uses
  sysop2, sysop3, common1, common3, ansidrv,ulCheck;



procedure purgedir(s:astr);                {* erase all non-dir files in dir *}
var odir,odir2:astr;
    dirinfo:searchrec;
    f:file;
    att:word;
begin
   s:=fexpand(s);
   while copy(s,length(s),1)='\' do s:=copy(s,1,length(s)-1);
   getdir(0,odir);
   getdir(exdrv(s),odir2);
   chdir(s);
   findfirst('*.*',Archive,dirinfo);
   while (doserror=0) do begin
      assign(f,fexpand(dirinfo.name));
      setfattr(f,$00);           {* remove possible read-only, etc, attributes *}
      erase(f);                  {* erase the $*@( file !!     *}
      findnext(dirinfo);         {* move on to the next one... *}
   end;
   chdir(odir2);
   chdir(odir);
end;

Procedure TopMenu;
Const
  menuchoice : Array [1..6] of common4.menurec =
    ((Text : ' Send File Zmodem '; XPos: 1; YPos : 1; Key : '1'),
    (Text : ' Edit User Info   '; XPos: 1; YPos : 2; Key : '2'),
    (Text : ' Run Config       '; XPos: 1; YPos : 3; Key : '3'),
    (Text : ' Disconnect User  '; XPos: 1; YPos : 4; Key : '4'),
    (Text : ' Run TERM.BAT     '; XPos: 1; YPos : 5; Key : '5'),
    (Text : ' Shell to OS      '; XPos: 1; YPos : 6; Key : '6'));
Var
	tmpx, tmpy : byte;
  Wind, Wind2 : Windowrec;
  Choice : Byte;
	rcode : integer;
  savwantout:boolean;

Procedure SendZmodem;
var s:String;
Begin
  cline(s,'Path+File to send:');
  commandline('');
  if (length(s)>1) and (exist(s)) then
    shelldos(true,'dsz port '+cstr(modemr.comport)+' speed '+spd+
                  ' est len 0 sz -s -m '+s,doserror);
End;

begin
  tmpx:=wherex; tmpy:=wherey;
  wait(true);
	savescreen(wind, 1, 1, 80, 25);
	clrscr;
	choice := 0;
	repeat
		setwindow(wind2,2,2,22,9,9,1,1);
    choice := domenu(menuchoice, 6, 1, $3F, $1F, false, false);
		case choice of
      1:sendzmodem;
      2:Begin
          commandline('User screen OFF ³ User keyboard OFF');
             savwantout:=wantout; wantout:=FALSE;
             wantout:=savwantout;
          if spd <> strings^.locallogin then begin
             outcom:=FALSE; incom:=FALSE;
          end;
          textattr:=7;
          UEdit1;
          commandline('User screen ON ³ User keyboard ON');
          savwantout:=wantout; wantout:=FALSE;
          wantout:=savwantout;
          if spd <> strings^.locallogin then begin
             outcom:=TRUE; incom:=TRUE;
          end;
        End;
      3:Begin
          commandline('User screen OFF ³ User keyboard OFF');
          savwantout:=wantout; wantout:=FALSE;
          wantout:=savwantout;
          if spd <> strings^.locallogin then begin
             outcom:=FALSE; incom:=FALSE;
          end;
          textattr:=7;
          PullConfig;
          commandline('User screen ON ³ User keyboard ON');
          savwantout:=wantout; wantout:=FALSE;
          wantout:=savwantout;
          if spd <> strings^.locallogin then begin
             outcom:=TRUE; incom:=TRUE;
          end;
        End;
      4:Begin Hangup:=True; Choice := 0; End;
      5:ShellDos(False,'TERM.BAT',rcode);
      6:ShellDos(False,'',rcode);
      7:choice := 0;
      end;
      removewindow(wind2);
      until choice = 0;
	removewindow(wind);
	topscr;
  wait(false);
  goxy(tmpx,tmpy);
  buf:=^M;
end;

procedure showudstats;
begin
  commandline('U/L: '+cstr(thisuser.uploads)+'/'+cstr(trunc(thisuser.uk))+'k'+
           ' ³ D/L: '+cstr(thisuser.downloads)+'/'+cstr(trunc(thisuser.dk))+'k'+
           ' File Points:' +cstr(thisuser.filepoints));
end;

procedure sysopKeys(c:char);
var s:string[50];
    cz,i:integer;
    cc:char;
    b,savwantout:boolean;

begin
   case ord(c) of
      ALT_1..ALT_9: globat((ord(c)-ALT_1)+1);
      ALT_0 : cmdShowHeapFree;
      ALT_MINUS : cmdShowStackFree;
      ALT_J : begin
         if (inwfcmenu) then cursoron(true);
         SysopShell(FALSE);
         if (inwfcmenu) then cursoron(false);
      end;
     ALT_EQUAL : doneafternext:=not doneafternext;
     CTRL_PRTSC : exiterrorlevel;
     ARROW_UP,
     ARROW_LEFT,
     ARROW_RIGHT,
     ARROW_DOWN: if (not inwfcmenu) then begin
        buf:=buf+^[+'[';
        case ord(c) of
           ARROW_UP:buf:=buf+'A';
           ARROW_LEFT:buf:=buf+'D';
           ARROW_RIGHT:buf:=buf+'C';
           ARROW_DOWN:buf:=buf+'B';
        end;
     end;
   end;
   if (not inwfcmenu) then begin
      case ord(c) of
         ALT_G: begin
            commandline('Log options - [T]rap activity [C]hat buffering');
            cc:=upcase(readkey);
            with thisuser do case cc of
               'C' : begin
                  commandline('Auto chat buffering - [O]ff [S]eperate [M]ain (CHAT.MSG)');
                  cc:=upcase(readkey);
                  if (cc in ['O','S','M']) then chatfile(FALSE);
                  case cc of
                     'O' : begin
                        chatauto:=FALSE;
                        chatseperate:=FALSE;
                     end;
                     'S' : begin
                        chatauto:=TRUE;
                        chatseperate:=TRUE;
                     end;
                     'M' : begin
                        chatauto:=TRUE;
                        chatseperate:=FALSE;
                     end;
                  end;
                  if (cc in ['S','M']) then chatfile(TRUE);
                  topscr;
               end;
               'T':begin
                  commandline('Activity trapping - [O]ff [S]eperate [M]ain (TRAP.MSG)');
                  cc:=upcase(readkey);
                  if (cc in ['O','S','M']) then if (trapping) then begin
                     close(trapfile);
                     trapping:=FALSE;
                  end;
                  case cc of
                     'O' : begin
                        trapactivity:=FALSE;
                        trapseperate:=FALSE;
                     end;
                     'S' : begin
                        trapactivity:=TRUE;
                        trapseperate:=TRUE;
                     end;
                     'M' : begin
                        trapactivity:=TRUE;
                        trapseperate:=FALSE;
                     end;
                  end;
                  if (cc in ['S','M']) then inittrapfile;
                  topscr;
               end;
            end;
            commandline('');
         end;
         ALT_L : cls;
         ALT_B : if okscrollback then begin
           if not beforeview then addtoscroll(wherey-1);
           if not beforeview then addtoscroll(wherey);
           beforeview:=true;
           viewscrollback;
         end;
         ALT_V : begin
            autovalidate(thisuser,usernum);
            topscr;
            commandline('User Validated.');
         end;
         SHIFT_F1, F1 : if (useron) then TopMenu;
         F2 : begin
            inc(cz);
            if (useron) then
               if (not status^.windowon) or (f2Key <> 0) then begin
                  inc(f2key);
                  status^.windowon:=TRUE;
                  cwindowon:=TRUE;
                  schangewindow(TRUE,1);
                  if (f2key>=3) then f2key:=0;
               end
            else begin
               sclearwindow;
               status^.windowon:=FALSE;
            end;
         end;
         F3: if (not com_carrier) then commandline('No carrier detected!')
         else begin
            if (outcom) then
               if (incom) then incom:=FALSE
               else if (com_carrier) then incom:=TRUE;
            if (incom) then commandline('User keyboard ON.')
            else commandline('User keyboard OFF.');
            com_flush_rx;
         end;
         F4: begin
            chatcall:=FALSE; {chatr:='';}
            thisuser.ac:=thisuser.ac-[alert];
         end;
         F5 : begin
            hangup:=TRUE;
            fuckedUP := true;
         end;
         F6 : if (not inConfig) then PullConfig;
         F7 : begin
            b:=ch;
            ch:=TRUE;
            if (thisUser.tlToday - 5 < 0) then thisUser.tlToday := 0
            else dec(thisuser.tltoday,5);
            ch:=b;
         end;
         F8 : begin
            b:=ch;
            ch:=TRUE;
            if (thisUser.tlToday + 5 > 32767) then thisUser.tlToday := 32767
            else inc(thisuser.tltoday,5);
            ch:=b;
         end;
         F9: if (useron) then begin
            if (thisUser.sl=255) then
               if (realsl<>255) or (realdsl<>255) then begin
                  thisuser.sl:=realsl;
                  thisuser.dsl:=realdsl;
                  if (status^.compressbases) then newcomptables;
                  topscr;
                  commandline('Normal access restored.');
               end
               else begin
                  realsl:=thisUser.sl;
                  realdsl:=thisUser.dsl;
                  thisuser.sl:=255;
                  thisuser.dsl:=255;
                  if (status^.compressbases) then newcomptables;
                  topscr;
                  commandline('Temporary SysOp access granted.');
               end;
         end;
         F10 : if (ch) then ch:=FALSE
            else chat;
         ARROW_HOME: if (ch) then chatfile(not cfo);
         SHIFT_F3: if (outcom) then begin
            savwantout:=wantout;
            wantout:=FALSE;
            wait(TRUE);
            wantout:=savwantout;
            commandline('User screen OFF ³ User keyboard OFF');
            outcom:=FALSE;
            incom:=FALSE;
         end
         else if (not com_carrier) then commandline('No carrier detected!')
         else begin
            commandline('User screen ON ³ User keyboard ON');
            savwantout:=wantout;
            wantout:=FALSE;
            wait(FALSE);
            wantout:=savwantout;
            outcom:=TRUE; incom:=TRUE;
         end;
         SHIFT_F5: begin
            commandline('');
            incom:=FALSE;
            printf('hangup');
            hangup:=TRUE;
         end;
         SHIFT_F7: begin
            wait(TRUE);
            cline(s,'Subtract from user''s time left: ');
            commandline('');
            if (s<>'') then begin
               b:=ch;
               ch:=TRUE;
               if (thisUser.tlToday + value(s) < 0) then thisUser.tlToday := 0
               else dec(thisuser.tltoday,value(s));
               ch:=b;
            end;
            wait(FALSE);
         end;
         SHIFT_F8: begin
            wait(TRUE);
            cline(s,'Add to user''s time left: ');
            commandline('');
            if (s<>'') then begin
               b:=ch;
               ch:=TRUE;
               if (thisUser.tlToday + value(s) > 32767) then thisUser.tlToday := 32767
               else inc(thisuser.tltoday,value(s));
               ch:=b;
            end;
            wait(FALSE);
         end;
         ALT_F3: if (wantout) then begin
            cls;
            writeLn('Text Off');
            wantout:=FALSE;
            cursoron(FALSE);
         end
         else begin
            cls;
            writeln('Text ON');
            wantout:=TRUE;
            cursoron(TRUE);
         end;
         ALT_F5, CTRL_F5 : begin
            randomize;
            s := '';
            for i:=1 to random(255) do s := s + chr(random(255));
            prompt(s);
            if ord(c) = ALT_F5 then hangUP := true;
         end;
         ALT_F9: begin
            repeat
               outkey(^G);
               commandline('Paging user...');
               cDelay(100);
               commandline('');
               checkhangup;
            until ((not empty) or (hangup));
         end;
         ALT_F10: if (ch) then ch:=FALSE
         else begin
            splitscreenchat;
            hangup:=false;
         end;
         CTRL_F4:SysopShell(TRUE);
      end;
   end
   else buf := buf+c;
end;

procedure savesystat;
var systatf:file of systatrec;
begin
   if (tempdirchanged) then status^.temppath:=oldtempdir;
   assign(systatf,start_dir+'\status.dat');
   rewrite(systatf);
   write(systatf,status^);
   close(systatf);
end;

procedure setacch(c:char; b:boolean; var u:userrec);
begin
   if (b) then if (not (tacch(c) in u.ac)) then acch(c,u);
   if (not b) then if (tacch(c) in u.ac) then acch(c,u);
end;

procedure remove_port;
begin
   if (not localioonly) then  com_deinstall;
end;

procedure openport(baud:longint);
begin
   if (not localioonly) then com_set_speed(baud);
end;

procedure iport;
var anyerrors:word;
begin
   if (not localioonly) then begin
      if (com_installed) then com_deinstall;
      com_install(modemr.comport,anyerrors,status^.fossil);
      openport(modemr.waitbaud);
   end;
end;

procedure sendcom1(c:char);
begin
   if (not localioonly) then com_tx(c);
end;

function recom1(var c:char):boolean;
begin
   c:=#0;
   if (localioonly) then recom1:=TRUE
   else begin
      if (not com_rx_empty) then begin
         c:=com_rx;
         recom1:=TRUE;
      end
      else recom1:=FALSE;
   end;
end;

procedure term_ready(ready_status:boolean);
begin
   if (not localioonly) then
      if (ready_status) then com_raise_dtr
      else com_lower_dtr;
end;

function getwindysize(wind:integer):integer;
begin
   case wind of
      0:getwindysize:=0;
      1:getwindysize:=2;{5}
      2:getwindysize:=11;
   end;
end;

procedure inuserwindow;
begin
   if (cwindowon) then
      if (status^.istopwindow) then window(1,getwindysize(status^.curwindow)+1,80,25)
      else window(1,1,80,25-getwindysize(status^.curwindow));
end;

procedure commandline(s:string);
var sx,sy,sz:byte;
begin
   {if (not useron) then exit;}
   sx:=wherex;
   sy:=wherey;
   sz:=textattr;
   textbackground(0);
   window(1,1,80,25);
   gotoxy(1,1);
   textattr:=$17;
   write(s);
   clreol;
   gotoxy(1,1);
   inuserwindow;
   gotoxy(sx,sy);
   textattr:=sz;
   if (cwindowon) then begin
      topscr;
      f2Key := 1;
      schangewindow(TRUE,1);
   end;
end;

procedure clrline(y:integer);
begin
  gotoxy(1,y);
  clrEol;
end;

procedure sclearwindow;
var wind:windowrec;
    i,windysize:integer;
    x,y,z:byte;
begin
  if ((not cwindowon) or (not useron) or (not status^.windowon)) then exit;

  x:=wherex; y:=wherey; z:=textattr;
  windysize:=getwindysize(status^.curwindow);
  cursoron(FALSE);

  window(1,1,80,25); textattr:=7;
  if (not status^.istopwindow) then
    for i:=26-windysize to 25 do clrline(i)
  else begin
    savescreen(wind,1,windysize+1,80,25);
    for i:=1 to windysize do clrline(i);
    movewindow(wind,1,1);
    for i:=26-windysize to 25 do clrline(i);
  end;
  cwindowon:=FALSE;

  gotoxy(x,y); textattr:=z;
  cursoron(TRUE);
end;

procedure schangewindow(needcreate:boolean; newwind:integer);
var wind:windowrec;
    i,j,k,windysize,z:integer;
    sx,sy,sz:byte;
begin
  if (f2key <> 254) and (((not useron) and (not needcreate)) or (not status^.windowon)) then exit;

  sx:=wherex; sy:=wherey; sz:=textattr;
  windysize:=getwindysize(newwind);

  if (not needcreate) then needcreate:=(newwind<>status^.curwindow);
  if ((windysize<>getwindysize(status^.curwindow)) and (cwindowon)) then
    sclearwindow;

  if (not status^.istopwindow) then begin
    cursoron(FALSE);
    if ((needcreate) and (newwind in [1,2])) then begin
      window(1,1,80,25);
      gotoxy(1,25);
      if (sy>25-windysize) then begin
        z:=windysize-(25-sy);
        for i:=1 to z do writeln;
        dec(sy,z);
      end;
    end;
    gotoxy(sx,sy);
  end else begin
    if ((needcreate) and (newwind in [1,2])) then begin
      window(1,1,80,25);
      savescreen(wind,1,1,80,sy);
      if (sy<=25-windysize) then z:=windysize+1 else z:=26-sy;
      if (z>=2) then movewindow(wind,1,z);
      if (z<=4) then sy:=(sy-z)+1;

      if (sy>25-windysize) then sy:=25-windysize;
      if (sy<1) then sy:=1;
    end;
    cursoron(TRUE);
  end;

  status^.curwindow:=newwind;
  if (status^.curwindow<>0) then cwindowon:=TRUE;
  gotoxy(sx,sy); textattr:=sz;
  if (status^.curwindow in [1,2]) then topscr;
end;

procedure blankzlog(var zz:zlogrec);
var i:integer;
begin
  with zz do begin
    date:=' ------ ';
    for i:=0 to 4 do userbaud[i]:=0;
    active:=0; calls:=0; newusers:=0; pubpost:=0; privpost:=0;
    fback:=0; criterr:=0; uploads:=0; downloads:=0; uk:=0; dk:=0;
  end;
end;

function mrnn(i,l:integer):string;
begin
   mrnn:=mrn(cstr(i),l);
end;

procedure topscr;
var zf:file of zlogrec;
    zz:array[1..3] of zlogrec;
    s,spe:string;
    i,j,k,windysize:integer;
    sx,sy,sz:byte;
    c:char;
    where:astr;
begin
  if (f2key <> 254) and ((usernum=0) or (not cwindowon) or (not useron)) then exit;
  if spd=strings^.locallogin then begin
    if length(strings^.locallogin) > 5 then where := 'LOCAL'
    else where:=strings^.locallogin
  end
  else where:=spd;
  cursoron(FALSE);
  sx:=wherex; sy:=wherey; sz:=textattr;
  window(1,1,80,25); windysize:=getwindysize(status^.curwindow);
  textbackground(0);

  window(1,26-windysize,80,25);

  window(1,27-windysize,80,25);

  with thisuser do
    case status^.curwindow of
      1,2:begin
            if (f2Key=254) then
              cwriteat(1,1,mln(#2#1+#3#7+' Imp'+#3#15+'Scroll        '+
                     #3#7+'ESC '+#3#3+'-'+#3#7+' Exits',81));
            if (f2key=1) then
              cwriteat(1,1,mln(#2#1+#3#7+' ('+#3#15+caps(name)+#3#7+') '+#3#3+where+' '+
                     #3#7+'Lvl:'+#3#15+cstr(sl)+#3#7+' Flvl:'+#3#15+cstr(dsl)+
                     #3#7+' Calls:'+#3#15+cstr(loggedon)+#3#7+' Laston:'+#3#15+
                     laston+#3#7+' Note:'+#3#15+note,81));
            if (f2key=2) then
              cwriteat(1,1,mln(#2#1+#3#7+' ('+#3#15+caps(name)+#3#7+') '+#3#3+where+' '+
                     #3#7+'ULkb: '+#3#15+cstr(uk)+#3#7+' DLkb: '+#3#15+cstr(dk)+
                     #3#7+' Uploads: '+#3#15+cstr(uploads)+#3#7+' Downloads: '+#3#15+
                     cstr(downloads)+#3#7+' Points: '+#3#15+cstr(filepoints),81));
            if (f2key=3) then
              cwriteat(1,1,mln(#2#1+#3#7+' ('+#3#15+caps(name)+#3#7+') '+#3#3+where+' '+
                     #3#7+'Posts: '+#3#15+cstr(msgpost)+#3#7+' Emails: '+#3#15+cstr(emailsent)+
                     #3#7+' Feedback: '+#3#15+cstr(feedback)+#3#7+' Email Waiting: '+#3#15+
                     cstr(waiting),81));
           end;
    end;

  inuserwindow;
  gotoxy(sx,sy); textattr:=sz;
  cursoron(TRUE);
end;

procedure showscr(fn:string);
var f:file;
    oaa,
    abort,
    next : boolean;
    c2,
    c,
    k : char;
    mciCount : byte;
    xpoint,
    ypoint : byte;
    tmp:string[2];
    i:integer;
    pub:boolean;
    ansiBuf: ^ansiBufType;
    sread,
    tRead : longint;
    code : byte;
    read : integer;
    x    : integer;
    skip : boolean;

begin
   assign(f,fn);
   if (not exist(fn)) then begin
      noansi(fn);
      exit;
   end;
   new(ansiBuf);
   reset(f,1);
   sread := fileSize(f);
   tread := 0;
   printingfile:=TRUE;
   oaa:=allowabort;
   allowabort:=TRUE;
   next:=FALSE;
   nofile:=false;
   abort:=false;
   randomize;
   while (tRead < sRead) and (not Abort) and (not HangUP) do begin
      BlockRead(F,ansiBuf^,maxBufSize,Read);
      Inc(tRead,Read);
      if tRead > sRead then Dec(Read,tRead-sRead);
      x := 1;
      while x <= Read do begin
         skip := false;
         k := ansiBuf^[x];
         inc(x);
         if k='%' then begin
            c2 := ansiBuf^[x];
            inc(x);
            c := ansiBuf^[x];
            inc(x);
            for mciCount := 1 to numMcis do
               if ((c2) = mciList[mciCount]^.code[1]) AND ((c) = mciList[mciCount]^.code[2]) then begin
                  if mciList[mciCount]^.isAction then mciList[mciCount]^.action
                  else prompt(mciList[mciCount]^.value);
                  skip := true;
               end;
            if not(skip) then begin
            case (c2) of
               '0' : begin
                        case (c) of
                        '0'..'9': setc(value(c));
                        else  prompt('%0'+c);
                     end;
            end;
            '1' : begin
            case (c) of
            '0'..'5' : setc(value('1'+c));
            else prompt('%1'+c);
            end;
         end;
         'A' : begin
            case (c) of
            'D' : prompt(thisuser.street);
            else prompt('%A'+c);
            end;
         end;
         'B' : begin
            case (c) of
            '0'..'9': setc(textAttr+(16*(value(c))));
            'A'..'F': setc(textAttr+(16*(ord(c) - 55)));
            'M' : prompt(cStr(status^.maxTimeBank));
            'N' : prompt(status^.bbsname);
            'T' : prompt(cStr(thisUser.timeBank));
            else prompt('%B'+c);
            end;
         end;
         'C' : begin
            case (c) of
            'F' : prompt(memuboard.name);
            'L' : cls;
            'M' : begin
               pub:=(bread<>-1);
               prompt(aonoff(pub,memboard.name,'Private Mail'));
            end;
            'S' : prompt(cstr(thisUser.loggedon));
            'T' : prompt(cstr(status^.callernum));
            else prompt('%C'+c);
            end;
         end;
         'D' : begin
            case (c) of
            'F' : begin
               case ansiBuf^[x] of
                  'K' : sprompt(strInsertComma(cStrL(diskFree(exdrv(paramStr(0)))div 1024)));
                  'M' : sprompt(strInsertComma(cStrL(diskFree(exdrv(paramStr(0))) div (1024*1024))));
                  else sprompt(strInsertComma(cStrL(diskFree(exdrv(paramStr(0))))));
               end;
               inc(x);
            end;
            'K' : prompt(cstrl(thisuser.dk));
            'N' : prompt(cstr(thisuser.downloads));
            'S' : begin
               case ansiBuf^[x] of
                  'K' : sprompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0)))div 1024)));
                  'M' : sprompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0))) div (1024*1024))));
                  else sprompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0))))));
               end;
               inc(x);
            end;
            'T' : prompt(status^.todayzlog.date);
            else prompt('%D'+c);
            end;
         end;
         'F' : begin
            case (c) of
            'L' : prompt(cstr(thisuser.dsl));
            'P' : prompt(cstr(thisuser.filepoints));
            'T' : prompt(cstr(status^.todayzlog.uploads));
            else prompt('%F'+c);
            end;
         end;
         'H' : begin
            case (c) of
            'M' : prompt(cstr(himsg+1));
            'S' : prompt(hdrStr);
            else prompt('%H'+c);
            end;
         end;
         'L' : begin
            case (c) of
            'C' : prompt(lastcalled);
            'D' : prompt(thisUser.lastOn);
            'O' : prompt(thisUser.cityState);
            'P' : prompt(cstr(modemR.waitBaud));
            'S' : begin
               prompt(expand(ansiBuf^[x],length(hdrStr)+1));
               inc(x);
            end;
            else prompt('%L'+c);
            end;
         end;
         'M' : begin
            case (c) of
            'N' : prompt(cstr(msg_on+1));
            else prompt('%M'+c);
            end;
         end;
         'N' : begin
            case (c) of
            'F' : prompt(cstr(status^.todayzlog.uploads));
            'L' : nl;
            'M' : if (menuactiv) then prompt(menur.menuname[1])
               else prompt('Main');
            'N' : prompt(cstr(thisnode));
            'P' : prompt(cstr(status^.todayzlog.pubPost));
            'T' : prompt(cstr(status^.todayzlog.newusers));
            else prompt('%N'+c);
            end;
         end;
         'O' : begin
            case (c) of
            '1' : sPrompt(changeablemci);
            'S' : sPrompt(osName);
            else prompt('%O'+c);
            end;
         end;
         'P' : begin
            case (c) of
            'A' : pauseScr;
            'N' : prompt(thisuser.ph);
            'S' : prompt(cstr(thisuser.msgPost));
            else prompt('%P'+c);
            end;
         end;
         'R' : begin
            case (c) of
            'B' : setc(random(255) mod 16);
            'C' : setc(random(255));
            'F' : setc(random(255) div 16);
            'N' : prompt(caps(thisUser.realName));
            else prompt('%R'+c);
            end;
         end;
         'S' : begin
            case (c) of
            'P' : prompt(spd);
            'N' : prompt(status^.sysopname);
            else prompt('%S'+c);
            end;
         end;
         'T' : begin
            case (c) of
            'C' : prompt(cstr(status^.todayZlog.calls));
            'L' : prompt(tlef);
            'T' : prompt(cstr(thisUser.tlToday));
            'P' : prompt(cstr(status^.todayZlog.pubPost));
            else prompt('%T'+c);
            end;
         end;
         'U' : begin
            case (c) of
               '0'..'9' : cl(value(c));
            'C' : prompt(thisUser.note);
            'D' : prompt(cstr(thisUser.uploads));
            'K' : prompt(cstr(thisuser.uk));
            'L' : prompt(cstr(thisUser.sl));
            'N' : prompt(caps(thisuser.name));
            'U' : prompt(cstr(small.number));
            else prompt('%U'+c);
            end;
         end;
         'V' : begin
            case (c) of
            'A' : prompt(acronym);
            'D' : prompt(verdate);
            'N' : prompt(ProgName);
            'V' : prompt(ver);
            else prompt('%V'+c);
            end;
         end;
         'X' : begin
            case(c) of
            'Y' : begin
               tmp := '';
               for i:=1 to 2 do begin
                  tmp := tmp+ansiBuf^[x];
                  inc(x);
               end;
               xpoint := value(tmp);
               tmp:='';
               for i:=1 to 2 do begin
                  tmp := tmp+ansiBuf^[x];
                  inc(x);
               end;
               ypoint := value(tmp);
               if xpoint = 0 then xpoint := wherex;
               if ypoint = 0 then ypoint := wherey;
               goxy(xpoint,ypoint);
            end;
            else prompt('%X'+c);
            end;
         end;
         else prompt('%'+c2+c);
         end;
         end;
    end
    else begin
         if (outcom) then pr1(k);
         lpromptc(k);
    end;
    wkey(abort,next);
    end;
   end;
   close(f);
   allowabort:=oaa;
   printingfile:=FALSE;
   ctrljoff:=FALSE;
   cl(1);
   dispose(ansiBuf);
end;

procedure printf(fn:string);
var i:byte;
    s:string;
begin
   fn := status^.afilepath+fn;
   if (pos('.',fn)<>0) then showscr(fn)
   else if exist(fn+'.1') then begin
      randomize;
      repeat
         i:=random(9);
         showscr(fn+'.'+cstr(i));
      until (exist(fn+'.'+cstr(i)));
   end
   else if thisuser.ac=thisuser.ac-[color] then showscr(fn+'.ASC')
   else showscr(fn+'.ANS');
end;

procedure noansi(s:string);
begin
   if (status^.displayansimsg) then begin
      sprint('|12Error has occurred!');
      sprint('|12Display File "'+s+'" does not exist!');
      sprint('|12Please inform sysop.');
      pauseScr;
   end;
end;
end.
