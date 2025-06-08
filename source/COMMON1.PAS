{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit common1;

interface

uses
  Crt, Dos, records,
  myio, tmpcom;

procedure splitscreenchat;
function checkpw:boolean;
procedure newcomptables;
procedure cline(var s:string; dd:string);
procedure pausescr;
procedure wait(b:boolean);
procedure inittrapfile;
procedure chatfile(b:boolean);
procedure local_input1(var i:string; ml:integer; tf:boolean);
procedure local_input(var i:string; ml:integer);
procedure local_inputl(var i:string; ml:integer);
procedure local_onek(var c:char; ch:string);
function chinkey:char;
procedure inli1(var s:string);
procedure chat;
procedure sysopshell(takeuser:boolean);
procedure globat(i:integer);
procedure exiterrorlevel;
procedure readinzscan;
procedure savezscanr;
procedure redrawforansi;

implementation

uses
  common5,common, common2, common3, file1;

function checkpw:boolean;
var s:string[20];
begin
   checkpw:=TRUE;
   if status^.sysoppw <> '' then begin
      sprompt(strings^.sysoppass);
      echo:=FALSE;
      input(s,20);
      echo:=TRUE;
      if (s<>status^.sysoppw) then begin
         checkpw:=FALSE;
         if (incom) and (s<>'') then sysoplog('* Wrong SysOp Password = '+s+' *');
      end;
   end;
end;

procedure newcomptables;
var savuboard:ulrec;
    savboard:boardrec;
    savreaduboard,savreadboard,i,j:integer;
    bfo,ulfo,done:boolean;
begin
  for i:=0 to 1 do for j:=0 to maxuboards do ccuboards[i][j]:=j;
  for i:=0 to 1 do for j:=1 to maxboards do ccboards[i][j]:=j;
  if (status^.compressbases) then begin

    savuboard:=memuboard; savreaduboard:=readuboard;
    savboard:=memboard; savreadboard:=readboard;

    bfo:=(filerec(bf).mode<>fmclosed);
    ulfo:=(filerec(ulf).mode<>fmclosed);
    if (not bfo) then reset(bf);
    if (not ulfo) then reset(ulf);

    seek(ulf,0); i:=0; j:=0; done:=FALSE;
    while ((not done) and (i<=maxuboards)) do begin
      {$I-} read(ulf,memuboard); {$I+}
      done:=(ioresult<>0);


      if (not done) then
        if (i>maxulb) then begin
          ccuboards[0][i]:=maxuboards+1;
          ccuboards[1][i]:=maxuboards+1;
        end else
          if (aacs(memuboard.acs)) then begin
            ccuboards[1][i]:=j; ccuboards[0][j]:=i;
            inc(j);
          end;
      inc(i);
    end;
{    seek(ulf,loaduboard); read(ulf,memuboard);}
    if (maxulb<maxuboards) then begin
      ccuboards[1][maxulb+1]:=j;
      ccuboards[0][j]:=maxulb+1;
    end;

    seek(bf,0); i:=1; j:=1; done:=FALSE;
    while ((not done) and (i<=maxboards)) do begin
      {$I-} read(bf,memboard); {$I+}
      done:=(ioresult<>0);

      if (not done) then
        if (i>numboards) then begin
          ccboards[0][i]:=maxboards+1;
          ccboards[1][i]:=maxboards+1;
        end else
          if (mbaseac(i)) then begin
            ccboards[1][i]:=j; ccboards[0][j]:=i;
            inc(j);
          end;
      inc(i);
    end;
{    seek(bf,loadboard); read(bf,memboard);}
    if (numboards<maxboards) then begin
      ccboards[1][numboards+1]:=j;
      ccboards[0][j]:=maxboards+1;
    end;

    if (not bfo) then close(bf);
    if (not ulfo) then close(ulf);

    memuboard:=savuboard; readuboard:=savreaduboard;
    memboard:=savboard; readboard:=savreadboard;

  end;
end;

procedure cline(var s:string; dd:string);
var sx,sy,sz:byte;
    savwindowon:boolean;
begin
   sx:=wherex;
   sy:=wherey;
   sz:=textattr;
   savwindowon:=cwindowon;
   if (not cwindowon) then begin
      cwindowon:=TRUE;
      schangewindow(TRUE,1);
   end;
   commandline('');
   window(1,1,80,25);
   gotoXY(2,25);
   textColor(15);
   textbackground(1);
   clrEol;
   write(dd+' ');
   local_inputl(s,78-wherex);
   inuserwindow;
   gotoxy(sx,sy);
   textattr:=sz;
   if (not savwindowon) then sclearwindow;
end;

procedure pausescr;
var x:integer;
    str : string;
    c:char;
begin
   if (not(ch)) then begin
      x:=pipeLen(strings^.pause);
      sprompt(strings^.pause);
      lil:=0;
      getkey(c);
      if ((okansi) and (not hangup)) then begin
         goxy(wherex-x,wherey);
         prompt(expand(' ',x));
         goxy(whereX-x,wherey);
      end
      else begin
         str := '';
         prompt(expand(^H' '^H,x));
      end;
   end;
end;

procedure wait(b:boolean);
const lastc:byte=0;
begin
   if (b) then begin
      lastc:=curco;
      sprompt(strings^.wait)
   end
   else begin
      prompt(expand(^H+' '+^H,pipeLen(strings^.wait)));
      setc(0);
  end;
end;


procedure inittrapfile;
begin
   if (status^.globaltrap) or (thisuser.trapactivity) then trapping:=TRUE
   else trapping:=FALSE;
   if (trapping) then begin
      if (thisuser.trapseperate) then
         assign(trapfile,status^.trappath+'trap'+cstr(usernum)+'.msg')
      else assign(trapfile,status^.trappath+'trap.msg');
      {$I-} append(trapfile); {$I+}
      if (ioresult<>0) then begin
         rewrite(trapfile);
         writeln(trapfile);
      end;
      writeln(trapfile,'***** User Audit - '+nam+' on at '+date+' '+time+' *****');
   end;
end;

procedure chatfile(b:boolean);
var bf:file of byte;
    s:string[91];
    cr:boolean;
begin
   s:='chat';
   if (thisuser.chatseperate) then s:=s+cstr(usernum);
   s:=status^.trappath+s+'.msg';
   if (not b) then begin
      if (cfo) then begin
         commandline('Chat Capture OFF (Recorded in "'+s+'")');
         cfo:=FALSE;
         if (textrec(cf).mode<>fmclosed) then close(cf);
      end;
   end
   else begin
      cfo:=TRUE;
      if (textrec(cf).mode=fmoutput) then close(cf);
      assign(cf,s);
      assign(bf,s);
      cr:=FALSE;
      {$I-} reset(cf); {$I+}
      if (ioresult<>0) then rewrite(cf)
      else append(cf);
      writeln(cf,^M^J^M^J+dat+^M^J+'Recorded with user: '+nam+^M^J+'------------------------------------'+^M^J);
      commandline('Chat Capture ON ("'+s+'")');
   end;
end;

procedure local_input1(var i:string; ml:integer; tf:boolean);
var r:real;
    cp:integer;
    cc:char;
begin
  cp:=1;
  repeat
    cc:=readkey;
    if (not tf) then cc:=upcase(cc);
    if (cc in [#32..#255]) then
      if (cp<=ml) then begin
        i[cp]:=cc;
        inc(cp);
        write(cc);
      end
      else
    else
      case cc of
        ^H:if (cp>1) then begin
            cc:=^H;
            write(^H' '^H);
            dec(cp);
          end;
    ^U,^X:while (cp<>1) do begin
            dec(cp);
            write(^H' '^H);
          end;
      end;
  until (cc in [^M,^N]);
  i[0]:=chr(cp-1);
  if (wherey<=hi(windmax)-hi(windmin)) then writeln;
end;

procedure local_input(var i:string; ml:integer);  (* Input uppercase only *)
begin
   local_input1(i,ml,FALSE);
end;

procedure local_inputl(var i:string; ml:integer);   (* Input lower & upper case *)
begin
   local_input1(i,ml,TRUE);
end;

procedure local_onek(var c:char; ch:string);                    (* 1 key input *)
begin
   repeat
      c:=upcase(readkey)
   until (pos(c,ch)>0);
   writeln(c);
end;

function chinkey:char;
var c:char;
begin
   c:=#0;
   chinkey:=#0;
   {while (not keypressed) and (com_rx_empty) do TimeSlice;}
   if (keypressed) then begin
      c:=readkey;
      cl(status^.sysopcolor);
      wcolor:=TRUE;
      if (c=#0) then if (keypressed) then begin
         c:=readkey;
         sysopKeys(c);
         if (fuckedUP) then pauseScr;
         if (c=#68) then c:=#1
         else c:=#0;
         if (buf<>'') then begin
            c:=buf[1];
            buf:=copy(buf,2,length(buf)-1);
         end;
      end;
      chinkey:=c;
   end
   else if ((not com_rx_empty) and (incom)) then begin
      c:=cinkey;
      cl(status^.usercolor);
      wcolor:=FALSE;
      chinkey:=c;
   end;
end;

procedure inli1(var s:string);             (* Input routine for chat *)
var cv,cc,cp,g,i,j:integer;
    c,c1:char;
begin
  cp:=1;
  s:='';
  if (ll<>'') then begin
    prompt(ll);
    s:=ll; ll:='';
    cp:=length(s)+1;
  end;
  repeat
    getkey(c); checkhangup;
    case ord(c) of
      32..255:if (cp<79) then begin
                s[cp]:=c; pap:=cp; inc(cp);
                outkey(c);
                if (trapping) then write(trapfile,c);
              end;
      16:if okansi then begin
           getkey(c1);
           cl(ord(c1)-48);
         end;
      27:if (cp<79) then begin
           s[cp]:=c; inc(cp);
           outkey(c);
           if (trapping) then write(trapfile,c);
         end;
      8:if (cp>1) then begin
          dec(cp); pap:=cp;
          prompt(^H' '^H);
        end;
      24:begin
           for cv:=1 to cp-1 do prompt(^H' '^H);
           cp:=1;
           pap:=0;
         end;
       7:if (outcom) then sendcom1(^G);
      23:if cp>1 then
           repeat
             dec(cp); pap:=cp;
             prompt(^H' '^H);
           until (cp=1) or (s[cp]=' ');
       9:begin
           cv:=5-(cp mod 5);
           if (cp+cv<79) then
             for cc:=1 to cv do begin
               s[cp]:=' ';
               inc(cp); pap:=cp;
               prompt(' ');
             end;
         end;
  end;
  until ((c=^M) or (cp=79) or (hangup) or (not ch));
  if (not ch) then begin c:=#13; ch:=FALSE; end;
  s[0]:=chr(cp-1);
  if (c<>^M) then begin
    cv:=cp-1;
    while (cv>0) and (s[cv]<>' ') and (s[cv]<>^H) do dec(cv);
    if (cv>(cp div 2)) and (cv<>cp-1) then begin
      ll:=copy(s,cv+1,cp-cv);
      for cc:=cp-2 downto cv do prompt(^H);
      for cc:=cp-2 downto cv do prompt(' ');
      s[0]:=chr(cv-1);
    end;
  end;
  if (wcolor) then j:=1 else j:=2;
  nl;
end;

procedure splitscreenchat;
var chatstart,chatend,tchatted:datetimerec;
    s,xx,verstr:string;
    t1:real;
    i,savpap:integer;
    c:char;
    sysopbuffer,userbuffer:array[1..12] of string[80];
    sysopx,sysopy,userx,usery:byte;
    savecho,savprintingfile,half,savpause,savwin:boolean;
    lastspace,count:byte;

  procedure switchsysop;
  begin
    goxy(sysopx,sysopy);
    cl(status^.sysopcolor);
    half:=FALSE;
  end;

  procedure switchuser;
  begin
    goxy(userx,usery+12);
    cl(status^.usercolor);
    half:=TRUE;
  end;

  procedure handlesysopkey(thekey:char);
  var lastspace,count:byte;
      counti:integer;
      lastword,hsks:string[80];
    procedure movesysopup;
    var counter,counter1:integer;
    begin
      sysopy:=6;
      for counter:=1 to 6 do begin
	sysopbuffer[counter]:=sysopbuffer[counter+6];
      end;
      goxy(1,1);
      for counter:=1 to 5 do begin
	sprompt(sysopbuffer[counter]);
	for counter1:=length(sysopbuffer[counter])+1 to 80 do begin
	  sprompt(' ');
	end;
      end;
      for counter:=1 to 480 do begin
	sprompt(' ');
      end;
      switchsysop;
    end;
  begin
    xx:='';
    sysopbuffer[sysopy]:=sysopbuffer[sysopy]+thekey;
    sysopx:=sysopx+1;
    if(thekey=#13) then begin
      sysopbuffer[sysopy]:=copy(sysopbuffer[sysopy],1,sysopx-2);
      xx:=sysopbuffer[sysopy];
      sysopx:=1;
      sysopy:=sysopy+1;
      sysopbuffer[sysopy]:='';
      sprompt(#10);
      if(sysopy>11) then begin
	thekey:=#0;
	sprompt(#13);
	movesysopup;
      end;
    end;
    if(thekey=#8) then begin
      if(sysopx>1) then begin
	sysopbuffer[sysopy]:=copy(sysopbuffer[sysopy],1,sysopx-3);
	sysopx:=sysopx-2;
	sprompt(#8+' ');
      end else begin
	thekey:=#0;
      end;
    end;
    if(sysopx>80) then begin
      lastspace:=80;
      for count:=1 to 80 do begin
	if(sysopbuffer[sysopy][count]=' ') then lastspace:=count
      end;
      lastword:=copy(sysopbuffer[sysopy],lastspace+1,80-lastspace-1);
      sysopbuffer[sysopy]:=copy(sysopbuffer[sysopy],1,lastspace);
      xx:=sysopbuffer[sysopy];
      if(thekey<>' ') then begin
	sysopx:=80-lastspace+1;
	sysopy:=sysopy+1;
	sysopbuffer[sysopy]:=lastword+thekey;
      end else begin
	sysopx:=1;
	sysopy:=sysopy+1;
	sysopbuffer[sysopy]:='';
      end;
      if(lastspace<79) then begin
	for count:=1 to 79-lastspace do begin
	  sprompt(#8+' '+#8);
	end;
      end;
      sprompt(#13+#10);
      if(sysopy>11) then begin
	movesysopup;
      end;
      goxy(1,sysopy);
      sprompt(lastword);
      if(thekey<>' ') then
	sprompt(thekey);
    end else begin
      if(thekey<>#0) then
	sprompt(thekey);
    end;
   end;

  procedure handleuserkey(theukey:char);
  var lastspace,count:byte;
      counti:integer;
      lastword,hsks:string[80];
    procedure moveuserup;
    var counter,counter1:integer;
    begin
      usery:=6;
      for counter:=1 to 6 do begin
	userbuffer[counter]:=userbuffer[counter+6];
      end;
      goxy(1,13);
      for counter:=1 to 5 do begin
	sprompt(userbuffer[counter]);
	for counter1:=length(userbuffer[counter])+1 to 80 do begin
	  sprompt(' ');
	end;
      end;
      for counter:=1 to 480 do begin
	sprompt(' ');
      end;
      switchuser;
    end;
  begin
    xx:='';
    userbuffer[usery]:=userbuffer[usery]+theukey;
    userx:=userx+1;
    if(theukey=#13) then begin
      userbuffer[usery]:=copy(userbuffer[usery],1,userx-2);
      xx:=userbuffer[usery];
      userx:=1;
      usery:=usery+1;
      userbuffer[usery]:='';
      sprompt(#10);
      if(usery>11) then begin
	theukey:=#0;
	sprompt(#13);
	moveuserup;
      end;
    end;
    if(theukey=#8) then begin
      if(userx>1) then begin
	userbuffer[usery]:=copy(userbuffer[usery],1,userx-3);
	userx:=userx-2;
	sprompt(#8+' ');
      end else begin
	theukey:=#0;
      end;
    end;
    if(userx>80) then begin
      lastspace:=80;
      for count:=1 to 80 do begin
	if(userbuffer[usery][count]=' ') then lastspace:=count
      end;
      lastword:=copy(userbuffer[usery],lastspace+1,80-lastspace-1);
      userbuffer[usery]:=copy(userbuffer[usery],1,lastspace);
      xx:=userbuffer[usery];
      if(theukey<>' ') then begin
	userx:=80-lastspace+1;
	usery:=usery+1;
	userbuffer[usery]:=lastword+theukey;
      end else begin
	userx:=1;
	usery:=usery+1;
	userbuffer[usery]:='';
      end;
      if(lastspace<79) then begin
	for count:=1 to 79-lastspace do begin
	  sprompt(#8+' '+#8);
	end;
      end;
      sprompt(#13+#10);
      if(usery>11) then begin
	moveuserup;
      end;
      goxy(1,usery+12);
      sprompt(lastword);
      if(theukey<>' ') then
	sprompt(theukey);
    end else begin
      if(theukey<>#0) then
	sprompt(theukey);
    end;
  end;

begin
   nosound;
   getdatetime(chatstart);
   savprintingfile:=printingfile;
   savpap:=pap;
   ch:=TRUE;
   chatcall:=FALSE;
   savecho:=echo;
   echo:=TRUE;
   if (status^.autochatopen) then chatfile(TRUE)
   else if (thisuser.chatauto) then chatfile(TRUE);
   nl;
   nl;
   thisuser.ac:=thisuser.ac-[alert];
   thisuser.ac:=thisuser.ac-[alert];
   savpause:=(pause in thisuser.ac);
   thisuser.ac:=thisuser.ac-[pause];
   savwin:=status^.windowon;
   sprint(strings^.engage);
   cls;
   goxy(1,12);
   sprompt('|U9'+expand('-',80));
   goxy(11,12);
   sprompt('|08[|15'+status^.sysopname+'|08 ]');
   goxy(53,12);
   sprompt('|08[|15 '+caps(thisuser.name)+'|08]');
   goxy(37,12);
   sprompt('|152|08/|15Way');
   goxy(1,1);
   half:=FALSE;
   sysopx:=1;
   sysopy:=1;
   userx:=1;
   usery:=1;
   sysopbuffer[1]:='';
   userbuffer[1]:='';
   xx:='';
   cl(status^.sysopcolor);
   wcolor:=TRUE;
   repeat
      checkhangup;
      xx:='';
      if(keypressed) then begin
         c:=readkey;
         if((c=#0) and (keypressed)) then begin
	    c:=readkey;
	    sysopKeys(c);
         end
         else begin
	   if(half=TRUE) then switchsysop;
	   handlesysopkey(c);
         end;
      end
      else if(incom) then begin
         c:=cinkey;
         if(c<>#0) then begin
	    if(half=FALSE) then switchuser;
	    handleuserkey(c);
         end;
      end;
      if ((cfo) and (xx<>'')) then writeln(cf,xx);
   until ((not ch) or (hangup));
   if(savpause) then thisuser.ac:=thisuser.ac+[pause];
   if(savwin) then status^.windowon:=TRUE;
   cls;
   sprint(strings^.endchat);
   t1:=timer;
   while (abs(t1-timer)<0.6) and (empty) do;
   if (empty) then ch:=FALSE;
   getdatetime(chatend);
   timediff(tchatted,chatstart,chatend);
   freetime:=freetime+dt2r(tchatted);
   s:='Chatted for '+longtim(tchatted);
   if (cfo) then begin
      s:=s+'  -{ Recorded in CHAT';
      if (thisuser.chatseperate) then s:=s+cstr(usernum);
      s:=s+'.MSG }-';
   end;
   sysoplog(s);
   ch:=FALSE;
   echo:=savecho;
   if ((hangup) and (cfo)) then writeln(cf,^M^J'NO CARRIER'^M^J'Carrier lost...'^M^J);
   pap:=savpap;
   printingfile:=savprintingfile;
   commandline('');
   if (cfo) then chatfile(FALSE);
   buf:=^M;
end;


procedure chat;
var chatstart,chatend,tchatted:datetimerec;
    s,xx:string;
    t1:real;
    i,savpap:integer;
    c:char;
    savecho,savprintingfile:boolean;
begin
   nosound;
   getdatetime(chatstart);
   savprintingfile:=printingfile;
   savpap:=pap;
   ch:=TRUE;
   chatcall:=FALSE;
   savecho:=echo;
   echo:=TRUE;
   if (status^.autochatopen) then chatfile(TRUE)
   else if (thisuser.chatauto) then chatfile(TRUE);
   thisuser.ac:=thisuser.ac-[alert];
   sprompt('|NL|NL'+strings^.engage+'|NL|NL');
   cl(status^.sysopcolor);
   wcolor:=TRUE;
   repeat
      inli1(xx);
      if (cfo) then writeln(cf,xx);
   until ((not ch) or (hangup));
   sprint('|NL'+strings^.endchat);
   t1:=timer;
   while (abs(t1-timer)<0.6) and (empty) do;
   if (empty) then ch:=FALSE;
   getdatetime(chatend);
   timediff(tchatted,chatstart,chatend);
   freetime:=freetime+dt2r(tchatted);
   s:='Chatted for '+longtim(tchatted);
   if (cfo) then begin
      s:=s+'  -{ Recorded in CHAT';
      if (thisuser.chatseperate) then s:=s+cstr(usernum);
      s:=s+'.MSG }-';
   end;
   sysoplog(s);
   ch:=FALSE;
   echo:=savecho;
   if ((hangup) and (cfo)) then writeln(cf,^M^J'NO CARRIER'^M^J'Carrier lost...'^M^J);
   pap:=savpap;
   printingfile:=savprintingfile;
   commandline('');
   if (cfo) then chatfile(FALSE);
   buf:=^M;
end;

procedure sysopshell(takeuser:boolean);
var wind:windowrec;
    opath:string;
    t:real;
    sx,sy,ret:integer;
    bb:byte;

   procedure dosc;
   var s:string[6];
       i:integer;
   begin
      s:=^M^J+#27+'[0m';
      for i:=1 to length(s) do dosansi(s[i]);
   end;

begin
   bb:=curco;
   getdir(0,opath);
   t:=timer;
   if (useron) and (incom) then sprompt('|NL|NL'+strings^.shelldos1);
   sx:=wherex;
   sy:=wherey;
   setwindow(wind,1,1,80,25,7,0,0);
   dosc;
   if (not takeuser) then shelldos(FALSE,'',ret)
   else shelldos(FALSE,'remote.bat',ret);
   getdatetime(tim);
   if (useron) then com_flush_rx;
   chdir(opath);
   clrscr;
   removewindow(wind);
   gotoxy(sx,sy);
   if (useron) then begin
      freetime:=freetime+timer-t;
      topscr;
      if (incom) then sprint('|NL'+strings^.shelldos2);
   end;
   setc(bb);
end;

procedure globat(i:integer);
var wind:windowrec;
    s:string;
    t:real;
    xx,yy,z,ret:integer;
begin
   xx:=wherex;
   yy:=wherey;
   z:=textattr;
   getdir(0,s);
   chdir(start_dir);
   savescreen(wind,1,1,80,25);
   t:=timer;
   shelldos(FALSE,'globat'+chr(i+48),ret);
   getdatetime(tim);
   com_flush_rx;
   freetime:=freetime+timer-t;
   removewindow(wind);
   chdir(s);
   if (useron) then topscr;
   gotoxy(xx,yy); textattr:=z;
end;

procedure exiterrorlevel;
var wind:windowrec;
    s:string;
    xx,yy,z,ee:integer;
    c:char;
    re:boolean;
begin
   savescreen(wind,1,1,80,25);
   xx:=wherex;
   yy:=wherey;
   z:=textattr;
   cls;
   writeln('Exit at ERRORLEVEL 255, correct? ');
   write('[A]bort [Y]es [O]ther : ');
   repeat
      c:=upcase(readkey)
   until (c in ['A','Y','O',^M]);
   if (c<>^M) then write(c);
   nl;
   ee:=-1;
   case c of
      'O':begin
         writeln;
         write('Enter ERRORLEVEL (-1 to abort) : ');
         readln(s);
         if (s<>'') then ee:=value(s);
      end;
      'Y':ee:=255;
   end;
   if (ee<>-1) then begin
      writeln;
      write('Generate a run-time error? [Yes] : ');
      repeat
         c:=upcase(readkey)
      until (c in ['Y','N',^M]);
      re:=(c<>'N');
   end;
   removewindow(wind);
   if (useron) then topscr;
   gotoxy(xx,yy);
   textattr:=z;
   if (ee<>-1) then begin
      if (re) then runerror(0)
      else halt(ee);
   end;
end;

procedure readinzscan;
var zscanf:file of zscanrec;
    i,j:integer;
begin
   assign(zscanf,status^.gfilepath+'zscan.dat');
   {$I-} reset(zscanf); {$I+}
   if (ioresult<>0) then rewrite(zscanf);
   if (usernum<filesize(zscanf)) then begin
      seek(zscanf,usernum);
      read(zscanf,zscanr);
      close(zscanf);
      exit;
   end;
   with zscanr do begin
      for i:=1 to maxboards do
         for j:=1 to 6 do mhiread[i][j]:=0;
      mzscan:=[];
      fzscan:=[];
      for i:=1 to maxboards do mzscan:=mzscan+[i];
      for i:=0 to maxuboards do fzscan:=fzscan+[i];
   end;
   seek(zscanf,filesize(zscanf));
   repeat
      write(zscanf,zscanr)
   until (filesize(zscanf)>=usernum+1);
   close(zscanf);
end;

procedure savezscanr;
var zscanf:file of zscanrec;
begin
   assign(zscanf,status^.gfilepath+'zscan.dat');
   {$I-} reset(zscanf); {$I+}
   if (ioresult<>0) then rewrite(zscanf);
   if (usernum<filesize(zscanf)) then begin
      seek(zScanf,userNum);
      write(zscanf,zscanr);
      close(zscanf);
      exit;
   end;
   close(zscanf);
end;

procedure redrawforansi;
begin
   topscr;
   textattr:=7;
   curco:=7;
   if ((outcom) and (okansi)) then pr1(#27+'[0m');
end;

end.
