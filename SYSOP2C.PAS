{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
{ system access settings }
unit sysop2c;

interface

uses
   Crt,   Dos,  common1, common2, common3,
  common5,records,common4,common;

procedure poslsettings;

implementation

function qq(s:string):string;
var ss:string[22];
begin
   ss:='"'+s+'"';
   if (length(ss)<16) then ss:=mln(ss,16);
   qq:=ss;
end;

procedure poslsettings;
var s:acstring;
    c:char;
    done:boolean;
begin
   done:=FALSE;
   repeat
      with status^ do begin
         cls;
         header('System ACS settings');
         top;
         middle('A','Full SysOp          ['+qq(sop)+']');
         middle('B','Full Co-SysOp       ['+qq(csop)+']');
         middle('C','Message base SysOp  ['+qq(msop)+']');
         middle('D','File base SysOp     ['+qq(fsop)+']');
         middle('E','SysOp PW at logon   ['+qq(spw)+']');
         middle('F','See PW''s remotely   ['+qq(seepw)+']');
         middle('G','Post public         ['+qq(normpubpost)+']');
         middle('H','Send e-mail         ['+qq(normprivpost)+']');
         middle('I','See anon pub post   ['+qq(anonpubread)+']');
         middle('J','See anon E-mail     ['+qq(anonprivread)+']');
         middle('K','Post anon ANY base  ['+qq(anonpubpost)+']');
         middle('L','E-mail anon         ['+qq(anonprivpost)+']');
         middle('M','See unval. files    ['+qq(seeunval)+']');
         middle('N','DL unval. files     ['+qq(dlunval)+']');
         middle('O','No UL/DL ratio      ['+qq(nodlratio)+']');
         middle('P','No post/call ratio  ['+qq(nopostratio)+']');
         middle('R','No file pt checking ['+qq(nofilepts)+']');
         middle('S','ULs auto-credited   ['+qq(ulvalreq)+']');
         bottom;
         command('QABCDEFGHIJKLMNOPRS');
         onek(c,'QABCDEFGHIJKLMNOPRS'^M);
         case c of
            'A'..'P','R'..'S' : begin
               sprompt('|NLNew Acs: ');
               inputl(s,20);
               if (s<>'') then
                  case c of
                     'A':sop:=s;           'B':csop:=s;
                     'C':msop:=s;          'D':fsop:=s;
                     'E':spw:=s;           'F':seepw:=s;
                     'G':normpubpost:=s;   'H':normprivpost:=s;
                     'I':anonpubread:=s;   'J':anonprivread:=s;
                     'K':anonpubpost:=s;   'L':anonprivpost:=s;
                     'M':seeunval:=s;      'N':dlunval:=s;
                     'O':nodlratio:=s;     'P':nopostratio:=s;
                     'R':nofilepts:=s;     'S':ulvalreq:=s;
                  end;
            end;
            'Q' : done := true;
         end;
      end;
   until (done) or (hangup);
end;

end.
