{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mail5;

interface

uses
   Crt,      Dos,      common5,  records,  common,   timejunk, common1,
   common2,  common3,  mail0,    mail1,    mail2,    mail3,    mail4,
   mail6;

procedure post(replyto:longint; ttoi:fromtoinfo;hed:mheaderrec);
procedure scanmessages;
procedure qscan(b:integer; var quit:boolean);
procedure gnscan;
function isnew(cn:integer):boolean;
procedure updateptr(x:word; var zup:boolean);
procedure nscan(mstr:string);

implementation

uses menus, menus2;

procedure post(replyto:longint; ttoi:fromtoinfo;hed:mheaderrec);
var pinfo:pinforec;
    mheader:mheaderrec;
    mixr,mixr2:msgindexrec;
    saveit:string;
    i:integer;
    numread:word;
    savilevel:byte;
    ok:boolean;
    test:boolean;

   procedure nope(s:string);
   begin
      if (ok) then begin
         nl;
         print(s);
      end;
      ok:=FALSE;
   end;

begin
   ok:=TRUE;
   loadboard(board);
   mheader.title := hed.title;
   if (not aacs(memboard.postacs)) then nope('Your access does not allow you to post on this board.');
   if ((rpost in thisuser.ac) or (not aacs(status^.normpubpost))) then nope('Your access privledges do not include posting.');
   if ((ptoday>=status^.maxpubpost) and (not mso)) then nope('Too many messages posted today.');
   if (ok) then begin
      savepinfo(pinfo);
      initbrd(board);
      saveit:='';
      if (replyto<>-1) then begin
         mheader.toi:=ttoi;
         if (mheader.toi.anon<>0) then begin
            saveit:=mheader.toi.asn;
            mheader.toi.asn:=what00(mheader.toi.anon,mheader.toi.asn);
         end;
      end
      else mheader.toi.asn:='';
      if replyto <> -1 then test := TRUE
      else test := FALSE;
      if (fseMsg(TRUE,test,mheader.title,mixr,mheader)) then begin
         if (saveit<>'') then mheader.toi.asn:=saveit;
         seek(brdf,mixr.hdrptr);
         savemhead(mheader);
         if (replyto<>-1) then begin
            mixr.isreplyto:=replyto;
            mixr.numreplys:=0;
            mixr.msgindexstat:=mixr.msgindexstat+[mithreads];
            ensureloaded(replyto);
            mixr2:=mintab[getmixnum(replyto)];
            mixr.isreplytoid:=mixr2.msgid;
            inc(mixr2.numreplys);
            mixr2.msgindexstat:=mixr2.msgindexstat+[mithreads];
            ensureloaded(replyto);
            savemix(mixr2,replyto);
            ensureloaded(replyto);
         end;
         newmix(mixr);
         ensureloaded(himsg);
         sysoplog('+ "'+mheader.title+'" posted on '+#3#5+memboard.name);
         if (mheader.toi.asn<>'') then sysoplog('  To: "'+mheader.toi.asn+'"');
         topscr;
         sprint(strings^.msgsaved);
         inc(thisuser.msgpost);
         inc(ptoday);
         inc(status^.todayzlog.pubpost);
      end;
      loadpinfo(pinfo);
   end;
end;

function isnew1(msgdatepp:packdatetimepp):boolean;
var msgdate:packdatetime;
    l1,l2:longint;

   function zzzb(xx,yy:longint):longint;
   begin
      xx:=xx shl yy;
      zzzb:=xx;
   end;

begin
   l1 := 0; l2 := 0;
   msgdate:=msgdatepp^;
   isnew1:=FALSE;
   with zscanr do begin
      l1:=zzzb(msgdate[1],16);
      inc(l1,zzzb(msgdate[2],8));
      inc(l1,msgdate[3]);
      l2:=zzzb(mhiread[board][1],16);
      inc(l2,zzzb(mhiread[board][2],8));
      inc(l2,mhiread[board][3]);
      if (l1<l2) then exit;
      if (l1=l2) then begin
         l1:=zzzb(msgdate[4],16);
         inc(l1,zzzb(msgdate[5],8));
         inc(l1,msgdate[6]);
         l2:=zzzb(mhiread[board][4],16);
         inc(l2,zzzb(mhiread[board][5],8));
         inc(l2,mhiread[board][6]);
         if (l1<=l2) then exit;
      end;
      isnew1:=TRUE;
   end;
end;

function isnew(cn:integer):boolean;
var mixr:msgindexrec;
begin
   ensureloaded(cn);
   mixr:=mintab[getmixnum(cn)];
   isnew:=isnew1(@mixr.msgdate);
end;

procedure updateptr(x:word; var zup:boolean);
var mixr:msgindexrec;
    i:integer;
begin
   if (isnew(x)) then begin
      ensureloaded(x);
      mixr:=mintab[getmixnum(x)];
      for i:=1 to 6 do zscanr.mhiread[board][i]:=mixr.msgdate[i];
      zup:=TRUE;
   end;
end;

procedure pubreply(cn:word);
var t:text;
    mheader:mheaderrec;
    s:string;
    brdsig,dfdt1,dfdt2,newmsgptr,totload:longint;

begin
   if (gotlastmheader) then begin
      loadmhead(cn,mheader);
      assign(t,'msgtmp');
      rewrite(t);
      writeln(t,caps(mheader.fromi.alias));
      totload:=0;
      repeat
         blockreadstr2(brdf,s);
         inc(totload,length(s)+2);
         if (pos(' ',mheader.fromi.alias)<>0) then writeln(t,mln({mheader.fromi.alias[1]+
                                                    mheader.fromi.alias[pos(' ',mheader.fromi.alias)+1]+}'> '+s,75))
         else writeln(t,mln({mheader.fromi.alias[1]+mheader.fromi.alias[2]+}'> '+s,75));
      until (totload>=mheader.msglength);
      close(t);
      {post(-1,lastmheader.fromi);}
      post(cn,lastmheader.fromi,mheader);
      assign(t,'msgtmp');
      {$I-} reset(t); {$I+}
      if (ioresult=0) then begin
         close(t);
         erase(t);
      end;
   end
   else sprint(^M^J'Can''t reply YET.');
end;

type
  mstype =
    (msreadp,       { read prompt }
     msshowt,       { show titles }
     msreadm);      { read message }

  sttype =
    (stnewscan,     { NewScan mode }
     stscan);       { normal Scan mode }

procedure doscan(var quit:boolean; cn:word; st:sttype; ms:mstype);
var t:text;
    u:userrec;
    mheader:mheaderrec;
    mixr:msgindexrec;
    lastdate:packdatetime;
    inp,s:string;
    brdsig,getm,totload:longint;
    tmp : byte;
    i,j,k:integer;
    cmd:char;
    abort,askpost,b,donescan,hadunval,next,ufo,wasout,zup:boolean;

   procedure cbounds;
   begin
      wasout:=((cn<0) or (cn>himsg));
      if (not wasout) then exit;
      if (cn>himsg) then cn:=himsg
      else if (cn<0) then cn:=0;
   end;

   function tch(c:char; i:integer):string;
   var s:string;
   begin
      s:=cstr(i);
      if (i<10) then s:=c+s;
      tch:=s;
   end;

   procedure scaninput(var s:string; allowed:string);
   var os:string;
      i:integer;
      c:char;
      gotcmd:boolean;
   begin
      gotcmd:=FALSE;
      s:='';
      repeat
         getkey(c);
         c:=upcase(c);
         os:=s;
         if ((pos(c,allowed)<>0) and (s='')) then begin
            gotcmd:=TRUE;
            s:=c;
         end
         else if (pos(c,'0123456789')<>0) then if (length(s)<5) then s:=s+c
         else if ((s<>'') and (c=^H)) then s:=copy(s,1,length(s)-1)
         else if (c=^X) then begin
            prompt(expand(^H' '^H,length(s)));
            s:='';
            os:='';
         end
         else if (c=#13) then gotcmd:=TRUE;
         if (length(s)<length(os)) then prompt(^H' '^H);
         if (length(s)>length(os)) then prompt(copy(s,length(s),1));
      until ((gotcmd) or (hangup));
      nl;
   end;

begin
   last_menu := curmenu;
   curmenu := status^.menupath+'READP.MNU';
   readin;
   askpost:=FALSE;
   donescan:=FALSE;
   hadunval:=FALSE;
   zup:=FALSE;
   if (st=stscan) then readmsg(1,cn,cn+1,himsg+1,abort,next);
   loadmhead(cn,mheader);
   while ((not donescan) and (not hangup)) do begin
      getm := -1;
      if (ms=msshowt) then ms:=msreadp;
      if (ms=msreadp) then begin
         msg_on:=cn;
         cbounds;
         ensureloaded(cn);
         if (not abort) then begin
        {
        sprompt(strings^.scanmessage);
        scaninput(inp,'ABCDEHMPQRSTVWXZ-*!&?');
        }
        readpmnu := #0;
        mainmenuhandle(inp);
      end
      else inp:='';
      getm:=-1;
      cmd:=#0;
      if readpmnu = 'J' then begin
         sprompt(strings^.scanmessage);
         input(inp, 4);
         getm:=value(inp)-1;
         if (getm>-1) then
            if (st=stnewscan) then st:=stscan;
         inp := '';
      end;
      if readpmnu = 'N' then begin
         inp := '';
         readpmnu := #0;
         getm := cn+1;
      end;

      {
      if (inp='') then getm:=cn+1 else begin
        getm:=value(inp)-1;
        if (getm>-1) then
          if (st=stnewscan) then st:=stscan;
      end;
      }
      {if ((getm=-1) and (inp<>'')) then cmd:=inp[1];}
      case readpmnu of
        'A':getm:=cn;
        '-':begin
              getm:=cn-1;
              if (getm=-1) then begin
                 nl;
                 print('Already at the first message.');
                 nl;
              end;
              if (st=stnewscan) then st:=stscan;
            end;
      end;
      if (getm=-1){ and (cmd<>#0))} then
        case readpmnu of
          {
          '?':begin
                nl;
                sprint('|U9[|U0CR|U9]|U2Next message');
                lcmds(23,3,'-Prev. message','#Jump to #');
                lcmds(23,3,'Reply (pub/priv)','BNext board in NewScan');
                lcmds(23,3,'Post public','High message pointer');
                lcmds(23,3,'Again','Titles');
                lcmds(23,3,'Write public reply','ZToggle NewScan of this base');
                if (mso) then begin
                  lcmds(23,3,'Delete msg','*Toggle anonymous');
                  lcmds(23,3,'Validation toggle','Move msg to other base');
                  lcmds(23,3,'Xtract message to file','!Toggle permanence');
                  if (memboard.mbtype<>0) then
                   sprint('|U9[|U0S|U9]|U2Toggle scanned/outbound');
                end;
                nl;
              end;
           }
          'R':begin
                nl;
                if pynq('|07Is this to be a private reply? ',false) then autoreply
                  else pubreply(cn);
              end;
          'B':donescan:=TRUE;
          'D':if (mipermanent in mintab[getmixnum(cn)].msgindexstat) then begin
                nl; print('This is a permanent message.'); nl;
              end else begin
                loadmhead(cn,mheader);
                if ((cn>=0) and (cn<=himsg) and
                ((thisuser.name=mheader.fromi.alias) or (cso))) then begin
                  delmail(cn);
                  nl;
                  if (miexist in mintab[getmixnum(cn)].msgindexstat) then begin
                    print('Undeleted message.');
                    sysoplog('* Undeleted "'+mheader.title+'"');
                  end else begin
                    print('Deleted message.');
                    sysoplog('* Deleted "'+mheader.title+'"');
                  end;
                  nl;
                end else begin
                  nl; print('You didn''t post that!'); nl;
                end;

                if (cn>himsg) then cn:=himsg;
                if (himsg<=0) then begin donescan:=TRUE; askpost:=TRUE; end;
              end;
          'H':begin
                nl;
                i:=cn;
                print('Highest-read pointer for this base set to message #'+
                      cstr(i+1)+'.');
                nl;
                for i:=1 to 6 do
                  zscanr.mhiread[board][i]:=mintab[getmixnum(cn)].msgdate[i];
                savezscanr;
              end;
          'M': if (mso) then movemsg(cn);
          'P': begin
             nl;
             post(-1,mheader.fromi,mheader);
             nl;
          end;
          'Q': begin
             nl;
             quit:=TRUE;
             donescan:=TRUE;
          end;
          'T':ms:=msshowt;
          'V':if (mso) then begin
                loadmhead(cn,mheader); mixr:=mintab[getmixnum(cn)];
                if (miunvalidated in mixr.msgindexstat) then begin
                  nl; print('Message validated.'); nl;
                  mixr.msgindexstat:=mixr.msgindexstat-[miunvalidated];
                  sysoplog('* Validated "'+mheader.title+'"');
                end else begin
                  nl; print('Message unvalidated.'); nl;
                  mixr.msgindexstat:=mixr.msgindexstat+[miunvalidated];
                  sysoplog('* Unvalidated "'+mheader.title+'"');
                end;
                savemix(mixr,cn);
              end;
          'W':pubreply(cn);
          'X':if (mso) then begin
                nl;
                prompt('Extract filename? (default="EXT.TXT") : ');
                input(s,40);
                if (s='') then s:='EXT.TXT';
                if pynq('Are you sure? ',true) then begin
                  b:=pynq('Strip color codes from output? ',true);

                  loadmhead(cn,mheader);

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
          'Z':begin
                nl;
                sprompt(#3#5+memboard.name+#3#3);
                if (board in zscanr.mzscan) then begin
                  zscanr.mzscan:=zscanr.mzscan-[board];
                  sprint(' will NOT be scanned in future NewScans.');
                  sysoplog('* Took "'+#3#5+memboard.name+#3#1+'" out of NewScan');
                end else begin
                  zscanr.mzscan:=zscanr.mzscan+[board];
                  sprint(' WILL be scanned in future NewScans.');
                  sysoplog('* Put "'+#3#5+memboard.name+#3#1+'" back in NewScan');
                end;
                nl;
                savezscanr;
              end;
          '!':if (mso) then begin
                nl;
                loadmhead(cn,mheader); mixr:=mintab[getmixnum(cn)];
                if (mipermanent in mixr.msgindexstat) then begin
                  mixr.msgindexstat:=mixr.msgindexstat-[mipermanent];
                  print('Message is no longer permanent.');
                  sysoplog('* "'+mheader.title+'" made unpermanent');
                end else begin
                  mixr.msgindexstat:=mixr.msgindexstat+[mipermanent];
                  print('Message is now permanent.');
                  sysoplog('* "'+mheader.title+'" made permanent');
                end;
                savemix(mixr,cn);
                nl;
              end;
          '*':if (cso) then begin
                nl;
                j:=getmixnum(cn); loadmhead(j,mheader);
                if (mheader.fromi.anon in [1,2]) then
                  mheader.fromi.anon:=0
                else begin
                  i:=mheader.fromi.usernum;
                  ufo:=(filerec(uf).mode<>fmclosed);
                  if (not ufo) then reset(uf);
                  if ((i>=1) and (i<=filesize(uf)-1)) then begin
                    seek(uf,i); read(uf,u);
                    b:=aacs1(u,i,status^.csop);
                  end else
                    b:=FALSE;
                  if (not ufo) then close(uf);
                  if (b) then mheader.fromi.anon:=2 else mheader.fromi.anon:=1;
                end;
                seek(brdf,mintab[j].hdrptr);
                savemhead(mheader);
                if (mheader.fromi.anon=0) then begin
                  print('Message is no longer anonymous.');
                  sysoplog('* "'+mheader.title+'" made non-anonymous');
                end else begin
                  print('Message is now anonymous.');
                  sysoplog('* "'+mheader.title+'" made anonymous');
                end;
                nl;
              end;
          'S':if ((cso) and (memboard.mbtype<>0)) then begin
                nl;
                loadmhead(cn,mheader); mixr:=mintab[getmixnum(cn)];
                if (miscanned in mixr.msgindexstat) then begin
                  mixr.msgindexstat:=mixr.msgindexstat-[miscanned];
                  print('Message is no longer marked as scanned.');
                  sysoplog('* "'+mheader.title+'" not marked as scanned');
                end else begin
                  mixr.msgindexstat:=mixr.msgindexstat+[miscanned];
                  print('Message is now marked as "sent".');
                  sysoplog('* "'+mheader.title+'" marked as scanned');
                end;
                savemix(mixr,cn);
                nl;
              end;
        end;
    end;

    if (getm<>-1) then cn:=getm;
    cbounds;
    if (wasout) then begin
        donescan:=TRUE;
        if (ms=msreadp) then askpost:=TRUE;
    end;
    if (not donescan) then begin
      if (getm<>-1) then ms:=msreadm;
      if (ms=msreadm) then begin
        cls;
        if (miunvalidated in mintab[getmixnum(cn)].msgindexstat) then
          hadunval:=TRUE;
        readmsg(1,cn,cn+1,himsg+1,abort,next);
        updateptr(cn,zup);
        for i:=1 to 6 do lastdate[i]:=mintab[getmixnum(cn)].msgdate[i];
        if (not next) then ms:=msreadp else inc(cn);
        inc(mread);
      end;
    end;
  end;

  if ((hadunval) and (mso)) then begin
    nl;
    if pynq(^G'Validate messages here? ',true) then
      for i:=0 to himsg do begin
        ensureloaded(i); mixr:=mintab[getmixnum(i)];
        if (miunvalidated in mixr.msgindexstat) then begin
          mixr.msgindexstat:=mixr.msgindexstat-[miunvalidated];
          savemix(mixr,i);
        end;
      end;
  end;
  if ((askpost) and (aacs(memboard.postacs)) and
     (not (rpost in thisuser.ac)) and (ptoday<status^.maxpubpost)) then begin
    nl;
    if pynq(strings^.posthere,false) then
      post(-1,mheader.fromi,mheader);
  end;
  if (zup) then savezscanr;
  closebrd;
  curmenu := last_menu;
  readin;
end;

procedure scanmessages;
var cn:word;
    s:string;
    i:integer;
    c:char;
    quit:boolean;
begin
   initbrd(board);  { loads memboard }
   nl;
   if (himsg<>-1) then begin
      sprint(strings^.numbermsgs);
      sprompt(strings^.startmsgread);
      input(s,5);
      i:=value(s)-1;
      cn:=0;
      if (i<0) then i:=0
      else if (i<=himsg) then cn:=i;
      if (s<>'') then c:=s[1]
      else c:=^M;
      if (c<>'Q') then doscan(quit,cn,stscan,msshowt);
   end
   else sprint(strings^.nomsgs);
   closebrd;
end;

procedure qscan(b:integer; var quit:boolean);
var cn:word;
    oldboard,savlil,i:integer;
    abort,next:boolean;
begin
   oldboard:=board;
   if (not quit) then begin
      if (board<>b) then changeboard(b);
      if (board=b) then begin
         nl;
         initbrd(board);
         lil:=0;
         sprompt(strings^.newscan1);
         if (himsg<>-1) then begin
            cn:=0;
            while ((not isnew(cn)) and (cn<=himsg)) do inc(cn);
            if ((cn<=himsg) and (isnew(cn))) then doscan(quit,cn,stnewscan,msreadm)
            else quit:=FALSE;
         end;
         closebrd;
         if (not quit) then begin
            lil:=0;
            sprompt(strings^.newscan2);
         end;
      end;
      wkey(quit,next);
   end;
   board:=oldboard;
end;

procedure gnscan;
var bb,oldboard:integer;
    quit:boolean;
begin
   sysoplog('NewScan of message bases');
   oldboard:=board;
   nl;
   bb:=1;
   quit:=FALSE;
   repeat
      if (bb in zscanr.mzscan) then qscan(bb,quit);
      inc(bb);
   until ((bb>numboards) or (quit) or (hangup));
   nl;
   board:=oldboard;
   initbrd(board);
end;

procedure nscan(mstr:string);
var next:boolean;
begin
   next:=FALSE;
   if (mstr='C') then qscan(board,next)
   else if (mstr='G') then gnscan
   else if (value(mstr)<>0) then qscan(value(mstr),next)
   else begin
      nl;
      if pynq(strings^.globalnewscan,true) then gnscan else qscan (board,next);
   end;
end;

end.
