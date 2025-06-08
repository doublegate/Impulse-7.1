{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit file1;

interface

uses
   Crt,      Dos,      ulcheck,  file3,    execbat,  common4,  common5,
   records,  common1,  common2,  common3,  multinod, myio,     common,
   strproc,  cmd,      sys;

procedure dodl(fpneed:integer);
procedure doul(pts:integer);
procedure showuserfileinfo;
function okdl(f:ulfrec):boolean;
procedure dlx(f1:ulfrec; rn:integer; var abort:boolean);
procedure dl(fn:astr);
procedure dodescrs(var f:ulfrec; var v:verbrec; var pl:integer; var tosysop:boolean; var afterhang:boolean; var dir:string;
                   checkpriv:boolean);
procedure writefv(rn:integer; f:ulfrec; v:verbrec);
procedure newff(f:ulfrec; v:verbrec);
procedure doffstuff(var f:ulfrec; fn:astr; var gotpts:integer);
procedure arcstuff(var ok,convt:boolean; var blks:integer; var convtime:real;
                   itest:boolean; fpath:astr; var fn,descr:astr);
procedure idl;
procedure zipdiz(fn:string);
procedure fbaselist;
procedure unlisted_download(s:astr);
procedure do_unlisted_download;
function nfvpointer:longint;

implementation

uses
   mail2,    file0,    file4,    file8,    file12,   file14,   file6;

var
  locbatup:boolean;
  dizthere:boolean;

procedure dodl(fpneed:integer);
begin
   nl;
   nl;
   if (not aacs(status^.nofilepts)) or (not (fnofilepts in thisuser.ac)) then begin
      if (fpneed>0) then dec(thisuser.filepoints,fpneed);
      if (thisuser.filepoints<0) then thisuser.filepoints:=0;
   end;
end;

procedure doul(pts:integer);
begin
   if (aacs(status^.ulValReq) OR (status^.ulDlRatio) or (pts<>0)) then inc(thisUser.filePoints,pts);
   {if (not aacs(status^.ulvalreq)) then begin
   end
   else if ((not status^.uldlratio) and (not status^.fileptratio) and (pts=0)) then begin
   end else inc(thisuser.filepoints,pts);}
end;

procedure showuserfileinfo;
begin
   with thisuser do commandline('U/L: '+cstr(uploads)+'/'+cstr(trunc(uk))+'k ³ D/L: '+cstr(downloads)+'/'+cstr(trunc(dk))+'k');
end;

function okdl(f:ulfrec):boolean;
var s:astr;
    b:boolean;

   procedure nope(s:astr);
   begin
      if (b) then sprint(s);
      b:=FALSE;
   end;

begin
   b:=TRUE;
   if (isrequest in f.filestat) then begin
      printf('reqfile');
      if (nofile) then begin
         nl;
         sprint(#3#5+'You must Request this file -- Ask '+ status^.sysopname+' for it.');
         nl;
      end;
      if (pynq('Request this file now? [Y] : ',true)) then begin
         s:=sqoutsp(f.filename);
         irt:='File Request of "'+s+'" from file base #'+cstr(ccuboards[1][fileboard]);
         imail(1);
      end;
      b:=FALSE;
      exit;
   end;
   b:=true;
   if ((f.priv) and (pos(caps(f.privfor),caps(thisuser.name))=0)) then nope('This file is marked as PRIVATE.');
   if ((resumelater in f.filestat) and (not fso)) then nope('You can''t do anything with RESUME-LATER files.');
   if ((notval in f.filestat) and (not aacs(status^.dlunval))) then nope('You can''t do anything with UNVALIDATED files.');
   if (thisuser.filepoints<f.filepoints) and (f.filepoints>0) and (not aacs(status^.nofilepts)) and
      (not (fnofilepts in thisuser.ac)) and (not (fbnoratio in memuboard.fbstat)) then
      nope(strings^.nofilepts);
   if (nsl<rte*f.blocks) then nope('Not enough time to download.');
   if (not exist(memuboard.dlpath+f.filename)) then begin
      nope('File isn''t really there!');
      sysoplog('File missing in file list: '+sqoutsp(memuboard.dlpath+f.filename));
   end;
   okdl:=b;
end;

procedure dlx(f1:ulfrec; rn:integer; var abort:boolean);
var u:userrec;
    tooktime,xferstart,xferend:datetimerec;
    i,ii,tt,bar,s:astr;
    rl,tooktime1:real;
    cps,lng:longint;
    c:char;
    next,ps,ok:boolean;
begin
   abort:=FALSE;
   next:=FALSE;
   nl;
   fileinfo(f1,FALSE,abort,next);
   ps:=TRUE;
   abort:=FALSE;
   if (not okdl(f1)) then ps:=TRUE
   else begin
      ps:=FALSE;
      showuserfileinfo;
      getdatetime(xferstart);
      send1(memuboard.dlpath+f1.filename,ok,abort);
      getdatetime(xferend);
      timediff(tooktime,xferstart,xferend);
      if (-lastprot in [10,11,12]) then
         if (not abort) then
            if (not ok) then begin
               sysoplog(#3#3+'Tried download "'+sqoutsp(f1.filename)+'" from '+memuboard.name);
               ps:=TRUE;
            end
            else begin
               inc(thisuser.downloads);
               thisuser.dk:=thisuser.dk+(f1.blocks div 8);
               inc(status^.todayzlog.downloads);
               inc(status^.todayzlog.dk,(f1.blocks div 8));
               numbatchfiles:=0;
               if (not incom) then nl;
               lng:=f1.blocks;
               lng:=lng*128;
               s:=#3#3+'Download "'+sqoutsp(f1.filename)+'" from '+memuboard.name;
               tooktime1:=dt2r(tooktime);
               if (tooktime1>=1.0) then begin
                  cps:=f1.blocks; cps:=cps*128;
                  cps:=trunc(cps/tooktime1);
               end else cps:=0;
               s:=s+#3#3+' ('+cstr(f1.blocks div 8)+'k, '+ctim(dt2r(tooktime))+', '+cstr(cps)+' cps)';
               sysoplog(s);
               if (not (fbnoratio in memuboard.fbstat)) and (f1.filepoints>0) then dodl(f1.filepoints);
               showuserfileinfo;
               if (rn<>-1) then begin
                  inc(f1.nacc);
                  seek(ulff,rn);
                  write(ulff,f1);
               end;
            end;
   end;
   if (ps) then begin
      nl;
      sprompt(#3#5+'Continue with <CR> or [Q]uit :'+#3#3);
      onek(c,'Q '^M);
      abort:=(c='Q');
   end;
end;

procedure dl(fn:astr);
var pl,rn:integer;
    f:ulfrec;
    abort:boolean;
begin
   abort:=FALSE;
   recno(fn,pl,rn);
   if (baddlpath) then exit;
   if (rn=0) then print('File not found.')
   else makenode(5);
   while (rn<>0) and (not abort) and (not hangup) do begin
      reset(ulff);
      seek(ulff,rn);
      read(ulff,f);
      nl;
      dlx(f,rn,abort);
      nrecno(fn,pl,rn);
   end;
   reset(uf);
   close(uf);
   close(ulff);
end;

procedure idl;
var s:astr;
down:boolean;
begin
   down:=TRUE;
   if (not intime(timer,status^.dllowtime,status^.dlhitime)) then down:=FALSE;
   if (spd='300') then
      if (not intime(timer,status^.b300dllowtime,status^.b300dlhitime)) then down:=FALSE;
   if (not down) then printf('dlhours')
   else begin
      writehdr('Download');
      sprint(strings^.downloadline);
      sprompt(strings^.enterfilenametoul);
      input(s,12);
      if (s<>'') then dl(s);
   end;
end;

procedure zipdiz(fn:string);
var errlevel:integer;
begin
   fn:=sqoutsp(fn);
   cmdGetSauce(fn,status^.afilePath+'SAUCE.TMP',status^.tempPath,false);
   if (pos('.ZIP',fn)<>0) then shelldos(false,'Pkunzip -o '+fn+' file_id.diz '+status^.temppath+' > nul',errlevel)
   else if (pos('.LZH',fn)<>0) then shelldos(false,'lha e f '+fn+' file_id.diz '+status^.temppath+' > nul',errlevel)
   else if (pos('.ARJ',fn)<>0) then shelldos(false,'arj e f '+fn+' file_id.diz '+status^.temppath+' > nul',errlevel);
end;

procedure dodescrs(var f:ulfrec;              {* file record      *}
                   var v:verbrec;             {* verbose description record *}
                   var pl:integer;            {* # files in dir   *}
                   var tosysop:boolean;       {* whether to-SysOp *}
                   var afterhang:boolean;     {* did user hangup after file*}
                   var dir:string;            {* file base its in *}
                   checkpriv:boolean);        {* show info or not *}

var i,maxlen:integer;
    bb,isgif:boolean;
    temp:string[30];
    dizem:string[50];
    dizzer:text;
    x,dz:integer;
    fn:string[40];
    ndtemp:string;
    ndtmpn:integer;
    filesbbs:text;
    fbname:string[80];
    found:boolean;

begin
   bb:=false;
   loaduboard(fileboard);
   isgif:=isgifext(f.filename);
   maxlen:=54;
   found:=false;
   assign(filesbbs,memuboard.dlpath+'FILES.BBS');
   If Exist(memuboard.dlpath+'FILES.BBS') then begin
      reset(filesbbs);
      while (not eof(filesbbs)) and (not found) do begin
         readln(filesbbs,fbname);
         if (pos(sqoutsp(f.filename),fbname)<>0) then begin
            f.description:=copy(fbname,18,length(fbname));
            found:=true;
         end;
      end;
      f.vpointer:=-1;
      close(filesbbs);
      exit;
   end;
   if ((fbusegifspecs in memuboard.fbstat) and (isgif)) then dec(maxlen,14);
   if (status^.ulcheckit) and (not hangup) then ulexamine(dir,f.filename)
   else begin
      if (length(status^.startdir)<>0) then External_ULcheck(dir+f.filename);
      zipdiz(dir+f.filename);
   end;
   i:=1;
   if (not exist(status^.temppath+'file_id.diz')) then begin
      assign(dizzer,status^.temppath+'File_id.diz');
      rewrite(dizzer);
      reset(dizzer);
      bb:=true;
      close(dizzer);
   end;
   if exist(status^.temppath+'file_id.diz') then begin
      for i:=1 to 10 do v.descr[i]:='';
      assign(dizzer,status^.temppath+'file_id.diz');
      reset(dizzer);
      i:=1;
      readln(dizzer,dizem);
      f.description:=stripansi(dizem);
      while (not eof(dizzer)) and (i<10) do begin
         readln(dizzer,dizem);
         v.descr[i]:=stripansi(dizem);
         {if (v.descr[i]='') then i:=20;}
         inc(i)
      end;
      {$I-} close(dizzer); erase(dizzer); {$I+}
   End;
   if (not checkpriv) then f.description := ''
   else if (bb) and (not afterhang) and (not hangup) then begin
      tosysop:=true;
      sprompt('|U2Enter Description: ');
      inputl(f.description,45);
   end;
   f.Priv:=false;
   If (not hangup) and (not afterhang) then
      if (checkpriv) then
         if (Pynq('Private File? ',false)) then begin
            f.priv:=true;
            nl;
            prompt(strings^.fileprivate);
            inputl(temp,30);
            f.privfor:=mln(temp,9);
         end;
   if (v.descr[1]<>'') and (not bb) then f.vpointer:=nfvpointer;
   if (v.descr[1]='') or (bb) then f.vpointer:=-1;
   tosysop:=false;
end;

procedure writefv(rn:integer; f:ulfrec; v:verbrec);
var vfo:boolean;
begin
   seek(ulff,rn);
   write(ulff,f);
   if (v.descr[1]<>#1#1#0#1#1) and (f.vpointer<>-1) then begin
      vfo:=(filerec(verbf).mode<>fmclosed);
      if (not vfo) then reset(verbf);
      seek(verbf,f.vpointer); write(verbf,v);
      if (not vfo) then close(verbf);
   end;
end;

procedure newff(f:ulfrec; v:verbrec); {* ulff needs to be open before calling *}
var i,pl:integer;
    fo:boolean;
    f1:ulfrec;
begin
   seek(ulff,0);
   read(ulff,f1);
   pl:=f1.blocks;

   for i:=pl downto 1 do begin
      seek(ulff,i);
      read(ulff,f1);
      seek(ulff,i+1);
      write(ulff,f1);
   end;
   writefv(1,f,v);
   inc(pl);
   f1.blocks:=pl;
   seek(ulff,0);
   write(ulff,f1);
end;

procedure doffstuff(var f:ulfrec; fn:astr; var gotpts:integer);
var rfpts:real;
begin
   f.filename:=align(fn);
   f.priv:=false;
   f.owner:=usernum;
   f.stowner:=strAllCaps(thisuser.name);
   f.date:=date;
   f.daten:=daynum(date);
   f.nacc:=0;
   if (not status^.fileptratio) then begin
      f.filepoints:=0;
      gotpts:=0;
   end
   else begin
      rfpts:=(f.blocks/8);{/status^.fileptcompbasesize;}
      f.filepoints:=round(rfpts);
      gotpts:=round(rfpts*status^.fileptcomp);
      if (gotpts<1) then gotpts:=1;
   end;
   f.filestat:=[];
   if (not fso) and (not status^.validateallfiles) then f.filestat:=f.filestat+[notval];
   f.ft:=255; {* ft; *}
end;

procedure arcstuff(var ok,convt:boolean;    { if ok - if converted }
                   var blks:integer;        { # blocks     }
                   var convtime:real;       { convert time }
                   itest:boolean;           { whether to test integrity }
                   fpath:astr;              { filepath     }
                   var fn:astr;             { filename     }
                   var descr:astr);         { description  }
var fi:file of byte;
    s,sig:astr;
    x,y,c:word;
begin
   convtime:=0.0;
   ok:=TRUE;
   assign(fi,fpath+fn);
   {$I-} reset(fi); {$I+}
   if (ioresult<>0) then blks:=0
   else begin
      blks:=trunc((filesize(fi)+127.0)/128.0);
      close(fi);
   end;

   fn:=sqoutsp(fn);

   if ((isgifext(fn)) and (fbusegifspecs in memuboard.fbstat)) then begin
      getgifspecs(memuboard.dlpath+fn,sig,x,y,c);
      s:='('+cstrl(x)+'x'+cstrl(y)+','+cstr(c)+'c) ';
      descr:=s+descr;
      if (length(descr)>60) then descr:=copy(descr,1,60);
   end;
end;

procedure fBaseList;
var b,b2 : integer;
begin
   b:=1;
   printf('filebase.top');
   while (b <= maxUlB) and (not hangup) do begin
      if (fbUnHidden in memuboard.fbstat) or (fBaseAc(b)) then begin
           b2:=ccboards[1][b];
           sysMciSet('FE',memuboard.name);
           sysMciSet('FI',cstr(b2));
           if (b in zscanr.fzscan) then sysMciSet('FS','Yes')
           else sysMciSet('FS','NO');
           printf('filebase.mid');
           sysResetMciList;
      end;
      inc(b);
   end;
  printf('filebase.bot');
end;
{procedure fbaselist;
var s,os:astr;
    onlin,nd,b,b2,i:integer;
    abort,next,acc,showtitles:boolean;

   procedure shortlist;
   begin
      nl;
      writehdr('Filebase List');
      while (b<=maxulb) and (not abort) do begin
         acc:=fbaseac(b); { fbaseac will load memuboard }
 {        if ((fbunhidden in memuboard.fbstat) or (acc)) then begin
            if (acc) then begin
               b2:=ccuboards[1][b];
               s:='|U0'+cstr(b2);
               if (b2<10) then s:=' '+s;
               if (b in zscanr.fzscan) then s:=s+'|U9 û'
               else s:=s+'  ';
            end
            else s:='    ';
            s:=s+' |U1'+memuboard.name;
            if (fbnoratio in memuboard.fbstat) then s:=s+#3#1+' <NR>';
            inc(onlin);
            inc(nd);
            if (onlin=1) then begin
               if (thisuser.linelen>=80) and (b<maxulb) and (pipelen(s)>40) then
                  s:=mlnmci(s,50);
               sprompt(s);
               os:=s;
            end
            else begin
               i:=40-pipelen(os);
               os:='';
               if (thisuser.linelen>=80) then begin
                  while (pipelen(os)<i) do os:=os+' ';
                  if (pipelen(s)>48) then s:=mlnmci(s,48);
               end
               else nl;
               sprint(os+s);
               onlin:=0;
            end;
            if (not empty) then wkey(abort,next);
         end;
         inc(b);
      end;
      if (onlin=1) and (thisuser.linelen>=80) then nl;
   end;

begin
   nl;
   abort:=FALSE;
   onlin:=0;
   s:='';
   b:=1;
   nd:=0;
   printf('fbaseall');
   if (not(exist(status^.afilepath+'fbaseall.*'))) then begin
      shortlist;
      if (nd=0) then sprompt(#3#7+'No file bases available.');
   end;
end;}

procedure unlisted_download(s:astr);
var dok,kabort:boolean;
    oldnumbatchfiles,oldfileboard:integer;
begin
   if (s<>'') then begin
      if (not exist(s)) then print('File not found.')
      else if (iswildcard(s)) then print('Can''t specify wildcards.')
      else begin
         oldnumbatchfiles:=numbatchfiles;
         oldfileboard:=fileboard;
         fileboard:=-1;
         send1(s,dok,kabort);
         if (numbatchfiles=oldnumbatchfiles) and (dok) and (not kabort) then
            dodl(5);
         fileboard:=oldfileboard;
      end;
   end;
end;

procedure do_unlisted_download;
var s:astr;
begin
   nl;
   print('Enter file name to download (x:\path\filename.ext)');
   prompt(':');
   input(s,78);
   unlisted_download(s);
end;

function nfvpointer:longint;
var i,x:integer;
    v:verbrec;
    vfo:boolean;
begin
   vfo:=(filerec(verbf).mode<>fmclosed);
   if (not vfo) then reset(verbf);
   x:=filesize(verbf);
   for i:=0 to filesize(verbf)-1 do begin
      seek(verbf,i);
      read(verbf,v);
      if (v.descr[1]='') then x:=i;
   end;
   if (not vfo) then close(verbf);
   nfvpointer:=x;
end;

end.
