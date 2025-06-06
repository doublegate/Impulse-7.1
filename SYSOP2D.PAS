{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
{ system variables }
unit sysop2d;

interface

uses
   Crt,   Dos,  common1, common2, common3,
  common5,common4,common;

procedure pogenvar;

implementation

procedure pogenvar;
var c:char;
    i:integer;
    bbb:byte;
    done:boolean;
begin
   done:=FALSE;
   repeat
      with status^ do begin
         cls;
         header('System variables');
         top;
         middle('A','Max private sent per call  ['+mn(maxprivpost,6)+']');
         middle('B','Max feedback sent per call ['+mn(maxfback,6)+']');
         middle('C','Max public posts per call  ['+mn(maxpubpost,6)+']');
         middle('D','Max chat attempts per call ['+mn(maxchat,6)+']');
         middle('E','Normal max mail waiting    ['+mn(maxwaiting,6)+']');
         middle('F','CoSysOp max mail waiting   ['+mn(csmaxwaiting,6)+']');
         middle('G','Maximum lines in a message ['+mn(maxlines,6)+']');
         middle('H','Maximum lines to quote     ['+mn(csmaxlines,6)+']');
         middle('I','Number of logon attempts   ['+mn(maxlogontries,6)+']');
         middle('J','Max time in time bank      ['+mn(maxTimeBank,6)+']');
         middle('K','SysOp chat color           ['+mn(sysopcolor,6)+']');
         middle('L','User chat color            ['+mn(usercolor,6)+']');
         Middle('M','Min. space for posts       ['+mn(minspaceforpost,6)+']');
         middle('N','Min. space for uploads     ['+mn(minspaceforupload,6)+']');
         middle('O','Back SysOp Log keep days   ['+mn(backsysoplogs,6)+']');
         middle('P','Blank WFC menu minutes     ['+mn(wfcblanktime,6)+']');
         middle('R','Default video line length  ['+mn(linelen,6)+']');
         middle('S','Default video page length  ['+mn(pagelen,6)+']');
         middle('T','Number of NUV Yes Votes    ['+mn(nuvyes,6)+']');
         middle('U','Number of NUV No Votes     ['+mn(nuvno,6)+']');
         bottom;
         command('QABCDEFGHIJKLMNOPRSTU');
         onek(c,'QABCDEFGHIJKLMNOPRSTU'^M);
         case c of
            'Q':done:=TRUE;
            'A'..'P','R'..'U': begin
               sprompt('|NLRange ');
               case c of
                 'K','L':print('(0-9)');
                 'O':print('(1-99)');
                 'R':print('(32-132)');
                 'S':print('(4-50)');
                 'M','N':print('Varies');
                 'J': print('(0-32767)');
                 else prt('(0-255)');
               end;
               sprompt('|NLNew Value: ');
               case c of
                  'M','N','J':inu(i);
                  else ini(bbb);
               end;
               if (not badini) then
                  case c of
                     'A':maxprivpost:=bbb;
                     'B':maxfback:=bbb;
                     'C':maxpubpost:=bbb;
                     'D':maxchat:=bbb;
                     'E':maxwaiting:=bbb;
                     'F':csmaxwaiting:=bbb;
                     'G':if (bbb in [1..255]) then maxlines:=bbb;
                     'H':if (bbb in [1..255]) then csmaxlines:=bbb;
                     'I':maxlogontries:=bbb;
                     'J': maxTimeBank := i;
                     'K':if (bbb in [0..9]) then sysopcolor:=bbb;
                     'L':if (bbb in [0..9]) then usercolor:=bbb;
                     'M':if (i>0) then minspaceforpost:=i;
                     'N':if (i>0) then minspaceforupload:=i;
                     'O':if (bbb in [1..99]) then backsysoplogs:=bbb;
                     'P':if (bbb in [0..255]) then wfcblanktime:=bbb;
                     'R':if (bbb in [32..132]) then linelen:=bbb;
                     'S':if (bbb in [4..50]) then pagelen:=bbb;
                     'T':nuvyes:=bbb;
		     'U':nuvno:=bbb;
                  end;
            end;
         end;
      end;
   until (done) or (hangup);
end;

end.
