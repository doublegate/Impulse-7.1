(* Menuing System *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-,T+}
Unit Menus;

Interface

Uses
   Crt,      Dos,      Sysop3,   Script,   Rumors,   Sysop7,   Sysop8,
   Sysop9,   Sysop2s,  Uconfig,  Sysop2,   Sysop11,  Mail0,    Mail1,
   Mail2,    Mail3,    Mail4,    Mail6,    Mail9,    File0,    File1,
   File2,    File4,    File5,    File6,    File8,    File9,    File10,
   File11,   File12,   File13,   File14,   Misc1,    Common2,  Common3,
   Misc2,    Misc3,    Misc4,    Misc5,    CUser,    Doors,    Menus3,
   MyIO,     Common,   mail7,    multinod, AnsiDrv,  Common1,  Mail5,
   Conf,     Menus2,   Sysop1,   common4,  Common5,  Records,  cmd,
   strProc,  output,   strProc2, nuv,      timeBank, impDos;


Procedure readin2;
Procedure mainmenuhandle(var cmd:string);
Procedure fcmd(cmd:string; var i:integer; noc:integer;
               var cmdexists,cmdnothid:boolean);
Procedure domenuexec(cmd:string; var newmenucmd:string);
Procedure domenucommand(var done:boolean; cmd:string; var newmenucmd:string);

Implementation

uses logon1,sys;


Procedure readin2;
var s:string;
    nacc:boolean;
    oldmnu : string;
begin
   readin;
   nacc:=FALSE;
   with menur do begin
      if ((not aacs(acs)) or (password<>'')) then begin
         nacc:=TRUE;
         if (password<>'') then begin
            prompt('Password: ');
            inputl(s,15);
            s:=strAllCaps(s);
            if (s=password) then nacc:=FALSE;
         end;
         if (nacc) then begin
            nl;
            print('Access denied.');
            pausescr;
            print('Dropping back to fallback menu...');
            curmenu:=status^.menupath+fallback+'.mnu';
            readin;
         end;
      end;
{      if (not nacc) then
         if (forcehelplevel<>0) then chelplevel:=forcehelplevel
         else if (novice in thisuser.ac) then chelplevel:=2
         else chelplevel:=1;}
   end;
end;

{procedure checkforcelevel;
begin
  if (chelplevel<menur.forcehelplevel) then chelplevel:=menur.forcehelplevel;
end;}

const numenters : byte = 0;

procedure getcmd(var s:string);
var s1,ss,oss,shas0,shas1:string;
    i:integer;
    c2,c,cc:char;
    oldco:byte;
    gotcmd,has0,has1,has2:boolean;
begin
   s:='';
   if (buf<>'') then if (copy(buf,1,1)='`') then begin
      buf:=copy(buf,2,length(buf)-1);
      i:=pos('`',buf);
      if (i<>0) then begin
         s:=strAllCaps(copy(buf,1,i-1));
         buf:=copy(buf,i+1,length(buf)-i);
         nl;
         exit;
      end;
   end;
   shas0:='?';
   shas1:='';
   has0:=FALSE;
   has1:=FALSE;
   has2:=FALSE;
   for i:=1 to noc do if (aacs(cmdr[i].acs)) then if (cmdr[i].ckeys[0]=#1) then begin
      has0:=TRUE;
      shas0:=shas0+cmdr[i].ckeys;
   end
   else if ((cmdr[i].ckeys[1]='/') and (cmdr[i].ckeys[0]=#2)) then begin
      has1:=TRUE;
      shas1:=shas1+cmdr[i].ckeys[2];
   end else has2:=TRUE;
   oldco:=curco;
   gotcmd:=FALSE;
   ss:='';
   if (not (onekey in thisuser.ac)) then input(s,60)
   else begin repeat
      getkey(c2);
      c:=upcase(c2);
      oss:=ss;
      if (ss='') then begin
         if (c=#13) then begin
            gotcmd:=TRUE;
            inc(numenters);
            if (numenters >= 5) then chatcall := false;
         end
         else numenters := 0;
         if ((c='/') and ((has1) or (has2) or (thisuser.sl=255))) then ss:='/';
         if ((c='=') and (cso)) then begin gotcmd:=TRUE; ss:=c; end;
         if (((fqarea) or (mqarea)) and (c in ['0'..'9'])) then ss:=c
         else if (pos(c,shas0)<>0) then begin
            gotcmd:=TRUE;
            ss:=c;
         end;
      end
      else if (ss='/') then begin
         if (c=^H) then ss:='';
         if ((c='/') and ((has2) or (thisuser.sl=255))) then ss:=ss+'/';
         if ((pos(c,shas1)<>0) and (has1)) then begin
            gotcmd:=TRUE;
            ss:=ss+c;
         end;
      end
      else if (copy(ss,1,2)='//') then begin
         if (c=#13) then gotcmd:=TRUE
         else if (c=^H) then ss:=copy(ss,1,length(ss)-1)
         else if (c=^X) then begin
            for i:=1 to length(ss)-2 do prompt(^H' '^H);
            ss:='//';
            oss:=ss;
         end
         else if ((length(ss)<62) and (c>=#32) and (c<=#127)) then ss:=ss+c;
      end else if ((length(ss)>=1) and (ss[1] in ['0'..'9']) and ((fqarea) or (mqarea))) then begin
         if (c=^H) then ss:=copy(ss,1,length(ss)-1);
         if (c=#13) then gotcmd:=TRUE;
         if (c in ['0'..'9']) then begin
            ss:=ss+c;
            if (length(ss)=3) then gotcmd:=TRUE;
         end;
      end;
      if ((length(ss)=1) and (length(oss)=2)) then setc(oldco);
      if (oss<>ss) then begin
         if (length(ss)>length(oss)) then prompt(c2);
         if (length(ss)<length(oss)) then prompt(^H' '^H);
      end;
      if ((not (ss[1] in ['0'..'9'])) and ((length(ss)=2) and (length(oss)=1))) then cl(6);
   until ((gotcmd) or (hangup));
   if (copy(ss,1,2)='//') then ss:=copy(ss,3,length(ss)-2);
   s:=ss;
end;

  {nl;}

  if (pos(';',s)<>0) then                 {* "command macros" *}
    if (copy(s,1,2)<>'\\') then begin
      if (onekey in thisuser.ac) then begin
        s1:=copy(s,2,length(s)-1);
         if (copy(s1,1,1)='/') then s:=copy(s1,1,2) else s:=copy(s1,1,1);
         s1:=copy(s1,length(s)+1,length(s1)-length(s));
      end else begin
        s1:=copy(s,pos(';',s)+1,length(s)-pos(';',s));
        s:=copy(s,1,pos(';',s)-1);
      end;
      while (pos(';',s1)<>0) do s1[pos(';',s1)]:=^M;
      dm(' '+s1,c);
    end;
end;

procedure mainmenuhandle(var cmd:string);
var fb:integer;
    fbnum:array [1..maxuboards] of integer;
    count:byte;
    mb:integer;
    mbnum:array [1..maxboards] of integer;
    ckey, newarea:integer;
    done, wantshow:boolean;
    newcmd, s:string;

   procedure mbnumchange;
   var acc:boolean;
       b2,b:integer;
   begin
      b:=1;
      mb:=1;
      while (b<=numboards) do begin
         acc:=mbaseac(b);
         if ((mbunhidden in memboard.mbstat) or (acc)) and ((memboard.conf=mconf) or (mconf=0)) then begin
            if (acc) then begin
               b2:=ccboards[1][b];
               mbnum[mb]:=b2;
               inc(mb);
            end;
         end;
         inc(b);
      end;
   end;

   procedure fbnumchange;
   var acc:boolean;
       b2,b:integer;
   begin
      b:=1;
      fb:=1;
      while (b<=maxulb) do begin
         acc:=fbaseac(b); { fbaseac will load memuboard }
         if ((fbunhidden in memuboard.fbstat) or (acc)) and ((memuboard.conf=fconf) or (fconf=0)) then begin
            if (acc) then begin
               b2:=ccuboards[1][b];
               fbnum[fb]:=b2;
               inc(fb);
            end;
         end;
         inc(b);
      end;
   end;

begin
   menuactiv:=true;
   for ckey:=1 to noc do begin
      if cmdr[ckey].ckeys='EVERYTIME' then begin
         done:=true;
         newcmd:='';
         domenucommand(done,cmdr[ckey].cmdkeys+cmdr[ckey].mstring,newcmd);
      end;
   end;

   makenode(3);
   If (Pulldown in Menur.Menuflags) then Begin
      cmd := '';
      DoPulls(cmd);
      MenuActiv:=False;
      Exit;
   End;
   showthismenu;
   macok:=TRUE;
{  checkforcelevel;}
   if ((forcepause in menur.menuflags) and (lastcommandgood)) then pausescr;
   lastcommandgood:=FALSE;
   readnodemessages;
   done:=false;
   if (chatcall) then begin
      pr1(^G^G);
      Sound(500);
      cDelay(150);
      Sound(750);
      cDelay(150);
      sprompt(strings^.chatcall2);
      NoSound;
   end;
   if (not(nomenuprompt in menur.menuflags)) AND (not(pulldown in menur.menuflags)) then begin
      if (autotime in menur.menuflags) then sprint('|U2Time Left: '+tlef);
      if (scanfilemsg) then sprompt(menur.menuprompt)
      else begin
         if (thisUser.prompt = 5) then sprompt(menur.menuprompt)
         else sprompt(prmp^[thisuser.prompt].prompt);
      end;
   end;

   getcmd(cmd);
   nl;
   cmd:=strAllCaps(cmd);

   if (cmd='?') then begin
      if cHelpLevel = 1 then cHelpLevel := 3;
   end;

{      inc(chelplevel);
    if (chelplevel>3) then chelplevel:=3;
    if ((menur.tutorial='*OFF*') and (chelplevel>=3)) then chelplevel:=2;
  end else
    if (menur.forcehelplevel<>0) then chelplevel:=menur.forcehelplevel
    else
      if (novice in thisuser.ac) then chelplevel:=2 else chelplevel:=1;}

{  checkforcelevel;}

   if (fqarea) or (mqarea) then begin
      newarea:=value(cmd);
      if ((newarea<>0) or (copy(cmd,1,1)='1')) then begin
         if (fqarea) then begin
            if (newarea>=1) and (newarea<=maxuboards) then begin
               fbnumchange;
               changefileboard(fbnum[newarea]);
               loaduboard(fbnum[newarea]);
               sprint(memuboard.name);
            end;
         end
         else if (mqarea) then if (newarea>=0) and (newarea<=maxboards) then begin
            mbnumchange;
            changeboard(mbnum[newarea]);
            loadboard(mbnum[newarea]);
            sprint(memboard.name);
         end;
         cmd:='';
      end;
   end;
   menuactiv:=false;
end;

procedure fcmd(cmd:string; var i:integer; noc:integer; var cmdexists,cmdnothid:boolean);
var done:boolean;
begin
   done:=FALSE;
   repeat
      inc(i);
      if (cmd=cmdr[i].ckeys) then begin
         cmdexists:=TRUE;
         if (oksecurity(i,cmdnothid)) then done:=TRUE;
      end;
   until ((i>noc) or (done));
   if (i>noc) then i:=0;
end;

procedure domenuexec(cmd:string; var newmenucmd:string);
var cmdacs,cmdnothid,cmdexists,done:boolean;
    nocsave,i:integer;
begin
   if (newmenucmd<>'') then begin
      cmd:=newmenucmd;
      newmenucmd:='';
   end;
   if (cmd<>'') then begin
      cmdacs:=FALSE;
      cmdexists:=FALSE;
      cmdnothid:=FALSE;
      done:=FALSE;
      nocsave:=noc;
      i:=0;
      repeat
         fcmd(cmd,i,nocsave,cmdexists,cmdnothid);
         if (i<>0) then begin
            cmdacs:=TRUE;
            domenucommand(done,cmdr[i].cmdkeys+cmdr[i].mstring,newmenucmd);
         end;
      until ((i=0) or (done));
      if (not done) then if ((not cmdacs) and (cmd<>'')) then begin
         nl;
         if ((cmdnothid) and (cmdexists)) then print('You don''t have enough access for this command.')
         else nl;
      end;
   end;
end;

procedure domenucommand(var done:boolean; cmd:string; var newmenucmd:string);
var filvar:text;
    mheader:mheaderrec;
    tmp,tmp1,tmp2,cms,s,s1,s2:string;
    l,i:integer;
    ws,cnfc,k,br,c1,c2,c:char;
    dn,abort,next,b,nocmd:boolean;

   function semicmd(x:integer):string;
   var s:string;
       i,p:integer;
   begin
      s:=cms;
      i:=1;
      while (i<x) and (s<>'') do begin
         p:=pos(';',s);
         if (p<>0) then s:=copy(s,p+1,length(s)-p) else s:='';
         inc(i);
      end;
      while (pos(';',s)<>0) do s:=copy(s,1,pos(';',s)-1);
      semicmd:=s;
   end;
begin
   newmenutoload:=FALSE;
   newmenucmd:='';
   cnfc:=#0;
   c1:=cmd[1];
   c2:=cmd[2];
   cms:=copy(cmd,3,length(cmd)-2);
   nocmd:=FALSE;
   lastcommandovr:=FALSE;
   case c1 of
      {nuvp.mnu}
      'V' : if c2 in ['Y','N','V','D'] then nuvpmnu := c2
      else noCmd := true;
      {filep.mnu}
      '!':if c2 in ['T','L','F','+','?','J','S','U','V','Q','I','U','D','C','N'] then filepmnu := c2
         else nocmd := true;
      {emailp.mnu, readp.mnu}
      '@','$':if c2 in ['A','-','R','B','D','H','M','P','Q','V','T','V','W','X','Z','!','*','S','N','J','I','F','D'] then
         readpmnu := c2
         else nocmd := true;
      'R':case c2 of
         'R':RandomRumor;
         'A':AddRumor;
         'L':ListRumors;
         'D':DeleteRumor;
         else nocmd:=true;
      end;
      'X':case c2 of
         'C':lastcaller;
         'S':printf('sysinfo');
         'U':userlist; {ulist;}
         'A':if (cms[1]='+') then thisuser.ar:=thisuser.ar+[cms[2]]
            else thisuser.ar:=thisuser.ar-[cms[2]];
         'H':thisuser.ac:=thisuser.ac+[onekey];
         'L':thisuser.ac:=thisuser.ac-[onekey];
         'Y':begin
            l:=length(cms);
            tmp2:=copy(cms,pos(';',cms)+1,length(cms));
            if (l>=pos(';',cms)) then delete(cms,pos(';',cms),1+l-pos(';',cms))
            else if (l>pos(';',cms)) then delete(cms,pos(';',cms),pos(';',cms)-l+1);
            tmp:=cms;
            if pos(';',tmp2)<>0 then delete(tmp2,pos(';',tmp2),1);
            if pynq(tmp,false) then domenucommand(dn,tmp2,tmp1);
         end;
         else nocmd:=true;
      end;
    'I':case c2 of
          'S':doscript(cms);
          'D':dodoorfunc('I',cms);
        else nocmd:=true;
        end;
    'C':case c2 of
          'N':if Cms[1]='F' then nf('')
              else if Cms[1]='M' then nscan('');
          'F':changeconf(1,cms[1]);
          'M':changeconf(0,cms[1]);
          'E':begin
                sprompt('Edit Message or File Conf? [M/F]: ');
                onek(cnfc,'MFQ'^M);
                case cnfc of
                  'M':if (cso) then editconf(0);
                  'F':if (cso) then editconf(1);
                end;
              end;
        else nocmd:=true;
        end;
    '-':case c2 of
          'V': bbsListView;
          'A': bbsListAdd;
          'N': newUserVote;
          'T': scanfilemsg:=TRUE; {forces the prompt in the *.MNU file}
          'C': commandline(cms);  {writes cms on the command line in inverted clrs}
          'F': printf(cms);  {displays cms}
          'L': sprint('|NL'+cms);{writes cms}
          'Q': readq(status^.afilepath+cms,0); {reads a questionaire file}
          'R': readasw1(cms); {takes a questionaire file.}
          'S': sysoplog(cms); {writes cms to the sysop log.}
          'W': writehdr(cms); {writes a header with the given string}
          'P': sprint(strLookupString(strStringToWord(cms))); {writes a given fstring}
          ';':begin
                s:=cms;
                while (pos(';',s)<>0) do s[pos(';',s)]:=^M;
                dm(' '+s,c);
              end;
          '$':if (semicmd(1)<>'') then begin
                if (semicmd(2)='') then prompt(':') else prompt(semicmd(2));
                input(s,20);
                if (s<>semicmd(1)) then begin
                  done:=TRUE;
                  if (semicmd(3)<>'') then sprint(semicmd(3));
                end;
              end; {the below cmds change to a diff menu}
          '^','/','\':begin
                        scanfilemsg:=false;
                        dochangemenu(done,newmenucmd,c2,cms);
                      end
                      else  nocmd:=TRUE;
                      end;
    'N':case c2 of
           'C':begin
              cls;
              entermultichat;
           end;{multinode chat}
           'S':begin
              cls;
              enternodemessage;
           end;{sends msg to node}
           'W':begin
              cls;
              whoson;
           end; {shows who's online}
        else nocmd:=true;
        end;
    'B':case c2 of
          '?':batchinfo;
          'C':if (cms='U') then clearubatch else clearbatch;
          'D':begin batchdl; clearbatch; end;
          'L':if (cms='U') then listubatchfiles else listbatchfiles;
          'R':if (cms='U') then removeubatchfiles else removebatchfiles;
          'U':batchul;
        else  nocmd:=TRUE;
        end;
    'D':case c2 of
          'C','D','G','S','W','-':dodoorfunc(c2,cms);
        else  nocmd:=TRUE;
        end;
    'F':case c2 of { filep.mnu }
          'A':fbasechange(done,cms);
          'D':if (numbatchfiles<>0) then batchdl else idl;
          'F':begin
               sprompt('|NL'+strings^.multitask);
               onek(ws,'WSQ'^M);
               case ws of
                 'W':search;
                 'S':searchd;
               end;
              end;
          'K':cstuff(5,2,thisuser);
          'L':listfiles;
          'N':ConfNewscan('F',cms); {newscans a base.  if not given does all}
          'P':pointdate;
          'R':remove;  {removes a files}
          'S':search;  {searches for file matching the *.* or whatever}
          'U':batchul;  {upload}
          'V':lfii;  {file view}
          'Y':printf('filestat');
          'Z':setdirs;
          '#':sprint('|NLEnter the number of the file base to change to it.');
        else  nocmd:=TRUE;
        end;
    'H': case c2 of
          'C': if pynq('|NL'+cms,false) then begin
              cls;
              printf('logoff');
              hangup:=TRUE;
              hungup:=FALSE;
          end;
          'I': hangup:=TRUE;
          'M': begin
                nl;
                printf(cms);
                hangup:=TRUE;
          end;
          else noCmd := true;
        end;
    'M':case c2 of
          'A': mbasechange(done,cms);
          'E': ssmail(cms); {sends mail to user name or # in the cms field}
	  'Q': sendqwk;
	  'R': recieverep;
          'J': dopurgepub(cms);
          'K': purgepriv;
          'L': smail(TRUE);
          'M': readmail;
          'N': ConfNewscan('M',cms);
          'P': begin
                  post(-1,mheader.fromi,mheader);
                  closebrd;
               end;
          'S': scanmessages;
          'Z': chbds;
          '#': sprint('|NLEnter the number of the message base to change to it.');
          '$': mbasestats;
        else  nocmd:=TRUE;
        end;
    'O':case c2 of
          '1'..'3':tshuttlelogon:=ord(c2)-48;
          'A':autovalidate(thisuser,usernum);
          'C':reqchat(cms); {chat request}
          'I':begin
                 printf('logon');
                 pauseScr;
                 printf('system');
              end;
          'P':cstuff(value(cms),2,thisuser); {does user settings}
          'S':bulletins(cms);  {reads bulletin.cms}
          'B':bulletins('');
          'K':changeuconfig;
          'L':cmdOneliners;
          'Y':yourinfo;
        else  nocmd:=TRUE;
        end;
    'U':case c2 of
          'A': replyAutoMsg;
          'D': timeBankDeposit(value(cms));
          'L': timeBankWithDraw(value(cms));
          'R': readAutoMsg;
          'W': writeAutoMsg;
          else  nocmd:=TRUE;
        end;
    'S':case c2 of
          'P':pauseScr;
          'C':cls;
          'S':cmdShowStackFree;
          'H':cmdShowHeapFree;
          else noCmd := true;
        end;
    '%': if (checkPw) then begin
        case c2 of
          'S':pullconfig;
	  'P':exproedit;
          'L':printfile(status^.afilepath+'user.log');
          'B':begin
             sysoplog('* Message base edit');
             boardedit;
          end;
          'C':chuser;
          'D':begin
                sysoplog('* Entered impDos');
                miniDos;
              end;
          'F':begin
                sysoplog('* File base edit');
                dlboardedit;
              end;
          {'L':showlogs;}
          'U':begin
                sysoplog('* User editor');
                uedit1;
              end;
          'Z':begin
                sysoplog('+ Viewed ZLOG');
                zlog;
              end;
          '1':begin
                sysoplog('* Edited files'); editfiles;
              end;
          '2':begin
                sysoplog('* Sorted files'); sort;
              end;
          '3':begin
                sysoplog('* Read private mail'); mailr;
              end;
          '4':begin
                if (cms='') then do_unlisted_download
                else unlisted_download(cms);
              end;
          '5':move;
          '6':uploadall;
          '7':validatefiles;
          '8':addgifspecs;
          '9':packmessagebases;
          '#':begin
                sysoplog('* Menu edit');
                last_menu:=curmenu;
                menu_edit;
                first_time:=TRUE;
                curmenu:=last_menu;
                readin2;
              end;
          '$':dirf(TRUE);
          '%':dirf(FALSE);
        else  nocmd:=TRUE;
        end;
        end
        else nocmd := true;
  else
        nocmd:=TRUE;
  end;
  lastcommandgood:=not nocmd;
  if (lastcommandovr) then lastcommandgood:=FALSE;
  if (nocmd) then
    if (cso) then
    begin
      sysoplog('Invalid command : Cmdkeys "'+cmd+'"');
      nl; {print('Invalid command : Cmdkeys "'+cmd+'"');}
    end;
  if (newmenutoload) then
  begin
    readin{2};
    lastcommandgood:=FALSE;
    if (newmenucmd='') then begin
      i:=1;
      while ((i<=noc) and (newmenucmd='')) do
      begin
        if (cmdr[i].ckeys='FIRSTCMD') then
          if (aacs(cmdr[i].acs)) then newmenucmd:='FIRSTCMD';
        inc(i);
      end;
    end;
  end;
end;

end.
