{ Misc Code }
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2h;

interface

uses
   Crt,      Dos,      common4,  common5,  common1,  common,   records,
   common3,  cmd,      common2;

procedure pomisc;

implementation

procedure instring(p:astr; var v:astr; len:integer);
var changed:boolean;
begin
   prompt('Enter new "'+p+'" string:');
   if (incom) then inputwnwc(v,len,changed)
   else v:=NInput(WhereX,WhereY,v,len);
end;

procedure promptedit;
var r,c:char;
    c1:integer;
    bb,done,changed:boolean;
    f : file;
begin
   done:=FALSE;
   bb:=false;
   repeat
      cls;
      HEADER('Prompt Editor');
      top;
      middle('1',prmp^[1].name);
      middle('2',prmp^[2].name);
      middle('3',prmp^[3].name);
      middle('4',prmp^[4].name);
      middle('N','Names');
      bottom;
      command('1234QN - [V]iew [?]MCI Help');
      onek(c,'1234QVN?'^M);
      nl;
      case c of
         'Q':done:=true;
         '1':instring('Prompt #1',prmp^[1].prompt,80);
         '2':instring('Prompt #2',prmp^[2].prompt,80);
         '3':instring('Prompt #3',prmp^[3].prompt,80);
         '4':instring('Prompt #4',prmp^[4].prompt,80);
         '?':printf('mcihelp.ans');
         'N':begin
            sprompt('Which? ');
            onek(r,'1234');
            case r of
               '1':instring('Prompt #1 Name',prmp^[1].name,20);
               '2':instring('Prompt #2 Name',prmp^[2].name,20);
               '3':instring('Prompt #3 Name',prmp^[3].name,20);
               '4':instring('Prompt #4 Name',prmp^[4].name,20);
            end;
            pauseScr;
         end;
         'V':begin
            sprompt('Which? ');
            onek(r,'1234Q');
            case r of
               '1':sprint(prmp^[1].prompt);
               '2':sprint(prmp^[2].prompt);
               '3':sprint(prmp^[3].prompt);
               '4':sprint(prmp^[4].prompt);
            end;
            pauseScr;
         end;
      end;
   until ((done) or (hangup));
   assign(f,status^.gFilePath+'prompts.dat');
   {$I-} reset(f,1); {$I+}
   blockwrite(f,prmp^,sizeOf(prmp^));
   close(f);
end;

procedure pomisc;
var c:char;
    c1:integer;
    done,changed:boolean;
begin
   done:=FALSE;
   repeat
      with status^ do begin
         cls;
         HEADER('Miscellaneous Config');
         top;
         middle('A','System start-out menu        :'+allstartmenu);
         middle('B','Default bulletin prefix file :'+bulletprefix);
         middle('C','*BROKEN*                     :');
         {      middle('C','Message Editor               :'+aonoff(status^.msgedit,'FSE','Line'));}
         middle('D','Display "FILE.ANS MISSING"   :'+aonoff(status^.displayansimsg,'Yes','No'));
         middle('E','Use Screen Pauses            :'+aonoff(status^.scrpause,'Yes','No'));
         middle('F','Prompt Editor');
         bottom;
         command('QABDEF');
         onek(c,'QABDEF'^M);
         nl;
         case c of
            'F':promptedit;
            'D':displayansimsg := not displayansimsg;
            'E':scrpause := not scrpause;
            {        'C':msgedit:=not msgedit;}
            'Q':done:=TRUE;
            'A'..'B':begin
               prompt('Enter new: ');
               case c of
                  'A':inputwn(allstartmenu,8,changed);
                  'B':inputwn(bulletprefix,8,changed);
               end;
            end;
         end;
      end;
   until ((done) or (hangup));
end;

end.
