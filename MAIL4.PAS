{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mail4;

interface

uses
   Crt,   Dos, common5,
  common, timejunk,records,
  sysop3,common1, common2, common3,
  misc3, misc5,
  mail0, mail1, mail2, mail3;

procedure autoreply;
procedure readmail;

implementation

uses menus, menus2;

var emailtitle:string[20];

procedure autoreply;
var i:integer;
begin
  i:=lastfrom;
  email1(i,emailtitle);
end;

procedure readmail;
const hellfreezesover=FALSE;
var t:text;
    u:userrec;
    mheader:mheaderrec;
    mixr:msgindexrec;
    pdt:packdatetime;
    dt:ldatetimerec;
    cmds,s,s1:string;
    brdsig,totload:longint;
    crec,i,j,k,mnum,mw,snum:integer;
    c:char;
    bb:byte;
    abort,b,bakw,done,done1,dotitles,errs,found,hasshown1,holdit,
      justdel,next,noreshow,seeanon:boolean;

  procedure findit;
  var orec:integer;
      abort:boolean;
  begin
    orec:=crec; done1:=TRUE; found:=FALSE;
    if (bakw) then begin
      repeat
        dec(crec); abort:=(crec<0);
        if (not abort) then begin
          ensureloaded(crec);
          mixr:=mintab[getmixnum(crec)];
        end;
      until ((abort) or
             ((mixr.messagenum=usernum) and (miexist in mixr.msgindexstat)));
      found:=not abort; if (abort) then crec:=orec;
      if (crec<>orec) then dec(mnum);
      exit;
    end;
    repeat
      inc(crec); abort:=(crec>himsg);
      if (not abort) then begin
        ensureloaded(crec);
        mixr:=mintab[getmixnum(crec)];
      end;
    until ((abort) or
           ((mixr.messagenum=usernum) and (miexist in mixr.msgindexstat)));
    found:=not abort; if (abort) then crec:=orec;
    if (crec<>orec) then inc(mnum);
    if ((justdel) and (not found)) then begin done1:=FALSE; bakw:=TRUE; end;
    exit;
  end;

  function tch(c:char; i:integer):string;
  var s:string;
  begin
    s:=cstr(i); if (i<10) then s:=c+s;
    tch:=s;
  end;

  procedure getout;
  begin
    closebrd;
    thisuser.waiting:=mw;
    readingmail:=FALSE;
  end;

begin
  readingmail:=TRUE;
  abort:=FALSE; next:=FALSE;
  dotitles:=TRUE;
  mailread:=TRUE;
  if curmenu = '' then curmenu := status^.menupath + 'main.mnu';
  last_menu := curmenu;
  curmenu := status^.menupath+'EMAILP.MNU';
  readin;

  repeat
    if (dotitles) then begin
      abort:=FALSE; next:=FALSE;

      nl;
      hasshown1:=FALSE;
      if (thisuser.waiting=0) then
        sprint(strings^.nomailwaiting);

      initbrd(-1);
      i:=0; mw:=0;
      while (i<=himsg) do begin
        ensureloaded(i);
        if ((mintab[getmixnum(i)].messagenum=usernum) and
            (miexist in mintab[getmixnum(i)].msgindexstat)) then begin
          inc(mw);
          if (not abort) then begin
            loadmhead(i,mheader); ensureloaded(i);
            for j:=1 to 6 do pdt[j]:=mintab[getmixnum(i)].msgdate[j];
            pdt2dt(pdt,dt);

            with dt do begin
              j:=hour;
              if (j>12) then dec(j,12);
              if (j=0) then j:=12;
              s:=tch(' ',j)+':'+tch('0',min)+aonoff((hour>=12),'p','a');

              s:=#3#3+copy('JanFebMarAprMayJunJulAugSepOctNovDec',(month-1)*3+1,3)+
                 ' '+tch('0',day)+' '+cstr(year)+' - '+s;
            end;

            s1:=what00(mheader.fromi.anon,mheader.fromi.asn);
            if (mheader.fromi.anon in [1,2]) then begin
              case mheader.fromi.anon of
                1:seeanon:=aacs(status^.anonprivread);
                2:seeanon:=aacs(status^.csop);
              end;
              if (seeanon) then
                s1:=s1+' ('+caps(mheader.fromi.alias)+' #'+
                    cstr(mheader.fromi.usernum)+')'
              else
                s:='                   ';
            end;

            if ((not hasshown1) and (thisuser.waiting=0)) then begin
              nl;
            end;

            hasshown1:=TRUE;
          end;
        end;
        inc(i);
        wkey(abort,next);
      end;

      if (mw<>0) then nl;

      if (thisuser.waiting<>mw) then begin
        if (mw=0) then
          sprint('');
      end;

      thisuser.waiting:=mw;

      if (mw=0) then
        begin
        curmenu := last_menu;
        readin;
        getout;
        exit;
        end;

      abort:=FALSE; done:=FALSE; next:=FALSE;
      snum:=1;
    end;

    bakw:=FALSE; done:=FALSE; dotitles:=FALSE;
    holdit:=FALSE; justdel:=FALSE; noreshow:=FALSE;

    crec:=-1; mnum:=0;

    repeat
      if (not holdit) then
        repeat
          findit;
          if (crec=-1) then begin done1:=TRUE; dotitles:=TRUE; end;
        until (done1);
      justdel:=FALSE;
      if (mnum=snum) then snum:=0;
      if ((snum=0) and (crec>=0)) then begin
        next:=FALSE;
        if (not noreshow) then begin
          cls;
          readmsg(2,crec,mnum,mw,abort,next);
        end else
          noreshow:=FALSE;
        if (not next) then begin
          readpmnu := #0;
          mainmenuhandle(cmds);
          {
          sprompt(strings^.scanmessage);
          cmds:='Q?-ADFGIRSTN'^M;
          if (cso) then cmds:=cmds+'UVX';
          if (mso) then cmds:=cmds+'Z';
          onek(c,cmds);
          }
        end else
          readpmnu:='N';

        c := readpmnu;
      {
      if readpmnu = 'J' then
        begin
        sprompt(strings^.scanmessage);
        input(inp, 4);
        getm:=value(inp)-1;
        if (getm>-1) then
          if (st=stnewscan) then st:=stscan;
        inp := '';
        end;
      }
      abort:=FALSE; bakw:=FALSE; holdit:=TRUE; next:=FALSE;

      nl;

        case readpmnu of
          '-':begin bakw:=TRUE; holdit:=FALSE; end;
          'F':begin
                nl;
                prt('Forward letter to which user? '); finduserws(i);
                if (i<1) then print('Unknown user.')
                else
                if (i<>usernum) then begin
                  reset(uf); seek(uf,i); read(uf,u);

                  j:=0; b:=TRUE;
                  while ((j<4) and (b)) do begin
                    inc(j); b:=moremail(u,i,j);
                    if (not b) then
                      case j of
                        1:print('Mailbox is full.');
                        2:print('Mailbox is closed.');
                        3:if (mso) then print('That user is deleted.')
                            else print('Can''t send mail to that user.');
                        4:print('Can''t send mail to yourself!');
                      end;
                  end;

                  close(uf);

                  if (b) then begin
                    mixr:=mintab[getmixnum(crec)]; mixr.messagenum:=i;
                    savemix(mixr,crec);
                    dec(thisuser.waiting);
                    reset(uf);
                    seek(uf,i); read(uf,u); inc(u.waiting);
                    seek(uf,i); write(uf,u);
                    close(uf);
                    sysoplog('Forwarded letter to '+caps(u.name)+' #'+cstr(i));
                    dec(mw); crec:=-1;
                    snum:=mnum; mnum:=0; if (snum>mw) then dec(snum);
                    holdit:=FALSE;
                  end;
                end;
              end;
          'J':begin
                sprompt(strings^.scanmessage); inu(i);
                if ((not badini) and (i>=1) and (i<=mw)) then
                  begin holdit:=FALSE; crec:=-1; mnum:=0; snum:=i; end;
              end;
          'I','N',^M:holdit:=FALSE;
          'Q':begin
              getout;
              curmenu := last_menu;
              readin;
              exit;
              end;
          'A':;
          'T':dotitles:=TRUE;
          'X':if (mso) then begin
                nl;
                prt('Extract filename? (default="EXT.TXT") : ');
                input(s,40);
                if (s='') then s:='EXT.TXT';
                if pynq('Are you sure? ',true) then begin
                  b:=pynq('Strip color codes from output? ',true);

                  loadmhead(crec,mheader);

                  assign(t,s);
                  {$I-} append(t); {$I+}
                  if (ioresult<>0) then rewrite(t);
                  totload:=0;
                  repeat
                    blockreadstr2(brdf,s);
                    inc(totload,length(s)+2);
                    if ((b) and (pos(#3,s)<>0)) then s:=stripcolor(s);
                    writeln(t,s);
                  until (totload>=mheader.msglength);
                  close(t);

                  nl;
                  print('Done!');
                end;
              end;
      'R','S',
      'D','Z':begin
                b:=TRUE;
                if (readpmnu in ['R','S']) then begin
                  reset(uf);
                  loadmhead(crec,mheader);
                  emailtitle:='Re: '+mheader.title;
                  i:=lastfrom;
                  if ((i>=1) and (i<=filesize(uf)-1)) then begin
                    seek(uf,i); read(uf,u);
                  end;

                  j:=0; b:=TRUE;
                  while ((j<4) and (b)) do begin
                    inc(j); b:=moremail(u,i,j);
                    if (not b) then
                      case j of
                        1:print('That user''s mailbox is full.');
                        2:print('That user''s mailbox is closed.');
                        3:if (mso) then print('That user is deleted.')
                            else print('Can''t send mail to that user.');
                        4:print('Can''t send mail to yourself!');
                      end;
                  end;
                  close(uf);
                end;

                if (b) then begin
                  if ((c='Z') and (not mso)) then c:='D';
                  if (c='Z') or (c='D') or (c='R') then begin
                  if (pynq(strings^.deletereadmail,true)) then
                  begin
                    s:=rmail(crec);
                    dec(mw); crec:=-1;
                    snum:=mnum; mnum:=0; if (snum>mw) then dec(snum);
                    holdit:=FALSE;
                  end;
                 end;
                end;
              end;
        end;
        if (readpmnu in ['R','S']) then begin
          i:=thisuser.waiting;
          autoreply; inc(mw,thisuser.waiting-i);
        end;
      end;
      if ((mw=0) or ((crec=-1) and (snum=0))) then done:=TRUE;
    until ((done) or (dotitles) or (hangup));
    if (done) then
      begin
      curmenu := last_menu;
      readin;
      getout;
      exit;
      end;
  until (hellfreezesover);
  getout;  { just in case hell freezes over! <grin> }
  curmenu := last_menu;
  readin;
end;

end.
