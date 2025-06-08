{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
{Init/Destroy}
unit sys;

interface

uses
   common,   records;

procedure sysInitVariables;
procedure sysDestroyVariables;
procedure sysDestroyFiles;
procedure sysInitUserRecFile;
procedure sysInitSmallRecFile;
procedure sysInitMsgBoardFile;
procedure sysInitFileBoardFile;
procedure sysResetMciList;
procedure sysMciSet(code,value : string);
Function FindTasker : Byte;
Procedure TimeSlice;

implementation

procedure sysInitVariables;
begin
   new(strings);
   new(prmp);
   new(areaNum);
{   new(strs);}
end;

procedure sysDestroyVariables;
begin
   dispose(strings);
   dispose(status);
   dispose(prmp);
   dispose(areaNum);
{   dispose(strs);}
end;

procedure sysDestroyFiles;
var f : file;
begin
   if (status^.globaltrap) then
      if exist(status^.trappath+'trap.msg') then begin
         assign(f,status^.trappath+'trap.msg');
         erase(f);
      end;

   if exist('msgtmp') then begin
      assign(f,'msgtmp');
      erase(f);
   end;
end;

procedure sysInitUserREcFile;
var user : userREc;
begin
   rewrite(uf);
   with user do begin
     fillchar(user,sizeOf(user),0);
     name     := 'SYSOP';
     realName := 'Sysop';
     pw       := 'SYSOP';
     ph       := '000-000-0000';
     bday     := '01/01/80';
     firston  := '01/01/80';
     laston   := '01/01/80';
     street   := '';
     cityState := '';
     zipCode := '92557';
     autosig := 'Impulse Sysop';
     note    := autosig;
     prompt := 1;
     lockedout := false;
     deleted := false;
     novotes := 0;
     yesvotes := 0;
     pagelen := 24;
     linelen := 80;
     sex := 'M';
     sl := 255;
     dsl := 255;
     ac := [ansi,onekey,color];
     fflag := [fname,fsize,fdesc];
     cols[TRUE,0] := 15;
     cols[TRUE,1] := 7;
     cols[TRUE,2] := 1;
     cols[TRUE,3] := 11;
     cols[TRUE,4] := 9;
     cols[TRUE,5] := 8;
     cols[TRUE,6] := 25;
     cols[TRUE,7] := 3;
     cols[TRUE,8] := 7;
     cols[TRUE,9] := 8;
     lastmsg := 0;
     lastfil := 0;
     trapactivity := false;
     trapseperate := false;
     slogseperate := false;
     clsmsg := 1;
     avadjust := 1;
   end;
   write(uf,user);
   write(uf,user);
end;

procedure sysInitSmallRecFile;
var small : smalRec;
begin
   rewrite(sf);
   small.name := 'SYSOP';
   small.deleted := false;
   small.number := 1;
   write(sf,small);
end;

procedure sysInitMsgBoardFile;
begin
   with memboard do begin
      name:='Unused';
      filename:='NEWBOARD';
      lastmsgid:=0;
      mbtype:=0;
      msgpath:='';
      acs:='s30';
      postacs:='s30';
      maxmsgs:=100;
      anonymous:=atno;
      password:='';
      permindx:=0;
      zone:=0;
      net:=0;
      node:=0;
      point:=0;
      if (fidor.origin<>'') then origin:=fidor.origin
      else origin:=copy(stripcolor(status^.bbsname),1,50);
      text_color:=fidor.text_color;
      quote_color:=fidor.quote_color;
      tear_color:=fidor.tear_color;
      origin_color:=fidor.origin_color;
      mbstat:=[];
      if (fidor.skludge) then mbstat:=mbstat+[mbskludge];
      if (fidor.sseenby) then mbstat:=mbstat+[mbsseenby];
      if (fidor.sorigin) then mbstat:=mbstat+[mbsorigin];
      if (fidor.scenter) then mbstat:=mbstat+[mbscenter];
      if (fidor.sbox) then mbstat:=mbstat+[mbsbox];
      if (fidor.mcenter) then mbstat:=mbstat+[mbmcenter];
      if (fidor.addtear) then mbstat:=mbstat+[mbaddtear];
      conf:=1;
      res[1] := 0;
      res[2] := 0;
   end;
   seek(bf,0);
   write(bf,memboard);
   inc(numboards);
end;

procedure sysInitFileBoardFile;
var s:string;
    i:integer;
begin
   with memuboard do begin
      getdir(0,s);
      name:='Unused';
      filename:='NEWDIR';
      dlpath:=s[1]+':DLOADS\';
      ulpath:=dlpath;
      maxfiles:=2000;
      password:='';
      arctype:=1;
      cmttype:=1;
      fbdepth:=0;
      fbstat:=[];
      acs:='';
      ulacs:='';
      nameacs:='';
      permindx:=0;
      conf:=1;
      for i:=1 to 5 do res[i]:=0;
   end;
   seek(ulf,0);
   write(ulf,memuboard);
   inc(maxulb);
end;

procedure sysResetMciList;
var cnt : word;
begin
   for cnt := NumMcis downto 1 do dispose(mciList[cnt]);    { dispose old mcis }
   numMcis := 0;
end;

procedure sysMciSet(code,value : string);
begin
   inc(numMcis);
   new(mciList[numMcis]);
   mciList[numMcis]^.code[1] := code[1];
   mciList[numMcis]^.code[2] := code[2];
   mciList[numMcis]^.value := value;
   mciList[numMcis]^.isAction := false;
end;

Function FindTasker : Byte; Assembler;
Asm
    MOV   AX, $3001
    INT   $21
    CMP   AL, $14
    JE    @SETOS2
    MOV   AX, $160A
    INT   $2F
    CMP   AX, $00
    JE    @SETWIN
    MOV   AX, $2B01
    MOV   CX, $4445
    MOV   DX, $5351
    INT   $21
    CMP   AL, $FF
    JNE   @SETDVX
    MOV   AX, $7A00
    INT   $2F
    CMP   AL, $FF
    JE    @SETNOV
    MOV   TASKER, 0
    JMP   @EXIT
@SETOS2:
    MOV   TASKER, 2
    JMP   @EXIT
@SETWIN:
    MOV   TASKER, 3
    JMP   @EXIT
@SETDVX:
    MOV   TASKER, 1
    JMP   @EXIT
@SETNOV:
    MOV   TASKER, 4
@EXIT:
    MOV   AL, TASKER
End;
Procedure TimeSlice; Assembler;
Asm
    CMP   TASKER, 0
    JE    @GIVEDOS
    CMP   TASKER, 1
    JE    @GIVEDVX
    CMP   TASKER, 2
    JE    @GOS2WIN
    CMP   TASKER, 3
    JE    @GOS2WIN
    MOV   BX,$000A
    INT   $7A
    JMP   @DONE
@GIVEDOS:
    INT   $28
    JMP   @DONE
@GIVEDVX:
    MOV   AX, $1000
    INT   $15
    JMP   @DONE
@GOS2WIN:
    MOV   AX, $1680
    INT   $2F
@DONE:
End;
end.
