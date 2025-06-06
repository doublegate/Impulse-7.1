{$A+,B+,F+,I+,L-,O+,R-,S-,V-,Q-}

Unit AnsiDrv;

Interface

Uses Crt,common;

procedure CAnsi(ch : char);
procedure SAnsi(s : string);

Implementation

Const ControlCh: Set of Char = ['A','B','C','D','f','s','u','H','J','K','m',';','r'];
      MaxParms = 200;
CONST OrTable : ARRAY[30..47] OF BYTE =
       (0,4,2,6,1,5,3,7,0,0,0,64,32,96,16,80,48,112);
Var
  AvState : Word;
  AvAttr : Byte;
  CheckPositions : Boolean;
  AnsiParm : Array [1..MaxParms] of Byte;
  AnsiParmNo : Byte;
  SaveX : Byte;
  SaveY : Byte;
  XC, YC : ShortInt;

Procedure AVReset;
Begin
  AvState := 0;
  AvAttr := 3;
  TextAttr := 3;
  ClrScr;
End;

Procedure ColorParm(Parm:Byte); ASSEMBLER;
ASM
    MOV     BL, TextAttr
    MOV     AL, Parm

  @TEST0:
    CMP     AL, 0
    JNE     @TEST1
    MOV     BL, 7
    JMP     @EXIT

  @TEST1:
    CMP     AL, 1
    JNE     @TEST4
    OR      BL, $08
    JMP     @EXIT

  @TEST4:
    CMP     AL, 4
    JNE     @TEST5
    AND     BL, $F8
    OR      BL, 1
    JMP     @EXIT

  @TEST5:
    CMP     AL, 5
    JNE     @TEST7
    OR      BL, $80
    JMP     @EXIT

  @TEST7:
    CMP     AL, 7
    JNE     @TEST8
    MOV     BH, BL
    AND     BL, $88
    AND     BH, $77
    ROL     BH, 4
    OR      BL, BH
    JMP     @EXIT

  @TEST8:
    CMP     AL, 8
    JNE     @TEST30
    AND     BL, $88
    JMP     @EXIT

  @TEST30:
    MOV   DX, Seg(OrTable)
    MOV   ES, DX
    MOV   DI, Offset(OrTable)  {ES:DI -> OrTable}
    XOR   AH, AH
    SUB   AL, 30
    ADD   DI, AX               {ES:DI -> OrTable[Parm]}
    CMP   AL, 8
    JAE   @AND8F
    AND   BL, $F8          {Parm in 30..37}
    JMP   @DONE

  @AND8F:
    AND   BL, $8F          {Parm > 37}

  @DONE:
    OR    BL, ES:[DI]

  @EXIT:
    MOV     TextAttr, BL
END;

Procedure ProcCtl(Ch : Char);
Var I : Integer;
Begin
  Case Ch of
    ';' : ASM
             INC  AnsiParmNo
             CMP  AnsiParmNo, 11
             JB   @EXIT
             MOV  AnsiParmNo, 10
            @EXIT:
          End;
    'A' : Begin
            If AnsiParm[1] = 0 Then AnsiParm[1] := 1;
            I := WhereY;
            Dec(I,AnsiParm[1]);
            If I < 0 Then I := 0;
            GoToXy(WhereX, I);
            AvState := 0;
          End;
    'B' : Begin
            If AnsiParm[1] = 0 Then AnsiParm[1] := 1;
            GoToXy(WhereX, WhereY + AnsiParm[1]);
            AvState := 0;
          End;
    'C' : Begin
            If AnsiParm[1] = 0 Then AnsiParm[1] := 1;
            GoToXy(WhereX + AnsiParm[1], WhereY);
            AvState := 0;
          End;
    'D' : Begin
            If AnsiParm[1] = 0 Then AnsiParm[1] := 1;
            I := WhereX;
            Dec(I, AnsiParm[1]);
            If I < 0 Then I := 0;
            GoToXy(I, WhereY);
            AvState := 0;
          End;
    'H',
    'f' : Begin
            If AnsiParm[1] = 0 Then AnsiParm[1] := 1;
            If Ansiparm[2] = 0 Then AnsiParm[2] := 1;
            GoToXy(Ansiparm[2],Ansiparm[1]);
            AvState := 0;
          End;
    'J' : Begin
            AvState := 0;
            If AnsiParm[1] = 2 Then Cls;
          End;
    'K' : Begin
            AvState := 0;
            ClrEol;
          End;
    's' : Begin
            SaveX := WhereX;
            SaveY := WhereY;
            AvState := 0;
          End;
    'r' : Begin
          window(1,1,80,ansiparm[2]);
          avstate:=0;
          writeln;
          end;
    'u' : Begin
            GoToXy(SaveX, SaveY);
            AvState := 0;
          End;
    'm' : Begin
            AvState := 0;
            If AnsiParmNo > 0 Then
              For i := 1 to AnsiParmNo Do ColorParm(AnsiParm[i]);
          End;
  End;
End;


Procedure Parse_AVT1(ch:Char);
Var I : Integer;
Begin
  If CheckPositions Then If XC <> -1 Then GotoXY(XC, YC);
  TextAttr := AvAttr;
  Case AvState of
    0 : Case Ch of
          #27 : AvState := 1;
        Else
          Write(Ch);
        End;
    1 : Case Ch of
          #27 : Begin
                  AvState := 1;
                  Write(#27);
                End;
          '[' : Begin
                  AvState := 2;
                  AnsiParmNo := 1;
                  FillChar(AnsiParm, SizeOf(AnsiParm), 0);
                End;
          #12 : Begin
                  AvReset;
                  AvState := 0;
                End;
          (* from here *)
          #25 : Begin
                  Write(#27);
                  AvState := 5;
                End;
          #22 : Begin
                  Write(#27);
                  AvState := 6;
                End;
          (* to here *)

          Else Write(#27+ch);
        End;
    2 : Case Ch of
          #27 : Begin
                  AvState := 1;
                  Write(#27+'[');
                End;
          '0'..
          '9' : AnsiParm[AnsiParmNo] := (AnsiParm[AnsiParmNo] * 10)  + (Byte(Ch) - 48);
          '?' :;
          Else
          If Ch in ControlCh Then ProcCtl(Ch) Else AvState :=0;
        End;
    5 : Begin
          AnsiParm[1] := Byte(Ch);
          AvState := 6;
        End;
    6 : Begin
          AvState := 0;
          I := 1;
          While I <= Byte(Ch) Do
          Begin
            Write(Char(AnsiParm[1]));
            Inc(I);
          End;
        End;
    End;
    AvAttr := TextAttr;
    If CheckPositions Then
    Begin
      XC := WhereX;
      YC := WhereY;
    End;
End;

procedure CAnsi(ch : char);
begin
 parse_avt1(ch);
end;

procedure SAnsi(s : string);
var i : byte;
begin
 for i := 1 to length(s) do parse_avt1(s[i]);
end;

Begin
  XC := -1;
  SaveX := 0;
  SaveY := 0;
  AvState := 0;
  AvAttr := 3;
  TextAttr := 3;
End.