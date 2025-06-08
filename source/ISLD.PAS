{$M 4096, 0, 1024}
program ISL_DeCompiler;

var
  ISL_text : text;
  ISL_bin  : text;
  infile   ,
  outfile : string;
  total_bytes : longint;

function decomp_line(s : string) : string;
var cnt : byte;
    result : string;
begin
  result := '';
  for cnt := 1 to length(s) do result := result + chr(255 - ord(s[cnt]));
  decomp_line := result;
end;

procedure error(err : byte);
begin
write('ERROR - ');
case err of
  1 : writeln(Infile+' does not exist!');
  2 : writeln('Unable to open '+Infile);
  end;
writeln;
writeln('Syntax is ISLD.EXE <filename.ext>');
writeln('ex:  ISLD.EXE TEST.I');
halt;
end;

procedure startup;
begin
writeln('Impulse Scripting Language DeCompiler.  (c)1997 by Brandon Sneed (Nivenh)');
writeln('Revision 0.02 Compiled on 2-23-98');
end;

procedure upstr(var s : string);
var i : byte;
begin
  for i := 1 to length(s) do s[i] := upcase(s[i]);
end;

var curline : string;

begin
  startup;

  infile := paramstr(1);
  outfile := copy(paramstr(1), 1, pos('.',infile))+'ISL';

  upstr(infile);
  upstr(outfile);

  if infile = '' then
    error(1);
  assign(ISL_text, infile);
  {$I-}
  reset(ISL_text);
  {$I+}
  if ioresult <> 0 then
    error(2);
  assign(ISL_bin, outfile);
  rewrite(ISL_bin);

  while (not eof(ISL_text)) do
    begin
    readln(ISL_text, curline);
    inc(total_bytes, length(curline));
    writeln(ISL_bin, decomp_line(curline));
    end;

  writeln;

  writeln('DeCompiled '+infile+', ',total_bytes,' bytes of code');
  writeln('Created '+outfile);

  close(ISL_bin);
  close(ISL_text);
end.
