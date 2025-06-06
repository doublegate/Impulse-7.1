{$A+,B+,E+,F+,I+,L+,N-,O+,R-,V-}
unit initp;

interface

uses
   Overlay,  Crt,      Dos,      common4,  sysop2,   myio,     common1,
   common2,  common3,  records,  common5,  common,   sys,      wfcmenu,
   strProc,  sysop8;

procedure initBBS;

implementation

const
  configonly:boolean=FALSE;

function CheckTasking : string;
begin
   Case FindTasker OF
      0: CheckTasking := 'Dos';
      1: CheckTasking := 'DeskView';
      2: CheckTasking := 'OS/2';
      3: CheckTasking := 'Windows';
      4: CheckTasking := 'Novell';
      else checkTasking := 'UnKnown';
   End;
End;

procedure amtdone( sub : string; item : string);
begin
   writeLn(AcroNym+'_'+sub+': '+item);
end;

procedure readp;
var d:astr;
    a:integer;
    bb:boolean;

   function sc(s:astr; i:integer):char;
   begin
      sc:=s[i];
   end;

begin
   cursoron(FALSE);
   exteventtime:=0;
   answerbaud:=0;
   quitafterdone:=FALSE;
   nightly:=FALSE;
   localioonly:=FALSE;
   a:=0;
  thisnode:=1;
  textAttr := 7;
  while (a<paramcount) do begin
    inc(a);
    if ((sc(paramstr(a),1)='-') or (sc(paramstr(a),1)='/')) then
      case upCase(sc(paramstr(a),2)) of
        'B':answerbaud:=strStringToWord(copy(paramstr(a),3,length(paramstr(a))-2));
        'E':if (length(paramstr(a))>=4) then begin
              d:=strAllCaps(paramstr(a));
              case d[3] of
                'E':exiterrors:=value(copy(d,4,length(d)-3));
                'N':exitnormal:=value(copy(d,4,length(d)-3));
              end;
            end;
        'S':configonly := true;
        'L':localioonly:=TRUE;
        'M':begin
              thisnode:=value(copy(paramstr(a),3,length(paramstr(a))-2));
              bb:=false;
            end;
        'N':nightly:=TRUE;
        'P':begin packbasesonly:=TRUE; halt(255); end;
        '?':begin
              writeln(ProgName+' BBS v'+ver+' by WeNeedAName Software');
              writeln('  Command line:');
              writeln('    IMP.EXE <-B,-D,-E,-L,-M,-N,-S,-P,-Q,-X,-O,-?>');
              writeln;
              writeln('    -B<baud> Load Impulse At <baud>bps');
              writeln('    -D Data will load from the current directory');
              writeln('    -E<E,N,#> Exits (E)rrorlevel or (N)ormal or (#)number');
              writeln('    -L Loads Impulse in local mode');
              writeln('    -M<node> Loads Impulse node #<node>');
              writeln('    -N Run Nightly Events and Exit');
              writeln('    -P Pack Email and Msg bases and Exit');
              writeln('    -Q Quit after next user hangs up');
              writeln('    -S Loads Impulse in Config then Exits');
              writeln('    -X<time> Exit to run external event at <time>');
              writeln('    -? This help screen');
              writeln; writeln;
              cursoron(true);
              halt(exitnormal);
            end;
        'Q':quitafterdone:=TRUE;
        'X':exteventtime:=strStringToWord(copy(paramstr(a),3,length(paramstr(a))-2));
        'D': begin
            with status^ do begin
               gFilePath := start_dir + '\DATA\';
               aFilePath := start_dir + '\ANSI\';
               menuPath := start_dir + '\MENU\';
               trapPath := start_dir + '\TRAP\';
               msgPath := start_dir + '\MSGS\';
               tFilePath := start_dir + '\SCRIPT\';
               tempPath := start_dir + '\TEMP\';
               swapPath := start_dir + '\SWAP\';
            end;
        end;
      end;
  end;
  {if (paramstr(1)<>'') then
    thisnode:=1;}
  allowabort:=TRUE;
end;


procedure initp1;

var fp:file;
    wind:windowrec;
    v:verbrec;
    confr:confrec;
    numread,
    i:integer;
    npatch:boolean;
    lcall : lcallers;


  function existdir(fn:astr):boolean;
  var srec:searchrec;
  begin
     while (fn[length(fn)]='\') do fn:=copy(fn,1,length(fn)-1);
     findfirst(fexpand(sqoutsp(fn)),anyfile,srec);
     existdir := (doserror=0) and (srec.attr and directory=directory);
  end;

{  procedure abend(s:astr);
  begin
    writeln('['+s+']  --  Aborting');
    halt(exiterrors);
  end;}

  procedure findbadpaths;
  var s,s1,s2:astr;
      i:integer;
  begin
    with status^ do
      for i:=1 to 8 do begin
        case i of 1:s1:='DATA'; 2:s1:='MSGS'; 3:s1:='MENU'; 4:s1:='TEXT';
                  5:s1:='ANSI'; 6:s1:='LOG'; 7:s1:='TEMP'; 8:s1:='SWAP';
        end;
        case i of
          1:s:=gfilepath;  2:s:=msgpath;
          3:s:=menupath;   4:s:=tfilepath;
          5:s:=afilepath;  6:s:=trappath;
          7:s:=temppath;   8:s:=swappath;
        end;
        if (not existdir(s)) then begin
          cursoron(TRUE);
          textattr := 15;
          writeln(s1+' path is currently "'+s+'"');
          writeln('This path is bad or missing.');
          repeat
            s2 := s;
            write('New '+s1+' path: ');
            s2 := strAllCaps(sqOutSp(nInput(whereX,whereY+1,s2,60)));
            if (s=s2) or (s2='') then {abend('Illegal pathname error')}
            else begin
              if (s2<>'') then
                if (copy(s2,length(s2),1)<>'\') then s2:=s2+'\';
                case i of
                  1:gfilepath:=s2;  2:msgpath:=s2;
                  3:menupath:=s2;   4:tfilepath:=s2;
                  5:afilepath:=s2;  6:trappath:=s2;
                  7:temppath:=s2;   8:swappath:=s2;
                end;
              if (not existdir(s2)) then {$I-} mkdir(s2); {$I+}
            end;
          until (existdir(s2));
          cursoron(FALSE);
          writeln;
        end;
      end;
  end;

begin

   wantout:=TRUE;
   ldate:=daynum(date);
   ch:=FALSE;
   lil:=0;
   buf:='';
   chatcall:=FALSE;
   spd:='';
   lastname:='';
   ll:='';
   amtdone('file','status.dat');
   sysInitVariables;
   amtdone('init','variables');
   osName := checkTasking;
   amtdone('mutlitask','detected'+osName);
   tasker := FindTasker;
   amtdone('mem',cStr(memAvail)+' bytes of heap free');
   amtdone('mem',cStr(sPtr)+' bytes of stack free');
   If (overlayinems) then amtdone('ovrPos','XMS/EMS')
   else amtdone('ovrPos','DISK');
   amtdone('ovrBuffer',cStr(ovrGetBuf)+' bytes allocated');
   amtdone('ansi','v2 Installed - 4kb Buffer');
   sysResetMciList;
   amtdone('mci','list reset');
   assign(fp,status^.gfilepath+'modem.'+cStr(thisnode));
   {$I-} reset(fp,1); {$I+}
   if ioResult <> 0 then begin
      with modemr do begin
         waitbaud:=57600;
         comport:=0;
         init:='ATZ|';
         answer:='ATA|';
         hangup:='~~~+++~~~ATH0|';
         offhook:='ATH1M0|';
         nocallinittime:=30;
         hardware:=FALSE;
      end;
      rewrite(fp,1);
      blockWrite(fp,modemR,sizeOf(modemR));
   end
   else blockRead(fp,modemR,sizeOf(modemR));
   close(fp);
   if (modemr.comport=0) then localioonly:=TRUE;
   amtdone('file','modem.'+cStr(thisNode));

   assign(fp,status^.gfilepath+'laston.dat');
   {$I-} reset(fp,1); {$I+}
   if ioResult <> 0 then begin
      rewrite(fp,1);
      lastcalled := '';
   end
   else begin
      blockRead(fp,lcall,sizeOf(lcall));
      lastcalled := lcall.name;
   end;
   close(fp);
   amtdone('file','laston.dat');
   status^.temppath:=copy(status^.temppath,1, length(status^.temppath)-2)+cstr(thisnode);
   if not exist(status^.temppath) then begin
      {$I-} mkdir(status^.temppath); {$I+}
      tempdirchanged := true;
   end;
   status^.temppath:=status^.temppath+'\';
   if tempdirchanged then begin
      {$I-} mkdir(status^.temppath+'1'); {$I+}
      {$I-} mkdir(status^.temppath+'2'); {$I+}
      {$I-} mkdir(status^.temppath+'3'); {$I+}
   end;

   findbadpaths;
   amtdone('checking','paths');

   assign(fp,status^.gFilePath+'prompts.dat');
   {$I-} reset(fp,1); {$I+}
   if (ioresult<>0) then begin
      rewrite(fp,1);
      prmp^[1].name:='Impulse Standard';
      prmp^[1].prompt:='|03ÚÄÄÄ Ä ú|NL|11ÀÄ|03Â´|07|NM|NL|11ÚÄ|03ÁÄÄú |07|TL|03 ÃÄú|NL|03ÀÄÄ|08: |15';
      prmp^[2].name:='Nukka Grizeen';
      prmp^[2].prompt:='|10 Ü|B2±|02ßÜ²ß|10þ |07|NM|NL|10ßÛÜ|B2±|02ß|08Ü|B2²|08Üþ |15';
      prmp^[3].name:='Blueberry';
      prmp^[3].prompt:='|15|NM |08ø |15|TL|NL|09ÖÄ|01Ä|09Ä|01ÄÄÄÄù|NL|01ÓÄÄÄÄø: |07';
      prmp^[4].name:='pImpulse Prizompt';
      prmp^[4].prompt:='|08Ú nex|03/|112|08ÃÄ.|15|NM|NL|08³Ú:Ù. ÄÄÄú|NL|08ÀÁÄÄÙ: |15';
      blockWrite(fp,prmp^,sizeOf(prmp^));
   end
   else blockRead(fp,prmp^,sizeOf(prmp^));
   close(fp);
   amtdone('file','prompts.dat');

  assign(multinodef,status^.gFilePath+'nodes.dat');
  {$I-} reset(multinodef); {$I+}
  if (ioresult<>0) then begin
    rewrite(multinodef);
    nodeinfo.desc:='Node 1';
    nodeinfo.acs:='s30';
    nodeinfo.status:=0;
    nodeinfo.nodespd := 'NONE';
    nodeinfo.username:='Nobody';
    nodeinfo.usernum:=0;
    write(multinodef,nodeinfo);
    reset(multinodef);
  end;
  if(filesize(multinodef)<thisnode) then begin
    thisnode:=filesize(multinodef)+1;
    nodeinfo.desc:='Node '+cstr(thisnode);
    nodeinfo.acs:='';
    nodeinfo.status:=0;
    nodeinfo.nodespd := 'NONE';
    nodeinfo.username:='Nobody';
    nodeinfo.usernum:=0;
    seek(multinodef,filesize(multinodef));
    write(multinodef,nodeinfo);
    reset(multinodef);
  end;
  seek(multinodef,thisnode-1);
  read(multinodef,nodeinfo);
  close(multinodef);
  amtdone('file','nodes.dat');

  tempdirchanged:=false;

  assign(fp,'msgtmp');
  if exist('msgtmp') then begin
     erase(fp);
     amtdone('clean','msgtmp erased');
  end;
  assign(sysopf,status^.trappath+'sysop.'+cstr(thisnode));
  {$I-} append(sysopf); {$I+}
  if (ioresult<>0) then begin
    rewrite(sysopf);
    writeln(sysopf);
  end;
  assign(fp,status^.gfilepath+'nuorder.dat');
  {$I-}reset(fp,1); {$I+}
  if ioResult <> 0 then begin
     rewrite(fp,1);
     fillchar(nuOrder,sizeOf(nuOrder),0);
     nuorder[1] := 7;
     nuorder[2] := 10;
     nuorder[3] := 9;
     nuorder[4] := 4;
     nuorder[5] := 1;
     nuorder[6] := 8;
     nuorder[7] := 12;
     nuorder[8] := 2;
     nuorder[9] := 6;
     nuorder[10] := 13;
     nuorder[11] := 11;
     nuorder[12] := 3;
     nuorder[13] := 24;
     nuorder[14] := 26;
     nuorder[15] := 5;
     blockwrite(fp,nuOrder,sizeOf(nuOrder));
  end;
  blockread(fp,nuOrder,sizeOf(nuOrder));
  close(fp);
  amtdone('file','nuorder.dat');
  assign(sysopf1,status^.trappath+'slogxxxx.log');

  first_time:=TRUE;
  sl1(#3#7+'---------------> '+#3#5+'System booted on '+dat+#3#7+' <---------------');

  assign(fp,status^.gFilePath+'string.dat');
  {$I-} reset(fp,1); {$I+}
  if ioResult <> 0 then exit;
  blockRead(fp,strings^,sizeOf(strings^));
  close(fp);
  amtdone('file','string.dat');

  assign(fp,status^.gFilePath+'fidonet.dat');
  {$I-} reset(fp,1); {$I+}
  if (ioresult<>0) then begin
     rewrite(fp,1);
    with fidor do begin
      zone:=0; net:=0; node:=0; point:=0;
      for i:=1 to 50 do origin[i]:=chr(0);
      origin:=copy(stripcolor(status^.bbsname),1,50);
      text_color:=1; quote_color:=3; tear_color:=9; origin_color:=5;
      skludge:=TRUE; sseenby:=TRUE; sorigin:=FALSE;
      scenter:=TRUE; sbox:=TRUE; mcenter:=TRUE;
      {for i:=1 to sizeof(res) do res[i]:=0;}
    end;
    blockWrite(fp,fidoR,sizeOf(fidoR));
  end
  else blockRead(fp,fidoR,sizeOf(fidoR));
  close(fp);

  amtdone('file','fidonet.dat');

  assign(sf,status^.gfilepath+'names.lst');
  {$I-} reset(sf); {$I+}
  if (ioresult<>0) then begin
     pauseScr;
     sysInitSmallRecFile;
     reset(sf);
  end;
  if (status^.numusers<>filesize(sf)) then begin
    status^.numusers:=filesize(sf);
    savesystat;
  end;
  close(sf);
  amtdone('file','names.lst');
  assign(uf,status^.gfilepath+'user.lst');
  {$I-} reset(uf); {$I+}
  if ((ioResult <> 0)) then begin
     sysInitUserRecFile;
     reset(uf);
  end;
  seek(uf,1);
  read(uf,thisuser);
  close(uf);
  amtdone('file','user.lst');

  assign(fp,status^.gFilePath+'fconf.dat');
  {$I-} reset(fp,1); {$I+}
  if (ioresult<>0) then begin
     rewrite(fp,1);
     for i:=1 to 20 do confr.confname[i]:='';
     for i:=1 to 20 do confr.acslevel[i]:='';
     confr.confname[1]:='Main Conference';
     confr.acslevel[1]:='s30';
     blockWrite(fp,confR,sizeOf(confR));
  end
  else blockRead(fp,confR,sizeOf(confR));
  close(fp);

  amtdone('file','fconf.dat');

  minTabLoaded := 0;
  assign(fp,status^.gFilePath+'mConf.dat');
  {$I-} reset(fp,1); {$I+}
  if (ioresult<>0) then begin
     rewrite(fp,1);
     for i:=1 to 20 do begin
        confr.confname[i]:='';
        confr.acslevel[i]:='';
     end;
     confr.confname[1]:='Main Conference';
     confr.acslevel[1]:='s30';
     blockWrite(fp,confR,sizeOf(confR));
  end
  else blockRead(fp,confR,sizeOf(confR));
  close(fp);
  amtdone('file','mconf.dat');

  assign(verbf,status^.gfilepath+'verbose.dat');
  {$I-} reset(verbf); {$I+}
  if (ioresult<>0) then begin
     rewrite(verbf);
     v.descr[1] := '';
     write(verbf,v);
  end
  else read(verbf,v);
  close(verbf);

  amtdone('file','verbose.dat');

  assign(xf,status^.gfilepath+'protocol.dat');
  reset(xf);
  close(xf);

  amtdone('file','protocol.dat');

  new(events[0]);
  with events[0]^ do begin
    active:=nightly;
    description:='Nightly Events';
    etype:='D';
    execdata:='night.bat';
    busytime:=15;
    exectime:=240;  {* 4:00am *}
    busyduring:=TRUE;
    duration:=1;
    execdays:=127;  {* SMTWTFS *}
    monthly:=FALSE;
  end;
  assign(fp,status^.gfilepath+'events.dat');
  {$I-} reset(fp,1); {$I+}
  if (ioresult<>0) then begin
    rewrite(fp,1);
    numevents:=1;
    new(events[1]);
    with events[1]^ do begin
      active:=FALSE;
      description:='Event #1';
      etype:='D';
      execdata:='event.bat';
      busytime:=5;
      exectime:=0;
      busyduring:=TRUE;
      duration:=1;
      execdays:=0;
      monthly:=FALSE;
    end;
    blockWrite(fp,events[1]^,sizeOf(events[1]^));
    dispose(events[1]);
    dispose(events[0]);
  end else begin
    numevents:=0;
    repeat
      inc(numevents);
      new(events[numevents]);           (* DEFINE DYNAMIC MEMORY! *)
      blockread(fp,events[numevents]^,sizeof(eventrec),numread);
      if ((numread<>sizeof(eventrec)) and (numread<>0)) then npatch:=TRUE;
    until (numread<>sizeof(eventrec)) or (eof(fp));
  end;
  if (npatch) then begin
    rewrite(fp,1);
    for i:=1 to numevents do begin
      blockWrite(fp,events[1]^,sizeOf(events[1]^));
      dispose(events[i]);
    end;
  end;
  close(fp);
  amtdone('file','events.dat');

  assign(bf,status^.gfilepath+'boards.dat');
  numboards := 0;
  {$I-} reset(bf); {$I+}
  if (ioResult <> 0) then begin
     rewrite(bf);
     sysInitMsgBoardFile;
     reset(bf);
  end
  else begin
     repeat
        inc(numboards);
        read(bf,memboard);
     until eof(bf);
     close(bf);
  end;
  amtdone('file','boards.dat');

  assign(ulf,status^.gfilepath+'uploads.dat');
  maxulb := -1;
  {$I-} reset(ulf); {$I+}
  if (ioResult <> 0) then begin
     rewrite(ulf);
     sysInitFileBoardFile;
     reset(ulf);
  end
  else begin
     repeat
        inc(maxulb);
        read(ulf,memuboard);
     until eof(ulf);
     close(ulf);
  end;
  amtdone('file','uploads.dat');
{  assign(fp,status^.gfilepath+'uploads.dat');
  assign(ulf,status^.gfilepath+'uploads.dat');
  reset(fp,1); maxulb:=-1;
  repeat
    inc(maxulb);
    blockread(fp,memuboard,sizeof(ulrec),numread);
    if ((numread<>sizeof(ulrec)) and (numread<>0)) then npatch:=TRUE;
  until (numread<>sizeof(ulrec)) or (eof(fp));
  close(fp);

  amtdone('file','uploads.dat');}
  cfo:=FALSE;
  textattr := 7;
end;

procedure initBBS;
var rcode:integer;
begin
  if (daynum(date)=0) then begin
    writeln('Please set the date & time, it is required for operation.');
    halt(exiterrors);
  end;
  hangup:=FALSE;
  incom:=FALSE;
  outcom:=FALSE;
  echo:=TRUE;
  doneday:=FALSE;
  checkbreak:=TRUE;
  slogging:=TRUE;
  trapping:=FALSE;
  readingmail:=FALSE;
  sysopon:=FALSE;
  {directvideo:=not status^.usebios;}
  readp;
  initp1;
  if (configonly) then begin
     wfcmdefine;
     window(1,1,80,25);
     pullconfig;
     cursoron(true);
     halt(0);
  end;
  iport;
  cursoron(false);
  if (exist('bbsstart.bat')) then shelldos(FALSE,'bbsstart.bat',rcode);
end;
end.