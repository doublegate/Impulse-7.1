{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit file9;

interface

uses
   Crt,      Dos,      common5,  records,  myio,     common1,  common2,
   common3,  file0,    file1,    file2,    common,   strproc;

function info:astr;
procedure dir(cd,x:astr; expanded:boolean);
procedure dirf(expanded:boolean);
procedure deleteff(rn:integer; var pl:integer; killverbose:boolean);
procedure remove;
procedure setdirs;
procedure pointdate;

implementation

function align2(s:astr):astr;
begin
   align2:=mln(s,12);
end;

function info:astr;
var pm:char;
    i:integer;
    s:astr;
    dt:datetime;

   function ti(i:integer):astr;
   var s:astr;
   begin
      ti:=tch(cstr(i));
   end;

begin
   s:=dirinfo.name;
   if (dirinfo.attr and directory)=directory then s:=mln(s,13)+'<DIR>   '
   else s:=align2(s)+'  '+mrn(cstrl(dirinfo.size),7);
   unpacktime(dirinfo.time,dt);
   with dt do begin
      if hour<13 then pm:='a'
      else begin
         pm:='p';
         hour:=hour-12;
      end;
      s:=s+'  '+mrn(cstr(month),2)+'-'+ti(day)+'-'+ti(year-1900)+ '  '+mrn(cstr(hour),2)+':'+ti(min)+pm;
   end;
   info:=s;
end;

procedure dir(cd,x:astr; expanded:boolean);
var abort,next,nofiles:boolean;
    s:astr;
    onlin:integer;
    ts : string[80];
    dfs:longint;
    numfiles:integer;
begin
   if (copy(cd,length(cd),1)<>'\') then cd:=cd+'\';
   abort:=FALSE;
   cd:=cd+x;
   if (fso) then begin
      printacr(' Directory of  '+copy(cd,1,length(cd)),abort,next);
      nl;
   end;
   s:='';
   onlin:=0;
   numfiles:=0;
   nofiles:=TRUE;
   ffile(cd);
   while (found) and (not abort) do begin
      if (not (dirinfo.attr and directory=directory)) or (fso) then
         if (not (dirinfo.attr and volumeid=volumeid)) then
            if (not (dirinfo.attr and dos.hidden=dos.hidden)) then
               if ((dirinfo.attr and dos.hidden=dos.hidden) and
                  (not (dirinfo.attr and directory=directory))) or
                  (not (dirinfo.attr and dos.hidden=dos.hidden)) then begin
                  nofiles:=FALSE;
                  if (expanded) then printacr(lowcase(info),abort,next)
                  else begin
                     inc(onlin);
                     if (dirinfo.attr=$10) then s:=s+strallcaps(mln('['+(dirinfo.name)+']',12))
                     else s:=s+align2(lowcase(dirinfo.name));
                     if onlin<>5 then s:=s+'    '
                     else begin
                        printacr(lowcase(s),abort,next);
                        s:='';
                        onlin:=0;
                     end;
                  end;
                  inc(numfiles);
               end;
               nfile;
   end;
   if (not found) and (onlin in [1..5]) then printacr(lowcase(s),abort,next);
   if (nofiles) then s:='File not found'
   else s:=mrn(cstr(numfiles)+' File(s) ',17);
   sprint(s+'|NL'+strInsertComma(cStrL(freek(exdrv(cd))))+' bytes free');
end;

procedure dirf(expanded:boolean);
var fspec:astr;
    abort,next:boolean;
begin
   nl;
   print('Raw directory.');
   gfn(fspec);
   abort:=FALSE;
   next:=FALSE;
   nl;
   loaduboard(fileboard);
   dir(memuboard.dlpath,fspec,expanded);
end;

procedure deleteff(rn:integer; var pl:integer; killverbose:boolean);
var i:integer;
    f:ulfrec;
    v:verbrec;
begin
   if (rn<=pl) and (rn>0) then begin
      dec(pl);
      seek(ulff,rn);
      read(ulff,f);
      if (f.vpointer<>-1) and (killverbose) then begin
         assign(verbf,status^.gfilepath+'verbose.dat');
         reset(verbf);
         seek(verbf,f.vpointer);
         read(verbf,v);
         if (ioresult=0) then begin
            v.descr[1]:='';
            seek(verbf,f.vpointer);
            write(verbf,v);
         end;
         close(verbf);
      end;
      for i:=rn to pl do begin
         seek(ulff,i+1);
         read(ulff,f);
         seek(ulff,i);
         write(ulff,f);
      end;
      seek(ulff,0);
      f.blocks:=pl;
      write(ulff,f);
   end;
end;

procedure remove;
var done,abort,next,subit:boolean;
    c:char;
    pl,rn:integer;
    s,fn:astr;
    ff:file;
    f:ulfrec;
    u:userrec;
begin
   nl;
   print('Remove files.');
   gfn(fn);
   abort:=FALSE;
   next:=FALSE;
   nl;
   recno(fn,pl,rn);
   if (baddlpath) then exit;
   if (fn='') or (pos('.',fn)=0) or (rn=0) then print('No matching files.')
   else begin
      lastcommandovr:=TRUE;
      c:=#0;
      while (fn<>'') and (rn<>0) and (not abort) and (not hangup) do begin
         seek(ulff,rn);
         read(ulff,f);
         reset(uf);
         seek(uf,f.owner);
         read(uf,u);
         if (rn<>0) then begin
            done:=FALSE;
            repeat
               if (c<>'?') then begin
                  nl;
                  fileinfo(f,FALSE,abort,next);
                  nl;
               end;
               if (next) then c:='N'
               else begin
                  prompt('Remove files (?=help) : ');
                  if (f.owner<>usernum) and (not fso) then onek(c,'QN?'^M)
                  else onek(c,'QDN?'^M);
               end;
               case c of
                  ^M:c:=#0;  {* do nothing *}
                  '?':begin
                     nl;
                     print('<CR>Redisplay entry');
                     if (f.owner<>usernum) and (not fso) then lcmds(12,3,'Next file','Quit')
                     else begin
                        lcmds(12,3,'Delete file','Next file');
                        lcmds(12,3,'Quit','');
                     end;
                     nl;
                  end;
                  'D': if (f.owner<>usernum) and (not fso) then sprint(#3#7+'You can''t delete this!!')
                     else begin
                        deleteff(rn,pl,TRUE);
                        dec(lrn);
                        s:='Removed "'+sqoutsp(f.filename)+'" from Dir#'+cstr(fileboard);
                        nl;
                        if (not exist(memuboard.dlpath+f.filename)) then sprint(#3#5+'File does not exist!')
                        else if (fso) then
                           if pynq('Erase file too? ',false) then begin
                              assign(ff,memuboard.dlpath+f.filename);
                              {$I-} erase(ff); {$I+}
                              if (ioresult=0) then s:=s+' [FILE DELETED]'
                              else s:='Tried deleting "'+sqoutsp(f.filename)+'" from Dir#'+cstr(fileboard);
                           end;
                           subit:=(strAllcaps(f.stowner)=strallcaps(u.name));
                           if (fso) then
                              if (not subit) then begin
                                 print('Uploader name does not match user name!');
                                 print('Cannot remove credit from user.');
                              end
                              else subit:=pynq('Remove from '+#3#5+caps(u.name)+' #'+cstr(f.owner)+#3#7+'''s ratio? ',false);
                           if (subit) then begin
                              if (f.owner=usernum) then u:=thisuser;
                              with u do begin
                                 uk:=uk-(f.blocks div 8);
                                 dec(uploads);
                                 if (uk<0) then uk:=0;
                                 if (uploads<0) then uploads:=0;
                              end;
                              seek(uf,f.owner);
                              write(uf,u);
                              if (f.owner=usernum) then thisuser:=u;
                           end;
                           sysoplog(s);
                           done:=TRUE;
                  end;
                  else done:=TRUE;
               end;
            until ((done) or (hangup));
            abort:=FALSE; next:=FALSE;
            if (c='Q') then abort:=TRUE;
            if (c<>'?') then nrecno(fn,pl,rn);
         end;
      end;
      reset(uf);
      close(uf);
      close(ulff);
   end;
end;

procedure setdirs;
var s:astr;
    i:integer;
    done:boolean;
begin
   nl;
   if (novice in thisuser.ac) then begin
      fbaselist;
      nl;
   end;
   done:=FALSE;
   repeat
      prompt('Set NewScan file bases (Q=Quit,?=List,#=Toggle base) : ');
      input(s,3);
      if (s='Q') then done:=TRUE;
      if (s='?') then begin
         fbaselist;
         nl;
      end;
      i:=ccuboards[0][value(s)];
      if (fbaseac(i)) then { loads memuboard }
         if (i>=0) and (i<=maxulb) and (length(s)>0) and (s[1] in ['0'..'9']) then begin
            nl;
            sprompt(#3#5+memuboard.name+#3#3);
            if (i in zscanr.fzscan) then begin
               sprint(' will NOT be scanned.');
               zscanr.fzscan:=zscanr.fzscan-[i];
            end
            else begin
               sprint(' WILL be scanned.');
               zscanr.fzscan:=zscanr.fzscan+[i];
            end;
            nl;
         end;
   until (done) or (hangup);
   lastcommandovr:=TRUE;
   savezscanr;
end;

procedure pointdate;
var s:astr;
begin
   nl;
   print('Enter limiting date for new files -');
   print('Date is currently set to '+newdate);
   nl;
   prompt('(mm/dd/yy): ');
   input(s,8);
   if (daynum(s)=0) then print('Illegal date.')
   else newdate:=s;
   nl;
   print('Current limiting date is '+newdate);
end;


end.
