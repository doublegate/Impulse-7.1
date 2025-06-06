{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit wfcmenu;

interface

uses
   Crt,      Dos,      sysop3,   sysop7,   sysop8,   sysop9,   common5,
   sysop11,  mail2,    mail4,    mail5,    mail6,    misc1,    misc2,
   misc3,    misc5,    Common1,  Common2,  Common3,  Sysop2,   common4,
   sysop1,   records,  conf,     cuser,    multinod, tmpcom,   mmodem,
   msgpack,  myio,     common,   newusers, sys;

procedure wfcmdefine;
procedure wfcmenus(wanthangup:boolean);

implementation

type Twfc = record
   x,
   y,
   n,
   h : byte;
end;
var
  lactive : real;
  blankmenunow : boolean;
  dt,
  ddt,
  lastkeypress : datetimerec;
  wfc : Twfc;

procedure wfcmdefine;
begin
   etoday:=0;
   ptoday:=0;
   ftoday:=0;
   chatt:=0;
   shutupchatcall:=FALSE;
   contlist:=FALSE;
   badfpath:=FALSE;
   cwindowon:=FALSE;
   telluserevent:=0;
   fastlogon:=FALSE;
   fileboard:=1; board:=1;
   readuboard:=-1; readboard:=-1;
   inwfcmenu:=TRUE;
   wantfilename:=FALSE;
   {nopfile:=FALSE;}
   doitt:=FALSE;
   reading_a_msg:=FALSE;
   mailread:=FALSE;
   outcom:=FALSE;
   useron:=FALSE;
   ll:='';
   buf:='';
   hangup:=FALSE;
   usernum:=1;
   chatcall:=FALSE;
   hungup:=FALSE;
   pap:=0;
   lactive:=timer;
   reset(uf);
   if (filesize(uf)>1) then begin
      seek(uf,usernum); {*!*}
      read(uf,thisuser);
      close(uf);
      newcomptables;
      usernum:=1;
      readinzscan;
      thisuser.ac:=+[onekey,ansi,color]
   end
   else begin
      close(uf);
      with thisuser do begin
         linelen:=80; pagelen:=24;
         ac:=[onekey,pause,novice,ansi,color];
      end;
   end;
end;

procedure getcallera(var c:char; var chkcom:boolean);
var rl,
    rl1:real;
    s:astr;
    mdm,
    mdmr,
    mdmc:string;
    ring:boolean;
  procedure getresultcode(rs:astr);
  var i,j:integer;
  begin
   if (pos('CONNECT',rs)<>0) then begin
   if (pos('CONNECT 2400',rs)<>0)  then spd:='2400' else
   if (pos('CONNECT 4800',rs)<>0)  then spd:='4800' else
   if (pos('CONNECT 7200',rs)<>0)  then spd:='7200' else
   if (pos('CONNECT 9600',rs)<>0)  then spd:='9600' else
   if (pos('CONNECT 12000',rs)<>0) then spd:='12000' else
    if (pos('CONNECT 14400',rs)<>0) then spd:='14400' else
    if (pos('CONNECT 16800',rs)<>0) then spd:='19200' else
    if (pos('CONNECT 19200',rs)<>0) then spd:='19200' else
    if (pos('CONNECT 21600',rs)<>0) then spd:='21600' else
    if (pos('CONNECT 24000',rs)<>0) then spd:='24000' else
    if (pos('CONNECT 24600',rs)<>0) then spd:='24600' else
    if (pos('CONNECT 26400',rs)<>0) then spd:='26400' else
    if (pos('CONNECT 28800',rs)<>0) then spd:='28800' else
    if (pos('CONNECT 29333',rs)<>0) then spd:='29333' else
    if (pos('CONNECT 30666',rs)<>0) then spd:='30666' else
    if (pos('CONNECT 31200',rs)<>0) then spd:='31200' else
    if (pos('CONNECT 32000',rs)<>0) then spd:='32000' else
    if (pos('CONNECT 33333',rs)<>0) then spd:='33333' else
    if (pos('CONNECT 33600',rs)<>0) then spd:='33600' else
    if (pos('CONNECT 34666',rs)<>0) then spd:='34666' else
    if (pos('CONNECT 36000',rs)<>0) then spd:='36000' else
    if (pos('CONNECT 37333',rs)<>0) then spd:='37333' else
    if (pos('CONNECT 38400',rs)<>0) then spd:='38400' else
    if (pos('CONNECT 38666',rs)<>0) then spd:='38666' else
    if (pos('CONNECT 40000',rs)<>0) then spd:='40000' else
    if (pos('CONNECT 41333',rs)<>0) then spd:='41333' else
    if (pos('CONNECT 42666',rs)<>0) then spd:='42666' else
    if (pos('CONNECT 44000',rs)<>0) then spd:='44000' else
    if (pos('CONNECT 45333',rs)<>0) then spd:='45333' else
    if (pos('CONNECT 46666',rs)<>0) then spd:='46666' else
    if (pos('CONNECT 48000',rs)<>0) then spd:='48000' else
    if (pos('CONNECT 49333',rs)<>0) then spd:='49333' else
    if (pos('CONNECT 50666',rs)<>0) then spd:='50666' else
    if (pos('CONNECT 52000',rs)<>0) then spd:='52000' else
    if (pos('CONNECT 53333',rs)<>0) then spd:='53333' else
    if (pos('CONNECT 54666',rs)<>0) then spd:='54666' else
    if (pos('CONNECT 56000',rs)<>0) then spd:='56000' else
    if (pos('CONNECT 57333',rs)<>0) then spd:='57333' else
    if (pos('CONNECT 57600',rs)<>0) then spd:='57600' else
    if (pos('CONNECT 58666',rs)<>0) then spd:='58666' else
    if (pos('CONNECT 60000',rs)<>0) then spd:='60000' else
    if (pos('CONNECT 61333',rs)<>0) then spd:='61333' else
    if (pos('CONNECT 62666',rs)<>0) then spd:='62666' else
    if (pos('CONNECT 64000',rs)<>0) then spd:='64000' else
    if (pos('CONNECT 76800',rs)<>0) then spd:='76800' else
    if (pos('CONNECT 115200',rs)<>0) then spd:='115200' else
    if (pos('CONNECT 128000',rs)<>0) then spd:='128000' else
    if (pos('CONNECT TELNET',rs)<>0) then spd:= 'TELNET' else
			spd := '38400';
    chkcom:=true;
{		cDelay(150);}
    exit;
   end;
  end;

  procedure wmb(s:astr);
  begin
    sprint('|07'+s[1]+'|15'+copy(s,2,length(s)-1));
  end;

begin
  ring:=false;
  mdmr:='';
  if (chkcom) then begin
    repeat
    if (recom1(c)) then ;
    mdmr:=mdmr+c;
    if (pos('RING',mdmr)<>0) then begin
      chkcom:=TRUE; rl:=timer;
      ring:=true;
      while (c<>#13) and (abs(rl-timer)<0.2) do begin
        c:=cinkey;
      end;
    end;
    until (ring) or (length(mdmr)>10);
    if (chkcom) then begin
      begin
        com_flush_rx;
        if (answerbaud=0) then outmodemstring(modemr.answer);
      end;
      if (sysopon) then
      {cDelay(50);} com_flush_rx; rl1:=timer; s:=''; rl:=0.0;
      mdmc:='';
      repeat
        chkcom:=FALSE;
        gotoxy(1,1);
        textColor(7);
        if (answerbaud>2) then begin
          spd:=cstr(answerbaud);
          chkcom:=TRUE;
          answerbaud:=0;
        end;
        if (keypressed) then begin
          c:=upcase(readkey);
          if (c='H') then begin
            chkcom:=TRUE;
{            cDelay(200);}
            com_flush_rx;
          end;
          chkcom:=TRUE;
        end;
        c:=cinkey;
        if (rl<>0.0) and (abs(rl-timer)>2.0) and (c=#0) then c:=#13;
        if (c<#32) and (c<>#13) then c:=#0;
        if c<>#0 then
          if c<>#13 then begin
            mdmc:=mdmc+c;
            rl:=timer;
          end else begin
            if (pos('NO CARRIER',mdmc)<>0) then chkcom:=TRUE;
            getresultcode(mdmc);
            rl:=0.0;
          end;
        if (c=#13) then s:='';
        if (abs(timer-rl1)>45.0) then chkcom:=TRUE;
      until chkcom;
      if (abs(timer-rl1)>45.0) then begin c:='X'; end;
    end;
    if (spd<>strings^.locallogin) then incom:=TRUE;
  end;
end;

procedure wfcmenu1;
var  wfcFile : file;
begin
   window(1,1,80,25);
   cursoron(FALSE);
   cls;
   wantout:=TRUE;
   assign(wfcFile,status^.gFilePath+'wfc.bin');
   reset(wfcFile,1);
   blockRead(wfcFile,mem[$B800:0],4000);
   blockRead(wfcFile,wfc,sizeOf(wfc));
   close(wfcFile);
end;


procedure wfcmenus(wanthangup:boolean);
var u:userrec;
    times:integer;
    ltime,
    s,
    wfcmessage:astr;
    rr,
    rl,
    rl1,
    rl2,
    lastinit:real;
    i,
    j,
    k,
    sk,
    rcode:integer;
    c,
    c1,
    c2:char;
    nogoodcmd,
    wfcm,
    phoneoffhook,
    chkcom,
    tdoneafternext,
    oldphoneoffhook,
    sysopfo:boolean;

   procedure i1;
   var s:astr;
       rl,
       rl1:real;
       tryc:integer;
       c,
       isc:char;
       mdm:string;
       done:boolean;
       tries : byte;
   begin
      tries := 0;
      if ((modemr.init<>'') and (answerbaud=0) and (not localioonly)) then begin
         if (not keypressed) then begin
            c:=#0; s:='';
            done:=FALSE;
            tryc:=0;
            rl:=timer;
            while (keypressed) do c:=upCase(readkey);
            repeat
               if tries = MAXTRIES then done := true;
               inc(tries);
               com_set_speed(modemr.waitbaud);
               outmodemstring(modemr.init);
               com_flush_rx;
               rl1:=timer;
               mdm:='';
               gotoxy(8,24);
               repeat
                  if (recom1(c)) then begin
                     c:=upcase(c);
                     mdm:=mdm+c;
                     if (pos('OK',mdm)<>0) then done:=TRUE;
                  end;
               until ((abs(timer-rl1)>2.5) or (done)) or (keypressed);
               if (done) then cDelay(100);
               inc(tryc);
               if (tryc>10) then done:=TRUE;
            until ((done) or (keypressed));
         end;
         while (keypressed) do isc:=readkey;
         com_flush_rx;
         rl1:=timer;
         repeat
            c:=cinkey
         until (abs(timer-rl1)>0.1);
      end;
      phoneoffhook:=FALSE;
      wfcmessage:='';
      lastinit:=timer;
      while (keypressed) do c:=readkey;
      com_flush_rx;
   end;

   procedure takeoffhook;
   begin
      if (not localioonly) then begin
         dophoneoffhook(TRUE);
         phoneoffhook:=TRUE;
         wfcmessage:='Phone off hook';
      end;
   end;

  procedure beephim;
  var rl,
      rl1 : real;
      ch:char;
  begin
    takeoffhook;
    rl:=timer;
    repeat
      sound(1500); cdelay(20);
      sound(1000); cdelay(20);
      sound(800); cdelay(20);
      nosound;
      rl1:=timer;
      while (abs(rl1-timer)<0.9) and (not keypressed) do;
    until (abs(rl-timer)>30.0) or (keypressed);
    if keypressed then ch:=readkey;
    i1;
  end;

  procedure packallbases;
  var b:boolean;
  begin
    cls;
    b:=(pause in thisuser.ac);
    thisuser.ac:=thisuser.ac-[pause];
    doshowpackbases;
    if (b) then thisuser.ac:=thisuser.ac+[pause];
    cls;
    wfcm:=FALSE;
    sysoplog('Packed the message bases');
  end;

  procedure chkevents;
  var i,rcode:integer;
  begin
    if (checkevents(0)<>0) then
      for i:=0 to numevents do begin
        if (checkpreeventtime(i,0)) then
          if (not phoneoffhook) then begin
            takeoffhook;
            wfcmessage:='Phone off hook in preparation for event at '+
                        copy(ctim(events[i]^.exectime),4,5)+':00';
          end;
        if (checkeventtime(i,0)) then
          with events[i]^ do begin
            i1;
            if (busyduring) then takeoffhook;
            cls; write('- '+copy(ctim(exectime),4,5)+':00 - Event: ');
            writeln('"'+description+'"');
            sl1('');
            sl1('[> Ran Event "'+description+'" on '+date+' '+time);
            case etype of
              'D':begin
                    sysopfo:=(textrec(sysopf).mode<>fmclosed);
                    if (sysopfo) then close(sysopf);

                    shelldos(FALSE,execdata,rcode);
                    cursoron(FALSE);
                    if (sysopfo) then append(sysopf);
                    sl1('[> Returned from "'+description+'" on '+date+' '+time);
                    cls;
{                    cdelay(1000);}
                    outmodemstring1(modemr.hangup);
{                    cdelay(300);}
                    i1;
                    wfcm:=FALSE;
                  end;
              'E':begin

                    doneday:=TRUE;
                    elevel:=value(execdata);
                  end;
              'P':begin
                    packallbases;
                    i1;
                  end;
            end;
          end;
      end;
  end;

  procedure closemenu;
  begin
    if (status^.localscreensec) then wantout:=FALSE;
    sysopon:=FALSE; sk:=0;
    wfcm:=FALSE;
  end;

type typeListItems = array[1..20] of string[21];
Var
  list : ^typeListItems;
  Cont,
  NumItems,
  pos,
  first,
  cur:byte;
  MenuC:Char;
  Pause:Boolean;

Procedure InitItems;
begin
  new(list);
  List^[1 ] := '    System Setup    ';
  List^[2 ] := '    Answer Phone    ';
  List^[3 ] := '     Dos Shell      ';
  List^[4 ] := '    Local Login     ';
  List^[5 ] := '    Menu Editor     ';
  List^[6 ] := '    System Info     ';
  List^[7 ] := '     Read Email     ';
  List^[8 ] := '   Read User Mail   ';
  List^[9 ] := '    Write Email     ';
  List^[10] := '    User Editor     ';
  List^[11] := '   Exec TERM.BAT    ';
  List^[12] := '   Message Bases    ';
  List^[13] := '     File Bases     ';
  List^[14] := '   Pack Messages    ';
  List^[15] := '      Off-Hook      ';
  List^[16] := '    Hangup Modem    ';
  List^[17] := '     Protocols      ';
  List^[18] := '   Sysop Loggings   ';
  List^[19] := ' Conference Editor  ';
  List^[20] := '    Quit Impulse    ';
  NumItems := 20;
end;

procedure runoption(item : byte);
var cnfc : char;
begin
   if item in [1..20] then begin
      cursoron(true);
      case item of
         1  : pullconfig;
         2  : chkcom:=TRUE;
         3  : SysopShell(FALSE);
         4  : begin
            oldphoneoffhook:=phoneoffhook;
            if (status^.offhooklocallogon) then takeoffhook;
            gotoxy(2,23);
            sprompt('|07Log on? |08(|15Y|07es|08/|15N|07o|08/|15F|07ast|08) ');
            rl2:=timer;
            while (not keypressed) and (abs(timer-rl2)<30.0) do timeSlice;
            if (keypressed) then c:=upCase(readkey);
            case c of
               'F':begin
                  fastlogon:=TRUE;
                  c:=' ';
               end;
               'Y': c:=' ';
               else c:='@';
            end;
            if (c='@') then begin
               gotoxy(2,23);
               clreol;
               if ((status^.offhooklocallogon) and (not oldphoneoffhook)) then i1;
               nogoodcmd:=TRUE;
            end;
         end;
         5  : menu_edit;
         6  : begin
            cls;
            printf('sysinfo');
            pauseScr;
         end;
         7  : begin
            cls;
            mailr;
         end;
         8  :begin
            cls;
            reset(uf);
            seek(uf,1);
            write(uf,thisuser);
            close(uf);
            write('Read which user''s mail? ');
            finduser(s,i);
            writeln;
            if (i<1) then pausescr
            else begin
               usernum:=i;
               reset(uf);
               seek(uf,i);
               read(uf,thisuser);
               close(uf);
               readinzscan;
               if (thisuser.waiting<>0) then begin
                  cls;
                  macok:=TRUE;
                  readmail;
                  macok:=FALSE;
                  reset(uf);
                  seek(uf,i);
                  write(uf,thisuser);
                  close(uf);
               end else begin
                  writeln('You have no mail waiting.');
                  writeln;
                  pausescr;
               end;
               usernum:=1;
               reset(uf);
               seek(uf,1);
               read(uf,thisuser);
               close(uf);
               readinzscan;
            end;
         end;
         9  :begin
            cls;
            reset(uf);
            seek(uf,1);
            write(uf,thisuser);
            close(uf);
            write('Which user is sending mail? ');
            finduser(s,i);
            writeln;
            if (i<1) then pausescr
            else begin
               usernum:=i;
               reset(uf);
               seek(uf,i);
               read(uf,thisuser);
               close(uf);
               readinzscan;
               macok:=TRUE;
               smail(pynq('Send mass mail? ',false));
               macok:=FALSE;
               nl;
               pausescr;
               usernum:=1;
               reset(uf);
               seek(uf,1);
               read(uf,thisuser);
               close(uf);
               readinzscan;
            end;
         end;
         10 : pulledit;
         11 :if (exist('term.bat')) then begin
            cls;
            textcolor(14);
            writeln('Running TERM.BAT ....');
            sl1('');
            sl1('- Ran terminal package at '+date+' '+time);
            shelldos(FALSE,'term.bat',rcode);
            sl1('[> Returned from "TERM.BAT" at '+date+' '+time);
            chdir(start_dir);
            cls;
            i1;
         end;
         12 : boardedit;
         13 : dlboardedit;
         14 : begin
            cls;
            if (pynq('Do you REALLY want to pack the message bases? ',true)) then doshowpackbases;
         end;
         15 : begin
            if (not phoneoffhook) then takeoffhook
            else i1;
            nogoodcmd:=TRUE;
         end;
         16 : begin
            i1;
            nogoodcmd:=TRUE;
         end;
         17 : exproedit;
         18 : begin
            cls;
            showlogs;
            nl;
            pausescr;
         end;
         19 : begin
            cls;
            sprompt('|U1Edit Message or File Conf? |U9[|U0M/F|U9]:|U3 ');
            onek(cnfc,'MFQ'^M);
            case cnfc of
               'M':if (cso) then editconf(0);
               'F':if (cso) then editconf(1);
            end;
         end;
         20 : begin
            elevel:=exitnormal;
            hangup:=TRUE;
            doneday:=TRUE;
         end;
      end;
      wfcmenu1;
   end;
end;

begin
   CursorOn(False);
   status^.istopwindow:=false;
   if (not status^.localsec) then sysopon:=TRUE;
   getdatetime(lastkeypress);
   blankmenunow:=FALSE;
   wantout:=not status^.localscreensec;
   sk:=0;
   nogoodcmd:=FALSE;
   if (wanthangup) then begin
      dophonehangup(TRUE);
      wanthangup:=FALSE;
   end;
   wfcmdefine;
   wfcmenu1;
   wfcm:=TRUE;
   iport;
   term_ready(TRUE);
   i1;
   makenode(1);
   makenodeon('Nobody',0);
   tdoneafternext:=doneafternext;
   if (not status^.localsec) then sysopon:=TRUE;
   times:=0;
   InitItems;

   First := 1;
   Pos   := 1;

   goxy(wfc.x,wfc.y);
   textattr:=wfc.h;
   cur:=1;
   setC(7);
   write(' '+list^[1]+' ');
   cursoron(false);
   repeat
      inc(times);
      if (tdoneafternext) then wfcmessage:='Not answering any more calls.';
      if (not wfcm) then wfcm:=TRUE;
      if (daynum(date)<>ldate) then
         if (daynum(date)-ldate)=1 then inc(ldate)
         else begin
            cls;
            sprint('|15Date corrupted.');
            halt(1);
         end;
         randomize;
         incom:=FALSE;
         outcom:=FALSE;
         fastlogon:=FALSE;
         hangup:=FALSE;
         hungup:=FALSE;
         irt:='';
         lastname:='';
         macok:=TRUE;
         cfo:=FALSE;
         spd:=strings^.locallogin;
         c:=#0;
         chkcom:=FALSE;
         freetime:=0.0;
         extratime:=0.0;
         choptime:=0.0;
         bread:=0;
         lil:=0;
         cursoron(FALSE);
         textattr := 7;
         if ((not blankmenunow) and (status^.wfcblanktime>0)) then begin
            getdatetime(dt);
            timediff(ddt,lastkeypress,dt);
            if (ddt.min>=status^.wfcblanktime) then begin
               blankmenunow:=TRUE;
               cls;
            end;
         end;
         if (ltime<>time) then begin
            ltime:=time;
            inc(sk);
            if (timer-lastinit>modemr.nocallinittime*60) then begin
               lastinit:=timer;
               if (not phoneoffhook) then i1;
            end;
         end;
         rr:=timer;
         if (rr-lactive<0.0) then rr:=rr+(24.0*60*60);
         rr:=rr-lactive;
         if (sysopon) then
            if (sk=30) and (status^.localsec) then closemenu;
         if (nightly) or (numevents>=1) then chkevents;
         gotoxy(2,24);
         clreol;
         if (wfcmessage<>'') then sprompt('|15þ |07'+wfcmessage+'|15 þ');
         if (tdoneafternext) then begin
            takeoffhook;
            elevel:=exitnormal;
            hangup:=TRUE;
            doneday:=TRUE;
            cls;
         end;

         if (answerbaud>2) then  begin
            c:='A';
            chkcom:=TRUE;
         end;
         if not(blankmenunow) then begin
            For Cont := First To First + 3 do begin
               If (Cont - First + 1 = Pos) Then textattr:= wfc.h
               else textattr := wfc.n;
               GotoXY(wfc.x, wfc.y + Cont-First);
               if textattr=wfc.n then write(' '+List^[Cont]+' ')
               else write(' '+List^[Cont]+' ')
            end;
         end;
         while not(keyPressed) do timeSlice;
         if (keypressed) then begin
            if (blankmenunow) then begin
               blankmenunow:=FALSE;
               window(1,1,80,25);
               wfcmenu1;
               getdatetime(lastkeypress);
            end;
            menuc := readKey;
            if (menuC = #0) then begin
               menuC := readkey;
               If (menuc=#72) Then Begin
                  dec(cur);
                  if (cur<=1) then cur:=1;
                  If (Pos > 1) Then Dec(Pos, 1)
                  Else If (First > 1) Then Dec(First,1);
               end
               else If (menuc=#80) Then Begin
                  inc(cur);
                  if (cur>=NumItems) then cur:=numitems;
                  If (Pos < 4) Then Inc(Pos, 1)
                  Else If (First + 4 <= NumItems) Then Inc(First,1);
               end
               else If (menuc=^M) then runOption(Cur);
               getdatetime(lastkeypress);
            end
            else c := upCase(menuC);
         end;

         if (c<>#0) then begin
            wfcm:=FALSE;
            gotoxy(2,23);
            textColor(7);
            if (not sysopon) then
               case c of
                  'Q': runOption(20);
                  'L':begin
                     sysopon:=checkPw;
                     if (sysopon) then wantout:=TRUE;
                     c:=#1;
                  end;
                  else nogoodcmd:=TRUE;
               end
            else begin
               sk:=0;
               cl(1);
               if (c<>#0) then CursorOn(True);
               case c of
                  'H','+': runOption(16);
                  'A' : runoption(2);
                  'B': if checkPw then runOption(12);
                  'C','/':begin
                     cls;
                     printfile(status^.gfilepath+'user.log');
                     pausescr;
                  end;
                  'D':runOption(3);
                  'E':if checkPw then runOption(17);
                  'F':if checkPw then runOption(13);
                  'P':runOption(14);
                  'X':runOption(18);
                  'M':if checkPw then runOption(7);
                  'O':runOption(15);
                  'S':if checkPw then runOption(1);
                  'Q': runOption(20);
                  'R':if checkPw then runOption(8);
                  'T':runoption(11);
                  'U':if checkPw then runOption(10);
                  'W':if checkPw then runOption(9);
                  'Z':begin
                     cls;
                     zlog;
                     pausescr;
                  end;
                  '#':if checkPw then runOption(5);
                  'L':runOption(4);
                  else nogoodcmd:=TRUE;
                  getdatetime(lastkeypress);
               end;
               if (not nogoodcmd) then getdatetime(lastkeypress);
            end;
            if (not nogoodcmd) then begin
               if (c<>'A') then begin
                  curco:=7;
                  window(1,1,80,25);
                  cls;
                  com_flush_rx;
               end;
               if ((sysopon) and (c<>#1)) then lactive:=timer;
            end
            else begin
               nogoodcmd:=FALSE;
               wfcm:=TRUE;
            end;
         end;
         if (c in ['A'..'Z','+','/','@','#',' ']) then wfcmenu1;
         CursorOn(False);
         if (c<>' ') then c:=#0;
         if (not com_rx_empty) then chkcom:=TRUE;
         if ((c<>#0) or (not com_rx_empty) or (chkcom)) then begin
            if ((not phoneoffhook) and (not localioonly)) then begin
               getcallera(c1,chkcom);
               if (not incom) and ((spd=strings^.locallogin) and (c<>' ')) then begin
                  wfcm:=FALSE;
                  i1;
                  if (quitafterdone) then begin
                     elevel:=exitnormal;
                     hangup:=TRUE;
                     doneday:=TRUE;
                  end;
               end;
            end;
         end;
   until ((incom) or (c=' ') or (doneday));
   etoday:=0;
   ptoday:=0;
   ftoday:=0;
   chatt:=0;
   shutupchatcall:=FALSE;
   contlist:=FALSE;
   badfpath:=FALSE;
   dispose(list);

   if (not doneday) then begin
      window(1,1,80,25);
      cls;
      if spd <> strings^.locallogin then writeLn('['+spd+'] Baud - ');
   end;

   if (incom) then begin
      com_flush_rx;
      term_ready(TRUE);
      outcom:=TRUE;
      com_set_speed(value(spd));
   end
   else begin
      term_ready(FALSE);
      incom:=FALSE;
      outcom:=FALSE;
      wfcm:=FALSE;
   end;
   getdatetime(timeon); ftoday:=0;
   com_flush_rx;
   lil:=0;

   inwfcmenu:=FALSE;
   if (status^.localscreensec) then wantout:=FALSE;
   if (spd=strings^.locallogin) and (not wantout) then wantout:=TRUE;
   if (wantout) then CursorOn(True);
   if ((spd<>strings^.locallogin) AND (spd <> 'TELNET')) then
      case (value(spd) div 100) of
         3 : inc(status^.todayzlog.userbaud[0]);
         12 : inc(status^.todayzlog.userbaud[1]);
         24 : inc(status^.todayzlog.userbaud[2]);
         48 : inc(status^.todayzlog.userbaud[3]);
      else
      inc(status^.todayzlog.userbaud[4]);
   end;
   savesystat;
   chatt:=0;
end;

end.
