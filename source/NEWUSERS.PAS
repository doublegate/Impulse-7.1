{ New user stuff }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit newusers;

interface

uses
   Crt,      Dos,      uconfig,  common5,  mail0,    mail1,    mail2,
   mail3,    misc2,    misc3,    misc4,    misc5,    records,  cuser,
   common1,  common2,  common3,  common,   strProc;

procedure newuser;
procedure newuserinit(nam:astr);

implementation

var
  newusername:astr;

function autov:boolean;
var list:text;
    s:string;
begin
   autov:=false;
   if (exist(status^.gfilepath+'AUTOVAL.LST')) then begin
      assign(list,status^.gfilepath+'AUTOVAL.LST');
      reset(list);
      while (not eof(list)) do begin
         readln(list,s);
         if (pos(newusername,s)<>0) then autov:=true;
      end;
      close(list);
   end;
end;

procedure p1;
var i : integer;
begin
   printf('system');
   pauseScr;
   printf('newuser');
   pauseScr;
   if (newusername<>'') then thisuser.name:=newusername;
   cls;
   i := 1;
   repeat
      cstuff(nuorder[i],1,thisuser);
      inc(i);
   until (nuOrder[i]=0) or (hangup);
   cls;
   ChangeUConfig;
   sl1('New user applied for access : '+caps(thisuser.name));
end;

procedure p2;
var user:userrec;
    i,j:integer;
    nuvfile:file of nuvrec;
    thisnuv:nuvrec;
    done : boolean;
begin
   if (not hangup) then begin
      reset(uf);
      j:=0;
      for i:=1 to filesize(uf)-1 do begin
         seek(uf,i);
         read(uf,user);
         if ((user.deleted) and (j=0)) then j:=i;
      end;
      if (j<>0) then usernum:=j
      else usernum:=filesize(uf);
      with thisuser do begin
         deleted:=FALSE;
         waiting:=0;
         firston:=date;
         laston:=date;
         loggedon:=0;
         msgpost:=0;
         emailsent:=0;
         feedback:=0;
         ontoday:=0;
         illegal:=0;
         forusr:=0;
         downloads:=0;
         uploads:=0;
         dk:=0;
         uk:=0;
         linelen:=status^.linelen;
         pagelen:=status^.pagelen;
         ttimeon:=0;
         for i:=1 to 5 do boardsysop[i]:=255;
         lastmsg:=1;
         lastfil:=1;
         credit:=0;
         timebank:=0;
         readinzscan; { load old / create new zscan.dat record }
         with zscanr do begin
            for i:=1 to maxboards do
               for j:=1 to 6 do mhiread[i][j]:=0;
            mzscan:=[];
            fzscan:=[];
            for i:=1 to maxboards do mzscan:=mzscan+[i];
            for i:=0 to maxuboards do fzscan:=fzscan+[i];
         end;
         savezscanr;
         trapactivity:=FALSE;
         trapseperate:=FALSE;
         timebankadd:=0;
         {mpointer:=-1;}
         chatauto:=FALSE;
         chatseperate:=FALSE;
         slogseperate:=FALSE;
         avadjust:=0;
         reset(uf);
         seek(uf,1);
         read(uf,user);
         close(uf);
         cols:=user.cols;
         sl:=status^.newsl;
         dsl:=status^.newdsl;
         realsl:=sl;
         realdsl:=dsl;
         filepoints:=status^.newfp;
         ar:=status^.newar;
         tltoday:=status^.timeallow[sl];
         autosig:='';
         note:='';
         if (autov) then autovalidate(thisuser,usernum)
         else if (status^.usenuv) then begin
            cls;
            if ((exist(status^.afilepath+'newuser.inf')) or (exist(status^.gfilepath+'newuser.inf'))) then readq('newuser',0);
            cls;
	    printf('nuvalert');
            assign(nuvfile,status^.gfilepath+'nuv.dat');
            {$I-} reset(nuvfile); {$I+}
            if(ioresult<>0) then rewrite(nuvfile);
            while(not eof(nuvfile)) and (not done) do begin  { Find unused slot }
               read(nuvfile,thisnuv);
               if(not thisnuv.exists) then begin
                  done:=TRUE;
                  seek(nuvfile,filepos(nuvfile)-1);
               end;
            end;
            with thisnuv do begin
               unum:=usernum;
               handle:=thisuser.name;
               no:=0;
               yes:=0;
               totvoted:=0;
	       exists:=TRUE;
	       commentptr:=0;
            end;
            write(nuvfile,thisnuv);
            close(nuvfile);
         end;
      end;
   end;
   if hangup then pauseScr;
   reset(uf);
   seek(uf,usernum);
   write(uf,thisuser);
   close(uf);
   isr(thisuser.name,usernum);
   useron:=TRUE;
   window(1,1,80,25);
   cls;
   schangewindow(not cwindowon,status^.curwindow);
   topscr;
   nl;
end;

procedure newuser;
var i,tries:integer;
    pw:astr;
begin
{   if (status^.numusers>=status^.maxusers) then begin
      sl1(#3#9+'Maximum user count has been reached.');
      printf('maximum');
      if (nofile) then sprint(#3#9+'The maximum number of users has been reached on this system.');
      hangup:=TRUE;
   end
   else begin}
   if (status^.newuserpw<>'') then begin
      tries:=0;
      pw:='';
      while ((pw<>status^.newuserpw) and (tries<status^.maxlogontries) and (not hangup)) do begin
         sprompt(strings^.newuserp);
         echo:=FALSE;
         inputl(pw,20);
         pw:=strAllCaps(pw);
         echo:=TRUE;
	 if ((status^.newuserpw<>pw) and (pw<>'')) then begin
	    sl1(#3#8+'- '+#3#1+' Illegal Newuser Password - "'+pw+'"');
	    inc(tries);
	 end;
      end;
      if (tries>=status^.maxlogontries) then begin
        printf('nonewusr');
        hangup:=TRUE;
      end;
   end;
   p1;
   p2;
   cls;
   printf('newuinfo');
   pauseScr;
   cls;
   if (status^.newapp<>-1) then begin
      reset(uf);
      i:=forwardm(status^.newapp);
      close(uf);
      if (i=0) then i:=status^.newapp;
      sprint(strings^.msgsendto+status^.sysopname);
      email1(i,'Validation E-mail');
   end;
   inc(status^.todayzlog.newusers);
   wasnewuser:=TRUE;
   useron:=TRUE;
end;

procedure newuserinit(nam:astr);
begin
   newusername:=nam;
   cls;
   if (status^.closedsystem) then begin
      printf('system');
      pauseScr;
      printf('nonewusr');
      hangup:=TRUE;
   end
   else begin
      with thisuser do begin
         name:='NEW USER';
         trapactivity:=FALSE;
         trapseperate:=FALSE;
      end;
      inittrapfile;
   end;
end;
end.
