{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mail3;

interface

uses
   Crt,   Dos,  common1,common2,common3,records,scrlbk,
  common5,common, timejunk, mail0,strproc,sys;

function what00(b:byte; s:string):string;
procedure readmsg(style:byte; anum,mnum,tnum:longint; var abort,next:boolean);

implementation

uses ansidrv;

function what00(b:byte; s:string):string;
begin
   if (b=0) then s:=caps(s)
   else
      case b of
         0:s:=caps(s);
         1,
         2:s:='Anonymous';
         3:s:='"Abby"';
         4:s:='"Problemed Person"';
         5:s:=caps(s);
         else s:=strAllCaps(s);
      end;
   what00:=s;
end;



{ anum=actual, mnum=M#/t#, tnum=m#/T# }
procedure readmsg(style:byte; anum,mnum,tnum:longint; var abort,next:boolean);
var mhead:mheaderrec;
    mixr:msgindexrec;
    pdt:packdatetime;
    dt:ldatetimerec;
    pr,s,s1:string;
    code:char;
    debug,pp:integer;
    r,t:text;
    brdsig,lng,maxm,totload:longint;
    i,numread:word;
    done,pub,seeanon,usemci,usereal,isfido:boolean;

  procedure getout;
  begin
    gotlastmheader:=TRUE;
    lastmheader:=mhead;
  end;

{  function tnn(lng:longint):string;
  var s:string;
  begin
    if (lng>100) then lng:=lng mod 100;  { ex: 1989 --> 89 }
 {   s:=cstr(lng); while (length(s)<2) do s:='0'+s;
    tnn:=s;
  end;}
begin
   usereal:=(mbrealname in memboard.mbstat);
   isfido:=(memboard.mbtype<>0);
   with mhead,mixr do begin
      loadmhead(anum,mhead);
      ensureloaded(anum);
      mixr:=mintab[getmixnum(anum)];
      usemci:=(miallowmci in mixr.msgindexstat);

    loadboard(board);
    wasyourmsg:=(fromi.usernum=usernum);
    pub:=(bread<>-1);
    if (pub) then seeanon:=aacs(status^.anonpubread)
      else seeanon:=aacs(status^.anonprivread);

    if (mhead.fromi.anon=2) then seeanon:=aacs(status^.csop);

    abort:=FALSE;
    next:=FALSE;
    cls;

    sysMciSet('BN',mln(aonoff(pub,memboard.name,'Private Mail'),30));
    sysMciSet('DT',pdt2dat(@msgdate,msgdowk));
    sysMciSet('FM',fromi.alias);
    sysMciSet('MN',cstr(mnum));
    sysMciSet('OU',cstr(tnum));
    sysMciSet('RT',cstr(isReplyToId));
    sysMciSet('TL',title);
    if (length(toi.alias)<1) then sysMciSet('TO','All')
    else sysMciSet('TO',caps(toi.alias));
    if (not (miexist in mixr.msgindexstat)) then sysMciSet('ST','Deleted')
    else if (miunvalidated in mixr.msgindexstat) then sysMciSet('ST','Not Validated')
    else if ((pub) and (mipermanent in msgindexstat)) then sysMciSet('ST','Permanent')
    else sysMciSet('ST','Public');
    printf('header');
{    if (exist(status^.afilepath+'header.*')) then begin
    printf('header');
   usereal:=(mbrealname in memboard.mbstat);
   isfido:=(memboard.mbtype<>0);
     end else
    begin

    sprint('|08컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴');

    s:=#3#0+'Number: '+#3#3+cstr(mnum)+'/'+cstr(tnum)+#3#1+' - '+#3#5+
      aonoff(pub,memboard.name,'Private Mail');
    sprint(s);

    s:='';
    if (not (miexist in mixr.msgindexstat)) then begin
      s:=#3#0+'Status: '+#3#8+'Deleted';
      if ((not mso) and (not wasyourmsg)) then begin
        sprint(s);
        getout;
        exit;
      end;
    end;

    if (miunvalidated in mixr.msgindexstat) then begin
      if (s='') then s:=#3#0+'Status:';
      s:=s+#3#7+' Not Validated';
      if ((not mso) and (not wasyourmsg)) then begin
        sprint(s);
        getout;
        exit;
      end;
    end;

    if ((pub) and (mipermanent in msgindexstat)) then begin
      if (s='') then s:=#3#0+'Status:';
      s:=s+#3#3+' Permanent';
    end;

    if (s<>'') then sprint(s);

    if ((fromi.anon=0) or (seeanon)) then s:=pdt2dat(@msgdate,msgdowk)
      else s:='Anonymous';
    s:=#3#0+'Date  : '+#3#3+s;

    if (style=4) then begin
      s:=s+#3#1+'  ('+aonoff(pub,#3#0+'Posted: ',#3#0+'Sent: ')+#3#9;
      for i:=1 to 6 do
        pdt[i]:=mixr.msgdate[i];
      pdt2dt(pdt,dt);
      s1:=tnn(dt.month)+'/'+tnn(dt.day)+'/'+tnn(dt.year);
      i:=daynum(date)-daynum(s1);
      s:=s+cstr(i)+' day'+aonoff((i=1),'','s')+' ago'+#3#1+')';
    end;

    sprint(s);
    if (origindate<>'') then
      if ((fromi.anon=0) or (seeanon)) then
        sprint(#3#0+'Origin: '+#3#3+origindate);

    s1:=fromi.asn;
    if (pub) then begin
      if (usereal) then begin
        s1:=fromi.real;
        if (not isfido) then s1:=s1+' #'+cstr(fromi.usernum);
      end;
    end;
    s:=#3#0+'From  : '+#3#5+caps(what00(fromi.anon,s1));
    if (not abort) then sprint(s);

    if ((seeanon) and (fromi.anon<>0) and (toi.anon=0) and (not isfido)) then begin
      s1:=#3#0+'(Real : '+#3#5;
      if (usereal) then s1:=s1+caps(fromi.real)
        else s1:=s1+caps(fromi.alias);
      s1:=s1+' #'+cstr(fromi.usernum)+#3#1+')';

      sprint(s1);
    end;

    if (style<>2) then begin
      s1:=toi.asn;
      if (pub) then begin
        if ((toi.asn='') and (isfido)) then s1:='All' else begin
          if ((usereal) and (toi.real<>'')) then begin
            s1:=strAllCaps(toi.real);
            if (not isfido) then s1:=s1+' #'+cstr(toi.usernum);
          end;
        end;
      end;
      if (s1<>'') then begin
        s:=#3#0+'To    : '+#3#5+caps(what00(toi.anon,s1));
        if (not usemci) then printacr(s,abort,next) else begin
          sprint(s);
          wkey(abort,next);
        end;
      end;
      if ((seeanon) and (toi.anon<>0) and (not isfido)) then begin
        if (fromi.anon=0) then begin
          s1:=#3#0+'(Real : '+#3#5;
          if (usereal) then s1:=s1+caps(toi.real)
            else s1:=s1+caps(toi.alias);
          s1:=s1+' #'+cstr(toi.usernum)+#3#1+')';
          sprint(s1);
        end else begin
          s1:=#3#0+'(Real : '+#3#5;
          if (usereal) then s1:=s1+caps(fromi.real)
            else s1:=s1+caps(fromi.alias);
          s1:=s1+' --> ';
          if (usereal) then s1:=s1+caps(toi.real)
            else s1:=s1+caps(toi.alias);
          s1:=s1+#3#1+')';
          sprint(s1);
        end;
      end;
    end;

    if (not usemci) then
      sprint(#3#0+'Title : '+#3#3+title)
    else begin
      sprint(#3#0+'Title : '+#3#3+title);
      wkey(abort,next);
    end;

    if (mixr.isreplyto<>65535) then
      sprint(#3#0+'Reply to message '+#3#5+cstr(mixr.isreplyto+1));
    i:=mixr.numreplys;
    if (i<>0) then
      sprint(#3#0+'This message has '+#3#5+cstr(i)+#3#3+' repl'+
        aonoff((i=1),'y','ies'));
    sprint(#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴');
    if ((fromi.anon=0) or (seeanon)) then
      lastname:=caps(what00(fromi.anon,fromi.asn))
    else
      lastname:='';
  end;}
    lastfrom:=fromi.usernum;
    if (not abort) then begin
      assign(t,'msgtmp');
      rewrite(t);
      append(t);
      writeln(t,caps(fromi.alias));
      reading_a_msg:=TRUE;
      read_with_mci:=(miallowmci in msgindexstat);
      totload:=0;
      abort:=FALSE;
      next:=FALSE;
      seek(brdf,mhead.msgptr);
      thisuser.ac:=thisuser.ac+[pause];
      repeat
        blockreadstr2(brdf,s);
        inc(totload,length(s)+2);
        if (okscrollback) then addtoscroll(wherey-1);
        if (pos(#3,s)<>0) then
          delete(s,pos(#3,s),2);
        if (pos('|',s)<>0) then sprint(s) else printacr(s,abort,next);
          if (pos(' ',fromi.alias)<>0) then
         writeln(t,mln({fromi.alias[1]+fromi.alias[pos(' ',fromi.alias)+1]+}'> '+s,75))
       else
        writeln(t,mln({fromi.alias[1]+fromi.alias[2]+}'> '+s,75));
      until ((totload>=msglength) or (abort));
      reset(t);
      close(t);
      myscrolly:=wherey;
      read_with_mci:=FALSE;
      reading_a_msg:=FALSE;
      sprint('');
      if (dosansion) then redrawforansi;
    end;
  end;
  sysResetMciList;
  getout;
end;

end.
