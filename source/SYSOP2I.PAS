{Fido Setup}
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2i;

interface

uses
   Crt,   Dos,  common1, common2, common3,
  records,common5,common4,common;

procedure pofido;

implementation

(*procedure incolor(msg:string; var i:byte);
var c:char;
begin
  prt('Enter new '+msg+' color (0-9) : ');
  onek(c,^M'0123456789');
  if (c<>^M) then i:=ord(c)-48;
end;*)

procedure pofido;
var fidorf:file of fidorec;
    c:char;
    cc:integer;
    s:string[27];
    done,changed:boolean;
begin
   done:=FALSE;
   repeat
      with fidor do begin
         cls;
         header('FidoNet configuration');
         top;
         s:=cstr(zone)+':'+cstr(net)+'/'+cstr(node)+'.'+cstr(point);
         middle('A','FidoNet address         : '+s);
         middle('B','Origin line             : "'+origin+'"');
         middle('C','Strip IFNA kludge lines : '+syn(skludge));
         middle('D','Strip SEEN-BY lines     : '+syn(sseenby));
         middle('E','Strip origin line       : '+syn(sorigin));
         middle('F','Add tear/origin lines   : '+syn(addtear));
         bottom;
         command('QABCDEF');
         onek(c,'QABCDEF'^M);
         nl;
         case c of
            'Q':done:=TRUE;
            'A':begin
               prompt('Enter your zone number  : ');
               inu(cc);
               if (not badini) then zone:=cc;
               prompt('Enter your net number   : ');
               inu(cc);
               if (not badini) then net:=cc;
               prompt('Enter your node number  : ');
               inu(cc);
               if (not badini) then node:=cc;
               prompt('Enter your point number : ');
               inu(cc);
               if (not badini) then point:=cc;
            end;
            'B': begin
               prompt('Enter new origin line:');
               inputwn(origin,50,changed);
            end;
            'C': skludge:=not skludge;
            'D': sseenby:=not sseenby;
            'E': sorigin:=not sorigin;
            'F': addtear:=not addtear;
         end;
      end;
   until ((done) or (hangup));
   assign(fidorf,status^.gfilepath+'FIDONET.DAT');
   reset(fidorf);
   write(fidorf,fidor);
   close(fidorf);
end;

end.
