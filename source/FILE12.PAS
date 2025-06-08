{$A+,B+,D-,F+,I+,L+,O+,R-,S-,V-}
unit file12;

interface

uses
   Crt,      Dos,      records,  file0,    file8,    file1,    file2,
   file4,    file6,    file9,    execbat,  file3,    common1,  common2,
   common3,common5,    mmodem,   common;

procedure delubatch(n:integer);
procedure listubatchfiles;
procedure removeubatchfiles;
procedure clearubatch;
procedure batchul;
procedure batchinfo;

implementation

procedure delubatch(n:integer);
var c:integer;
begin
  if ((n>=1) and (n<=numubatchfiles)) then begin
    if (n<>numubatchfiles) then
      for c:=n to numubatchfiles-1 do ubatch[c]:=ubatch[c+1];
    dec(numubatchfiles);
  end;
end;

procedure listubatchfiles;
var s,s1:astr;
    i,j:integer;
    abort,next,vfo:boolean;
begin
  if (numubatchfiles=0) then begin
    nl; print('Upload batch queue empty.');
  end else begin
    abort:=FALSE; next:=FALSE;
    nl;
    printacr(#3#3+'#  Filename.Ext Area Description',abort,next);
    printacr(#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴',abort,next);

    i:=1;
    while ((not abort) and (i<=numubatchfiles) and (not hangup)) do begin
      with ubatch[i] do begin
        if (section=status^.tosysopdir) then s1:=#3#7+'Sysp'
          else s1:=mrn(cstr(section),4);
        s:=#3#3+mn(i,2)+#3#4+':'+#3#5+align(fn)+' '+s1+' '+
           #3#3+mln(description,55);
        printacr(s,abort,next);
        if (vr<>0) then
          if (ubatchv[vr]^.descr[1]<>'') then begin
            vfo:=(filerec(verbf).mode<>fmclosed);
            if (not vfo) then reset(verbf);
            if (ioresult=0) then
              for j:=1 to 4 do
                if ubatchv[vr]^.descr[j]='' then j:=4 else
                  printacr('                         '+#3#2+':'+
                           #3#4+ubatchv[vr]^.descr[j],abort,next);
            if (not vfo) then close(verbf);
          end;
      end;
      inc(i);
    end;

    printacr(#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴',abort,next);
  end;
end;

procedure removeubatchfiles;
var s:astr;
    i:integer;
begin
  if (numubatchfiles=0) then begin
    nl; print('Upload batch queue empty.');
  end else
    while (s<>'?') do
    begin
      nl;
      prt('File # to remove (1-'+cstr(numubatchfiles)+') (?=list) : ');
      input(s,2); i:=value(s);
      if (s='?') then listubatchfiles;
      if ((i>0) and (i<=numubatchfiles)) then begin
        print('"'+stripname(ubatch[i].fn)+'" deleted out of upload queue.');
        delubatch(i);
      end;
      if (numubatchfiles=0) then print('Upload queue now empty.');
    end;
end;

procedure clearubatch;
begin
  nl;
  if pynq('Clear upload queue? ',false) then begin
    numubatchfiles:=0;
    print('Upload queue now empty.');
  end;
end;


procedure batchul;
label ending;
var fi:file of byte;
    foundfile:file;
    dirinfo:searchrec;
    f:ulfrec;
    v:verbrec;
    xferstart,xferend,tooktime,takeawayulrefundgot1,ulrefundgot1:datetimerec;
    tconvtime1,st1:datetimerec;
    bb:string;
    pc,fn,s:astr;
    st,tconvtime,convtime,ulrefundgot,takeawayulrefundgot:real;
    totb,totfils,totb1,totfils1,cps,lng,totpts:longint;
    i,p,hua,pl,dbn,blks,gotpts,ubn,filsuled,oldboard,passn:integer;
    c:char;
    abort,ahangup,next,done,dok,kabort,wenttosysop,ok,convt,hangafter,
    foundit,hungafter,beepafter,dothispass,fok,nospace,savpause:boolean;

  function notinubatch(fn:astr):boolean;
  var i:integer;
  begin
    notinubatch:=FALSE;
    for i:=1 to numubatchfiles do
      if (sqoutsp(fn)=sqoutsp(ubatch[i].fn)) then exit;
    notinubatch:=TRUE;
  end;

  function ubatchnum(fn:astr):integer;
  var i:integer;
  begin
    fn:=sqoutsp(fn);
    ubatchnum:=0;
    for i:=1 to numubatchfiles do
      if (fn=sqoutsp(ubatch[i].fn)) then ubatchnum:=i;
  end;

  function plural:string;
  begin
    if (totfils<>1) then plural:='s' else plural:='';
  end;

  var ndtemp:string;
      tmp,ndtmpn:integer;
      nosysdir:boolean;

begin
  nosysdir:=false;

  if thisnode>1 then begin
    ndtemp:=status^.temppath;
    ndtmpn:=length(status^.temppath);  (* Multi-Node crapola *)
    delete(status^.temppath,ndtmpn,1);
    status^.temppath:=status^.temppath+cstr(thisnode)+'\';
  end;

  hangafter:=false;

  savpause:=(pause in thisuser.ac);
  if (savpause) then thisuser.ac:=thisuser.ac-[pause];

  oldboard:=fileboard;
  beepafter:=FALSE; done:=FALSE;
  sprompt(strings^.uploadLine);
{  if exist(status^.afilepath+'batchul.*') then printf('batchul') else
    writehdr('Upload');}
  reset(xf);
  done:=FALSE;
  foundit:=true;
  while (not done) and (not hangup) do
  begin
    nl;
    sprompt(strings^.protselect); mpkey(s);
    if (s='?') then begin
      nl;
      showprots(TRUE,FALSE,TRUE,FALSE);
    end else begin
      p:=findprot(s,TRUE,FALSE,TRUE,FALSE);
      if (p=-99) then print('Invalid entry.') else done:=TRUE;
    end;
  end;
  if (p<>-10) then begin
    seek(xf,p); read(xf,protocol); close(xf);
    nl;
    hangafter:=pynq(strings^.logoffafter,false);
    dok:=TRUE;
    begin

      lil:=0;
      nl; nl;
      if (useron) then print('Ready to receive batch queue!');
      lil:=0;

      getdatetime(xferstart);
      if (useron) then shel(caps(thisuser.name)+' is batch uploading!')
                  else shel('Receiving file(s)...');
      execbatch(dok,FALSE,'tgtemp1.bat','tgtest1.$$$',status^.temppath+'2\',
                bproline1(protocol.ulcmd),-1);
      shel2;
      getdatetime(xferend);
      timediff(tooktime,xferstart,xferend);

      showuserfileinfo;

      ulrefundgot:=(dt2r(tooktime))*(status^.ulrefund/100.0);
      freetime:=freetime+ulrefundgot;

      {*****}

      lil:=0;
      nl;
      lil:=0;

      tconvtime:=0.0; takeawayulrefundgot:=0.0;
      totb:=0; totfils:=0; totb1:=0; totfils1:=0; totpts:=0;

      findfirst(status^.temppath+'2\*.*',anyfile-directory,dirinfo);
      while (doserror=0) do begin
        inc(totfils1);
        inc(totb1,dirinfo.size);
        findnext(dirinfo);
      end;
      cps:=trunc(totb1/dt2r(tooktime));

      abort:=FALSE; next:=FALSE;

      if (totfils1=0) then begin
        sprint('No files detected!  Transfer aborted.');
        exit;
      end;

      if (hangafter) then hangup:=TRUE;

      ahangup:=FALSE;
      if (hangup) then begin
        if (spd<>'KB') then begin
          commandline('Hanging up and taking phone off hook...');
          dophonehangup(FALSE);
          dophoneoffhook(FALSE);
          spd:='KB';
        end;
        hangup:=FALSE; ahangup:=TRUE;
      end;

      r2dt(ulrefundgot,ulrefundgot1);

      fiscan(pl);

      for passn:=1 to 2 do begin
        findfirst(status^.temppath+'2\*.*',anyfile-directory,dirinfo);
        while (doserror=0) do begin
          fn:=sqoutsp(dirinfo.name);
          nl;
          dothispass:=FALSE;
          if (notinubatch(fn)) then begin
            ubn:=0;
            dothispass:=TRUE;

            close(ulff); fiscan(pl);
            f.filename:=fn;
            bb:=status^.temppath+'2\';
            inc(thisuser.uploads);
            inc(status^.todayzlog.uploads);
            hungafter:=false;
            if (ahangup) then begin
              hungafter:=true;
              dodescrs(f,v,pl,wenttosysop,hungafter,bb,false);
            end else dodescrs(f,v,pl,wenttosysop,hungafter,bb,true);
            doffstuff(f,fn,gotpts);
            doul(gotpts);
            wenttosysop:=true;
            if (status^.tosysopdir=255) or (status^.tosysopdir=0)
              then fileboard:=oldboard else
                 fileboard:=status^.tosysopdir;
            {$I-}
            close(ulff);
            fiscan(pl);
            {$I+}
            arcstuff(ok,convt,blks,convtime,TRUE,status^.temppath+'2\',
                     fn,f.description);
            tconvtime:=tconvtime+convtime; f.blocks:=blks;
            thisuser.uk:=thisuser.uk+(f.blocks div 8);
            inc(status^.todayzlog.uk,(f.blocks div 8));
            inc(thisuser.uploads);
            doffstuff(f,fn,gotpts);

            fok:=TRUE;
            loaduboard(fileboard);
            if (ok) then begin
              Sprint('Moving file to '+#3#5+memuboard.name);
              sprompt(#3#5+'Progress: ');
              movefile(fok,nospace,TRUE,status^.temppath+'2\'+fn,memuboard.dlpath+fn);
              if (fok) then begin
                nl;
                newff(f,v);
                sysoplog(#3#3+'Batch uploaded "'+sqoutsp(fn)+'" on '+
                         memuboard.name);
                inc(totfils);
                lng:=blks; lng:=lng*128;
                inc(totb,lng);
                inc(totpts,gotpts);
              end else begin
                sysoplog(#3#3+'Error moving batch upload "'+sqoutsp(fn)+'" into directory');
              end;
            end else begin
              if ((thisuser.sl>0) and
                  (f.blocks div 8>status^.minresume)) then begin
                nl;
                if pynq('Save file for a later resume? ',false) then begin
                  sprompt(#3#5+'Progress: ');
                  movefile(fok,nospace,TRUE,status^.temppath+'2\'+fn,memuboard.dlpath+fn);
                  if (fok) then begin
                    nl;
                    doffstuff(f,fn,gotpts);
                    f.filestat:=f.filestat+[resumelater];
                    newff(f,v);
                    s:='file saved for later resume';
                  end else begin
                    sysoplog(#3#3+'Error moving batch upload "'+sqoutsp(fn)+'" into directory');
                  end;
                end;
              end;
              if (not (resumelater in f.filestat)) then begin
                s:='file deleted';
                assign(fi,status^.temppath+'2\'+fn); erase(fi);
              end;
              sysoplog(#3#3+'Errors batch uploading "'+sqoutsp(fn)+'" - '+s);
            end;

            if (not ok) then begin
              st:=(rte*f.blocks);
              takeawayulrefundgot:=takeawayulrefundgot+st;
              r2dt(st,st1);
            end else
              if (ubn<>0) then delubatch(ubn);
          end;
          findnext(dirinfo);
        end;
      end;

      close(ulff);
      fileboard:=oldboard;
      fiscan(pl); close(ulff);

      if (not foundit) then begin
        nl;
        if (totfils<>totfils1) then
          r2dt(tconvtime,tconvtime1);
        r2dt(ulrefundgot,ulrefundgot1);
      end;

      if (choptime<>0.0) then begin
        choptime:=choptime+ulrefundgot;
        freetime:=freetime-ulrefundgot;
      end;

      if (takeawayulrefundgot<>0.0) then begin
        nl;
        r2dt(takeawayulrefundgot,takeawayulrefundgot1);
        freetime:=freetime-takeawayulrefundgot;
      end;

      if (ahangup) then begin
        commandline('Hanging up phone again...');
        dophonehangup(FALSE);
        hangup:=TRUE;
      end;

    end;
  end;
  if (savpause) then thisuser.ac:=thisuser.ac+[pause];
end;


procedure batchinfo;
var anyyet:boolean;
   procedure sayit(s:string);
   begin
      if (not anyyet) then begin
         anyyet:=TRUE;
         nl;
      end;
      sprint(s);
   end;

begin
  anyyet:=FALSE;
  if (numbatchfiles<>0) then
    sayit(#3#3+'You have '+cstr(numbatchfiles)+
          #3#3+' file'+aonoff(numbatchfiles<>1,'s','')+
               ' left in your download batch queue.');
  if (numubatchfiles<>0) then
    sayit(#3#3+'You have '+cstr(numubatchfiles)+
          #3#3+' file'+aonoff(numubatchfiles<>1,'s','')+
               ' left in your upload batch queue.');
end;

end.

