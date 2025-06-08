{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
Unit CONF; {File/Msg conference handling}

Interface

Uses common1, common4, common2, common3, Common, Mail5, Mail0, File11,
     records, common5,strproc;

Procedure ChangeConf(cnf:integer;num:string);
Procedure EditConf(Cnf:integer);
Procedure ConfNewscan(confType:Char;Cms:String);

Implementation


Var ConfDat:file of confrec;
    Confr:confrec;

Procedure ConfNewscan(confType:Char;Cms:string);
var i:integer;
    abort,next:boolean;
    bb,oldboard:integer;
    quit:boolean;
Begin
   confType:=Upcase(confType);
   if (confType = 'F') then begin
      CalConf(2);
      sysoplog('NewScan of file conference '+cstr(fconf));
      i:=0;
      abort:=FALSE;
      next:=FALSE;
      if (fconf = 0) then
         if (value(cms)>0) then searchb(value(cms),'*.*', false, abort,next, true)
         else while (not abort) and (i < maxulb) and (not hangup) do begin
            if (fbaseac(i)) then searchb(i,'*.*',false,abort,next,true);
            inc(i);
            wkey(abort,next);
            if (next) then begin
               abort:=FALSE;
               next:=FALSE;
            end;
         end;
      if (value(cms)>0) then searchb(areanum^[value(cms)],'*.*',false,abort,next,true)
      else while (not abort) and (areanum^[i]<>0) and (not hangup) do begin
         if (fbaseac(areanum^[i])) then searchb(areanum^[i],'*.*',false,abort,next, true);
         inc(i);
         wkey(abort,next);
         if (next) then begin
            abort:=FALSE;
            next:=FALSE;
         end;
      end;
   end
   else begin
      CalConf(1);
      sysoplog('NewScan of message bases in conference '+cstr(mconf));
      oldboard:=board;
      bb:=1;
      quit:=FALSE;
      if (mconf = 0) then
         if (value(cms)>0) then qscan(value(cms),quit)
         else while (bb<=numboards) and (not quit) and (not hangup) do begin
            if (mbaseac(bb)) then qscan(bb,quit);
            inc(bb);
         end;
      if (value(cms)>0) then qscan(areanum^[value(cms)],quit)
      else while (areanum^[bb]<=numboards) and (not quit) and (not hangup) do begin
         if (mbaseac(areanum^[bb])) then qscan(areanum^[bb],quit);
         inc(bb);
      end;
      board:=oldboard;
      initbrd(board);
   end;
end;

Procedure CheckConf(MsgFile:Char);
Begin
   If MsgFile='M' then Begin
      CalConf(1);
      ChangeBoard(Areanum^[1]);
      LoadBoard(Areanum^[1]);
   End
   else begin
      CalConf(2);
      ChangeFileBoard(Areanum^[1]);
      LoadUBoard(Areanum^[1]);
   End;
End;

Procedure OpenConfrec(confType:char;Open:Boolean);
Begin
   confType := upCase(confType);
   if (confType = 'M') then assign(confDat,status^.gfilePath+'MCONF.DAT')
   else assign(confDat,status^.gFilePath+'FCONF.DAT');
   if (open) then begin
      {$I-} reset(confDat); {$I+}
      if (ioResult = 0) then read(confDat,confr);
   end
   else if (ioResult <> 0) Or (not Open) then begin
      rewrite(confDat);
      write(confDat,confr);
   end;
   close(confDat);
end;

Procedure ChangeConf(cnf:integer;num:string);
var curnum, i:integer;
    chosen:string;
    chosen2:integer;
begin
   num:=strAllCaps(num);
   if (num='A') then chosen2:=0
   else if (not(num='A')) and (not(num='N')) and (not(num='')) then chosen2:=value(num)
   else if (not (num='A')) or (num='N') or (num='') then begin
      if (cnf <> 0) then writeHdr('File Confs')
      else writeHdr('Msg Confs');
      repeat
         Sprompt(strings^.entermsg1);
         input(chosen,2);
         if (chosen='?') then begin
            if (cnf <> 0) then openConfRec('F',true)
            else openConfRec('M',true);
            nl;
            sprint('|U9'+expand('-',20));
            curnum:=1;
            while (confr.confname[curnum]<>'') do begin
               sprint('|U0'+cstr(curnum)+' - '+confr.confname[curnum]);
               inc(curnum);
            end;
            sprint('|U00 - All Conferences');
            sprint('|U9'+expand('-',20));
            nl;
            if (cnf <> 0) then openConfRec('F',false)
            else openConfRec('M',false);
         end
         else if (chosen='') then chosen2:=0
            else chosen2:=value(chosen);
      until (chosen<>'?');
   end;
   if (chosen2=0) then begin
      if (aacs(confr.acslevel[chosen2])) then FConf:=Chosen2;
      if (aacs(confr.acslevel[chosen2])) then Mconf:=chosen2;
   end;
   if (cnf <> 0) then if (aacs(confr.acslevel[chosen2])) then FConf:=Chosen2
   else if (aacs(confr.acslevel[chosen2])) then Mconf:=chosen2;
   nl;
   if (not aacs(confr.acslevel[chosen2])) then sprint('You do not have ACS for that Conference');
   if (cnf <> 0) then checkConf('F')
   else checkConf('M');
end;

Procedure EditConf(Cnf:integer);
var total, curnum:integer;
    chosen:string;
    chosen2:integer;
    c:char;
    quit:boolean;
begin
   quit := false;
   cls;
   if (cnf <> 0) then begin
      writeHdr('File Conf Edit');
      openConfRec('F',true);
   end
   else begin
      Writehdr('Msg Conf Edit');
      openconfrec('M',true);
   end;
   sprint('|U9'+expand('-',20));
   curnum:=1;
   while (length(confr.confname[curnum])<>0) do begin
      sprint(cstr(curnum)+' - '+confr.confname[curnum]);
      inc(curnum);
   end;
   total:=curnum-1;
   sprint('|U9'+expand('-',20));
   nl;
   repeat
      Sprompt('|U0L|U2ist / |U0E|U2dit / |U0C|U2reate / |U0D|U2elete / |U0Q|U2uit : ');
      onek(c,'ECDQL');
      case c of
         'Q': quit:=true;
         'L': begin
            if (cnf <> 0) then writeHdr('File Conf Edit')
            else writeHdr('Msg Conf Edit');
            sprint('|U9'+expand('-',20));
            curnum:=1;
            while (length(confr.confname[curnum])<>0) do begin
               sprint(cstr(curnum)+' - '+confr.confname[curnum]);
               inc(curnum);
            end;
            total:=curnum-1;
            sprint('|U9'+expand('-',20));
            nl;
         end;
         'E': begin
            sprompt('Edit Which: ');
            input(chosen,2);
            chosen2:=value(chosen);
            if (chosen2<=total) then begin
               sprompt('Conf Name: ');
               inputl(confr.confname[chosen2],20);
               sprompt('ACS Level: ');
               inputl(confr.acslevel[chosen2],4);
            end;
         End;
         'C':begin
            inc(total);
            sprompt('Conf Name: ');
            inputl(confr.confname[total],20);
            sprompt('ACS Level: ');
            inputl(confr.acslevel[total],4);
         end;
         'D':begin
            sprompt('Delete Which: ');
            inputl(chosen,2);
            chosen2:=value(chosen);
            if (chosen2<=total) then begin
               confr.confname[chosen2]:='';
               confr.acslevel[chosen2]:='';
            end;
            dec(total);
         end;
      end;
   until (quit);
   if (cnf <> 0) then openConfRec('F',false)
   else openConfRec('M',false);
end;

end.
