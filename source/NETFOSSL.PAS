unit netFossl;
{
 The NetModem FOSSIL Interface - (c)Copyright 1998 Grant Passmore (skaboy101)

  *   The following unit is 100% netmodem compatable, and EXTREMELY
      fast over netmodem.  I recommend using this unit if you wanto
      have a FAST and POWERFUL FOSSIL interface for both dialup and
      telnet applications..

  *   This is most likely one of the most complete FOSSIL units to
      date, and I hope that alot can be learned from it.

  *                            coded 100% by
      skaboy101 [avenge/CiA]. - Released into the public domain, 1998

  *   Greetz go out to Sudden Death, Judge Dredd, Kargus, Nate Dogg,
      Crimson Blade, and of course Mag69.

  *   support - skaboy101@mindless.com / http://infusion.home.ml.org

  *   One final note, be sure that you use the physical value for the
      comm port (ie COM1 = PORT := 0, COM2 = PORT := 1, etc)...

 [-------------------------------------------------------------------------]
   Ok, here are a few notes as to what I have added.  The fBlockRead/write
   functions are now fixed so that the length bit (str[0]) will not be
   sent in the transmission.

   This is done in the following manner -

   fBlockRead :: increased the offset [DL] by 1, so reading starts at the
                 first usable character, and the length bit is then
                 determined by the returned value of AX.

   fBlockWrite : increased the offset [DL] of the string's location
                 so that the length bit isnt sent.
 [-------------------------------------------------------------------------]
}
interface
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetBaud(baudrate : longint);
Procedure fSendChar(c : char);
Function  fReceiveChar : char;
Function  fInputDataAvailable : boolean;
Function  fInputDataOverrun : boolean;
Function  fRoomAvailInOutputBuffer : boolean;
Function  fOutputBufferEmpty : boolean;
Function  fConnected : boolean;
Function  fInitPort(openPort : word) : boolean;
Procedure fDeInitPort;
Procedure fSetDtr(raise : boolean);
Procedure fSetTimerTick;
Procedure fFlushOutputBuffer;
Procedure fPurgeOutputBuffer;
Procedure fPurgeInputBuffer;
Function  fRoomInBuffer : boolean;
Function  fPeekInBuffer : char;
Procedure fSetFlowControl(enable : boolean);
Procedure fRebootSystem(warm : boolean);
Function  fBlockWriteStr(str : string) : boolean;
Function  fBlockReadStr : string;
Procedure fSetBreakStatus(start : boolean);
Function  fControlKCEntered(disable : boolean) : boolean;
Procedure fSendControlStr(str : string; display : boolean);
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
type
 fTimerRec  = record
   tickInt  : byte;                     { timer tick interrrupt number      }
   tickPs   : byte;                     { ticks per second on int AL        }
  miliPtck  : word;                     { miliseconds per tick              }
  end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
var port : word;
   timer : fTimerRec;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
implementation
var ch : char;
   int : integer;
   byt : byte;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fDelay(ms : integer); assembler;
asm
 mov  ax, 1000;
 mul  ms;
 mov  cx, dx;
 mov  dx, ax;
 mov  ah, 86h;
 int  15h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fExtendBaud(baudRate : word);
var fHighBaud : Boolean;
begin
  fHighBaud:=baudRate=576;
  asm
   mov ah, 1Bh;
   int 14h;
   cmp cx, '0X';
   jne @exit;
   mov ah, 1Eh;
   xor bx, bx;
   mov ch, 03h;
   mov dx, port;
   cmp fHighBaud, true;
   je @1;
   mov cl, 84h;
   jmp @int;
   @1:
   mov cl, 82h;
   @int:
   int 14h;
   @exit:
  end;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetBaud(baudrate : longint);
begin
 baudRate := (baudRate div 100);
 case baudRate of
    12 : byt := $83;
    24 : byt := $A3;
    48 : byt := $C3;
    96 : byt := $E3;
   192 : byt := $03;
   384 : byt := $23;
   576,
  1152 : begin
         fExtendBaud(baudRate);
         exit;
         end;
  end;
 asm
  mov ah, 00h;
  mov dx, port;
  mov al, byt;
  int 14h;
 end;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSendChar(c : char); assembler;
asm
 mov ah, 01h;
 mov al, c;
 mov dx, port;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fReceiveChar : char;
begin
 ch := #0;
 asm
  mov ah, 02h;
  mov dx, port;
  int 14h;
  mov ch, al;
 end;
 fReceiveChar := ch;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fInputDataAvailable : boolean;
begin
 asm
  mov ah, 03h;
  mov dx, port;
  int 14h;
  mov byt, ah;
 end;
if (byt=0) then fInputDataAvailable := true else
                fInputDataAvailable := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fInputDataOverrun : boolean;
begin
 asm
  mov ah, 03h;
  mov dx, port;
  int 14h;
  mov byt, ah;
 end;
if (byt=1) then fInputDataOverrun := true else
                fInputDataOverrun := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fRoomAvailInOutputBuffer : boolean;
begin
 asm
  mov ah, 03h;
  mov dx, port;
  int 14h;
  mov byt, ah;
 end;
if (byt=5) then fRoomAvailInOutputBuffer := true else
                fRoomAvailInOutputBuffer := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fOutputBufferEmpty : boolean;
begin
 asm
  mov ah, 03h;
  mov dx, port;
  int 14h;
  mov byt, ah;
 end;
if (byt=6) then fOutputBufferEmpty := true else
                fOutputBufferEmpty := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fConnected : boolean;
begin
 asm
  mov ah, 03h;
  mov dx, port;
  int 14h;
  mov byt, al;
 end;
if (byt=7) then fConnected := true else
                fConnected := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fInitPort(openPort : word) : boolean;
begin
 port := openPort;
 asm
  mov ah, 04h;
  mov dx, port;
  int 14h;
  mov int, ax;
 end;
 if (int=$1954) then
     fInitPort := true else
     fInitPort := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fDeInitPort; assembler;
asm
 mov ah, 05h;
 mov dx, port;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetDtr(raise : boolean); assembler;
asm
 mov ah, 06h;
 mov dx, port;
 mov al, raise;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetTimerTick; assembler;
 asm
  mov ah, 07h;
  int 14h;
  mov timer.tickInt, al;
  mov timer.tickPs, ah;
  mov timer.miliPtck, dx;
 end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fFlushOutputBuffer; assembler;    { sends it all, doesnt purge it }
 asm
  mov ah, 08h;
  mov dx, port;
  int 14h;
 end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fPurgeOutputBuffer; assembler;    { purges it all, doesnt send it }
 asm
  mov ah, 09h;
  mov dx, port;
  int 14h;
 end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fPurgeInputBuffer; assembler;
 asm
  mov ah, 0Ah;
  mov dx, port;
  int 14h;
 end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fRoomInBuffer : boolean;
begin
 asm
  mov ah, 0Bh;
  mov dx, port;
  int 14h;
  mov int, ax;
 end;
if (int=$0001) then fRoomInBuffer := true else
                    fRoomInBuffer := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fPeekInBuffer : char;
var tmp1 : byte;
    tmp2 : integer;
begin
 asm
  mov ah, 0Ch;
  mov dx, port;
  int 14h;
  mov tmp1, ah;
  mov tmp2, ax;
  mov ch, al;
 end;
if (tmp1=$00) then fPeekInBuffer := #0
    else fPeekInBuffer := ch;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetFlowControl(enable : boolean); assembler;
asm
 mov ah, 0Fh;
 mov dx, port;
 mov al, enable;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fRebootSystem(warm : boolean); assembler;
asm
 mov ah, 17h;
 mov al, warm;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetWatchDog(enable : boolean); assembler;
asm
 mov ah, 14h;
 mov dx, port;
 mov al, enable;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fBlockWriteStr(str : string) : boolean;
var len : word;
     sg : word;
begin
 len := ord(str[0]);
  sg := seg(str);
 asm
  mov ah, 19h;
  mov cx, len;
  mov dx, port;
  mov es, sg;
  mov di, offset str + 1;
  int 14h;
  mov len, ax;
 end;
if (len=ord(str[0])) then fBlockWriteStr := true else
                          fBlockWriteStr := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fBlockReadStr : string;
var strn : string;
      sg : word;
begin
 sg := seg(strn);
 asm
  mov ah, 18h;
  mov cx, 255;
  mov dx, port;
  mov es, sg;
  mov di, offset strn + 1;
  int 14h;
  mov sg, ax;
 end;
strn[0] := chr(sg);
fBlockReadStr := strn;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSetBreakStatus(start : boolean); assembler;
asm
 mov ah, 1Ah;
 mov dx, port;
 mov al, start;
 int 14h;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Function fControlKCEntered(disable : boolean) : boolean;
begin
 asm
  mov ah, 10h;
  mov al, disable;
  int 14h;
  mov int, ax;
 end;
if (int=$0001) then fControlKCEntered := true else
                    fControlKCEntered := false;
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure fSendControlStr(str : string; display : boolean);
begin
 while fInputDataAvailable and display do
       write(fReceiveChar);
 for byt := 1 to ord(str[0]) do
  begin
   if (str[byt]='|') then fSendChar(#13) else
   if (str[byt]='~') then fDelay(300) else
    begin
     fSendChar(str[byt]);
     if display then write(str[byt]);
    end;
  end;
 while fInputDataAvailable do
       write(fReceiveChar);
end;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
begin
end.
