{$A+,B+,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2;

interface

uses
  Crt,      Dos,      common1,  common2,  common3,  sysop1,   common5,
  records,  sysop2a,  sysop2b,  sysop2c,  sysop2d,  sysop2e,  sysop2f,
  sysop2g,  sysop2h,  sysop2i,  sysop2z,  sysop2s,  sysop21,  common;

procedure pullconfig;

implementation

const
  maxOpts=18; {how many options are there?}

procedure pullconfig;

type typeOption = array[1..maxOpts] of string[27];

Var OptionList : ^typeOption;
    Current    : Byte;
    X          ,
    Y          : Byte;
    C          : Char;
    Done       : Boolean;
Procedure DrawFirst;
var i:byte;
Begin
   Cls;
   optionList^[1]:=' Main Configuration ';
   optionList^[2]:=' Modem Configuration ';
   optionList^[3]:=' System Access Settings ';
   optionList^[4]:=' System Variables ';
   optionList^[5]:=' Flagged Options ';
   optionList^[6]:=' File System Configuration ';
   optionList^[7]:=' New User Settings ';
   optionList^[8]:=' Miscellaneous Setup ';
   optionList^[9]:=' FidoNet Specifications ';
   optionList^[10]:=' String Editor ';
   optionList^[11]:=' Color Configuration ';
   optionList^[12]:=' Time Allowances ';
   optionList^[13]:=' Call Allowances ';
   optionList^[14]:=' U/D File Limitations ';
   optionList^[15]:=' U/D KB Limitations ';
   optionList^[16]:=' Post/Call Ratios ';
   optionList^[17]:=' Protocol Configuration ';
   optionList^[18]:=' Quit to WFC ';
   goxy(1,1);
   sprint('|U1'+ProgName+' v'+ver+' Configuration');
   Goxy(1,3);
   for i:=1 to MaxOpts do sprint(optionList^[i]);
   goxy(1,3+(current-1));
   sprompt('|U6'+optionList^[1+(current-1)]);
end;

Begin
   inConfig := TRUE;
   new(optionList);
   Done:=False;
   Current:=1;
   DrawFirst;
   x:=1;
   y:=2;
   while (not done) and (not hangup) do begin
      getkey(c);
      c:=upcase(c);
      case c of
         'A','D','H': begin { up arrow }
            if current<>1 then begin
               goxy(1,current+y);
               sprompt('|U1'+optionList^[current]);
               dec(current);
               goxy(1,current+y);
               sprompt('|U6'+optionList^[current]);
            end;
         end;
         'B','C','P': begin { down arrow }
            if current <> MaxOpts then begin
               goxy(1,current+y);
               sprompt('|U1'+optionList^[current]);
               inc(current);
               goxy(1,current+y);
               sprompt('|U6'+optionList^[current]);
            end;
         end;
         'G': begin { home }
            goxy(1,current+y);
            sprompt('|U1'+optionList^[current]);
            current := 1;
            goxy(1,current+y);
            sprompt('|U6'+optionList^[current]);
         end;
         'O': begin { end }
            goxy(1,current+y);
            sprompt('|U1'+optionList^[current]);
            current := maxOpts;
            goxy(1,current+y);
            sprompt('|U6'+optionList^[current]);
         end;
         'Q': done := true; { quit }
         ^M,^N: begin { enter }
            case current of
               1:poFile;
               2:poModem;
               3:poslsettings;
               4:pogenvar;
               5:poflagfunc;
               6:pofilesconfig;
               7:ponewauto;
               8:pomisc;
               9:pofido;
               10:postring;
               11:pocolors;
               12:getsecrange('Time limitations',status^.timeallow);
               13:getsecrange('Call allowance per day',status^.callallow);
               14:getsecrange('UL/DL # files ratio (# files can DL per UL)',status^.dlratio);
               15:getsecrange('UL/DL K-bytes ratio (#k can DL per 1k UL)',status^.dlkratio);
               16:getsecrange('Post/Call ratio (# 1/10''s of calls per public post to have AR flag)',status^.postratio);
               17:exproedit;
               18:done:=true;
            end;
            drawfirst;
         end; { case ^M }
      end; { case c of }
   end; { while statement }
   dispose(optionList);
   savesystat;
   inConfig := false;
end;
end.
