program PtrProc;

uses Crt;

type
  TMyProcType = procedure (Fname: String; I: byte);
  blah        = procedure;

  TMyRec = record
    ProcPtr: Pointer;
  end;

var
  MyRec: TMyRec;

procedure Example(FName: String; I: byte); far;
begin
  Writeln('FName := ', FName);
  Writeln('I := ', I);
  Writeln('Hi, i''m one of many ways to do procedural pointers.');
end;
procedure gimpy;
begin
  write('iam  a supergimp');
end;


begin
	MyRec.ProcPtr := @Example;
	TMyProcType(MyRec.ProcPtr)('Test', 1);
        myRec.ProcPtr := @gimpy;
        blah(myRec.ProcPtr);
end.
