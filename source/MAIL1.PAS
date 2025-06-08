{$A+,B+,F+,I+,L+,O+,S-,R-,V-}
unit mail1;

interface

uses
   Crt,   Dos,  common1,common2,common3,common5,records,
  common,timejunk,strproc;
function fsemsg(pub,uti:boolean; ftit:string; var mixr:msgindexrec;
               var mheader:mheaderrec):boolean;

implementation

uses mail0,TmpCom,ansidrv,file8,ansiedit;

{function linemsg(pub,uti:boolean; ftit:string; var mixr:msgindexrec;
               var mheader:mheaderrec):boolean;
var
  inmsgfile:text;
  inserton,
  cmdsoff:boolean;
  lastline:string;
  CurX, CurY:Byte;
  Direction:Byte; {1up 2dn 3lft 4rght}

{procedure inli(var i:string);
var s:astr;
    cp,rp,cv,cc,xxy:integer;
    c,c1,ccc,d:char;
    hitcmdkey,hitbkspc,escp,dothischar,abort,next,savallowabort:boolean;

  procedure bkspc;
  begin
    if (cp>1) then begin
      if (i[cp-2]=#3) and (i[cp-1] in [#0..#9]) then begin
        dec(cp);
        cl(1);
      end else
        if (i[cp-1]=^H) then begin
          prompt(' ');
          inc(rp);
        end else
          if (i[cp-1]<>#10) then begin
            prompt(^H' '^H);
            dec(rp);
          end;
      dec(cp);
    end;
  end;

begin
  write_msg:=TRUE; hitcmdkey:=FALSE; hitbkspc:=FALSE;
  ccc:='1';
  escp:=FALSE;
  rp:=1; cp:=1;
  i:='';
  if (lastline<>'') then begin
    abort:=FALSE; next:=FALSE;
    savallowabort:=allowabort; allowabort:=FALSE;
    reading_a_msg:=TRUE;
    prompt(lastline); {printa1(lastline,abort,next);}
 {   reading_a_msg:=FALSE;
    allowabort:=savallowabort;
    i:=lastline; lastline:='';
    escp:=(pos(^[,i)<>0);
    cp:=length(i)+1;
    rp:=cp;
  end;
  repeat
    getkey(c);

    dothischar:=FALSE;
    if (not cmdsoff) then
      if ((c>=#32) and (c<=#255)) then begin
        if (c='/') and (cp=1) then hitcmdkey:=TRUE else dothischar:=TRUE;
      end else
        case c of
          ^[:dothischar:=TRUE;
          ^B:dm(' -'^N'/'^N'l'^N'\'^N,c);
          ^H:if (cp=1) then begin
               hitcmdkey:=TRUE;
               hitbkspc:=TRUE;
             end else
               bkspc;
          ^I:begin
               cv:=5-(cp mod 5);
               if (cp+cv<strlen) and (rp+cv<thisuser.linelen) then
                 for cc:=1 to cv do begin
                   outkey(' '); if (trapping) then write(trapfile,' ');
                   i[cp]:=' ';
                   inc(rp); inc(cp);
                 end;
             end;
          ^J:if (not (rbackspace in thisuser.ac)) then begin
               outkey(c); i[cp]:=c;
               if (trapping) then write(trapfile,^J);
               inc(cp);
             end;
          ^N:if (not (rbackspace in thisuser.ac)) then begin
               outkey(^H); i[cp]:=^H;
               if (trapping) then write(trapfile,^H);
               inc(cp); dec(rp);
             end;
          ^S:dm(' '+nam+' ',c);
          ^W:if (cp=1) then begin
               hitcmdkey:=TRUE;
               hitbkspc:=TRUE;
             end else
               repeat bkspc until (cp=1) or (i[cp]=' ') or
                                  ((i[cp]=^H) and (i[cp-1]<>#3));
          ^Y:begin
               cp:=1;
               for cv:=1 to rp-1 do prompt(^H' '^H);
               rp:=1;
               if (ccc<>'1') then begin
                 c1:=ccc; i[cp]:=#3;
                 inc(cp); i[cp]:=chr(ord(c1)-ord('0'));
                 inc(cp); cl(ord(i[cp-1]));
               end;
             end;
        end;

    if ((dothischar) or (cmdsoff)) and ((c<>^G) and (c<>^M)) then
      if ((cp<strlen) and (escp)) or
         ((rp<thisuser.linelen) and (not escp)) then begin
        if (c=^[) then escp:=TRUE;
        i[cp]:=c; inc(cp); inc(rp);
        outkey(c);
        if (trapping) then write(trapfile,c);
        inc(pap);
      end;
  until ((rp=(thisuser.linelen)) and (not escp)) or ((cp=strlen) and (escp)) or
        (c=^M) or (hitcmdkey) or (hangup);

  if (hitcmdkey) then begin
    if (hitbkspc) then i:='/'^H else i:='/';
  end else begin
    i[0]:=chr(cp-1);
    if (c<>^M) and (cp<>strlen) and (not escp) then begin
      cv:=cp-1;
      while (cv>1) and (i[cv]<>' ') and ((i[cv]<>^H) or (i[cv-1]=#3)) do dec(cv);
      if (cv>rp div 2) and (cv<>cp-1) then begin
        lastline:=copy(i,cv+1,cp-cv);
        for cc:=cp-2 downto cv do prompt(^H);
        for cc:=cp-2 downto cv do prompt(' ');
        i[0]:=chr(cv-1);
      end;
    end;

    if (escp) and (rp=thisuser.linelen) then cp:=strlen;
    if (cp<>strlen) then nl
    else begin
      rp:=1; cp:=1;
      i:=i+#29;
    end;
  end;

  write_msg:=FALSE;
end;

type mline=array [1..101] of string[80];
var upin:string[255];
    li:^mline;
    oldpause,gotit,temp1,temp2,ansmsg:boolean;
    tempmsg,ansfile:text;
    ansline:string;
    an:anontyp;
    mftit,fto,spc,s,s1,s2:string;
    tmpy,topy,t,maxli,lc,ii,i,j,k,quoteli:integer;
    c,c1:char;
    badnum,cantabort,saveline,goquote,exited,save,abortit,abort,next,ptl1,ufo:boolean;

  procedure listit(stline:integer; linenum,disptotal:boolean);
  var lasts:string;
      l:integer;
  begin
    if (disptotal) then nl;
    l:=stline;
    abort:=FALSE;
    next:=FALSE;
    dosansion:=FALSE;
    lasts:='';

    while ((l<lc) and (not abort)) do begin
      if (linenum) then print(cstr(l)+':');
      if ((pos(^[,li^[l])=0) and (pos(^[,lasts)=0)) then dosansion:=FALSE;
      reading_a_msg:=TRUE;
      if ((pub) and (aacs(memboard.mciacs))) then read_with_mci:=TRUE;
      print(li^[l]);
      read_with_mci:=FALSE;
      reading_a_msg:=FALSE;

      lasts:=li^[l];
      inc(l);
    end;
    dosansion:=FALSE;
    if (disptotal) then
      sprint('|NL|U7Total lines: |U4[|U3'+cstr(lc-1)+'|U4]');
    saveline:=FALSE;
  end;

  procedure ptl;
  var u:userrec;
      sr:smalrec;
      s,s1:astr;
      i,j,tl:integer;
      aa,aa1,done,ufo,sfo:boolean;
  begin
    tl:=60;
    s1:='';
     if ((pub) and (ptl1) and (fto<>'')) then begin
      sprompt(ftit);
      sprompt(strings^.msgsendto);
      s:=fto;
      if (mbrealname in memboard.mbstat) then begin
        if (mheader.toi.real<>'') then begin
          s:=caps(mheader.toi.real);
          if (memboard.mbtype=0) then s:=s+' #'+cstr(mheader.toi.usernum);
        end else
          s:='';
      end;
      ptl1:=FALSE;
      exit;
    end;
    if ((pub) and (not uti)) then begin
      sprompt(strings^.msgsendto);
      inputl(s,50);
      aa1:=FALSE;
      if (s<>'') then begin
        fto:=s;
        if (copy(s,1,1)='#') then s:=copy(s,2,length(s)-1);
        val(s,i,j);
        if ((i<>0) and (j=0)) then begin
          ufo:=(filerec(uf).mode<>fmclosed);
          if (not ufo) then reset(uf);
          if ((i>=1) and (i<=filesize(uf)-1)) then begin
            seek(uf,i);
            read(uf,u);
            fto:=caps(u.name)+' #'+cstr(i);
            if (pub) then begin
              if (mbrealname in memboard.mbstat) then fto:=caps(u.realname)
                else fto:=caps(u.name);
              if (memboard.mbtype=0) then fto:=fto+' #'+cstr(i);
            end;
          end else begin
            prompt(s);
            cl(1);
            fto:='';
            aa1:=TRUE;
          end;
          if (not ufo) then close(uf);
        end;
      end;
      aa:=(sqoutsp(fto)='');
      if (strAllCaps(sqoutsp(fto))='ALL') then aa:=TRUE;
      if (aa) then fto:='All';
      if (not ptl1) then sprint(#3#0+'Continue message...');
    end;
    if ((not ptl1) or (ftit<>'')) then begin
       sprompt(strings^.msgtitle);
       print(mftit);
{      if (ftit<>'') then begin
        sprompt(strings^.msgtitle);
        print(mftit);
      end else begin
        sprompt(strings^.msgtitle);
        print(mftit);
      end;}

  {  end else if ((pub) and (uti)) then begin
      if (strAllCaps(copy(lastmheader.title,1,3))<>'RE:') then
        s1:='Re: '+copy(lastmheader.title,1,64) else
        s1:=lastmheader.title;
      sprompt(strings^.msgtitle);
      print(s1);
    end;
    if (ftit='') then begin
      sprompt(strings^.msgtitle);
      inputl(s,tl);
      if (s<>'') then begin
        cl(1);
        mftit:=s
      end else begin
        if (s1<>'') then mftit:=s1;
        if (ptl1) then exit;
      end;
    end;
    {TAKE FROM HERE}
{   ptl1:=FALSE;
  end;

  procedure qnum(var i:integer);
  var s:string[5];
   begin
     badnum:=FALSE;
     input(s,5); if (s='') or (s='Q') then i:=0 else i:=value(s);
   if (s='') then badnum:=TRUE;
  end;

  procedure doquote;
  type qline=array [1..101] of string[80];
  var f:text;
      tmp,t1:integer;
      blah,done:boolean;
      pp,x,rr1,rr2:integer;
      quotefrom:string[40];
      qlinef:array[1..2] of string[80];
      spos:byte;
      ql:^qline;

    procedure openquotefile;
    begin
      done:=FALSE;
      assign(f,'msgtmp');
      {$I-} {reset(f); {$I+}
{      if (ioresult<>0) then done:=TRUE;
    end;

  begin
    reading_a_msg:=true;
    openquotefile;
    readln(f,quotefrom);

    QLineF[1]:=strings^.QuoteBegin;
    if pos('|QT',strings^.QuoteBegin)<>0 Then
    Begin
      spos:=pos('|QT',QLineF[1]);
      delete(QLineF[1],Spos,3);
      Insert(QuoteFrom,QLineF[1],Spos);
    End;
    QLineF[2]:=strings^.QuoteEnd;
    If pos('|QT',strings^.QuoteEnd)<>0 Then
    Begin
      spos:=pos('|QT',QLineF[2]);
      delete(QLineF[2],Spos,3);
      Insert(QuoteFrom,QLineF[2],Spos);
    End;

    new(ql);
    for tmp:=1 to 101 do ql^[tmp]:='';
    if (not done) then begin
      done:=FALSE;
      blah:=false;
      x:=1;
    repeat
      {$I-} {readln(f,ql^[x]); {$I+}
{      if (ioresult=0) then blah:=false else blah:=true;
      sprint(mln(cstr(x),2)+': '+ql^[x]);
      inc(x);
      tmpy:=wherey;
      if (tmpy>=20) then begin
        pausescr;
        goxy(1,topy);
        for tmp:=topy to tmpy do sprint(#27+'[K');
        goxy(1,topy);
      end;
      if (eof(f)) then blah:=true;
    until ((x=255) or (blah));
      if (not done) then begin
        if (memboard.mbtype=0) then sprompt(#3+cstr(fidor.quote_color))
          else sprompt(#3+cstr(memboard.quote_color));
          while (blah) do begin
              sprompt(strings^.Quote1);
              qnum(rr1);
              if (rr1<=0) then begin dispose(ql); exit; end;
              if (not badnum) and (rr1>=1) then
              begin
                if (rr1<1) then rr1:=1;
                done:=true; blah:=false;
              end;
              sprompt(strings^.Quote2);
              qnum(rr2);
              if (rr2<=0) then begin dispose(ql); exit; end;
              if (not badnum) and (rr2<=x) then
              begin
                if (rr2>x) then rr1:=x;
                done:=true; blah:=false;
              end;
              if rr1-rr2>status^.csmaxlines then begin
                sprint('|07Too many lines in quote');
                repeat dec(rr2) until (rr2-rr1<status^.csmaxlines);
              end;
          end;
          if (rr2>x) then rr2:=x;
          if (rr2<rr1) then rr2:=rr1;
          if (rr1<1) then rr1:=1;
            if (rr2=rr1) then begin
              print(ql^[rr1]);
              li^[lc]:=QLineF[1];
              inc(lc);
              li^[lc]:=ql^[rr1];
              inc(lc);
              li^[lc]:=QLineF[2];
              inc(lc);
            end else begin
              x:=rr1;
              li^[lc]:=QLineF[1];
              inc(lc);
                repeat
                 li^[lc]:=ql^[x];
                 inc(lc);
                 inc(quoteli);
                 inc(x);
                until (x=rr2+1);
              li^[lc]:=QLineF[2];
              inc(lc);
            end;
            li^[lc]:=' ';
            inc(lc);
            inc(quoteli);
            done:=true;
         end;
      end;
    dispose(ql);
    reading_a_msg:=false;
  end;

  procedure printmsgtitle;
  begin
    cls;

    with mheader, mixr do
    begin
    getpackdatetime(@msgdate);
    getdayofweek(msgdowk);
    printf3('linehead',
             mln(aonoff(pub,memboard.name,'Private Mail'),30),
             mftit,
             thisuser.name,
             fto,
             pdt2dat(@msgdate,msgdowk));
    end;

    topy:=wherey;
  end;

  procedure inputthemessage;
  var tmp,t1:integer;
  begin
    ansmsg:=false;
    cmdsoff:=FALSE;
    abort:=FALSE;
    next:=FALSE;
    ptl1:=TRUE;
    goquote:=FALSE;
    quoteli:=1;
    if (freek(exdrv(status^.msgpath))<status^.minspaceforpost) then begin
      mftit:='';
      nl;
      print('Not enough disk space to save a message.');
      c:=chr(exdrv(status^.msgpath)+64);
      if (c='@') then sysoplog(#3#8+'- '+#3#3+' Main BBS drive full!  Insufficient space to save a message!')
        else sysoplog(#3#8+'- '+#3#3+' '+c+': drive full!  Insufficient space to save a message!');
    end else begin
      lc:=1;
      spc:='                                                                              ';
      lastline:='';
      maxli:=status^.maxlines;
      ptl;
      if (pynq(strings^.uploadmsg,false)) then ansmsg:=true;
    end;
  if (not ansmsg) then begin
    if (mftit='') then
      if (not cantabort) then begin
        save:=FALSE;
        exit;
      end;
    printmsgtitle;
    repeat
      repeat
        saveline:=TRUE;
        {nofeed:=FALSE;}
 {       exited:=FALSE;
        save:=FALSE;
        abortit:=FALSE;
        write_msg:=TRUE;
        curx:=wherex;
        cury:=wherey;
        inli(s);
        write_msg:=FALSE;
        if (s='/'^H) then begin
          saveline:=FALSE;
          if (lc<>1) then begin
            dec(lc);
            lastline:=li^[lc];
            if (copy(lastline,length(lastline),1)=#1) then
              lastline:=copy(lastline,1,length(lastline)-1);
            if (cury>topy) then goxy(curx,cury-1);
          end;
        end;
        if (s='/') then begin
          sprompt(strings^.fseprompt);
          getkey(c);
          for t1:=1 to length(strings^.fseprompt) do
            prompt(^H' '^H);
          saveline:=FALSE;
          case upcase(c) of
            '?','H':printf('prhelp');
            'A':if (not cantabort) then
                  if pynq('|U3Abort message? ',false) then begin
                    exited:=TRUE;
                    abortit:=TRUE;
                end;
            'C':if pynq('|U3Clear message? ',false) then begin
                  printmsgtitle;
                  lc:=1;
                end;
            'L':listit(1,pynq('|U7List message with line numbers? ',true),TRUE);
            'Q':if (exist('msgtmp')) then goquote:=true;
            'S':if ((not cantabort) or (lc>1)) then begin
                  exited:=TRUE;
                  save:=TRUE;
                end;
            'T':ptl;
          end;
        end;

        if (goquote) then begin
          doquote;
          goquote:=FALSE;
          cls;
          printmsgtitle;
          if (lc>1) then
            if (lc>10) then listit(lc-10,FALSE,FALSE)
              else listit(1,FALSE,FALSE);
        end;

        if (saveline) then begin
          li^[lc]:=s;
          inc(lc);
          tmpy:=wherey;
          if (tmpy>=22) then begin
            goxy(1,topy);
            for tmp:=topy to tmpy do sprint(#27+'[K');
            goxy(1,topy);
            for tmp:=lc-10 to lc do print(li^[tmp]);
            goxy(1,wherey-1);
          end;
          if (lc>maxli) then begin
            print('You have used up your maximum amount of lines.');
            exited:=TRUE;
          end;
        end;
      until ((exited) or (hangup));
      if (hangup) then abortit:=TRUE;
      if ((not abortit) and (not save)) then
      begin
        sprint('Maximum number of lines reached.');
        nl;
        if (pynq('Save? ',false)) then save:=true else abortit:=true;
      end;
    until ((abortit) or (save) or (hangup));
    if (lc=1) then begin
      abortit:=TRUE;
      save:=FALSE;
    end;
   end else
     begin
       save:=false;
       if (not outcom) then begin
         sprompt(strings^.enterfilenametoul);
         s:='';
         mpl(40);
         inputl(s,40);
         if (exist(s)) then begin
           assign(ansfile,s);
           reset(ansfile);
           assign(tempmsg,'temp2.msg');
           rewrite(tempmsg);
           append(tempmsg);
           while (not eof(ansfile)) do begin
             readln(ansfile,ansline);
             writeln(tempmsg,ansline);
           end;
           close(ansfile);
           close(tempmsg);
           save:=true;
         end;
       end else begin
         receive1('temp2.msg',false,gotit,temp1,temp2);
         if (exist('temp2.msg')) then begin
           assign(tempmsg,'temp2.msg');
           reset(tempmsg);
           close(tempmsg);
           save:=true;
         end;
         end;
     end;
  end;

  function getorigin:string;
  var s:astr;
  begin
    if (memboard.origin<>'') then s:=memboard.origin
      else if (fidor.origin<>'') then s:=fidor.origin
        else s:=copy(stripcolor(status^.bbsname),1,50);
    while (copy(s,length(s),1)=' ') do
      s:=copy(s,1,length(s)-1);
    getorigin:=s;
  end;

  procedure saveit;
  var t:text;
      i,j,qcolor,tcolor:integer;
      c:char;
      s:astr;

    function getaddr(zone,net,node,point:integer):string;
    begin
      getaddr:=cstr(zone)+':'+cstr(net)+'/'+cstr(node)+'.'+cstr(point)+')';
    end;

  begin
    mheader.msglength:=0;
    with memboard do begin
      if (mbtype in [1,2]) then begin
        qcolor:=quote_color;
        tcolor:=text_color;
      end else begin
        qcolor:=fidor.quote_color;
        tcolor:=fidor.text_color;
      end;
      assign(t,'temp.msg'); rewrite(t);
    if (not ansmsg) then begin
      if ((pub) and (mbfilter in mbstat)) then begin
        for i:=1 to lc-1 do
          if length(li^[i])>0 then begin
            li^[i]:=stripcolor(li^[i]);
            for j:=1 to length(li^[i]) do begin
              c:=li^[i][j];
              if (c in [#0..#1,#3..#31,#127..#255]) then c:='*';
              li^[i][j]:=c;
            end;
          end;
      end;
      for i:=1 to lc-1 do begin
        s:=li^[i];
        j:=pos('>',stripcolor(s));
        if ((not pub) or (mbtype=0) or (copy(s,1,3)='`#[') or (s[1]=#2))
          then j:=0;
        writeln(t,s);
        inc(mheader.msglength,length(s)+2);
      end;
    end {if not ansmsg}
  {   else
       begin
         assign(tempmsg,'temp2.msg');
         reset(tempmsg);
         {$I-}
   {      s:='';
         repeat
           writeln(t,s);
           inc(mheader.msglength,length(s)+2);
           readln(tempmsg,s);
         until eof(tempmsg);
         {$I+}
    {     close(tempmsg); erase(tempmsg);
       end;

      if (thisuser.autosig<>'') then begin
        writeln(t,'');
        inc(mheader.msglength,2);
        s:=thisuser.autosig;
        writeln(t,s);
        inc(mheader.msglength,length(s)+2);
      end;


      if ((pub) and (mbtype in [1,2]) and (mbaddtear in mbstat)) then begin
        writeln(t,'');
        inc(mheader.msglength,2);
        s:='--- '+ProgName+' v'+ver;
        writeln(t,s);
        inc(mheader.msglength,length(s)+2);
        s:=' * Origin: '+getorigin+' (';
        if (zone<>0) then s:=s+getaddr(zone,net,node,point)
          else s:=s+getaddr(fidor.zone,fidor.net,fidor.node,fidor.point);
        writeln(t,s);
        inc(mheader.msglength,length(s)+2);
      end;

      close(t);
      outmessagetext('temp.msg',mheader,TRUE);
    end;
  end;

  procedure readytogo;
  var f:text;
  begin
    if (exist('msgtmp')) then begin
      assign(f,'msgtmp');
      {$I-} {reset(f); {$I+}
{      if (ioresult=0) then begin
        close(f);
        erase(f);
      end;
    end;
  end;

var clr:integer;
begin
  oldpause:=false;
  linemsg:=FALSE;
  if (uti) then fto:=caps(mheader.toi.asn)
    else fto:='';
  if (ftit<>'') then mftit:=ftit
    else mftit:='';
  if (copy(mftit,1,1)='\') then begin
    mftit:=copy(mftit,2,length(mftit)-1);
    cantabort:=TRUE;
  end else
    cantabort:=FALSE;
  new(li);

  for clr:=1 to 255 do li^[clr]:=''; {clear the nigz out}

 { if (thisuser.ac=thisuser.ac+[pause]) then oldpause:=true;
  thisuser.ac:=thisuser.ac-[pause];

  inputthemessage;

  if (not save) then begin
    sprint(strings^.aborted);
    dispose(li);
    readytogo;
    exit;
  end;

  if (oldpause) then thisuser.ac:=thisuser.ac+[pause];
  with mheader do begin
    signature:=$FFFFFFFF;
    title:=mftit;
    origindate:='';
    with fromi do begin
      anon:=0;
      usernum:=common.usernum;
      asn:=strAllCaps(thisuser.name);
      if ((not pub) or (memboard.mbtype=0)) then
        asn:=asn+' #'+cstr(common.usernum);
      real:=strAllCaps(thisuser.realname);
      alias:=strAllCaps(thisuser.name);
    end;
    if (not uti) then
      with toi do begin
        anon:=0;
        usernum:=0;
        asn:='';
        if (strAllCaps(sqoutsp(fto))='ALL') then fto:='';
        if (pub) then asn:=fto;
        real:='';
        alias:='';
      end;
  end;

  loadboard(board);
  if (pub) then begin
    an:=memboard.anonymous;
    if ((an=atno) and (aacs(status^.anonpubpost))) then an:=atyes;
    if (rpostan in thisuser.ac) then an:=atno;
  end else
    if (aacs(status^.anonprivpost)) then an:=atyes else an:=atno;
  case an of
    atno      :;
    atforced  :if (cso) then mheader.fromi.anon:=2
                        else mheader.fromi.anon:=1;
    atyes     :begin
                 nl;
                 if pynq(strings^.postanon,false) then
                   if (cso) then mheader.fromi.anon:=2
                            else mheader.fromi.anon:=1;
               end;
    atdearabby:begin
                 nl;
                 sprint(aonoff(pub,'Post as:','Send as:'));
                 nl;
                 sprint(#3#3+'1. ^1Abby');
                 sprint(#3#3+'2. ^1Problemed Person');
                 sprint(#3#3+'3. ^1'+nam);
                 nl;
                 prt('Which? '); onek(c,'123N'^M);
                 case c of
                   '1':mheader.fromi.anon:=3;
                   '2':mheader.fromi.anon:=4;
                 end;
               end;
    atanyname :begin
                 nl;
                 sprint('You can post your message under any name');
                 sprint('you want on this base.');
                 nl;
                 print('Enter name, or <CR> for your own.');
                 prt('Name: '); input(s,36);
                 if (s<>'') then begin
                   mheader.fromi.anon:=5;
                   mheader.fromi.asn:=caps(s);
                 end;
               end;
  end;
  if ((pub) and (himsg<>65535)) then begin
    j:=0;
    for i:=0 to himsg do begin
      ensureloaded(i);
      k:=mintab[getmixnum(i)].messagenum;
      if (k>j) then j:=k;
    end;
    mixr.messagenum:=j+1;
  end;
  with mixr do begin
    hdrptr:=filesize(brdf);
    isreplytoid:=0;
    isreplyto:=65535;
    numreplys:=0;
    getpackdatetime(@msgdate);
    getdayofweek(msgdowk);
    msgid:=memboard.lastmsgid;
    inc(memboard.lastmsgid);
    msgindexstat:=[miexist];
    if (pub) then begin
      if (rvalidate in thisuser.ac) then
        msgindexstat:=msgindexstat+[miunvalidated];
      if (aacs(memboard.mciacs)) then
        msgindexstat:=msgindexstat+[miallowmci];
    end;
  end;

  nl;
  while ((lc>1) and ((li^[lc-1]='') or (li^[lc-1]=^J))) do
    dec(lc);
  saveit;
  dispose(li);
  savesystat;
  ufo:=(filerec(bf).mode<>fmclosed);
  if (not ufo) then reset(bf);
  seek(bf,board-1);
  write(bf,memboard);
  if (not ufo) then close(bf);

  cl(9);
  readytogo;
  linemsg:=TRUE;
end;}


function fsemsg(pub,uti:boolean; ftit:string; var mixr:msgindexrec;
               var mheader:mheaderrec):boolean;
var
    oldpause,gotit,temp1,temp2,ansmsg:boolean;
    tempmsg,ansfile:text;
    ansline:string;
    an:anontyp;
    mftit,fto,spc,s,s1,s2:string;
    tmpy,topy,curx,cury,t,maxli,lc,ii,i,j,k,quoteli:integer;
    c,c1:char;
    badnum,cantabort,saveline,goquote,exited,save,abortit,abort,next,ptl1,ufo:boolean;
    inmsgfile:text;
    inserton,
    cmdsoff:boolean;
    lastline:string;
    msginfo:^message;

   procedure dotitle;
   var u:userrec;
       sr:smalrec;
       s,s1:astr;
       i,j,tl:integer;
       aa,aa1,ufo,sfo:boolean;
   begin
      tl:=60;
      s1:='';
      if ((pub) and (ptl1) and (fto<>'')) then begin
         {sprompt(strings^.msgsendto);}
         s:=fto;
         if (mbrealname in memboard.mbstat) then begin
            if (mheader.toi.real<>'') then begin
               s:=caps(mheader.toi.real);
               if (memboard.mbtype=0) then s:=s+' #'+cstr(mheader.toi.usernum);
            end
            else s:='';
         end;
         ptl1:=FALSE;
         exit;
      end;
      if ((pub) and (not uti)) then begin
         sprompt(strings^.msgsendto);
         inputl(s,50);
         aa1:=FALSE;
         if (s<>'') then begin
            fto:=s;
            if (copy(s,1,1)='#') then s:=copy(s,2,length(s)-1);
            val(s,i,j);
            if ((i<>0) and (j=0)) then begin
               ufo:=(filerec(uf).mode<>fmclosed);
               if (not ufo) then reset(uf);
               if ((i>=1) and (i<=filesize(uf)-1)) then begin
                  seek(uf,i);
                  read(uf,u);
                  fto:=caps(u.name)+' #'+cstr(i);
                  if (pub) then begin
                     if (mbrealname in memboard.mbstat) then fto:=caps(u.realname)
                     else fto:=caps(u.name);
                     if (memboard.mbtype=0) then fto:=fto+' #'+cstr(i);
                  end;
               end
               else begin
                  prompt(s);
                  cl(1);
                  fto:='';
                  aa1:=TRUE;
               end;
               if (not ufo) then close(uf);
            end;
         end;
         aa:=(sqoutsp(fto)='');
         if (strAllCaps(sqoutsp(fto))='ALL') then aa:=TRUE;
         if (aa) then fto:='All';
      end;
      if ((not ptl1) or (ftit<>'')) then begin
         sprint(strings^.msgtitle+mftit);
         {if (ftit<>'') then begin
         sprompt(strings^.msgtitle);
         print(mftit);
         end else begin
         sprompt(strings^.msgtitle);
         print(mftit);
         end;}
      end
      else if ((pub) and (uti)) then begin
         if (strAllCaps(copy(lastmheader.title,1,3))<>'RE:') then s1:='Re: '+copy(lastmheader.title,1,64)
         else s1:=lastmheader.title;
         sprint(strings^.msgtitle+s1);
      end;
      if (ftit='') then begin
         sprompt(strings^.msgtitle);
         inputl(s,tl);
         if (s<>'') then begin
            cl(1);
            mftit:=s
         end
         else begin
            if (s1<>'') then mftit:=s1;
            if (ptl1) then exit;
         end;
      end;
      ptl1:=FALSE;
   end;

   function getorigin:string;
   var s:astr;
   begin
      if (memboard.origin<>'') then s:=memboard.origin
      else if (fidor.origin<>'') then s:=fidor.origin
      else s:=copy(stripcolor(status^.bbsname),1,50);
      while (copy(s,length(s),1)=' ') do s:=copy(s,1,length(s)-1);
      getorigin:=s;
   end;

   procedure saveit;
   var t:text;
       i,
       j,
       qcolor,
       tcolor:integer;
       c:char;
       s:astr;

      function getaddr(zone,net,node,point:integer):string;
      begin
         getaddr:=cstr(zone)+':'+cstr(net)+'/'+cstr(node)+'.'+cstr(point)+')';
      end;
   begin
      mheader.msglength:=0;
      with memboard do begin
         if (mbtype in [1,2]) then begin
            qcolor:=quote_color;
            tcolor:=text_color;
         end
         else begin
            qcolor:=fidor.quote_color;
            tcolor:=fidor.text_color;
         end;
         {ansmsg:=false;}
         assign(t,'temp.msg');
         rewrite(t);
         append(t);
         if (not ansmsg) then begin
            for i:=1 to msginfo^.numlines do begin
               s:=msginfo^.text[i];
               writeln(t,s);
               inc(mheader.msglength,length(s)+2);
            end;
         end {if not ansmsg}
         else begin
            assign(tempmsg,'temp2.msg');
            reset(tempmsg);
            {$I-}
            s:='';
            repeat
               writeln(t,s);
               inc(mheader.msglength,length(s)+2);
               readln(tempmsg,s);
            until eof(tempmsg);
            {$I+}
            close(tempmsg);
            erase(tempmsg);
         end;
         if (thisuser.autosig<>'') then begin
            writeln(t,'');
            inc(mheader.msglength,2);
            s:=thisuser.autosig;
            writeln(t,s);
            inc(mheader.msglength,length(s)+2);
         end;
         if ((pub) and (mbtype in [1,2]) and (mbaddtear in mbstat)) then begin
            writeln(t,'');
            inc(mheader.msglength,2);
            s:='--- '+ProgName+' v'+ver;
            writeln(t,s);
            inc(mheader.msglength,length(s)+2);
            s:=' * Origin: '+getorigin+' (';
            if (zone<>0) then s:=s+getaddr(zone,net,node,point)
            else s:=s+getaddr(fidor.zone,fidor.net,fidor.node,fidor.point);
            writeln(t,s);
            inc(mheader.msglength,length(s)+2);
         end;
         close(t);
         outmessagetext('temp.msg',mheader,TRUE);
      end;
   end;

   procedure readytogo;
   var f:text;
   begin
      if (exist('msgtmp')) then begin
         assign(f,'msgtmp');
         {$I-} reset(f); {$I+}
         if (ioresult=0) then begin
            close(f);
            erase(f);
         end;
      end;
   end;

var clr:integer;

begin
   write_msg:=false;
   oldpause:=false;
   fsemsg:=FALSE;
   if (uti) then fto:=caps(mheader.toi.asn)
   else fto:='';
   if (ftit<>'') then mftit:=ftit
   else mftit:='';
   if (copy(mftit,1,1)='\') then begin
      mftit:=copy(mftit,2,length(mftit)-1);
      cantabort:=TRUE;
   end
   else cantabort:=FALSE;
   ansmsg:=false;
   cmdsoff:=FALSE;
   abort:=FALSE;
   next:=FALSE;
   ptl1:=TRUE;
   goquote:=FALSE;
   quoteli:=1;
   if (freek(exdrv(status^.msgpath))<status^.minspaceforpost) then begin
      mftit:='';
      nl;
      print('Not enough disk space to save a message.');
      c:=chr(exdrv(status^.msgpath)+64);
      if (c='@') then sysoplog(#3#8+'- '+#3#3+' Main BBS drive full!  Insufficient space to save a message!')
      else sysoplog(#3#8+'- '+#3#3+' '+c+': drive full!  Insufficient space to save a message!');
   end
   else begin
      lc:=1;
      spc:='                                                                              ';
      lastline:='';
      maxli:=status^.maxlines;
      dotitle;
   end;

   ansmsg:=pynq(strings^.uploadmsg,false);

   if (ansmsg) then begin
      with mheader, mixr, msginfo^ do begin
         basename := mln(aonoff(pub,memboard.name,'Private Mail'),30);
         msgtitle := mftit;
         from     := caps(thisuser.name);
         touser   := caps(fto);
         getpackdatetime(@msgdate);
         getdayofweek(msgdowk);
         dateleft := pdt2dat(@msgdate,msgdowk);
      end;
      save:=false;
      if (not outcom) then begin
         sprompt(strings^.enterfilenametoul);
         s:='';
         inputl(s,40);
         if (exist(s)) then begin
            assign(ansfile,s);
            reset(ansfile);
            assign(tempmsg,'temp2.msg');
            rewrite(tempmsg);
            append(tempmsg);
            while (not eof(ansfile)) do begin
               readln(ansfile,ansline);
               writeln(tempmsg,ansline);
            end;
            close(ansfile);
            close(tempmsg);
            save:=true;
         end
         else begin
            save := false;
            ansmsg := false;
         end;
      end
      else begin
         gotit := true;
         temp1 := false;
         temp2 := false;
         receive1('temp2.msg',false,gotit,temp1,temp2);
         if (exist('temp2.msg')) then begin
            assign(tempmsg,'temp2.msg');
            reset(tempmsg);
            close(tempmsg);
            save:=true;
         end
         else save := false;
      end;
   end
   { not ansMSG}
   else begin
      if (mftit='') then
         if (not cantabort) then begin
            save:=FALSE;
            exit;
         end;

      new(msginfo);
      if (thisuser.ac=thisuser.ac+[pause]) then oldpause:=true;
      thisuser.ac:=thisuser.ac-[pause];
      nl;
      cls;
      with mheader, mixr, msginfo^ do begin
         basename := mln(aonoff(pub,memboard.name,'Private Mail'),30);
         msgtitle := mftit;
         from     := caps(thisuser.name);
         touser   := caps(fto);
         getpackdatetime(@msgdate);
         getdayofweek(msgdowk);
         dateleft := pdt2dat(@msgdate,msgdowk);
      end;
      save:=ansireedit(msginfo^, true);
      if (not save) then begin
         sprint(strings^.aborted);
         readytogo;
         dispose(msginfo);
         exit;
      end;
   end; {if not ansmsg}

   if (oldpause) then thisuser.ac:=thisuser.ac+[pause];
   with mheader do begin
      signature:=$FFFFFFFF;
      title:=mftit;
      origindate:='';
      with fromi do begin
         anon:=0;
         usernum:=common.usernum;
         asn:=strAllCaps(thisuser.name);
         if ((not pub) or (memboard.mbtype=0)) then asn:=asn+' #'+cstr(common.usernum);
         real:=strAllCaps(thisuser.realname);
         alias:=strAllCaps(thisuser.name);
      end;
      if (not uti) then
         with toi do begin
            anon:=0;
            usernum:=0;
            asn:='';
            if (strAllCaps(sqoutsp(fto))='ALL') then fto:='';
            if (pub) then asn:=fto;
            real:='';
            alias:='';
         end;
   end;

   loadboard(board);

   if (pub) then begin
      an:=memboard.anonymous;
      if ((an=atno) and (aacs(status^.anonpubpost))) then an:=atyes;
      if (rpostan in thisuser.ac) then an:=atno;
   end
   else if (aacs(status^.anonprivpost)) then
      an:=atyes
      else an:=atno;
   case an of
      atno : ;
      atforced : if (cso) then mheader.fromi.anon:=2
         else mheader.fromi.anon:=1;
      atyes : begin
         if pynq('|NL'+strings^.postanon,false) then
            if (cso) then mheader.fromi.anon:=2
            else mheader.fromi.anon:=1;
      end;
      atdearabby:begin
         nl;
         sprint(aonoff(pub,'Post as:','Send as:'));
         nl;
         sprint(#3#3+'1. ^1Abby');
         sprint(#3#3+'2. ^1Problemed Person');
         sprint(#3#3+'3. ^1'+nam);
         nl;
         prompt('Which? ');
         onek(c,'123N'^M);
         case c of
            '1':mheader.fromi.anon:=3;
            '2':mheader.fromi.anon:=4;
         end;
      end;
      atanyname : begin
         nl;
         sprint('You can post your message under any name');
         sprint('you want on this base.');
         nl;
         print('Enter name, or <CR> for your own.');
         prt('Name: ');
         input(s,36);
         if (s<>'') then begin
            mheader.fromi.anon:=5;
            mheader.fromi.asn:=caps(s);
         end;
      end;
   end;
   if ((pub) and (himsg<>65535)) then begin
      j:=0;
      for i:=0 to himsg do begin
         ensureloaded(i);
         k:=mintab[getmixnum(i)].messagenum;
         if (k>j) then j:=k;
      end;
      mixr.messagenum:=j+1;
   end;
   with mixr do begin
      hdrptr:=filesize(brdf);
      isreplytoid:=0;
      isreplyto:=65535;
      numreplys:=0;
      getpackdatetime(@msgdate);
      getdayofweek(msgdowk);
      msgid:=memboard.lastmsgid;
      inc(memboard.lastmsgid);
      msgindexstat:=[miexist];
      if (pub) then begin
         if (rvalidate in thisuser.ac) then msgindexstat:=msgindexstat+[miunvalidated];
         if (aacs(memboard.mciacs)) then msgindexstat:=msgindexstat+[miallowmci];
      end;
   end;

   nl;
   saveit;
   if (not ansmsg) then dispose(msginfo);
   savesystat;
   ufo:=(filerec(bf).mode<>fmclosed);
   if (not ufo) then reset(bf);
   seek(bf,board-1);
   write(bf,memboard);
   if (not ufo) then close(bf);
   cl(9);
   readytogo;
   fsemsg:=TRUE;
   write_msg:=false;
end;
end.
