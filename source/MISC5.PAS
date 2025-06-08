{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit misc5;

interface

uses
   Crt,      Dos,      common5,  common,   common1,  common2,  common3,
   doors,    records,  misc1,    strProc;

procedure finduser(var s:astr; var usernum:integer);
procedure dsr(uname:astr);
procedure isr(uname:astr;usernum:integer);
procedure logon1st;
function blacklist(name:string):boolean;

implementation

function blacklist(name:string):boolean;
var list:text;
    s:string;
begin
   blacklist:=false;
   if (exist(status^.gfilepath+'BLACK.LST')) then begin
      assign(list,status^.gfilepath+'BLACK.LST');
      reset(list);
      while (not eof(list)) do begin
         readln(list,s);
         s:=strallcaps(s);
         if name=s then blacklist:=true;
      end;
      close(list);
   end;
end;

procedure finduser(var s:astr; var usernum:integer);
var user:userrec;
    sr:smalrec;
    nn:astr;
    i,ii,t:integer;
    sfo,ufo:boolean;
begin
   s:='';
   usernum:=0;
   inputl(nn,36);
   nn:=strallcaps(nn);
   if (blacklist(nn)) then begin
      printf('blacklist');
      hangup:=true;
      exit;
   end;
   if (nn='?') then exit;
   while (copy(nn,1,1)=' ') do nn:=copy(nn,2,length(nn)-1);
   while (copy(nn,length(nn),1)=' ') do nn:=copy(nn,1,length(nn)-1);
   while (pos('  ',nn)<>0) do delete(nn,pos('  ',nn),1);
   if (hangup) then exit;
   s:=nn;
   usernum:=value(nn);
   if (usernum<>0) then begin
      if (usernum<0) then usernum:=-3             (* illegal negative number entry *)
      else begin
         ufo:=(filerec(uf).mode<>fmclosed);
         if (not ufo) then reset(uf);
         if (usernum>filesize(uf)-1) then begin
            sprint(strings^.unKnownHandle);
            usernum:=0;
         end
         else begin
            seek(uf,usernum);
            read(uf,user);
            if (user.deleted) then begin
               sprint(strings^.unKnownHandle);
               usernum:=0;
            end;
         end;
         if (not ufo) then close(uf);
      end;
   end
   else begin
      if (nn<>'') then begin
         sfo:=(filerec(sf).mode<>fmclosed);
         if (not sfo) then reset(sf);
         ii:=0;
         t:=0;{##$#$}
         while ((t<=filesize(sf)-1) and (ii=0)) do begin
            seek(sf,t);
            read(sf,sr);
            if (nn=sr.name) then ii:=sr.number;
            inc(t);
         end;
         if (ii<>0) then usernum:=ii;
      end
      else sfo := true;
      if (nn='NEW') then usernum:=-1;
      if (usernum<=0) then sprint(strings^.unKnownHandle);
      if (not sfo) then close(sf);
   end;
end;

procedure dsr(uname:astr);
var t,ii:integer;
    sr:smalrec;
    sfo:boolean;
    txt:text;
begin
   if (status^.multitask) then begin
      assign(txt,status^.gfilepath+'black.lst');
      if (not (exist(status^.gfilepath+'black.lst'))) then rewrite(txt);
      if (exist(status^.gfilepath+'black.lst')) then
      begin
         append(txt);
         writeln(txt,uname);
      end;
      close(txt);
   end;
   sfo:=(filerec(sf).mode<>fmclosed);
   if (not sfo) then reset(sf);

   ii:=0;
   t:=1;
   while ((t<=filesize(sf)-1) and (ii=0)) do begin
      seek(sf,t);
      read(sf,sr);
      if (sr.name=uname) then ii:=t;
      sysoplog('* Deleted user: '+caps(sr.name));
      inc(t);
   end;

   if (ii<>0) then begin
      if (ii<>filesize(sf)-1) then
         for t:=ii to filesize(sf)-2 do begin
            seek(sf,t+1); read(sf,sr);
            seek(sf,t); write(sf,sr);
         end;
         seek(sf,filesize(sf)-1); truncate(sf);
         dec(status^.numusers); savesystat;
   end
   else sl1('*** Couldn''t delete "'+uname+'"');
   if (not sfo) then close(sf);
end;

procedure isr(uname:astr; usernum:integer);
var t,i,ii:integer;
    sr:smalrec;
    sfo:boolean;
begin
   sfo:=(filerec(sf).mode<>fmclosed);
   if (not sfo) then reset(sf);

   if (filesize(sf)=1) then ii:=0
   else begin
      ii:=0;
      t:=1;
      while ((t<=filesize(sf)-1) and (ii=0)) do begin
         seek(sf,t); read(sf,sr);
         if (uname<sr.name) then ii:=t;
         inc(t);
      end;
      for i:=filesize(sf)-1 downto ii+1 do begin
         seek(sf,i);
         read(sf,sr);
         seek(sf,i+1);
         write(sf,sr);
      end;
   end;
   with sr do begin
      name:=uname;
      number:=usernum;
   end;
   seek(sf,ii+1);
   write(sf,sr);
   inc(status^.numusers);
   savesystat;
   if (not sfo) then close(sf);
end;

procedure logon1st;
var ul:text;
    u:userrec;
    zf:file of zlogrec;
    fil:file of astr;
    d1,d2:zlogrec;
    s,s1:astr;
    n,z,c1,num,node,rcode:integer;
    c:char;
    abort:boolean;
begin
   node:=thisnode;
   if (spd<>strings^.locallogin) then begin
      inc(status^.callernum);
      inc(status^.todayzlog.calls);
   end;
   realsl:=thisuser.sl;
   realdsl:=thisuser.dsl;

   if (status^.lastdate<>date) then begin
      sprint(strings^.updatesys);
      commandline('Updating user time left ...');
      reset(uf);
      for n:=1 to filesize(uf)-1 do begin
         seek(uf,n);
         read(uf,u);
         with u do begin
            tltoday:=status^.timeallow[sl];
            timebankadd:=0;
            ontoday:=0;
         end;
         seek(uf,n);
         write(uf,u);
      end;
      close(uf);
      commandline('Updating ZLOG.DAT ...');
      assign(zf,status^.gfilepath+'zlog.dat');
      {$I-} reset(zf); {$I+}
      if (ioresult<>0) then begin
         rewrite(zf);
         d1.date:='';
         for n:=1 to 2 do write(zf,d1);
      end;
      d1:=status^.todayzlog;
      d1.date:=status^.lastdate;
      for n:=filesize(zf)-1 downto 0 do begin
         seek(zf,n);
         read(zf,d2);
         seek(zf,n+1);
         write(zf,d2);
      end;
      seek(zf,0);
      write(zf,d1);
      close(zf);
      status^.lastdate:=date;

      assign(multinodef,status^.gFilePath+'nodes.dat');
      reset(multinodef);
      commandline('Updating SysOp Log files ...');
      (* assign(ul,status^.trappath+'sysop'+cstr(status^.backsysoplogs)+'.'+cstr(node));
      {$I-} erase(ul); {$I+} num:=ioresult;*)
      for node:=1 to filesize(multinodef) do begin
         assign(ul,status^.trappath+'sysop'+cstr(status^.backsysoplogs)+'.'+cstr(node));
         {$I-} erase(ul); {$I+}
         num:=ioresult;
      end;

      for n:=status^.backsysoplogs-1 downto 1 do
         if (exist(status^.trappath+'sysop'+cstr(n)+'.'+cstr(node))) then begin
            assign(ul,status^.trappath+'sysop'+cstr(n)+'.'+cstr(node));
            rename(ul,status^.trappath+'sysop'+cstr(n+1)+'.'+cstr(node));
         end;

      d1:=status^.todayzlog;
      sl1('');
      sl1('Total time on........: '+cstr(d1.active));
      sl1('Number of Calls......: '+cstr(d1.calls));
      sl1('New users............: '+cstr(d1.newusers));
      sl1('Public posts.........: '+cstr(d1.pubpost));
      sl1('Private mail sent....: '+cstr(d1.privpost));
      sl1('Feedback sent........: '+cstr(d1.fback));
      sl1('Critical Errors......: '+cstr(d1.criterr));
      sl1('Downloads today......: '+cstr(d1.downloads)+'-'+cstrl(d1.dk)+'k');
      sl1('Uploads today........: '+cstr(d1.uploads)+'-'+cstrl(d1.uk)+'k');

      close(sysopf);
      rename(sysopf,status^.trappath+'sysop1.'+cstr(node));

      assign(sysopf,status^.trappath+'sysop.'+cstr(node));
      rewrite(sysopf); close(sysopf); append(sysopf);
      sl1('');
      sl1(ProgName+' Sysop Log for'+date);
      sl1('');
      sl1('');

      assign(ul,status^.afilepath+'user.log');
      rewrite(ul);
      writeln(ul);
      writeln(ul,'Log of callers for '+date+':');
      writeln(ul);
      close(ul);

      close(multinodef);
      status^.todayzlog.date:=date;
      with status^.todayzlog do begin
         for n:=0 to 4 do userbaud[n]:=0;
         active:=0;
         calls:=0;
         newusers:=0;
         pubpost:=0;
         privpost:=0;
         fback:=0;
         criterr:=0;
         uploads:=0;
         downloads:=0;
         uk:=0;
         dk:=0;
      end;
      if (exist('daystart.bat')) then shelldos(FALSE,process_door('daystart.bat @F @L @B @G @T @R'),rcode);
   end;

   if (thisuser.slogseperate) then begin
      assign(sysopf1,status^.trappath+'slog'+cstr(usernum)+'.'+cstr(node));
      {$I-} append(sysopf1); {$I+}
      if (ioresult<>0) then begin
         rewrite(sysopf1);
         append(sysopf1);
         s := expand('_',26+length(nam));
         s1 := expand(' ',26+length(nam));
         writeln(sysopf1,'');
         writeln(sysopf1,'  '+s);
         writeln(sysopf1,'>>'+s1+'<<');
         writeln(sysopf1,'>> '+ProgName+' SysOp Log for '+nam+': <<');
         writeln(sysopf1,'>>'+s+'<<');
         writeln(sysopf1,'');
      end;
      writeln(sysopf1);
      s:=#3#3+'Logon '+#3#5+'['+dat+']'+#3#4+' (';
      if (spd<>strings^.locallogin) then s:=s+spd+' baud)'
      else s:=s+strings^.localLogin;
      if (status^.stripclog) then s:=stripcolor(s);
      writeln(sysopf1,s);
   end;

   s:=#3#3+cstr(status^.callernum)+#3#4+' -- '+#3#0+nam+#3#4+' -- '+#3#3+'Today '+cstr(thisuser.ontoday+1);
   if (trapping) then s:=s+#3#0+'*';
   sl1(s);
   if (spd<>strings^.locallogin) then begin
      assign(ul,status^.afilepath+'user.log');
      {$I-} append(ul); {$I+}
      if (ioresult<>0) then begin
         rewrite(ul);
         append(ul);
      end;
      s:=#3#5+mln(cstr(status^.callernum),6)+#3#9+'- '+#3#0+mln(nam,26)+#3#9+' - '+#3#3+time+#3#9+' -'+#3#3+mrn(spd,5);
      if (wasnewuser) then s:=s+#3#5+' <New User>';
      writeln(ul,s);
      close(ul);
   end;
end;

end.
