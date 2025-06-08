{$M 4096, 0, 1024}
program isl_Compiler;

var
  isl_text : text;
  isl_bin  : text;
  infile ,
  outfile : string;
  total_bytes : longint;
  curline : string;

function comp_line(s : string) : string;
var cnt : byte;
    result : string;
begin
  result := '';
  for cnt := 1 to length(s) do result := result + chr(255 - ord(s[cnt]));
  comp_line := result;
end;

procedure error(err : byte);
begin
write('ERROR -');
case err of
  1 : writeln(Infile+' does not exist!');
  2 : writeln('Unable to open '+Infile);
  end;
writeln;
writeln('Syntax is ISlC.EXE <filename.ext>');
writeln('ex:  ISlC.EXE TEST.isl');
halt;
end;

procedure startup;
begin
writeln('Impulse Scripting Language Compiler.  (c)1997 by Brandon Sneed (Nivenh)');
writeln('Revision 0.02.  Compiled on 2-23-98');
end;

procedure upstr(var s : string);
var i : byte;
begin
  for i := 1 to length(s) do s[i] := upcase(s[i]);
end;


begin
  startup;

  infile := paramstr(1);
  outfile := copy(paramstr(1), 1, pos('.',infile))+'I';

  upstr(infile);
  upstr(outfile);

  if infile = '' then
    error(1);
  assign(isl_text, infile);
  {$I-} reset(isl_text); {$I+}
  if ioresult <> 0 then error(2);
  assign(isl_bin, outfile);
  rewrite(isl_bin);

  while (not eof(isl_text)) do begin
     readln(isl_text, curline);
     inc(total_bytes, length(curline));
     writeln(isl_bin, comp_line(curline));
  end;

  writeln;
  writeln('Compiled '+infile+', ',total_bytes,' bytes of code');
  writeln('Created '+outfile);

  close(isl_bin);
  close(isl_text);
end.
