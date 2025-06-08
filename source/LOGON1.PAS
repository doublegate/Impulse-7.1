{ Logon Procedure }
{$A+,B+,F+,I+,L+,O+,R-,S+,V-}
unit logon1;

interface

uses
   Crt,      Dos,      common5,  logon2,   newusers, records,  mail0,
   mail1,    mail2,    mail3,    mail4,    misc1,    misc2,    misc5,
   common4,  cuser,    common1,  common2,  common3,  doors,    scrlbk,
   strProc,  menus,    menus2,   common,   multinod;

function getuser:boolean;

implementation

var checkedacs:boolean;

procedure getPWS(var ok:boolean; var tries:integer);
var phone,pw,s:astr;
begin
   ok := true;
   echo := false;
   commandLine('User: '+thisUser.name+' Pass: '+thisUser.pw);
   sprompt(strings^.enterPW);
   inputL(pw,20);
   pw := strAllCaps(pw);
   if pw <> thisUser.pw then begin
      sprint(strings^.wrongPassword);
      inc(tries);
      if (tries>=status^.maxlogontries) then begin
         sprint(strings^.hack);
         hangup:=TRUE;
      end;
      echo := true;
      ok := False;
      exit;
   end;
   if (status^.phonepw) then begin
      sprompt(strings^.phonenum);
      commandline('User: '+thisUser.name+' Phone: "'+thisuser.ph+'"');
      input(phone,4);
      echo := true;
      if phone <> copy(thisuser.ph,9,4) then begin
         sprint(strings^.wrongPhone);
         inc(tries);
         if (tries>=status^.maxlogontries) then begin
            sprint(strings^.hack);
            hangup:=TRUE;
         end;
         ok := false;
         exit;
      end;
   end;
   echo := true;
   if ((aacs(status^.spw) and (not hangup) and (status^.sysopPw <>''))) then begin
      if not (checkPw) then begin
         inc(tries);
         if (tries >= status^.maxLogonTries) then hangUp := True;
         ok := False;
      end;
   end;
   if ((ok) and (status^.shuttlelog) and (thisuser.lockedout)) then begin
      printf(thisuser.lockedfile);
      sysoplog('[*] '+thisuser.name+' --> '+'Attempt to access system when locked out <--');
      hangup:=TRUE;
   end;
end;

{   ok:=TRUE;
   echo:=FALSE;
   commandline('Password - "'+thisuser.pw+'"');
   sprompt(strings^.enterpw);
   inputl(pw,20);
   pw:=strAllCaps(pw);
   if (status^.phonepw) then begin
      sprompt(strings^.phonenum);
      commandline('Phone # - "'+thisuser.ph+'"');
      input(phone,4);
      echo:=TRUE;
   end
   else phone:=(copy(thisuser.ph,9,4));
   echo:=TRUE;
   if ((thisuser.pw<>pw) or (copy(thisuser.ph,9,4)<>phone)) then begin
      if (not hangup) and (usernum<>0) then begin
         s := #3#8+'>>'+#3#1+' Illegal logon attempt! Tried: '+ caps(thisuser.name)+' #'+cstr(usernum);
         if (usernum<>1) then begin
            s:=s+' PW="'+pw+'"';
            if (status^.phonepw) then s:=s+', PH#="'+phone+'"';
         end;
         sl1(s);
      end;
      inc(thisuser.illegal);
      seek(uf,usernum);
      write(uf,thisuser);
      inc(tries);
      if (tries>=status^.maxlogontries) then begin
         sprompt(strings^.hack);
         hangup:=TRUE;
      end;
      ok:=FALSE;
   end;
   if ((aacs(status^.spw) AND (ok) and (not hangup) and (status^.sysopPw <>''))) then begin
      if not (checkPw) then begin
         inc(tries);
         if (tries >= status^.maxLogonTries) then hangUp := True;
         ok := False;
      end;
   end;
   if ((ok) and (status^.shuttlelog) and (thisuser.lockedout)) then begin
      printf(thisuser.lockedfile);
      sysoplog(#3#7+'['+#3#8+'*'+#3#7+'] '+#3#3+thisuser.name+#3#7+' --> '+#3#5+
               'Attempt to access system when locked out'+#3#7+' <--');
      hangup:=TRUE;
   end;
end;}

procedure doshuttle;
var s,cmd,phone,pw,newmenucmd:astr;
    tries,i:integer;
    gotname,loggedon,noneedname,ok,cmdnothid,cmdexists:boolean;
begin
  {*}
   cls;
   printf('premtx');
   loggedon:=FALSE;
   gotname:=FALSE;
   last_menu:='matrix.mnu';
   curmenu:=status^.menupath+last_menu;
   readin;
   loggedon:=FALSE;
   gotname:=FALSE;
   tries:=0;
   chelplevel:=1; {*}
   cmd := '';
   while (not loggedon) and (not hangup) do begin
      cls;
      thisuser.ac:=thisuser.ac+[ansi];
      thisuser.pagelen := status^.pagelen;
      tshuttlelogon:=0;
      mainmenuhandle(cmd);
      if ((not gotname) and (cmd<>'')) then begin
          noneedname:=TRUE;
          i:=0;
          repeat
             fcmd(cmd,i,noc,cmdexists,cmdnothid);
             if (i<>0) then
                if (cmdr[i].cmdkeys<>'OP') or (cmdr[i].cmdkeys<>'O1') or
                (cmdr[i].cmdkeys<>'O2') or (cmdr[i].cmdkeys<>'OA') or
                (cmdr[i].cmdkeys<>'O3') or (cmdr[i].cmdkeys[1]<>'H') then
                   noneedname:=FALSE;
          until (i=0);
          if (not noNeedName) then begin
             sprompt(strings^.enterhandle);
             finduser(s,usernum);
             sprint(s);
             if (usernum>=0) then begin
                reset(uf);
                seek(uf,usernum);
                read(uf,thisuser);
                getpws(ok,tries);
                gotname:=ok;
                if (gotname) then begin
                   readinzscan;
                   useron:=TRUE;
                   schangewindow(TRUE,status^.curwindow); commandline('');
                   sysoplog('Logged on to Matrix Menu as '+caps(thisuser.name)+' #'+ cstr(usernum));
                   if (thisuser.waiting<>0) then begin
                      sclearwindow;
                      if pynq(strings^.readmailq,true) then
                         readmail;
                   end; { thisuser.waiting<>0}
                end; { gotname }
             end else { usernumber >= 0 }
                print('You are not a member of this BBS.');
          end; { not(noneedname) }
      end; {((not gotname) and (cmd<>''))}
   {if (gotname) or (noneedname) then}
      begin
      if (cmdr[i].cmdkeys='OC') then begin
         sprompt(strings^.enterhandle);
         input(newmenucmd,30);
         commandline(' - '+newmenucmd+' is attempting chat.');
      end; { cmdr[i].cmdkeys='OC') }
      newmenucmd:='';
      repeat
         domenuexec(cmd,newmenucmd)
      until (newmenucmd=''); {repeat}
      case tshuttlelogon of
         1: if (status^.shuttlepw='') then
               loggedon:=TRUE
            else begin
               echo:=FALSE;
               sprompt(strings^.ansiq);
               inputl(pw,20);
               pw:=strAllCaps(pw);
               echo:=TRUE;
               if (pw=status^.shuttlepw) then
                  loggedon:=TRUE
               else begin
                  sl1(#3#1+' Illegal System Password: "'+pw+'"');
                  inc(tries);
               end; { else begin }
            end; {if status^.shuttlepw='' else begin }
         2: if (gotname) then begin
               print('You already ARE a user!');
               print('Why do you want to log on as new again!?');
               print('Sheesshhhhh.....');
               pauseScr;
            end
            else begin
               if pynq(strings^.logonnew,false) then begin
                  newuserinit('');
                  newuser;
                  if (usernum>0) and (not hangup) then begin
                     gotname:=TRUE;
                     useron:=TRUE;
                     logon1st;
                  end; { (usernum>0) and (not hangup) }
               end; {pynq(fstring)}
            end; { if gotname else begin }
         3: begin
               sprompt(strings^.enterhandle);
               finduser(s,usernum);
               if (usernum>=1) then begin
                  reset(uf);
                  seek(uf,usernum);
                  read(uf,thisuser);
                  sprompt(strings^.enterpw);
                  inputl(pw,20);
                  pw:=strAllCaps(pw);
                  if (status^.phonepw) then begin
                     commandline('Phone # - "'+thisuser.ph+'"');
                     sprompt(strings^.phonenum);
                     input(phone,4);
                     echo:=TRUE;
                  end { status^.phonepw }
                  else phone:=(copy(thisuser.ph,9,4));
                  echo:=TRUE;
                  if ((thisuser.pw=pw) and (copy(thisuser.ph,9,4)=phone)) then begin
                     if ((thisuser.sl>=status^.newsl) or (thisuser.dsl>=status^.newdsl)) then begin
                        sysoplog('Found out matrix password');
                        if (status^.shuttlepw='') then begin
                           printf('val');
                           loggedon:=pynq('Would you like to log on now? ',true);
                           checkedacs:=true;
                        end {status^.shuttlepw = ''}
                        else begin
                           sprint('System Password: '+#3#5+status^.shuttlepw);
                           loggedon:=pynq('Would you like to log on now? ',true);
                           checkedacs:=true;
                        end; {status^.shuttlepw else begin}
                     end {this user.sl>status^.newsl) or (thisuser.dsl>}
                     else begin
                        printf('notval');
                        if (nofile) then
                           sprint('You have not been validated yet.');
                     end; {this user.sl>status^.newsl) or (thisuser.dsl> else begin}
                  end {((thisuser.pw=pw) and (copy(thisuser.ph,9,4)=phone))}
                  else
                     sprint('Illegal Logon');
               end; { if usernum >= 1}
            end; {case 3}
         end; { case }
      end;
      if (tries=status^.maxlogontries) then hangup:=TRUE;
  end; {while}
end;

procedure getacsuser(eventnum:integer; acsreq:astr);
var user:userrec;
    sr:smalrec;
    r:real;
    s,pw:astr;
    cp,un,i:integer;
    c:char;
    sfo:boolean;

   procedure dobackspace;
   begin
      dec(cp);
      prompt(^H' '^H);
   end;

begin
   printf('acsea'+cstr(eventnum));
   if (nofile) then begin
      print('Restricted time zone.');
      print('Only certain users allowed online at this time.');
   end;
   print('Current time: '+date+' '+time+'.');
   print('Enter your user name/number *now*.');
   print('If you do not enter within 20 seconds, you will be hung up:');
   checkhangup;
   if (hangup) then exit;
   r:=timer;
   s:='';
   cp:=1;
   echo:=TRUE;
   repeat
      checkhangup;
      c:=inkey;
      if (c<>#0) then
         case c of
            ^H:if (cp>1) then dobackspace;
            ^X:while (cp<>1) do dobackspace;
            #32..#255: if (cp<=36) then begin
               c:=upcase(c);
               outkey(c);
               s[cp]:=c; inc(cp);
            end;
         end;
      if (timer-r>20.0) then hangup:=TRUE;
   until ((c=^M) or (hangup));
   s[0]:=chr(cp-1);
   if (not hangup) then begin
      un:=value(s);
      if (un<>0) then begin
         reset(uf);
         if (un>filesize(uf)-1) then un:=0
         else begin
            seek(uf,un);
            read(uf,user);
         end;
         close(uf);
      end
      else begin
         sfo:=(filerec(sf).mode<>fmclosed);
         if (not sfo) then reset(sf);
         un:=0;
         i:=1;
         while ((i<=filesize(sf)-1) and (un=0)) do begin
            seek(sf,i);
            read(sf,sr);
            if (s=sr.name) then un:=sr.number;
            inc(i);
         end;
         if (un>filesize(sf)-1) then un:=0;
         if (not sfo) then close(sf);
         if (un<>0) then begin
            reset(uf);
            seek(uf,un);
            read(uf,user);
            close(uf);
         end;
      end;
      if (un<>0) then usernum:=un;
      if ((user.deleted) or (not aacs1(user,usernum,acsreq))) then un:=0;
      if (un=0) then begin
         print('Invalid user account.');
         printf('acseb'+cstr(eventnum));
         if (nofile) then begin
            print('This time window allows certain other users to get online.');
            print('Please call back later, after it has ended.');
         end;
         hangup:=TRUE;
      end
      else begin
         print('Valid user account - Welcome.');
         echo:=FALSE;
         sprompt(strings^.enterpw);
         inputl(pw,20);
         pw:=strAllCaps(pw);
         if (pw<>user.pw) then begin
            print('Invalid password.  Hanging up.');
            printf('acseb'+cstr(eventnum));
            if (nofile) then begin
               print('This time window allows certain other users to get online.');
               print('Please call back later, after it has ended.');
            end;
            hangup:=TRUE;
         end
         else echo:=TRUE;
      end;
   end;
end;

function getuser:boolean;
var pw,s,phone,newusername,acsreq:astr;
    lng:longint;
    tries,i,ttimes,z,zz,eventnum:integer;
    done,nu,ok,toomuch,wantnewuser,acsuser:boolean;
begin
   if (status^.useScrollBack) then begin
      if (not okscrollback) then begin
         okscrollback:=true;
         new(scrlpt);
         new(scrlatpt);
         initializescrlback;
      end;
   end;
   wasnewuser:=FALSE;
   thisuser.tltoday:=15;  { allow user 15 minutes to log on >MAX< }
   extratime:=0.0;
   freetime:=0.0;
   choptime:=0.0;
{   with thisuser do begin
      usernum:=-1;
      name:='NO USER';
      realname:='Not entered yet';
      sl:=0;
      dsl:=0;
      ar:=[];
      ac:=[onekey,pause,novice,color,ansi];
      ac:=ac+status^.newac;
      linelen:=80;
      pagelen:=24;
   end;}
   getdatetime(timeon);
   mread:=0;
   realsl:=-1;
   realdsl:=-1;
   newusername:='';
   sl1('');
   wantnewuser:=FALSE;
   macok:=FALSE;
   nu:=FALSE;
   echo:=TRUE;
   pw:='';
   if (pos('9600',spd)<> 0) then begin
      if (status^.lock300) then begin
         printf('no9600.msg');
         if (nofile) then print('9600 baud callers not allowed on this BBS.');
         hangup:=TRUE;
      end;
      if ((status^.b300lowtime<>0) or (status^.b300hitime<>0)) then
         if (not intime(timer,status^.b300lowtime,status^.b300hitime)) then begin
            printf('no9600h.msg');
            if (nofile) then print('9600 baud calling hours are from '+ctim(status^.b300lowtime)+' to '
                                   +ctim(status^.b300hitime));
            hangup:=TRUE;
         end;
      if (not hangup) then
         if ((status^.b300lowtime<>0) or (status^.b300hitime<>0)) then begin
            printf('yes9600h.msg');
            if (nofile) then begin
               print('NOTE: 9600 baud calling times are');
               print('restricted to the following hours ONLY:');
               print('  '+ctim(status^.b300lowtime)+' to '+ctim(status^.b300hitime));
            end;
         end;
   end;
   acsuser:=FALSE;
   for i:=0 to numevents do
      with events[i]^ do
         if ((etype='A') and (active) and (checkeventtime(i,0))) then begin
            acsuser:=TRUE;
            acsreq:=events[i]^.execdata;
            eventnum:=i;
         end;

   checkedacs:=false;
   if (acsuser) then getacsuser(eventnum,acsreq);
   if ((status^.shuttlelog) and (not fastlogon)) then doshuttle;
   if (not wantnewuser) and (not fastlogon) and (not hangup) then begin
      thisuser.ac:=thisuser.ac+[ansi];
      printf('welcome');
      z:=0;
   end;
   ttimes:=0;
   tries:=0;
   s:='';
   if (not checkedacs) then repeat
      repeat
         if (not wantnewuser) then sprompt(strings^.enterhandle);
         if (status^.shuttlelog) and (wantnewuser) then begin
            s:='';
            usernum:=-1;
         end
         else finduser(s,usernum);
         if (pos('%',s)<>0) then begin
            print('Nice try, idiot - that never worked.');
            print('(SysOp has been notified.)');
            sl1('Idiot tried to gain illegal system info with @@ MCI usage at logon');
            hangup:=TRUE;
         end;
         if (not hangup) then begin
            newusername:='';
            if (usernum=0) then
               if (s<>'') then begin
                  sprint(strings^.namenotfound);
                  if pynq(strings^.logonnew,true) then usernum:=-1;
                  nl;
                  newusername:=s;
               end
               else begin
                  inc(ttimes);
                  if (ttimes>status^.maxlogontries) then hangup:=TRUE;
               end;
         end;
      until ((usernum<>0) or (hangup));
      ok:=TRUE;
      done:=FALSE;
      if (not hangup) then
         case usernum of
            -1:begin
               newuserinit(newusername);
               nu:=TRUE;
               done:=TRUE;
               ok:=FALSE;
            end
            else if (usernum=-3) or (usernum=-2) then begin
               print('Nice try, idiot - that no longer works.');
               print('(SysOp has been notified.)');
               sl1('Idiot tried to crash system with negative number entry at logon');
               hangup:=TRUE;
            end
            else begin
               reset(uf);
               seek(uf,usernum);
               read(uf,thisuser);
               echo:=FALSE;
               if (not status^.localsec) then begin
                  if (not useron) then begin
                     useron:=TRUE;
                     schangewindow(TRUE,status^.curwindow);
                  end
                  else schangewindow(FALSE,status^.curwindow);
                  {commandline('Password - "'+thisuser.pw+'"');}
                  useron:=FALSE;
               end;

               getpws(ok,tries);
               if (ok) then begin
                  done:=TRUE;
                  readinzscan;
               end;

               close(uf);
               if (not ok) then begin
                  useron:=TRUE;
                  sclearwindow;
                  useron:=FALSE;
               end;
            end;
         end;
   until ((done) or (hangup));
   if ((thisuser.lockedout) and (not hangup)) then begin
      printf(thisuser.lockedfile);
      sysoplog(#3#7+'['+#3#8+'*'+#3#7+'] '+#3#3+thisuser.name+#3#7+' --> '+#3#5+
               'Attempt to access system when locked out'+#3#7+' <--');
      hangup:=TRUE;
   end;
   if ((not nu) and (not hangup)) then begin
      toomuch:=FALSE;
      if (thisuser.laston<>date) then begin
         thisuser.ontoday:=0;
         thisuser.tltoday:=status^.timeallow[thisuser.sl];
      end;
      if (((rlogon in thisuser.ac) or (status^.callallow[thisuser.sl]=1)) and
         (thisuser.ontoday>=1) and (thisuser.laston=date)) then begin
         printf('2manycal');
         if (nofile) then print('You can only log on once per day.');
         toomuch:=TRUE;
      end
      else if ((thisuser.ontoday>=status^.callallow[thisuser.sl]) and (thisuser.laston=date)) then begin
         printf('2manycal');
         if (nofile) then print('You can only log on '+cstr(status^.callallow[thisuser.sl])+' times per day.');
         toomuch:=TRUE;
      end
      else if ((thisuser.tltoday<=0) and (thisuser.laston=date)) then begin
         printf('notlefta');
         if (nofile) then prompt('You can only log on for '+cstr(status^.timeallow[thisuser.sl])+' minutes per day.');
         toomuch:=TRUE;
         if (thisuser.timebank>0) then begin
            sprint('However, you have '+cstrl(thisuser.timebank)+' minutes left in your Time Bank.');
            if pynq('Withdraw from Time Bank? ',true) then begin
               prompt('Withdraw how many minutes? ');
               inu(zz);
               lng:=zz;
               if (lng>0) then begin
                if (lng>thisuser.timebank) then lng:=thisuser.timebank;
                dec(thisuser.timebankadd,lng);
                if (thisuser.timebankadd<0) then thisuser.timebankadd:=0;
                dec(thisuser.timebank,lng);
                inc(thisuser.tltoday,lng);
                sprint('^5In your account: ^3'+cstr(thisuser.timebank)+
                       '^5   Time left online: ^3'+cstr(trunc(nsl) div 60));
                sysoplog('TimeBank: No time left at logon, withdrew '+cstrl(lng)+' minutes.');
              end;
            end;
            if (nsl>=0) then toomuch:=FALSE else sprint(#3#7+'Hanging up.');
          end;
        end;
      if (toomuch) then begin
         sl1(#3#7+' [*] '+#3#1+thisuser.name+' #'+cstr(usernum)+' tried logging on more than allowed.');
         hangup:=TRUE;
      end;
      if (tries=status^.maxlogontries) then hangup:=TRUE;
      if (not hangup) then inc(thisuser.ontoday);
   end;
   if ((usernum>0) and (not hangup)) then begin
      getuser:=nu;
      useron:=TRUE;
      schangewindow(not cwindowon,status^.curwindow);
      commandline('Successful Logon');
      useron:=FALSE;
      inittrapfile;
      makenode(2);
      makenodeon(thisuser.name,usernum);
   end;
   if (hangup) then getuser:=FALSE;
end;

end.
