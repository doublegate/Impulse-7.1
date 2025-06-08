(*Modem Configuration*)
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2a;

interface

uses
   Crt,      Dos,      common4,  common5,  records,  cuser,    common1,
   common2,  common3,  common;

procedure pomodem;

implementation

function wantit(str : string):boolean;
begin
   changeablemci := str;
   wantit:=pynq('NL'+strings^.areyousure,true);
end;

function showmodemstring(s:astr):astr;
var o:astr;
    i:integer;
begin
   o:='';
   for i:=1 to length(s) do
      case s[i] of
         ^@..^L,^N..^[: o:=o+'^'+chr(ord(s[i])+64);
         ^M:o:=o+'|';
      else o:=o+s[i];
   end;
   showmodemstring:=o;
end;

function usefoss:astr;
begin
   if status^.fossil then useFoss := 'FOSSIL v7'
   else useFoss := 'Internal';
end;

function hardhand:astr;
begin
   if (modemR.hardware) then hardHand := 'ON'
   else hardHand := 'OFF';
end;

procedure newmodemstring(var vs:astr; what:astr; len:integer);
var i:integer;
    changed:boolean;
begin
   print('Current modem '+what+' string: "'+showmodemstring(vs)+'"');
   nl;
   print('Use: "^" preceding control codes (^@..^[)');
   print('     "~" for a half-second delay');
   nl;
   print('Enter NEW modem '+what+' string:');
   prt(':');
   inputwn(vs,len,changed);
   if (changed) then begin
      for i:=1 to length(vs) do
         case vs[i] of
            '|':vs[i]:=^M;
            '^':if ((i<>length(vs)) and (vs[i+1] in ['@'..'['])) then begin
               vs[i]:=chr(ord(vs[i+1])-64);
               delete(vs,i+1,1);
            end;
         end;
   end
   else print('No Change');
end;

procedure pomodem;
var modemrf:file of modemrec;
    c1,cc:integer;
    c,ccc:char;
    done:boolean;
begin
   done:=FALSE;
   repeat
      with modemr do begin
         cls;
         header('Modem configuration');
         top;
         middle('A','Maximum baud rate    ['+cstr(waitbaud)+']');
         middle('B','COM port number      ['+cstr(comport)+']');
      {
      middle('C','IRQ Number           ['+cstr(irqnum)+']');
      middle('D','In/Out Buffer Size   ['+cstr(inoutbuff)+']');
      }
        middle('E','Modem Init String    ['+showmodemstring(init)+']');
        middle('F','Modem answer string  ['+showmodemstring(answer)+']');
        middle('G','Modem hangup string  ['+showmodemstring(hangup)+']');
        middle('H','Modem offhook string ['+showmodemstring(offhook)+']');
        middle('I','No-call init time    ['+cstr(nocallinittime)+']');
      {
      middle('J','Locked baud rate     ['+cstrl(arq9600rate)+' Baud]');
      }
        middle('K','Hardware Handshaking ['+hardhand+']');
        middle('L','Comm Driver Type     ['+usefoss+']');
        bottom;
        command('QABCDEFGHIJKL');
        onek(c,'QABCDEFGHIJKL'^M); nl;
        case c of
           'A':if (incom) then begin
              sprint(#3#7+'This can only be changed locally!!|NL');
              pausescr;
           end
           else begin
              print('This should be your Modem''s MAXIMUM Speed.');
              nl;
              print('The current rate is: '+cstrl(waitbaud)+' baud.');
              nl;
              print('A:300 B:1200 C:2400 D:4800 E:9600 F:19200 G:38400 H:57600 I:115200 J:128000');
              prt('Modem speed? (A-J) : ');
              onek(ccc,'QABCDEFGHIJ'^M);
              if (ccc in ['A'..'J']) then
                 case ccc of
                     'A':waitbaud:=300;
                     'B':waitbaud:=1200;
                     'C':waitbaud:=2400;
                     'D':waitbaud:=4800;
                     'E':waitbaud:=9600;
                     'F':waitbaud:=19200;
                     'G':waitbaud:=38400;
                     'H':waitbaud:=57600;
                     'I':waitbaud:=115200;
                     'J':waitbaud:=128000;
                 end;
           end;
           'B':if (incom) then begin
              sprint(#3#7+'This can only be changed locally!!|NL');
              pausescr;
           end
           else begin
              prt('Com port (1-8) (0 - Local)? ');
              inu(cc);
              if (cc in [0..8]) then
                 if (wantit('change the comport')) then begin
                    remove_port;
                    comport:=cc;
                    iport;
                 end
                 else print('No Change');
           end;
        {
        'C':modemirq;
        'D':buffsize;
        }
           'E':newmodemstring(init,'init',80);
           'F':newmodemstring(answer,'answer',40);
           'G':newmodemstring(hangup,'hangup',40);
           'H':newmodemstring(offhook,'offhook',40);
           'I':begin
              prt('No call modem re-initialization: ');
              inu(c1);
              if (not badini) then nocallinittime:=c1;
           end;
        {
        'J':begin
              print('Locked Port Speed.');
              nl;
              print('The current rate is: '+cstrl(arq9600rate)+' baud.');
              nl;
              prt('A:9600 B:19200 C:38400 D:57600 E:115200 F:128000 or (Q)uit : ');
              onek(ccc,'QABCDE'^M);
              if (ccc in ['A'..'F']) then
                case ccc of
                  'A':arq9600rate:=9600;
                  'B':arq9600rate:=19200;
                  'C':arq9600rate:=38400;
                  'D':arq9600rate:=57600;
                  'E':arq9600rate:=115200;
                  'F':arq9600rate:=128000;
                end;
            end;
        }
           'K':modemr.hardware:=not modemr.hardware;
           'L':status^.fossil:=not status^.fossil;
           'Q':done:=TRUE;
        end;
      end;
   until ((done) or (hangup));
   assign(modemrf,status^.gfilepath+'modem.'+cstr(thisnode));
   reset(modemrf);
   write(modemrf,modemr);
   close(modemrf);
end;

end.
