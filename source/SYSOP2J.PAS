{ Node Configuration }
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2j;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common4,  common5,
   common,   records,  multinod;

procedure ponode;

implementation

procedure ponode;
var fidorf:file of fidorec;
    c:char;
    done,changed:boolean;
begin
   done:=FALSE;
   repeat
      with nodeinfo do begin
         cls;
         header('Node configuration');
         nl;
         top;
         middle('A','Node description: '+desc);
         middle('B','ACS required    : '+acs);
         bottom;
         command('AB');
         onek(c,'QAB'^M);
         nl;
         case c of
            'Q':done:=TRUE;
            'A':begin
               prompt('Enter new node description:');
               inputwn(desc,40,changed);
            end;
            'B':begin
              prompt('Enter ACS required to log in to this node:');
              inputwn(acs,40,changed);
            end;
         end;
      end;
   until ((done) or (hangup));
   savenode;
end;

end.
