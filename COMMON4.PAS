{$A+,B+,F+,I+,L-,O-,R-,S-,V-}
Unit Common4;

Interface

Uses Records,Common,Common5, Dos,  Crt;
type
  menurec = record
            text : string; { description }
            xpos,              { X coord }
            ypos : byte;       { Y coord }
            key  : char;       { hot key }
            end;

procedure middle(c:string;s:string);
procedure top;
procedure bottom;
procedure header(s:string);
procedure command(s:string);
procedure calconf(cnf:byte); {1=msg, 2=file}
Function StripAnsi(InString : String) : String;
function domenu(var menu; numitems, start, highclr, loclr : byte; mse, timer : boolean) : byte;

Implementation

procedure header(s:string);
begin
   sprint(#3#9+'þ '+#3#1+s);
end;

procedure top;
begin;
   sprint('|08ÚÄÄÄ¿');
end;

procedure middle(c:string;s:string);
begin
   sprint('|08³ |15'+c+' |08³  |07'+s);
end;

procedure bottom;
begin
   sprint('|08ÀÄÄÄÙ');
end;

procedure command(s:string);
begin
   sprompt(#3#0+'Command '+#3#5+'['+#3#1+s+#3#5+'] '+#3#0);
end;


Procedure CalConf(cnf:byte);

  Procedure FileConf;
  var fb,b,b2,i:integer;
      acc:boolean;
  begin
    b:=1;
    acc:=false;
    fb:=1;
    while (b<=maxulb) do begin
      acc:=fbaseac(b); { fbaseac will load memuboard }
      if ((fbunhidden in memuboard.fbstat) or (acc)) and
        ((memuboard.conf=fconf) or (fconf=0))
      then begin
        if (acc) then begin
          b2:=ccuboards[1][b];
          areanum^[fb]:=b2;
          inc(fb);
        end;
      end;
      inc(b);
    end;
  end;

  procedure MsgConf;
  var mb,b,b2,i:integer;
      acc:boolean;
  begin
    b:=1;
    acc:=false;
    mb:=1;
    while (b<=numboards) do begin
      acc:=mbaseac(b);
      if ((mbunhidden in memboard.mbstat) or (acc)) and
        ((memboard.conf=mconf) or (mconf=0))
      then begin
        if (acc) then begin
          b2:=ccboards[1][b];
          areanum^[mb]:=b2;
          inc(mb);
        end;
      end;
      inc(b);
    end;
  end;

begin
  fillchar(areanum^,150,0);
  case cnf of
  1:msgconf;
  2:fileconf;
  end;
end;

Function StripAnsi(InString : String) : String;
Const Esc         = Chr(27);
      Ansi_Letter = 'JDCABHmfsuK'+Chr(13);

Var HiC,HiB              : Byte;
    SaveAnsiX,SaveAnsiY  : Byte;
    I : Byte;
    A : Byte;
    D : Boolean;
   Begin
      D := False;
      If InString = '' Then
         Begin
            StripAnsi := '';
            Exit;
         End;
      Repeat
         Begin
            A := Pos(Chr(27)+'[',InString);
            If A > 0 Then
               Begin
                  Delete(InString,A,2);
                  I := A;
                  While (I < Length(InString)) And (Pos(InString[I],Ansi_Letter) = 0) Do
                     Inc(I);
                  Delete(InString,A,I-A+1);
               End
            Else
               D := True;
         End;
      Until D Or (InString = '');
      StripAnsi := InString;
   End;

function domenu(var menu; numitems, start, highclr, loclr : byte; mse, timer : boolean) : byte;
type
  menucast = array [1..6] of menurec;
var
  cnt : byte;
  choice : byte;
  getkey : char;
  oldsec : word;
  hour,min,sec,hs : word;
begin
  if start > numitems then start := 1;
  begin
  { draw the menu }
  for cnt := 1 to numitems do
    with menucast(menu)[cnt] do begin
    textattr := loclr;
    gotoxy(xpos, ypos);
    write(text);
    end;
  end;

  choice := start;
  if timer then
    begin
    Dos.GetTime(hour, min, oldsec, hs);
    if OldSec > 50 then OldSec := 10;
    end;
  repeat
    if timer then
      begin
      Dos.GetTime(hour, min, sec, hs);
      if (sec > oldsec + 10) then
        begin
        domenu := 1;
        exit;
        end;
      end;
    if (choice <= numitems) and (choice > 0) then
    with menucast(menu)[choice] do
      begin
      textattr := highclr;
      gotoxy(xpos, ypos);
      write(text);
      textattr := loclr;
      repeat
      until keypressed;
      getkey := readkey;
      { if a arrowkey, or any function type key is sent, a #0 always is sent
        first, and THEN the actual key value is sent. so, the first call to
        readkey will get #0, and the second will get the key we're looking
        for. }
      if getkey = #0 then
        begin
        getkey := readkey;
        case getkey of
          { up }
          #72 : if choice > 1 then
                begin
                textattr := loclr;
                gotoxy(xpos, ypos);
                write(text);
                dec(choice);
                end;
          { down }
          #80 : if choice < numitems then
                begin
                textattr := loclr;
                gotoxy(xpos, ypos);
                write(text);
                inc(choice);
                end;
          end;
        end;
      { if they hit ESC, set choice to 0 and exit }
      if getkey = #27 then choice := 0;
      { if they press a key, see if the key the pressed is a hotkey }
      getkey := upcase(getkey);
      if getkey in ['A'..'Z'] then
        begin
        for cnt := 1 to numitems do
          if getkey = menucast(menu)[cnt].key then
            begin
            choice := cnt;
            getkey := #13;
            end;
        end;
      end;
  until (getkey = #13) or (getkey = #27);
  domenu := choice;
end;

End.
