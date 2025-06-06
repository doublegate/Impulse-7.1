{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
{ impulse script language compiler }
unit ISLU;
interface
uses common,common5,strproc;
procedure islC(fn : string);
procedure islD(fn : string);

implementation
var
  isl_text : text;
  isl_bin  : text;
  infile ,
  outfile : string;

procedure error(err : byte);
begin
   sprint('ERROR - ');
   case err of
      1 : sprint(Infile+' does not exist!');
      2 : sprint('Unable to open '+Infile);
   end;
   sprint('Syntax is ISLC <filename.ext>');
   sprint('ex:  ISLC TEST.ISL');
end;


procedure islC(fn : string);
var curline : string;

  function comp_line(s : string) : string;
  var cnt : byte;
      result : string;
  begin
     result := '';
     for cnt := 1 to length(s) do result := result + chr(255 - ord(s[cnt]));
     comp_line := result;
  end;

begin
   sprint('Impulse Scripting Language Compiler.  (c)1997 by Brandon Sneed (Nivenh)');
   sprint('Revision 0.02 Compiled on 02-26-98');
   infile := strAllCaps(fn);
   outfile := strAllCaps(copy(fn, 1, pos('.',infile))+'I');
   if infile = '' then begin
      error(1);
      exit;
   end;
   assign(isl_text, infile);
   {$I-}reset(isl_text);{$I+}
   if ioresult <> 0 then begin
      error(2);
      exit;
   end;
   assign(isl_bin, outfile);
   rewrite(isl_bin);
   while (not eof(isl_text)) do begin
      readln(isl_text, curline);
      writeln(isl_bin, comp_line(curline));
   end;
   sprint('Compiled '+infile+^M^J+' to '+outfile);
   close(isl_bin);
   close(isl_text);
end;

procedure islD(fn : string);
var curline : string;

  function decomp_line(s : string) : string;
  var cnt : byte;
      result : string;
  begin
    result := '';
    for cnt := 1 to length(s) do result := result + chr(255 - ord(s[cnt]));
    decomp_line := result;
  end;

begin
   sprint('Impulse Scriptinging Language DeCompiler.  (c)1997 by Brandon Sneed (Nivenh)');
   sprint('Revision 0.02 compiled on 02-26-98');
   infile := strAllCaps(fn);
   outfile := strAllCaps(copy(fn, 1, pos('.',infile))+'isl');
   if infile = '' then begin
      error(1);
      exit;
   end;
   assign(isl_text, infile);
   {$I-}reset(isl_text);{$I+}
   if ioresult <> 0 then begin
      error(2);
      exit;
   end;
   assign(isl_bin, outfile);
   rewrite(isl_bin);
   while (not eof(isl_text)) do begin
      readln(isl_text, curline);
      writeln(isl_bin, decomp_line(curline));
   end;
   sprint('Compiled '+infile+^M^J+' to '+outfile);
   close(isl_bin);
   close(isl_text);
end;
end.
