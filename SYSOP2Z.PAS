{ Color Configuration }
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2z;

interface

uses
  Crt,   Dos, common5,
  cuser,common4,records,
  common;

procedure pocolors;

implementation

procedure pocolors;
var u:userrec;
    c:char;
    done,col:boolean;
begin
   reset(uf);
   seek(uf,1);
   read(uf,u);
   close(uf);
   col:=color in u.ac;
   done:=FALSE;
   repeat
      cls;
      header('Default color editor');
      top;
      middle('A','Edit Multiple colors');
      middle('B','Edit B&W colors');
      bottom;
      command('QAB');
      onek(c,'QAB');
      if c='Q' then done:=TRUE
      else begin
         if c='A' then u.ac:=u.ac+[color] else u.ac:=u.ac-[color];
         cstuff(21,3,u);
      end;
   until (done) or (hangup);
   if col then u.ac:=u.ac+[color] else u.ac:=u.ac-[color];
   reset(uf);
   seek(uf,1);
   write(uf,u);
   close(uf);
end;

end.
