UNIT BPTrap;

INTERFACE

uses myio, crt, dos, common5, common, ansidrv, records,
     common2, common4, sys,misc1;

FUNCTION  Trap:Boolean; FAR;
{* Returns False on installation.
 * After trapping a runtime error it jumps back to where the function was
   called returning True.
 * The procedure that calls Trap must NOT return as long as Trap is installed
   (so it is safest to call Trap from the main program, if possible)!
 * You must call this function AFTER installing all other Exitprocs (if any).
 * In Real mode: You must NOT call it from an overlayed unit.
 * In Protected mode and Windoze: You must call it from a code segment with
   the following attributes: FIXED PRELOAD PERMANENT. (I am not sure if this
   is really necessary...).}

FUNCTION UnTrap:Boolean;
{Returns True iff Trap could be uninstalled.}

procedure exitbbs(elevel : integer);

IMPLEMENTATION

TYPE ptrrec=RECORD ofs,sgm:Word END;

CONST
  addrsave:Pointer=NIL;
  codesave:Word=0;

VAR
  exitsave,trapaddr:Pointer;
  trapsp,trapbp:Word;

procedure exitbbs(elevel : integer);
var
  fr : file;

   procedure showxy(x, y : byte; s : string);
   begin
      gotoxy(x, y);
      write(s);
   end;

begin
   { cosmetic stuff }
   window(1,1,80,25);
   cls;
   TextAttr := 7;
   if status^.specialFx then highbackground(FALSE);

   { Destroy stuff }
   sysdestroyFiles;
   sysdestroyVariables;

   { show exit }
   sansi('[?7h[255D[0;32mﬂ [37m‹[1m‹‹ [0m‹[1m‹ [0;32mﬂ'^M^J);
   sansi(' [1;37;47m≤[40m€ﬂ€€ ﬂ€'^M^J);
   sansi('[0;32m‹ [1;37mﬂﬂ[0;34m‹‹‹[1;37mﬂ [0;32m‹›'^M^J);
   sansi('ﬂ[1;42m∞[1C[0;34m€€[1;44mﬂ[46m‹[0;34m€ [32mﬂ'^M^J);
   sansi('    [34mﬂﬂﬂ[0m'^M^J);
   textattr := $0F;
   showxy(13, 1, progname+' '+copyright);
   textattr := $03;
   showxy(13, 2, 'Version '+ver);
   textattr := $07;
   if elevel = 0 then elevel := exitcode;
   showxy(13, 4, 'Errorlevel '+cstr(elevel));
   writeln;
   writeln;
end;


Procedure CapToTxt(error : integer; addr : pointer);
var
	VidSeg:Word;
	s : string;
	f : text;
	c : char;
	y, x : byte;
begin
	assign(f, 'ERROR.LOG');
	{$I-}
	append(f);
	{$I+}
	if ioresult <> 0 then
		begin
		rewrite(f);
		append(f);
		end;
	writeln(f, '-[ ERROR ',error,' OCCURRED ',seg(addr^),':',ofs(addr^),' ]-');
	vidseg := $B800;
  for y:=1 to 25 do
  begin
    s:='';
    for x:=1 to 80 do
    begin
      c:=chr(mem[vidseg:(160*(y-1)+2*(x-1))]);
      if (c=#0) then c:=#32;
      if ((x=wherex) and (y=wherey)) then c:=#178;
      if ((x<>80) or ((x=80) and (c<>#32))) then s:=s+c;
    end;
    writeln(f,s);
  end;
	writeln(f, '-[ END OF ERROR ',error,' ]-');
	close(f);
end;

Procedure ErrorHandle;
const
  errMenu : array [1..3] of common4.MenuRec =
    ((Text : ' Ignore Error and Continue                                                  ';
      XPos : 3; YPos : 12; Key : '1'),
    (Text : ' Save files and Exit                                                        ';
      XPos : 3; YPos : 13; Key : '2'),
    (Text : ' Exit Program                                                               ';
      XPos : 3; YPos : 14; Key : '3'));
var optionscr : windowrec;
		oldx, oldy : byte;
		option : byte;
		errcode : integer;
begin
	cursoron(false);
	oldx := wherex;
	oldy := wherey;
  setwindow(optionscr, 2, 2, 79, 15, 12, 4, 1);
	window(1,1,80,25);
  errcode := codesave;
  if addrsave <> Nil then begin
		textattr := $4C;
		gotoxy(4, 2);
		write('[ Impulse : ');
    case errcode of
      1..18     : write('DOS ERROR ]');
      100..106  : write('I/O ERROR ]');
      150..162,
      200..216  : write('CRITICAL ERROR ]');
		else
			write('ERROR ]');
    end;
		textattr := $4F;
		gotoxy(3, 3);
		Write('ErrorCode: ',Errcode);
		gotoxy(3, 4);
    Write('ErrorAddr: ',Seg(addrsave^), ':', Ofs(addrsave^));
		gotoxy(3, 5);
		Write('Cause    : ');
      Case errcode of
          0 : write  ('Authorized Program Termination.');
          1 : Write  ('Invalid function number.');
          2 : Write  ('File not found.');
          3 : Write  ('Path not found.');
          4 : Write  ('Too many open files.');
          5 : Write  ('File access denied.');
          6 : Write  ('Invalid file handle.');
         12 : Write  ('Invalid file access code.');
         15 : Write  ('Invalid drive number.');
         16 : Write  ('Cannot remove current directory.');
         17 : Write  ('Cannot rename across drives.');
         18 : Write  ('No More Files.');
        100 : Write  ('Disk read error.');
        101 : Write  ('Disk write error.');
        102 : Write  ('File not assigned.');
        103 : Write  ('File not open.');
        104 : Write  ('File not open for input.');
        105 : Write  ('File not open for output.');
        106 : Write  ('Invalid numeric format.');
        150 : Write  ('Disk is write-protected.');
        151 : Write  ('Unknown unit.');
        152 : Write  ('Drive not ready.');
        153 : Write  ('Unknown command.');
        154 : Write  ('CRC error in data.');
        155 : Write  ('Bad drive request structure length.');
        156 : Write  ('Disk seek error.');
        157 : Write  ('Unknown media type.');
        158 : Write  ('Sector not found.');
        159 : Write  ('Printer out of paper.');
        160 : Write  ('Device write fault.');
        161 : Write  ('Device read fault.');
        162 : Write  ('Hardware failure.');
        200 : Write  ('Division by zero.');
        201 : Write  ('Range check error.');
        202 : Write  ('Stack overflow error.');
        203 : Write  ('Heap overflow error.');
        204 : Write  ('Invalid pointer operation.');
        205 : Write  ('Floating point overflow.');
        206 : Write  ('Floating point underflow.');
        207 : Write  ('Invalid floating point operation.');
        208 : Write  ('Overlay manager not installed.');
        209 : Write  ('Overlay file read error.');
        210 : Write  ('Object not initialized.');
        211 : Write  ('Call to abstract method.');
        212 : Write  ('Stream registration error.');
        213 : Write  ('Collection index out of range.');
        214 : Write  ('Collection overflow error.');
        215 : Write  ('Arithmetic overflow error.');
        216 : Write  ('General Protection fault.');
        250 : Write  ('Fossil Driver not installed.');
      else
        Write(^G + 'Unknown.');
      end; { Case }
		gotoxy(2,6);
		textattr := $4C;
		write('√',expand('ƒ',76),'¥');
		gotoxy(3,7);
    write(mln('Impulse has encountered an error.  The screen has been saved to ERROR.LOG',76));
		gotoxy(3,8);
    write(mln('Please keep and send ERROR.LOG to the author so it may help assist him ',76));
		gotoxy(3,9);
		write(mln('in fixing bugs.  Please report bugs to terrace@dfw.net and be descriptive.',76));
    gotoxy(2,11);
		write('√',expand('ƒ',76),'¥');
    textattr := $4F;
    gotoxy(3,10);
    write('Auto-Selecting option #1 in 10 seconds.');
  end;
  option := domenu(errMenu, 3, 1, $19, $4C, false, true);
	removewindow(optionscr);
	gotoxy(oldx, oldy);
	textattr := $07;
	cursoron(true);
  capToTxt(errcode, addrsave);
	case option of
		1 : begin
				end;
		2 : begin
        exitbbs(errcode);
				ErrorAddr := Nil;
        halt;
				end;
    3 : begin
        writeln('Runtime error ',ErrCode,' occured at ',Seg(AddrSave^), ':', Ofs(AddrSave^));
        halt;
        end;
		end;
end; { ErrorExit }


{$S-}
PROCEDURE Trapexit; FAR;
BEGIN
  IF Erroraddr<>NIL
    THEN {Trapping runtime error}
      BEGIN
        {Install Trapexit again (in case another runtime error occurs later)!}
        Exitproc:=@Trapexit;

        {Keep error address and exit code and reset these variables}
        addrsave:=Erroraddr;
        codesave:=Exitcode;
        Erroraddr:=NIL;
        Exitcode:=0;

        {If you want, you can do something here to indicate the user that an
         error occurred. You could e.g. pop up a message telling the user to
         quit the program asap and report the error to the programmer.}
        ErrorHandle;

        ASM
          {Load the saved SP and BP registers}
          MOV  SP,trapsp
          MOV  BP,trapbp

          {Continue at saved address returning True}
          MOV  AL,1
          JMP  [trapaddr]
        END
      END

    ELSE {Programm finished without an error}
      BEGIN
        {Continue with other exit procs}
        Exitproc:=exitsave;


        {Restore error address and exit code of the last trapped error, if any}
        {
        IF addrsave<>NIL THEN
          BEGIN
            Erroraddr:=addrsave;
            Exitcode:=codesave
          END
        }
      END
END;

FUNCTION Trap:Boolean; ASSEMBLER;
ASM
   {Install Trapexit as an Exitproc}
   MOV  AX,OFFSET Trapexit
   MOV  DX,SEG Trapexit
   CMP  Exitproc.ptrrec.ofs,AX
   JNE  @1
   CMP  Exitproc.ptrrec.sgm,DX
   JE   @2
@1:XCHG Exitproc.ptrrec.ofs,AX
   XCHG Exitproc.ptrrec.sgm,DX
   MOV  exitsave.ptrrec.ofs,AX
   MOV  exitsave.ptrrec.sgm,DX

   {Save SP and BP registers and the return address}
@2:MOV  trapbp,BP
   MOV  SI,SP
   {$IFDEF WINDOWS}
   ADD  SI,4
   ADD  trapbp,6
   {$ENDIF}
   LES  DI,SS:[SI]
   MOV  trapaddr.ptrrec.ofs,DI
   MOV  trapaddr.ptrrec.sgm,ES
   ADD  SI,4
   MOV  trapsp,SI

   {Return False}
   XOR  AX,AX
END;

FUNCTION UnTrap:Boolean;
BEGIN
  IF Exitproc=@Trapexit
    THEN
      BEGIN
        Exitproc:=exitsave;
        UnTrap:=True
      END
    ELSE UnTrap:=False
END;
END.
