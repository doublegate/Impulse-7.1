{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit myio;

interface

uses
   Crt,      dos;

const
  infield_seperators:set of char=[' ','\','.'];
  vidseg:word=$B800;
  ismono:boolean=FALSE;

type
  windowrec = array[0..4003] of byte;

procedure cursoron(b:boolean);
procedure infield1(x,y:byte; var s:string; len:integer);
procedure infielde(var s:string; len:byte);
procedure infield(var s:string; len:byte);
procedure cwrite(s:string);
procedure cwriteat(x,y:integer; s:string);
function cstringlength(s:string):integer;
procedure cwritecentered(y:integer; s:string);
procedure box(linetype,TLX,TLY,BRX,BRY:integer);
procedure checkvidseg;
procedure savescreen(var wind:windowrec; TLX,TLY,BRX,BRY:integer);
procedure setwindow(var wind:windowrec; TLX,TLY,BRX,BRY,tcolr,bcolr,boxtype:integer);
procedure removewindow(wind:windowrec);
procedure removewindow1(wind:windowrec);
procedure movewindow(wind:windowrec; TLX,TLY:integer);

implementation

Uses
   Common,   common5;

procedure cursoron(b:boolean);
var reg:registers;
begin
   with reg do begin
      if (b) then begin
         ch:=$07;
         cl:=$08;
      end
      else begin
         ch:=$09;
         cl:=$00;
      end;
      ah:=1;
      intr($10,reg);
   end;
   cursorthere:=not cursorthere;
end;

Function NInput(X,Y: Byte;StartStr,BackG,PassChar: String;MaxLen,StartPos:
               Integer;Ins: Boolean;
               var InputStatus: Byte):String;
var
 S      :String[80];
 ISB     :Byte;

Function Left(s: String;nr: byte): String;
begin
 Delete(s,nr+1,length(s));
 Left:=s;
end;

Function Mid(s: String;nr,nr2: byte): String;
begin
   Delete(s,1,nr-1);
   Delete(s,nr2+1,length(s));
   Mid:=s;
end;

Procedure WriteXY(x,y: Byte;s: String);
var
loop:   Word;
begin (* This can be _higly_ optimized *)
 for loop:=x to x+length(s)-1 do
 Mem[$B800:(loop-1)*2+(y-1)*160]:=Ord(S[loop-x+1]);
end;

procedure clr(c:integer);
var i,x,y:integer;
begin
   cl(6);
   x:=wherex;
   y:=wherey;
   prompt(expand(' ',c));
   gotoxy(x,y);
   dec(pap,c);
end;

Procedure NormalCursor; Assembler;
asm
 mov ah,1
 mov ch,6
 mov cl,7
 int $10
end;

Procedure BlockCursor; Assembler;
asm
 mov ah,1
 mov ch,0
 mov cl,7
 int $10
end;

Var
P         :Byte;
Exit      :Boolean;
ch        :Char;
tmp,ext   :Char;
t         :String[1];
arrow     :boolean;

begin

clr(maxlen);

Exit:=False;                                      { Don't quit on me yet! }
if Length(PassChar)>1 then PassChar:=PassChar[1]; { Just in Case... ;-) }
if Length(BackG)>1 then BackG:=BackG[1];
if Length(BackG)=0 then BackG:=' ';
if Length(StartStr)>MaxLen then StartStr:=Left(StartStr,MaxLen);
if StartPos>Length(StartStr) then StartPos:=Length(StartStr);
if StartPos=-1 then StartPos:=Length(StartStr);
If StartPos>=MaxLen then StartPos:=MaxLen-1;

s:=StartStr;                                { Put StartStr into Edit Buffer }

WriteXY(X,Y,expand(BackG,MaxLen));

if StartStr<>'' then begin
if passchar='' then WriteXY(X,Y,StartStr) else
                    WriteXY(X,Y,expand(PassChar,Length(StartStr)));
end;

p:=StartPos;
GotoXY(X+StartPos,Y);

repeat
 if Ins then NormalCursor else BlockCursor;
 ext:=#0;
 ch:=ReadKey;
 if ch=#0 then ext:=ReadKey;
 if ch=#27 then begin
                 InputStatus:=27;
                 Exit:=True;
                end;
if (ch<#255) and (ch>#31) then
 begin   { Welcome to the jungle...}
  t:=ch;
   if (p=length(s)) and (Length(s)<MaxLen) then
    begin
     s:=s+t;
     if PassChar='' then WriteXY(X+P,Y,T) else WriteXY(X+P,Y,PassChar);
     Inc(p);
    end else
     if length(s)<MaxLen then begin
      if Ins then Insert(T,S,P+1) else s[p+1]:=Ch;
      if PassChar='' then WriteXY(X+P,Y,Copy(S,P+1,Length(S))) else
WriteXY(X+Length(S)-1,Y,PassChar);      Inc(p);
     end else if (Length(s)=MaxLen) and (not Ins) then
      begin
       s[p+1]:=ch;
       if PassChar='' then WriteXY(X+P,Y,T) else WriteXY(X+P,Y,PassChar);
       Inc(p);
      end;
   ch:=#0;
   if p>MaxLen-1 then p:=MaxLen-1;
   GotoXY(X+P,Y);
  end else begin

 case ch of { CTRL-Y }
  #25:   begin
          WriteXY(X,Y,expand(BackG,Length(S)));
          P:=0;
          S:='';
          GotoXY(X,Y);
         end;

 {Backspace}
 #8: If (P>0) then
  begin
   if (p+1=MaxLen) and (p<length(s)) then Ext:=#83 else
    begin
     Delete(S,P,1);
     Dec(P);
     GotoXY(X+P,Y);
      if PassChar='' then WriteXY(X+P,Y,Copy(S,P+1,Length(s))+BackG) else
       if P>0 then WriteXY(X+Length(s)-1,Y,PassChar+BackG) else
        WriteXY(X+Length(s),Y,BackG);
    end;
  end;

  #9: begin { Exit on TAB }
       InputStatus:=9;
       Exit:=True;
      end;

 #13: begin
       InputStatus:=13;
       Exit:=True;
      end;
 end; { Case CH of }

 case ext of
 #75: if P>0 then begin
 {Left Arrow}      Dec(P);
                   GotoXY(X+P,Y);
                  end;

 #77: if (P<Length(s)) and (P+1<MaxLen) then begin
 {Right Arrow}             Inc(P);
                           GotoXY(X+P,Y);
                          end;

 #82: Ins:=Not(Ins); {Insert}
 {Delete}
 #83: If P<Length(s) then
  begin
   Delete(S,P+1,1);
    if PassChar='' then WriteXY(X+P,Y,Copy(S,P+1,Length(s))+BackG) else
     if p>0 then WriteXY(X+Length(S)-1,Y,PassChar+BackG) else
      WriteXY(X+Length(S),Y,BackG);
   end;

 #71: begin
       p:=0;
       GotoXY(X+P,Y);
      end;

 #79: begin
       p:=Length(s);
       if p>=MaxLen then P:=MaxLen-1;
       GotoXY(X+P,Y);
      end;

 #72,#73,#80,#81,#59..#68:
  begin
   InputStatus:=Ord(Ext);
   Exit:=True;
  end;

 end; {Case of EXT }
end; { if not normal char }

until Exit;
normalcursor;
if (s=' ') then if (pynq('|NL|NLNull String?',true)) then s:='';
NInput:=S;
end;

procedure infield1(x,y:byte; var s:string; len:integer);
Var InputStat:Byte;
Begin
  s:=NInput(X,Y,S,'°','',len,0,True,InputStat);
End;

procedure infielde(var s:string; len:byte);
begin
  infield1(wherex,wherey,s,len);
end;

procedure infield(var s:string; len:byte);
begin
  s:=''; infielde(s,len);
end;

procedure cwrite(s:string);
var i:integer;
    c:char;
    lastb,lastc:boolean;
begin
  lastb:=FALSE; lastc:=FALSE;
  for i:=1 to length(s) do begin
    c:=s[i];
    if ((lastb) or (lastc)) then begin
      if (lastb) then
        textbackground(ord(c))
      else
        if (lastc) then
          textcolor(ord(c));
      lastb:=FALSE; lastc:=FALSE;
    end else
      case c of
        #2:lastb:=TRUE;
        #3:lastc:=TRUE;
      else
           write(c);
      end;
  end;
end;

procedure cwriteat(x,y:integer; s:string);
begin
  gotoxy(x,y);
  cwrite(s);
end;

function cstringlength(s:string):integer;
var len,i:integer;
begin
  len:=length(s); i:=1;
  while (i<=length(s)) do begin
    if ((s[i]=#2) or (s[i]=#3)) then begin dec(len,2); inc(i); end;
    inc(i);
  end;
  cstringlength:=len;
end;

procedure cwritecentered(y:integer; s:string);
begin
  cwriteat(40-(cstringlength(s) div 2),y,s);
end;

{*
 *  ÚÄÄÄ¿   ÉÍÍÍ»   °°°°°   ±±±±±   ²²²²²   ÛÛÛÛÛ   ÖÄÄÄ·  ÕÍÍÍ¸
 *  ³ 1 ³   º 2 º   ° 3 °   ± 4 ±   ² 5 ²   Û 6 Û   º 7 º  ³ 8 ³
 *  ÀÄÄÄÙ   ÈÍÍÍ¼   °°°°°   ±±±±±   ²²²²²   ÛÛÛÛÛ   ÓÄÄÄ½  ÔÍÍÍ¾
 *}
procedure box(linetype,TLX,TLY,BRX,BRY:integer);
var i,j:integer;
    TL,TR,BL,BR,hline,vline:char;
begin
  window(1,1,80,25);
  case linetype of
    1:begin
        TL:=#218; TR:=#191; BL:=#192; BR:=#217;
        vline:=#179; hline:=#196;
      end;
    2:begin
        TL:=#201; TR:=#187; BL:=#200; BR:=#188;
        vline:=#186; hline:=#205;
      end;
    3:begin
        TL:=#176; TR:=#176; BL:=#176; BR:=#176;
        vline:=#176; hline:=#176;
      end;
    4:begin
        TL:=#177; TR:=#177; BL:=#177; BR:=#177;
        vline:=#177; hline:=#177;
      end;
    5:begin
        TL:=#178; TR:=#178; BL:=#178; BR:=#178;
        vline:=#178; hline:=#178;
      end;
    6:begin
        TL:=#219; TR:=#219; BL:=#219; BR:=#219;
        vline:=#219; hline:=#219;
      end;
    7:begin
        TL:=#214; TR:=#183; BL:=#211; BR:=#189;
        vline:=#186; hline:=#196;
      end;
    8:begin
        TL:=#213; TR:=#184; BL:=#212; BR:=#190;
        vline:=#179; hline:=#205;
      end;
  else
      begin
        TL:=#32; TR:=#32; BL:=#32; BR:=#32;
        vline:=#32; hline:=#32;
      end;
  end;
  gotoxy(TLX,TLY); write(TL);
  gotoxy(BRX,TLY); write(TR);
  gotoxy(TLX,BRY); write(BL);
  gotoxy(BRX,BRY); write(BR);
  for i:=TLX+1 to BRX-1 do begin
    gotoxy(i,TLY);
    write(hline);
  end;
  for i:=TLX+1 to BRX-1 do begin
    gotoxy(i,BRY);
    write(hline);
  end;
  for i:=TLY+1 to BRY-1 do begin
    gotoxy(TLX,i);
    write(vline);
  end;
  for i:=TLY+1 to BRY-1 do begin
    gotoxy(BRX,I);
    write(vline);
  end;
  if (linetype>0) then window(TLX+1,TLY+1,BRX-1,BRY-1)
                  else window(TLX,TLY,BRX,BRY);
end;

procedure checkvidseg;
begin
  if (mem[$0000:$0449]=7) then vidseg:=$B000 else vidseg:=$B800;
  ismono:=(vidseg=$B000);
end;

procedure savescreen(var wind:windowrec; TLX,TLY,BRX,BRY:integer);
var x,y,i:integer;
begin
  checkvidseg;

  wind[4000]:=TLX; wind[4001]:=TLY;
  wind[4002]:=BRX; wind[4003]:=BRY;

  i:=0;
  for y:=TLY to BRY do
    for x:=TLX to BRX do begin
      inline($FA);
      wind[i]:=mem[vidseg:(160*(y-1)+2*(x-1))];
      wind[i+1]:=mem[vidseg:(160*(y-1)+2*(x-1))+1];
      inline($FB);
      inc(i,2);
    end;
end;

procedure setwindow(var wind:windowrec; TLX,TLY,BRX,BRY,tcolr,bcolr,boxtype:integer);
var i:integer;
begin
  savescreen(wind,TLX,TLY,BRX,BRY);        { save under window }
  window(TLX,TLY,BRX,BRY);                 { set window size }
  textColor(tcolr);
  textBackGround(bcolr);
  cls;                                  { clear window for action }
  box(boxtype,TLX,TLY,BRX,BRY);            { Set the border }
end;

procedure removewindow(wind:windowrec);
var TLX,TLY,BRX,BRY,x,y,i:integer;
begin
  checkvidseg;

  window(1,1,80,25);
  textAttr := 14;
  TLX:=wind[4000]; TLY:=wind[4001];
  BRX:=wind[4002]; BRY:=wind[4003];

  i:=0;
  for y:=TLY to BRY do
    for x:=TLX to BRX do begin
      inline($FA);
      mem[vidseg:(160*(y-1)+2*(x-1))]:=wind[i];
      mem[vidseg:(160*(y-1)+2*(x-1))+1]:=wind[i+1];
      inline($FB);
      inc(i,2);
    end;
end;

procedure removewindow1(wind:windowrec);
var oldx1,oldy1,oldx2,oldy2,sx,sy,sz:byte;
begin
  sx:=wherex; sy:=wherey; sz:=textattr;
  oldx1:=lo(windmin); oldy1:=hi(windmin);
  oldx2:=lo(windmax); oldy2:=hi(windmax);

  removewindow(wind);

  window(oldx1,oldy1,oldx2,oldy2);
  gotoxy(sx,sy); textattr:=sz;
end;

procedure movewindow(wind:windowrec; TLX,TLY:integer);
var BRX,BRY,x,y,i:integer;
begin
  checkvidseg;

  window(1,1,80,25);
  textAttr := 14;

  BRX:=wind[4002]; BRY:=wind[4003];
  inc(BRX,TLX-wind[4000]); inc(BRY,TLY-wind[4001]);

  i:=0;
  for y:=TLY to BRY do
    for x:=TLX to BRX do begin
      inline($FA);
      mem[vidseg:(160*(y-1)+2*(x-1))]:=wind[i];
      mem[vidseg:(160*(y-1)+2*(x-1))+1]:=wind[i+1];
      inline($FB);
      inc(i,2);
    end;
end;

end.
