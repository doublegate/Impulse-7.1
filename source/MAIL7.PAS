{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mail7;

interface

uses
   Crt,   Dos,  common1, common3,common5,records,strproc,
  common, common2, timejunk, mail0, mail5, file8, execbat, file2;

procedure sendqwk;
procedure recieverep;

implementation

type
     bsingle = array [0..3] of byte;
Var
   mbpointer:array[1..150] of byte;

procedure real_to_msb (preal : real; var b : bsingle);
var
     r : array [0 .. 5] of byte absolute preal;
begin
     b [3] := r [0];
     move (r [3], b [0], 3);
end; { procedure real_to_msb }

function msb_to_real (b : bsingle) : real;
var
     preal : real;
     r : array [0..5] of byte absolute preal;
begin
     r [0] := b [3];
     r [1] := 0;
     r [2] := 0;
     move (b [0], r [3], 3);
     msb_to_real := preal;
end; { procedure msb_to_real }

procedure dosread(var f:file; var s:astr; c:integer);
var i:integer;
    toread:array[1..256] of byte;
begin
  blockread(f,toread,c);
  s[0]:=chr(c);
  for i:=1 to c do begin
    s[i]:=chr(toread[i]);
  end;
end;

procedure doswrite(var f:file; s:astr);  { This is used to write a string without  }
var towrite:array [1..256] of byte;      { It's length character so it will appear }
    i:integer;                           { Normal in a text file, but to use an    }
begin                                    { untyped file, (not a "text")            }
  for i:=1 to length(s) do begin
    towrite[i]:=ord(s[i]);
  end;
  blockwrite(f,towrite,length(s));
end;

procedure doswritecr(var f:file; s:astr);
begin
  doswrite(f,s+#13+#10);
end;

procedure doswritenum(var f:file; s:astr; num:integer; pad:char);
begin
  while(length(s)<num) do s:=s+pad;
  if(length(s)>num) then s[0]:=chr(num);
  doswrite(f,s);
end;

procedure doswritenumb(var f:file; s:astr; num:integer; pad:char);
begin
  while(length(s)<num) do s:=pad+s;
  if(length(s)>num) then s[0]:=chr(num);
  doswrite(f,s);
end;

procedure purge_space(var s:astr);
begin
  while(s[length(s)]=' ') do s:=copy(s,1,length(s)-1);
end;

function acronym(s:astr):astr;
var r:astr;
    words:array [1..10] of astr;
    i,i1:integer;
begin
  i:=1;
  while(pos(' ',s)<>0) do begin
    words[i]:=copy(s,1,pos(' ',s)-1);
    s:=copy(s,pos(' ',s)+1,length(s)-pos(' ',s));
    inc(i);
  end;
  words[i]:=s;
  if(i=1) then begin
    if(length(words[1])>8) then begin
      r:=copy(words[1],1,8);
    end else begin
      r:=words[1];
    end;
  end else if(i=2) then begin
    if(length(words[1])>4) then begin
      r:=copy(words[1],1,4);
    end else begin
      r:=words[1];
    end;
    if(length(words[2])>4) then begin
      r:=r+copy(words[2],1,4);
    end else begin
      r:=r+words[2];
    end;
  end else begin
    r:='';
    if(i>8) then i:=8;
    for i1:=1 to i do begin
      r:=r+words[i1][1];
    end;
  end;
  for i:=1 to length(r) do begin
    r[i]:=upcase(r[i]);
  end;
  acronym:=r;
end;

procedure sendqwk;
var stripansi:boolean;
    s,s1:astr;
    f,f1,f2:file;
    i,i1,i2,messsofar,basesofar,oc,ptrsofar,msgsofar,bytesoutsofar,bytesinsofar,totbytes,confsofar:integer;
    recordnum:real;
    confptrs,realptrs:array [0..256] of integer;
    mhead:mheaderrec;
    mix:msgindexrec;
    md:ldatetimerec;
    u:userrec;
    mdp:packdatetime;
    mco,a,b:boolean;
    conv:bsingle;
    loc1,loc2:longint;
begin
  messsofar:=0;  basesofar:=0;
{  oc:=memboard.conf; } ptrsofar:=0;

  writehdr('QWK Mail');
  nl;
  stripansi:=true;
  nl;
  sprint('Preparing .QWK packet...');
  purgedir(status^.temppath+'3\');

      for i:=1 to numboards do begin
        mbpointer[i]:=i;
	realptrs[ptrsofar]:=mbpointer[i];
	confptrs[mbpointer[i]]:=ptrsofar;
	inc(ptrsofar);
      end;

  assign(f,status^.temppath+'3\MESSAGES.DAT');  { Assign the messages file }
  rewrite(f,1);

  assign(f1,status^.temppath+'3\PERSONAL.NDX');
  rewrite(f1,1);

  { Write the false message header }                                                         { CR  LF  EOF }
  doswritenum(f,'Produced by '+ProgName+' v'+ver,128,' ');

  recordnum:=2.0;  (* 2nd record because first was "Produced by..." *)
  msgsofar:=0;
  for i:=0 to (ptrsofar-1) do begin
    s:=cstr(i);
    while(length(s)<3) do s:='0'+s;
    assign(f2,status^.temppath+'3\'+s+'.NDX');
    rewrite(f2,1);

    if((realptrs[i]<>255) and (realptrs[i] in zscanr.mzscan)) then begin
      board:=realptrs[i];
      changeboard(realptrs[i]);
      initbrd(realptrs[i]);
      sprint(strings^.newscan1);
      for i1:=0 to himsg do begin
	ensureloaded(i1);
	mix:=mintab[getmixnum(i1)];
	for i2:=1 to 6 do begin
	  mdp[i2]:=mix.msgdate[i2];
	end;

	if(isnew(i1)) then begin
	  loadmhead(i1,mhead);

	  if(miexist in mix.msgindexstat) then begin
	    real_to_msb(recordnum,conv);
	    blockwrite(f2,conv,4);
	    doswrite(f2,#0);
	    if(pos(thisuser.name,mhead.toi.asn)=1) then begin
	      blockwrite(f1,conv,4);
	      doswrite(f1,#0);
	    end;

	    inc(msgsofar);

	    doswrite(f,' ');  { Write header }
	    doswritenum(f,cstr(i1+1),7,' ');

	    pdt2dt(mdp,md);
	    doswritenumb(f,cstr(md.month),2,'0');
	    doswrite(f,'-');
	    doswritenumb(f,cstr(md.day),2,'0');
	    doswrite(f,'-');
	    doswritenumb(f,cstr(md.year mod 100),2,'0');
	    doswritenumb(f,cstr(md.hour),2,'0');
	    doswrite(f,':');
	    doswritenumb(f,cstr(md.min),2,'0');

	    if(pos('#',mhead.toi.asn)<>0) then begin
	      doswritenum(f,copy(mhead.toi.asn,1,pos('#',mhead.toi.asn)-2),25,' ');
	    end else begin
	      doswritenum(f,mhead.toi.asn,25,' ');
	    end;
	    if(pos('#',mhead.fromi.asn)<>0) then begin
	      doswritenum(f,copy(mhead.fromi.asn,1,pos('#',mhead.fromi.asn)-2),25,' ');
	    end else begin
	      doswritenum(f,mhead.fromi.asn,25,' ');
	    end;
	    doswritenum(f,mhead.title,25,' ');
	    doswritenum(f,'',12,' ');

	    doswritenum(f,cstr(himsg-1),8,' ');
	    loc1:=filepos(f);
	    doswrite(f,'      ');  { Leave blank for now, fill in after writing }
				   { the actual message }
	    doswrite(f,#225);
	    blockwrite(f,i,2);
	    blockwrite(f,msgsofar,2);
	    doswrite(f,' ');

	    recordnum:=recordnum+1;

	    totbytes:=((mhead.msglength div 128)+1)*128;
	    bytesoutsofar:=0;  bytesinsofar:=0;
	    seek(brdf,mhead.msgptr);
	    while(bytesinsofar<mhead.msglength) do begin
	      blockreadstr2(brdf,s);
	      bytesinsofar:=bytesinsofar+length(s)+2;
	      while(pos(#3,s)<>0) do begin  { Convert telegard style color codes to ANSI }
		i2:=pos(#3,s);
		s1:=copy(s,1,i2-1)+getc(thisuser.cols[(color in thisuser.ac)][ord(s[i2+1])])+
		    copy(s,i2+2,length(s)-i2-1);
                s:=s1;
	      end;
              if(stripansi) then begin
                while(pos(#27,s)<>0) do begin  { Strip ANSI control codes }
                  i2:=pos(#27,s);
                  s1:=copy(s,1,i2-1);
                  s:=copy(s,i2,length(s)-i2+1);
                  while( ( ( ord(s[1])>ord('Z') ) or ( ord(s[1])<ord('A') ) ) and
			 ( ( ord(s[1])>ord('z') ) or ( ord(s[1])<ord('a') ) ) ) do begin
		    s:=copy(s,2,length(s)-1);
		  end;
		  s:=copy(s,2,length(s)-1);
		  s1:=s1+s;
		  s:=s1;
		end;
	      end;
	      doswrite(f,s+#227);
	      bytesoutsofar:=bytesoutsofar+length(s)+1;
	    end;
	    while((bytesoutsofar mod 128)<>0) do begin
	      doswrite(f,' ');
	      inc(bytesoutsofar);
	    end;
	    recordnum:=recordnum+(bytesoutsofar div 128);

	    loc2:=filepos(f);
	    seek(f,loc1);  { Fill in # of blocks now }
	    doswritenum(f,cstr((bytesoutsofar div 128)+1),6,' ');
	    seek(f,loc2);
	  end;
	end;
      end;
      updateptr(himsg,a);
      closebrd;
    end;
    close(f2);
  end;
  close(f1);
  savezscanr;

  close(f);  { Close message file }

  assign(f,status^.temppath+'3\CONTROL.DAT');  { Assign control file }
  rewrite(f,1);

  doswritecr(f,status^.bbsname);
  reset(uf);  seek(uf,1);  read(uf,u);  close(uf);
  doswritecr(f,u.citystate);
  doswritecr(f,status^.bbsphone);
  doswritecr(f,status^.sysopname);
  doswritecr(f,'0,'+acronym(status^.bbsname));

  getdatetime(md);
  doswritenumb(f,cstr(md.month),2,'0');
  doswrite(f,'-');
  doswritenumb(f,cstr(md.day),2,'0');
  doswrite(f,'-');
  doswrite(f,cstr(md.year));
  doswrite(f,',');          doswritenumb(f,cstr(md.hour),2,'0');
  doswrite(f,':');
  doswritenumb(f,cstr(md.min),2,'0');
  doswrite(f,':');
  doswritenumb(f,cstr(md.sec),2,'0');
  doswritecr(f,'');

  doswritecr(f,thisuser.name);
  doswritecr(f,'');
  doswritecr(f,'0');
  doswritecr(f,cstr(msgsofar));
  doswritecr(f,cstr(ptrsofar-1));

  ptrsofar:=0;  confsofar:=0;
      for i:=1 to numboards do begin
        changeboard(mbpointer[i]);
	loadboard(mbpointer[i]);
	doswritecr(f,cstr(ptrsofar));
	doswritecr(f,memboard.name);
	inc(ptrsofar);
      end;

  doswritecr(f,'WELCOME.ANS');
  doswritecr(f,'NEWS.ANS');
  doswritecr(f,'LOGOFF.ANS');
  close(f);  { Close control file }

  assign(f,status^.temppath+'3\DOOR.ID');
  rewrite(f,1);

  doswritecr(f,'DOOR = '+ProgName);
  doswritecr(f,'VERSION = '+ver);
  doswritecr(f,'SYSTEM = '+ProgName+' '+ver);
  doswritecr(f,'CONTROLNAME = '+ProgName);
  doswritecr(f,'CONTROLTYPE = ADD');
  doswritecr(f,'CONTROLTYPE = DROP');

  close(f);

  copyfile(a,b,FALSE,status^.afilepath+'WELCOME.ANS',status^.temppath+'3\WELCOME.ANS');
  copyfile(a,b,FALSE,status^.afilepath+'LOGON.ANS',status^.temppath+'3\NEWS.ANS');
  copyfile(a,b,FALSE,status^.afilepath+'LOGOFF.ANS',status^.temppath+'3\LOGOFF.ANS');

  sprint('Zipping '+acronym(status^.bbsname)+'.QWK...');
  shel('');
  b:=false;
  shelldos(true,'pkzip '+status^.temppath+'3\'+acronym(status^.bbsname)+
                '.QWK '+status^.temppath+'3\*.*',doserror);
  shel2;

  sprint('Sending '+acronym(status^.bbsname)+'.QWK...');
  send1(status^.temppath+'3\'+acronym(status^.bbsname)+'.QWK',a,b);
end;


procedure recieverep;
var a,b,c,ufo:boolean;
    confsofar,ptrsofar,oc,i,j,k:integer;
    confptrs,realptrs:array [0..256] of integer;
    f:file;
    s,s1:string;
    rcode,sl,blocks,conf:integer;
    mhead:mheaderrec;
    mixr:msgindexrec;
    t:text;

begin
  writehdr('REP Receive');
  nl;

  purgedir(status^.temppath+'3\');
  sprint('Recieving '+acronym(status^.bbsname)+'.REP');
  receive1(status^.temppath+'3\'+acronym(status^.bbsname)+'.REP',false,a,b,c);

  sprint('Unzipping '+acronym(status^.bbsname)+'.REP');
  shel('');
  a:=TRUE;
  shelldos(true,'pkunzip '+status^.temppath+'3\'+acronym(status^.bbsname)+
          '.REP '+acronym(status^.bbsname)+'.MSG '+status^.temppath+'3 ',rcode);
  shel2;

  confsofar:=0;  ptrsofar:=0;
      for i:=1 to numboards do begin
        mbpointer[i]:=i;
        realptrs[ptrsofar]:=mbpointer[i];
	confptrs[mbpointer[i]]:=ptrsofar;
	inc(ptrsofar);
      end;

  assign(f,status^.temppath+'3\'+acronym(status^.bbsname)+'.MSG');
  {$I-}  reset(f,1);  {$I+}
  if(ioresult=0) then begin
    dosread(f,s,128);
    if(pos(acronym(status^.bbsname),s)<>1) then begin
      sprint('You sent the WRONG REP packet bub.');
    end else begin
      while(not eof(f)) do begin
	dosread(f,s,21);  { Read the message header }
	dosread(f,s,25);
        purge_space(s);
	mhead.toi.asn:=s;
	dosread(f,s,25);
	purge_space(s);
	mhead.fromi.asn:=s;
	dosread(f,s,25);
        purge_space(s);
	mhead.title:=s;
	dosread(f,s,20);
	dosread(f,s,6);
        purge_space(s);
	blocks:=value(s);
	dosread(f,s,1);
	blockread(f,conf,2);
	dosread(f,s,3);

  if(mhead.toi.asn=strAllCaps(ProgName)) then begin
	  if(pos('ADD',mhead.title)=1) then begin
	    if(realptrs[conf]<>255) then begin
	      loadboard(realptrs[conf]);
	      if(not (realptrs[conf] in zscanr.mzscan)) then zscanr.mzscan:=zscanr.mzscan + [realptrs[conf]];
	      sprint(memboard.name+' added');
	    end;
	  end else if(pos('DROP',mhead.title)=1) then begin
	    if(realptrs[conf]<>255) then begin
	      loadboard(realptrs[conf]);
	      if(realptrs[conf] in zscanr.mzscan) then zscanr.mzscan:=zscanr.mzscan - [realptrs[conf]];
	      sprint(memboard.name+' dropped');
	    end;
	  end else if(pos('RESET',mhead.title)=1) then begin
	    if(realptrs[conf]<>255) then begin
	      loadboard(realptrs[conf]);
	      s:=copy(mhead.title,7,length(mhead.title)-6);
	      updateptr(value(s)-1,a);
	      sprint('Pointer on ^5'+memboard.name+'^3 has been reset to '+s+'^4 þ');
	    end;
	  end else begin
	    sprint('Unrecognized command - "'+mhead.title+'" þ');
	  end;
	  for i:=2 to blocks do begin
	    dosread(f,s,128);
	  end;
	end else begin
	  loadboard(realptrs[conf]);
	  initbrd(realptrs[conf]);
	  s1:='';
	  assign(t,'REPMSG.'+cstr(thisnode));
	  rewrite(t);
	  for i:=2 to blocks do begin
	    dosread(f,s,128);
	    s1:=s1+s;
	    mhead.msglength:=0;
            while(pos(#227,s1)<>0) do begin
	      s:=copy(s1,1,pos(#227,s1)-1);
	      s1:=copy(s1,pos(#227,s1)+1,length(s1)-pos(#227,s1));
	      writeln(t,s);
	      mhead.msglength:=mhead.msglength+length(s)+2;
	    end;
	  end;
          with mhead do begin
            signature:=$FFFFFFFF;
            sl:=thisuser.sl;
            toi.anon:=0;
            toi.usernum:=0;
            toi.real:='';
            toi.alias:=toi.asn;
            fromi.anon:=0;
            fromi.usernum:=usernum;
            fromi.real:=thisuser.realname;
            fromi.alias:=thisuser.name;
          end;
	  close(t);
	  if (himsg<>65535) then begin
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
	    if (rvalidate in thisuser.ac) then
	      msgindexstat:=msgindexstat+[miunvalidated];
	    if (aacs(memboard.mciacs)) then
	      msgindexstat:=msgindexstat+[miallowmci];
	  end;
          outmessagetext('REPMSG.'+cstr(thisnode),mhead,TRUE);
	  ufo:=(filerec(bf).mode<>fmclosed);
	  savesystat;
          newmix(mixr);
          closebrd;
	end;
      end;
    end;
    close(f);
  end;
  savezscanr;
end;

end.
