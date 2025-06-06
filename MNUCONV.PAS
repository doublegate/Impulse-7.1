{ Converts pre 7.X impulse menu files to the 7.X format }
program Menu_Converter;
uses records;

{ Convert a string to all caps }
function allcaps(s:string):string;
var i:byte;
begin
   for i :=1 to length(s) do s[i]:=upcase(s[i]);
   allcaps:=s;
end;

var infile  : text;             { the old menu file  }
    outfile : file;             { the new menu file  }
    Mnu     : MenuRec;          { menu record        }
    Cmd     : CommandRec;       { command recrod     }
    s       ,                   { temporary string   }
    ifn     ,                   { old menu file name }
    ofn     : string;           { new menu file name }
    n       : byte;             { a counter          }

begin
   ifn := paramStr(1);
   assign(infile,ifn);
   {$I-} reset(infile);{$I+}
   if ioresult <> 0 then begin
      writeLn('ERROR - ',ioresult);
      writeLn('USAGE: "mnuconv <filename.ext>"');
      halt(ioresult);
   end;
   ofn := copy(ifn, 1, pos('.',ifn))+'MEN';
   assign(outfile,ofn);
   {$I-}rewrite(outfile,1);{$I+}
   if ioresult <> 0 then begin
      write('ERROR - ',ioresult);
      halt(ioresult);
   end;
   with Mnu do begin
      readln(infile,menuname[1]);
      readln(infile,menuname[2]);
      readln(infile,menuname[3]);
      readln(infile,directive);
      readln(infile,tutorial);
      readln(infile,menuprompt);
      readln(infile,acs);
      readln(infile,password);
      readln(infile,fallback);
      readln(infile,forcehelplevel);
      readln(infile,gencols);
      for n := 1 to 3 do readln(infile,gcol[n]);
      readln(infile,s);
      s:=allcaps(s);
      menuflags:=[];
      if (pos('C',s)<>0) then menuflags:=menuflags+[clrscrbefore];
      if (pos('D',s)<>0) then menuflags:=menuflags+[dontcenter];
      if (pos('N',s)<>0) then menuflags:=menuflags+[nomenuprompt];
      if (pos('F',s)<>0) then menuflags:=menuflags+[forcepause];
      if (pos('P',s)<>0) then menuflags:=menuflags+[pulldown];
      if (pos('T',s)<>0) then menuflags:=menuflags+[autotime];
   end;
   blockwrite(outfile,Mnu,sizeOf(Mnu));
   repeat
      with cmd do begin
         readln(infile,ldesc);
         readln(infile,sdesc);
         readln(infile,xpoint);
         readln(infile,ypoint);
         readln(infile,ckeys);
         readln(infile,acs);
         readln(infile,cmdkeys);
         readln(infile,mstring);
         readln(infile,s);
         s:=allcaps(s);
         commandflags:=[];
         if (pos('P',s)<>0) then commandflags:=commandflags+[pull];
         if (pos('H',s)<>0) then commandflags:=commandflags+[hidden];
         if (pos('U',s)<>0) then commandflags:=commandflags+[unhidden];
      end;
      blockwrite(outfile,cmd,sizeOf(cmd));
   until eof(infile);
   close(outfile);
   close(infile);
   writeln('File succesfully converted!');
end.