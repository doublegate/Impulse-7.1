{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
uses crt,dos;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Var
 Tasker          : byte;
 MultiTaskerName : string[15];
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
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
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
Procedure CheckTasking;
 begin
  Case FindTasker OF
           0: MultiTaskerName := 'dOs [yeah!]';
           1: MultiTaskerName := 'deskview';
           2: MultiTaskerName := 'os.too';
           3: MultiTaskerName := 'windoze';
           4: MultiTaskerName := 'novell';
  End;
 End;
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
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
{:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::}
begin
 CheckTasking;
 While not KeyPressed do TimeSlice;
end.
