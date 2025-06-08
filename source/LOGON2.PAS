{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit logon2;

interface

uses
   Crt,      Dos,      tmpcom,   common5,  records,  mail0,    mail1,
   mail2,    mail3,    mail4,    mail9,    misc1,    misc2,    misc5,
   common2,  common4,  cuser,    common1,  common3,  doors,    menus,
   common,   nuv;

procedure logon;
procedure logoff;
implementation

procedure logon;
var lcallf:file of lcallers;
    lcall:lcallers;
    lng:longint;
    {lcts,hilc,}vna,z,rcode:integer;
    c:char;
    lastinit:boolean;
    nuvfile:file of nuvrec;
    anyyet,votedyet:boolean;
    i:integer;
    thisnuv:nuvrec;

  procedure findchoptime;
  var lng,lng2,lng3:longint;

    procedure onlinetime;
    var dt:datetimerec;
        secs:longint;
    begin
      secs:=trunc(nsl);
      dt.day:=secs div 86400; secs:=secs-(dt.day*86400);
      dt.hour:=secs div 3600; secs:=secs-(dt.hour*3600);
      dt.min:=secs div 60; secs:=secs-(dt.min*60);
      dt.sec:=secs;
      sprint(^G);
      sprint(#3#8+'- '+#3#5+'System event approaching - online time adjusted to:');
      sprint(#3#8+'- '+#3#0+longtim(dt));
      sprint(^G);
    end;

  begin
    if (exteventtime<>0) then
    begin
      lng:=exteventtime;
      if (lng<trunc(nsl/60.0)) then
      begin
        choptime:=(nsl-(lng*60.0))+120.0; onlinetime; exit;
      end;
    end;
    lng:=1; lng2:=trunc(nsl/60);
    if (lng2>180) then lng2:=180;
    while (lng<=lng2) do
    begin
      lng3:=lng*60;
      if (checkevents(lng3)<>0) then
      begin
        choptime:=(nsl-(lng*60.0))+60.0; onlinetime; exit;
      end;
      inc(lng,2);
    end;
  end;

begin
  getdatetime(timeon); mread:=0; extratime:=0.0; freetime:=0.0;
  useron:=TRUE; com_flush_rx;
  logon1st;
  Fconf:=1;
  MConf:=1;
{  if ((thisuser.sl>=100) and (not fastlogon)) then
  begin
    if pynq(strings^.turbologin,false) then fastlogon:=TRUE;
  end;}
  if (thisuser.sl<=status^.newsl) then hangup:=true;
  lastinit:=FALSE;
  assign(lcallf,status^.gfilepath+'laston.dat');
  {$I-} reset(lcallf); {$I+}
  if (ioresult<>0) then
  begin
    lastinit:=TRUE;
    rewrite(lcallf);
    lcall.callernum:=-1;
    for z:=0 to 9 do
    write(lcallf,lcall);
  end;
{  if (status^.lcallinlogon) then begin
    if (cso) then lcts:=10 else lcts:=4;
    lcall.callernum:=0; z:=0; hilc:=9;
    for z:=0 to 9 do
    begin
      seek(lcallf,z);
      read(lcallf,lcall);
      if (lcall.callernum=-1) and (hilc=9) then hilc:=z-1;
    end;
    if (hilc<>-1) then
    begin
      if (not cso) and (hilc>3) then hilc:=3;
      sprint('|U5Last few callers:');
      for z:=hilc downto 0 do
      begin
        seek(lcallf,z);
        read(lcallf,lcall);
        with lcall do
          sprint(#3#3+cstr(callernum)+': '+#3#0+name+
                 #3#2+' #'+#3#4+cstr(number)+#3#3+' from '+#3#0+citystate);
      end;
    end;
    nl;
  end;}
  if ((spd<>strings^.locallogin) or (lastinit)) then
  begin
    for z:=9 downto 1 do
    begin
      seek(lcallf,z-1);
      read(lcallf,lcall);
      seek(lcallf,z);
      write(lcallf,lcall);
    end;
    with lcall do
    begin
      callernum:=status^.callernum; name:=caps(thisuser.name);
      number:=usernum;
      citystate:=thisuser.citystate;
    end;
    seek(lcallf,0);
    write(lcallf,lcall);
  end;
  seek(lcallf,0);
  read(lcallf,lcall);
  lastcalled := lcall.name;
  close(lcallf);

{  if ((not fastlogon) and (not hangup)) then
  begin
    printf('logon');
   if (status^.lcallinlogon) then
     lastcaller;}
{    if (exist(status^.afilepath+'logon.*')) then pausescr;
    printf('sl'+cstr(thisuser.sl));
    printf('dsl'+cstr(thisuser.dsl));
    for c:='A' to 'Z' do
      if (c in thisuser.ar) then printf('arlevel'+c);
    printf('user'+cstr(usernum));}

{    if (exist('logon.bat')) then
    begin
      shelldos(FALSE,process_door('logon.bat @F @L @B @G @T @R'),rcode);
      topscr;
    end;
    nl;
    cl(5);
    cls;
    if (status^.autominlogon) then
    begin
      readamsg;
      pausescr;
    end;
    nl;
  end;}

  begin
    if (thisuser.prompt=0) then begin
    choosaprompt;
    end;
    if (thisuser.citystate='') then
      cstuff(4,1,thisuser);
{    if (thisuser.avadjust=0) then
    begin
      thisuser.avadjust:=1;
      thisuser.ac:=thisuser.ac-[avatar];
    end;}
  end;
  savesystat;

  with thisuser do
  begin
    if ((not fastlogon) and (not hangup)) then begin
{      if (status^.usewfclogo) then begin
         oneliners;
      end;
      if (status^.yourinfoinlogon) then begin
         yourinfo; end;}
      if (daynum(laston)<=daynum(status^.tfiledate)) and
        (daynum(laston)>0) then
      begin
        sprint(#3#5+'There may be new text files available.');
      end;
      vna:=0;

      if (forusr<>0) then
        sprint(#3#7+'Your mail is being forwarded to user #'+cstr(forusr));
      nl;
      topscr;
    end;
  end;

  findchoptime;

 if(thisuser.sl>status^.newsl) then begin  { Can't have unvalidated users voting on themselves }
  assign(nuvfile,status^.gfilepath+'nuv.dat');  { Checks to see if we need to vote on any users }
  {$I-} reset(nuvfile); {$I+}
  if(ioresult=0) then begin
    anyyet:=FALSE;
    while(not eof(nuvfile)) and (not anyyet) do begin
      read(nuvfile,thisnuv);
      if(thisnuv.exists) then begin
        votedyet:=FALSE;
        for i:=1 to thisnuv.totvoted do begin
          if(thisnuv.votedby[i]=usernum) then votedyet:=TRUE;
        end;
        if(not votedyet) then anyyet:=TRUE;
      end;
    end;
    close(nuvfile);
    if (anyyet) then newUserVote;
  end;
 end;


  with thisuser do
  begin
    ac:=ac-[smw];
    if ((alert in ac) and (so)) then chatcall:=TRUE;
    if (waiting<>0) then begin
      if pynq(strings^.readmailq,true) then readmail;
    end;
  end;
{  if ((not fastlogon) and (status^.bullinlogon)) then bulletins('');}

  fastlogon:=FALSE;
end;

procedure logoff;
var ddt,dt:datetimerec;
    i,tt,rcode:integer;
begin
  if ((useron) and (usernum>0)) then
    if (exist('logoff.bat')) then
      shelldos(FALSE,process_door('logoff.bat @F @L @B @G @T @R'),rcode);

  term_ready(FALSE);

  {temp directory cleanup}
    purgedir(status^.temppath+'1\');
    purgedir(status^.temppath+'2\');
    purgedir(status^.temppath+'3\');
  {duh end of it.}

  if ((useron) and (usernum>0)) then
  begin
    slogging:=TRUE;

    if (trapping) then
    begin
      if (hungup) then
      begin
        writeln(trapfile);
        writeln(trapfile,'NO CARRIER');
      end;
      close(trapfile); trapping:=FALSE;
    end;

    getdatetime(dt); timediff(ddt,timeon,dt); tt:=trunc((dt2r(ddt)+30)/60);

    thisuser.laston:=status^.lastdate; inc(thisuser.loggedon);

    (* if not logged in, but logged on *)
    if (realsl<>-1) then thisuser.sl:=realsl;
    if (realdsl<>-1) then thisuser.dsl:=realdsl;

    thisuser.illegal:=0; thisuser.ttimeon:=thisuser.ttimeon+tt;
    if (choptime<>0.0) then inc(thisuser.tltoday,trunc(choptime/60.0));
    thisuser.tltoday:=trunc(nsl/60.0);

    if thisuser.lastmsg=0 then thisuser.lastmsg:=1;
    if thisuser.lastfil=0 then thisuser.lastfil:=1;

    thisuser.lastmsg:=board; thisuser.lastfil:=fileboard;
    reset(uf);
    if ((usernum>=1) and (usernum<=filesize(uf)-1)) then
      begin seek(uf,usernum); write(uf,thisuser); end;
    close(uf);

    if (spd<>strings^.locallogin) then inc(status^.todayzlog.active,tt);
    inc(status^.todayzlog.fback,ftoday);
    inc(status^.todayzlog.privpost,etoday);
    savesystat;

    for i:=1 to hiubatchv do release(ubatchv[i]); {* release dynamic memory *}
    clrscr;
    if (hungup) then sl1(#3#7+'- Hung Up -');
    sl1(#3#4+'Read: '+#3#3+cstr(mread)+#3#4+' / Time on: '+#3#3+cstr(tt));
  end;
end;

end.
