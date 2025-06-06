{$A+,B+,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit file11;

interface

uses

  Crt, Dos,strproc,
  common4,common5,records,
  myio,common1,common2,common3,
  file0, file1,
  common;

function cansee(f:ulfrec):boolean;
procedure pbn;
procedure pfn(fnum:integer; f:ulfrec; var abort,next:boolean);
procedure searchb(b:integer; fn:astr; filestats:boolean; var abort,next:boolean; newf:boolean);
procedure search;
procedure listfiles;
procedure searchbd(b:integer; ts:astr; var abort,next:boolean);
procedure searchd;
{procedure newfiles(b:integer; var abort,next:boolean);}
procedure gnfiles;
procedure nf(mstr:astr);
procedure fbasechange(var done:boolean; mstr:astr);

implementation

uses file4, file8, file12, menus, menus2;

function cansee(f:ulfrec):boolean;
begin
   cansee:=((not (notval in f.filestat)) or (aacs(status^.seeunval)));
end;

function isulr:boolean;
begin
   isulr:=((status^.uldlratio) and (not status^.fileptratio));
end;

procedure pbn;
begin
   if (not bnp) then begin
      printF('filehdr');
   end;
   bnp:=TRUE;
end;

procedure pfndd(fnum:integer; ts:astr; f:ulfrec; var abort,next:boolean);
var rr,s,s1,dd,dd2:astr;
    v:verbrec;
    u:userrec;
    li:longint;
    i:integer;
    vfo:boolean;
    lst,iii,ldescp:integer;

begin
  li:=f.blocks; li:=li*128;
  s:=#3#2+mln(cstr(fnum),3)+#3#7+' ';
  iii:=1;
  if thisuser.fflag=thisuser.fflag+[fname] then s:=s+mln(f.filename,12);
  s:=s+#3#3+' ';
  if thisuser.fflag=thisuser.fflag+[fsize] then
    if (f.priv) then s:=s+mln('Private',7) else s:=s+mln(cstr(li),7);
  s:=s+#3#7+' ';
  if thisuser.fflag=thisuser.fflag+[fpts] then s:=s+mln(cstr(f.filepoints),3);
    {s:=ptsf;}
  s:=s+#3#3+' ';
  iii:=length(s);
  if thisuser.fflag=thisuser.fflag+[fdesc] then begin s:=s+mln(stripansi(f.description),45);
  s:=s+#3#3+' ';
  printacr(s,abort,next);
  if thisuser.fflag=thisuser.fflag+[extdesc] then begin
  if (f.vpointer<>-1) then begin
    vfo:=(filerec(verbf).mode<>fmclosed);
    {$I-} if (not vfo) then reset(verbf); {$I+}
    if (ioresult=0) then begin
      {$I-} seek(verbf,f.vpointer); read(verbf,v); {$I+}
      if (ioresult=0) then
        lst:=10;
        if thisuser.fflag=thisuser.fflag+[whoul] then dec(lst);
        if thisuser.fflag=thisuser.fflag+[dateul] then dec(lst);
        if thisuser.fflag=thisuser.fflag+[numdl] then dec(lst);
        for i:=1 to lst do
          if (v.descr[i]='') then i:=lst
          else begin
            dd:=#3#1+v.descr[i];
            if thisuser.fflag=thisuser.fflag+[extdesc] then
              {for ldescp:=1 to iii-10 do sprompt(' ');}
              sprompt(mln(' ',iii-10));
            printacr(mln(dd,45),abort,next);
          end;
      if (not vfo) then close(verbf);
    end;
  end;
 end;
  if thisuser.fflag=thisuser.fflag+[whoul] then begin
     for ldescp:=1 to iii-8 do sprompt(' '); printacr(#3#0+'Uploaded by'+#3#9+':'+#3#3+' '+caps(f.stowner),abort,next);
     end;
  if thisuser.fflag=thisuser.fflag+[dateul] then begin
     for ldescp:=1 to iii-8 do sprompt(' '); printacr(#3#0+'Uploaded on'+#3#9+':'+#3#3+' '+f.date,abort,next);
     end;
  if thisuser.fflag=thisuser.fflag+[numdl] then begin
     for ldescp:=1 to iii-8 do sprompt(' '); printacr(#3#0+'Number of D/L''s'+#3#9+':'+#3#3+' '+cstr(f.nacc),abort,next);
     end;
 end;
end;

procedure pfn(fnum:integer; f:ulfrec; var abort,next:boolean);
begin
   pfndd(fnum,'',f,abort,next);
end;

procedure searchb(b:integer; fn:astr; filestats:boolean; var abort,next:boolean; newf:boolean);
var f:ulfrec;
    v:verbrec;
    pp,li,totfils,lastfile,totsize:longint;
    oldboard,pl,rn:integer;
    filenum:array[1..20] of integer;
    fpos:array[1..20] of integer;
    fnum:array[1..20] of integer;
    s:astr;
    bk,temp,yy,tmp,tmppos,tmp2,tmp3,i,a1,a2,
           bx,by,numadd,savflistopt:integer;
    r,c:char;
    oldpage:byte;
    filep : string;
    jump,showp,vfo,botlist,eol,bb,abort1,done,done1,showlist:boolean;
begin
   bb:=false;
   eol:=false;
   botlist:=false;
   last_menu := curmenu;
   curmenu := status^.menupath+'FILEP.MNU';
   filemnu := true;
   readin;
   showp := false;
   fillchar(fileNum,20,0);
   fillChar(fPos,20,0);
   fillChar(fNum,20,0);
   tmppos:=0;
   oldboard:=fileboard;
   if (fileboard<>b) then changefileboard(b);
   if (fileboard=b) then begin
      totfils:=0;
      totsize:=0;
      lastfile:=0;
      recno(fn,pl,rn);
      if (baddlpath) then begin
         curmenu := last_menu;
         readin;
         filemnu := false;
         exit;
      end;
    while ((rn<=pl) and (not abort) and (not hangup) and (rn<>0)) do begin
      yy:=0;
      showp := false;
      seek(ulff,rn); read(ulff,f);
      if (not newf) then
        f.daten := daynum(date);
      if ((cansee(f)) and (f.daten>=daynum(newdate))) then
        begin
        pbn;
        lil := 0;
        yy:=wherey;
        temp:=yy;
        inc(tmppos);
        if (tmppos>=20) then tmppos:=0;
        fpos[tmppos]:=wherey;
        pfn(rn,f,abort,next);
        fnum[tmppos]:=rn;
        tmp2:=wherey;
        if (tmppos=1) then begin
          goxy(1,fpos[tmppos]);
          sprompt('|U6'+mln(cstr(rn),3)+'|U3')
        end;
        goxy(1,tmp2);
        inc(lastfile);
        if (filestats) then begin
          inc(totfils);
          li:=f.blocks; li:=li*128; inc(totsize,li);
        end;
      showp:=false;
      {test code}
      if (rn<pl) then begin
        seek(ulff,rn+1); read(ulff,f);        tmp:=0;
      if (f.vpointer<>-1) then begin
        vfo:=(filerec(verbf).mode<>fmclosed);
        if (not vfo) then reset(verbf);
        if (ioresult=0) then begin
          seek(verbf,f.vpointer); read(verbf,v);
          if (ioresult=0) then begin
            repeat inc(tmp); until length(v.descr[tmp])=0;
            yy:=wherey;
            inc(yy,tmp);
            if thisuser.fflag=thisuser.fflag+[whoul] then inc(yy);
            if thisuser.fflag=thisuser.fflag+[dateul] then inc(yy);
            if thisuser.fflag=thisuser.fflag+[numdl] then inc(yy);
            if (yy>=20) then showp:=true;
            yy:=temp;
          end;
        end;
        if (not vfo) then close(verbf);
      end;
      seek(ulff,rn-1); read(ulff,f);
      {test code}
      end;
    end;
    nrecno(fn,pl,rn);
    if (rn>pl) then abort:=true;
    if (yy>=thisuser.pagelen) or
       (lil>=20) or (showp) or (rn = 0)
    then begin
    bx:=wherex; by:=wherey;
    done:=false;
    oldboard:=fileboard;
    tmp2:=tmppos;
    tmp3:=1;
    tmppos:=0;
    abort:=false;
    repeat
      goxy(bx,by);
      filepmnu := #0;
      filep := '';
      mainmenuhandle(filep);
      c := filepmnu;
      s:='';
      case c of
      ' ':begin
            i:=fnum[tmp3];
            if (i>=1) and (i<=pl) then begin
              filenum[1]:=i;
              numadd:=1;
            if (checkfileratio=0) then begin
            a2:=0;
            seek(ulff,filenum[1]); read(ulff,f);
            a1:=numbatchfiles;
            if (okdl(f)) then ymbadd(memuboard.dlpath+f.filename);
            sprint(strings^.fileaddedtobq);
            if (numbatchfiles<>a1) then inc(a2);
            end else
              sprint(strings^.unbalance);
            end;
          end;
       end;
      goxy(1,21);
      if length(filep) > 1 then
      case filep[2] of
  {up}'A':begin
            if fnum[2]<>0 then begin
            dec(tmp3);
            if tmp3<=0 then tmp3:=1;
            if tmp3>0 then begin
              goxy(1,fpos[tmp3]);
              sprompt('|U6'+mln(cstr(fnum[tmp3]),3)+'|U3');
              goxy(1,fpos[tmp3+1]);
              sprompt('|U2'+mln(cstr(fnum[tmp3+1]),3)+'|U3');
            end;
            end;
          filep := '';
          end;
{down}'B':begin
            if fnum[2]<>0 then begin
            inc(tmp3);
            if tmp3>tmp2 then tmp3:=tmp2;
            if tmp3<=tmp2 then begin
              goxy(1,fpos[tmp3]);
              sprompt('|U6'+mln(cstr(fnum[tmp3]),3)+'|U3');
              goxy(1,fpos[tmp3-1]);
              sprompt('|U2'+mln(cstr(fnum[tmp3-1]),3)+'|U3');
            end;
            end;
          filep := '';
          end;
      end;
      goxy(bx,by);
      i:=fnum[tmp3];
      filepmnu := #0;
    filep := '';
    until (c in ['L','F','+','?','J','V','Q','T','I','R','S','N']);
    repeat
    goxy(bx,by);
    i:=fnum[tmp3];
    if (c in ['F','+','?','R','S','V','T','I']) then
      begin
      rn:=fnum[1];
      lrn := rn;
      jump:=true;
      bk:=rn;
      showlist:=TRUE;
      end;
    if (not (c in ['L','F','+','?','J','R','S','V','Q','T','I','N'])) then
    begin
      showlist:=true;
      done:=true;
    end else
    case c of
      'I':begin
            sprompt('|NL|U0Info on which file |U9[|U2CR='+cstr(i)+'|U9]?|U3 ');
            s:='';
            input(s,4);
            if (s<>'') then i:=value(s);
            if (i>=1) and (i<=pl) then begin
              seek(ulff,i); read(ulff,f);
              fileinfo(f,FALSE,abort1,next);
              pausescr;
            end;
            done:=true;
          end;
      'J':begin
            sprompt('|NL|U0Jump to which file |U9[|U0CR='+cstr(i)+'|U9] |U9(|U01|U9,|U0'+cstr(pl)+'|U9)? ');
            s:='';
            input(s,4);
            if (s<>'') then i:=value(s);
            if (i>=1) and (i<=pl) then begin
            rn:=i;
            lrn := i;
            jump:=true;
            bk:=rn;
            {seek(ulff,i); read(ulff,f);}
            showlist:=TRUE;
            end;
            done:=true;
          end;
      '?':begin
            nl;
            lcmds(9,3,'Download','Upload');
            lcmds(9,3,'Jump to file','Type file');
            lcmds(9,3,'Upload','Flag or + to tag files');
            lcmds(9,3,'Quit Listing','View interior');
            pausescr;
            done:=true;
          end;
      'T':begin
            sprompt('|NL|U0Type Which File |U9[|U2CR='+cstr(i)+'|U9]?|U3 ');
            s:='';
            input(s,4);
            if (s<>'') then i:=value(s);
            seek(ulff,i); read(ulff,f);
            printfile(memuboard.dlpath+f.filename);
            pausescr;
            done:=true;
          end;
   'L','N':begin
            for tmp2:=1 to 20 do fpos[tmp2]:=0;
            for tmp2:=1 to 20 do fnum[tmp2]:=0;
            showlist:=true;
            done:=true;
          end;  {* do nothing *}
  '+','F':begin
            sprompt('|NL|U0Flag Which File |U9[|U2CR='+cstr(i)+'|U9]?|U3 ');
            s:='';
            input(s,4);
            if (s<>'') then i:=value(s);
            if (i>=1) and (i<=pl) then begin
              filenum[1]:=i;
              numadd:=1;
            if (checkfileratio=0) then begin
            a2:=0;
            seek(ulff,filenum[1]); read(ulff,f);
            a1:=numbatchfiles;
            if (okdl(f)) then ymbadd(memuboard.dlpath+f.filename);
            sprint(strings^.fileaddedtobq);
            if (numbatchfiles<>a1) then inc(a2);
            end else
              sprint(strings^.unbalance);
            end;
            done:=true;
          end;
      'R':begin
          sprompt('|NL|U0Download Which File |U9[|U2CR='+cstr(i)+'|U9]?|U3 ');
          s:='';
          input(s,4);
          if (s<>'') then i:=value(s);
          if ((i>=1) and (i<=pl)) then begin
            seek(ulff,i); read(ulff,f);
            abort1:=FALSE;
            dlx(f,i,abort1);
          end;
          done:=true;
          end;
      'S':begin
            batchul;
            fiscan(pl);
            done:=true;
          end;
      'V':begin
            sprompt('|NL|U0View Which File |U9[|U2CR='+cstr(i)+'|U9]?|U3 ');
            s:='';
            input(s,4);
            if (s<>'') then i:=value(s);
            if (i>=1) and (i<=pl) then begin
              abort1:=FALSE; next:=FALSE;
              lfin(i,abort1,next);
            end;
            fiscan(pl);
            pausescr;
            done:=true;
          end;
      'Q':begin done:=true; bb:=true; abort:=true; end;
    end;
   if rn = 0 then abort := true;
   if (not bb) and (not abort) then begin printf('filehdr'); end;
  until (done) or (hangup) or (abort);
  end;
    end;
    nl;
    close(ulff);
  end;
  fileboard:=oldboard;
  curmenu := last_menu;
  readin;
  filemnu := false;
end;

procedure search2(b:integer; fn:astr; filestats:boolean; var abort,next:boolean);
var f:ulfrec;
    pp,li,totfils,lastfile,totsize:longint;
    x,y,oldboard,pl,rn:integer;
    s:astr;
    i,a1,a2,numadd,savflistopt:integer;
    r,c:char;
    oldpage:byte;
    abort1,done,done1,showlist:boolean;

begin
  oldboard:=fileboard;
  if (fileboard<>b) then changefileboard(b);
  loaduboard(b);
  sprint('|U0Searching |U9[|U3'+memuboard.name+'|U9]|U0');
  if (fileboard=b) then begin
    totfils:=0; totsize:=0;
    lastfile:=0;
    recno(fn,pl,rn);
    if (baddlpath) then exit;
    while ((rn<=pl) and (not abort) and (not hangup) and (rn<>0)) do begin
      seek(ulff,rn); read(ulff,f);
      if (cansee(f)) then begin
        {pbn;}
        pfn(rn,f,abort,next);
        sprompt(strings^.numisc1);
        x:=wherex; y:=wherey;
        onek(c,'AQ'^M);
        case c of
        'A':begin
              ymbadd(memuboard.dlpath+f.filename);
              sprint(strings^.fileaddedtobq);
              goxy(x,y); nl;
            end;
        'Q':abort:=true;
        end;
        inc(lastfile);
        if (filestats) then begin
          inc(totfils);
          li:=f.blocks; li:=li*128; inc(totsize,li);
        end;
      end;
      nrecno(fn,pl,rn);
    done:=false;
    oldboard:=fileboard;
    end;
    close(ulff);
  end;
  fileboard:=oldboard;
end;


procedure search;
var fn:astr;
    bn:integer;
    abort,next:boolean;
begin
  nl;
  sprint(strings^.searchline);
  sprint(strings^.pninfo);
  nl; gfn(fn);
  bn:=1; abort:=FALSE; next:=FALSE;
  while (not abort) and (bn<=maxulb) and (not hangup) do begin
    if (fbaseac(bn)) then search2(bn,fn,FALSE,abort,next);
    inc(bn);
    wkey(abort,next);
    if (next) then begin abort:=FALSE; next:=FALSE; end;
  end;
end;

procedure listfiles;
var fn:astr;
    blah:integer;
    r,abort,next:boolean;
begin
   sprint(strings^.listline);
   gfn(fn);
   abort:=FALSE;
   r:=false;
   blah:=thisuser.pagelen;
   thisuser.pagelen:=20 - 3{20};
   if (thisuser.ac=thisuser.ac+[pause]) then begin
      r:=true;
      thisuser.ac:=thisuser.ac-[pause];
   end;
   searchb(fileboard,fn,TRUE,abort,next, false);
   if (r) then thisuser.ac:=thisuser.ac+[pause];
   thisuser.pagelen:=blah;
end;

procedure searchbd(b:integer; ts:astr; var abort,next:boolean);
var x,y,oldboard,pl,rn,i:integer;
    f:ulfrec;
    c:char;
    ok,vfo:boolean;
    v:verbrec;
begin
  oldboard:=fileboard;
  if (fileboard<>b) then changefileboard(b);
  sprint('|U0Searching |U9[|U3'+memuboard.name+'|U9]|U0');
  if (fileboard=b) then begin
    vfo:=(filerec(verbf).mode<>fmclosed);
    {$I-} if not vfo then reset(verbf); {$I+}
    fiscan(pl);
    if (baddlpath) then exit;
    rn:=1;
    while (rn<=pl) and (not abort) and (not hangup) do begin
      seek(ulff,rn); read(ulff,f);
      if (cansee(f)) then begin
        ok:=((pos(ts,strAllCaps(f.description))<>0) or
             (pos(ts,strAllCaps(f.filename))<>0));
        if (not ok) then
           if (f.vpointer<>-1) then begin
            {$I-} seek(verbf,f.vpointer); read(verbf,v); {$I+}
            if (ioresult=0) then begin
              i:=1;
              while (v.descr[i]<>'') and (i<=10) and (not ok) do begin
                if pos(ts,strAllCaps(v.descr[i]))<>0 then ok:=TRUE;
                inc(i);
              end;
            end;
          end;
      end;
      if (ok) then begin
        {pbn;}
        pfndd(rn,ts,f,abort,next);
        sprompt(strings^.numisc1);
        x:=wherex; y:=wherey;
        onek(c,'AQ'^M);
        case c of
        'A':begin
              ymbadd(memuboard.dlpath+f.filename);
              sprint(strings^.fileaddedtobq);
              goxy(x,y); nl;
            end;
        'Q':abort:=true;
        end;
      end;
      inc(rn);
    end;
    close(ulff);
    reset(verbf); close(verbf);
  end;
  fileboard:=oldboard;
end;

procedure searchd;
var s:astr;
    blah,bn:integer;
    abort,next:boolean;
begin
  nl;
  sprint(strings^.findline1);
  nl;
  sprint(strings^.findline2+':');
  input(s,20);
  if (s<>'') then begin
    nl; print('Searching for "'+s+'"'); nl;
    if pynq('Search all directories? ',true) then begin
      bn:=1; abort:=FALSE; next:=FALSE;
      while (not abort) and (bn<=maxulb) and (not hangup) do begin
        if (fbaseac(bn)) then searchbd(bn,s,abort,next);
        inc(bn);
        wkey(abort,next);
        if (next) then begin abort:=FALSE; next:=FALSE; end;
      end;
    end else begin
      abort:=FALSE; next:=FALSE;
      blah:=thisuser.pagelen;
      thisuser.pagelen:=15;

      searchbd(fileboard,s,abort,next);

      thisuser.pagelen:=blah;
    end;
  end;
end;

procedure gnfiles;
var i:integer;
    abort,next:boolean;
begin
  sysoplog('NewScan of file bases');
  i:=0;
  abort:=FALSE; next:=FALSE;
  while (not abort) and (i<=maxulb) and (not hangup) do begin
    if ((fbaseac(i)) and (i in zscanr.fzscan)) then searchb(i,'*.*',true,abort,next,true);
    inc(i);
    wkey(abort,next);
    if (next) then begin abort:=FALSE; next:=FALSE; end;
  end;
end;

procedure nf(mstr:astr);
var bn:integer;
    abort,next:boolean;
begin
  if (mstr='C') then searchb(board,'*.*',true,abort,next,true)
  else if (mstr='G') then gnfiles
  else if (value(mstr)<>0) then searchb(value(mstr),'*.*',true,abort,next,true)
  else begin
    nl;
    sprint(strings^.newline);
    sprint(strings^.pninfo);
    nl;
    abort:=FALSE; next:=FALSE;

    if pynq(strings^.globalnewscan,true) then gnfiles
      else searchb(fileboard,'*.*',true,abort,next,true);
  end;
end;

procedure fbasechange(var done:boolean; mstr:astr);
var s:astr;
    i, oldBoard:integer;
    show:boolean;
begin
   oldBoard := fileboard;
   calconf(1);
   if mstr<>'' then begin
      case mstr[1] of
         '+' : begin
            i:=fileboard;
            if (fileboard>=maxulb) then i:=1
            else begin
               repeat
                  inc(i);
                  changeFileBoard(i);
               until (fileBoard=i) or (i>maxulb);
            end;
            if (fileBoard=1) then lastCommandOvr := true;
            loaduboard(i);
         end;
         '-':begin
                i:=fileboard;
                if fileboard<=1 then i:=maxulb
                else begin
                   repeat
                      dec(i);
                      changeFileBoard(i);
                   until (fileboard=i) or (i<=1);
                end;
                if fileboard = i then lastCommandOvr := true;
                loaduboard(i);
             end;
         'L' : begin
            show:=true;
            fBaseList;
         end;
         else begin
            fBaseList;
            changeFileBoard(value(mstr));
            if pos(';',mstr)>0 then begin
               s:=copy(mstr,pos(';',mstr)+1,length(mstr));
               curmenu:=status^.menupath+s+'.mnu';
               newmenutoload:=TRUE;
               done:=TRUE;
            end;
            lastcommandovr:=TRUE;
         end;
      end;
   end
   else begin
      s:='?';
      i:=fileboard;
      while (s='?') and (not hangup) do begin
         sprompt(strings^.batchadd);
         input(s,3);
         i:=value(s);
         if (s=^M) then i:=1;
         if (s='?') then begin
            show:=true;
            fBaseList;
         end;
      end;
      if (i<1) or (i>maxulb) then i:=fileboard;
      changeFileBoard(i);
      loaduboard(i);
      lastcommandovr:=TRUE;
   end;
   if oldBoard <> fileboard then sprint(strings^.fBaseJoined);
end;
end.
