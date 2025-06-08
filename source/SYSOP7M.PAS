{ Menu Editor }
{$A+,B+,D-,F+,I+,L+,O+,R-,S-,V-}
unit sysop7m;

interface

uses
   Crt,   Dos, common5,
  common, common3,
  file9,records,
  menus2;


procedure memm(scurmenu:astr; var menuchanged:boolean);

implementation

procedure instring(var v:astr; len:integer);
var changed:boolean;
begin
   if (incom) then inputwnwc(v,len,changed)
   else v:=NInput(WhereX,WhereY,v,len);
end;

procedure memm(scurmenu:astr; var menuchanged:boolean);
var xpnt,ypnt,i1,i2,ii,z:integer;
    c:char;
    s:astr;
    b:byte;
    redraw,bb:boolean;

begin
   prompt('Begin editing at which? (1-'+cstr(noc)+') : '); inu(ii);
   c:=' ';
   if (ii>=1) and (ii<=noc) then begin
      while (c<>'Q') and (not hangup) do begin
         repeat
            with cmdr[ii] do begin
               if (c<>'?') then begin
                  cls;
                  sprint('|U5þ |U0'+scurmenu); nl;
                  sprint('|U1Command '+cstr(ii)+' of '+cstr(noc)+'|NL');
                  if (pulldown in menur.menuflags) then begin
                     sprint('|U5[|U01|U5]|U1 Normal Text   :'+ldesc);
                     sprint('|U5[|U02|U5]|U1 Highlighted   :'+sdesc);
                     sprint('|U5[|U0X|U5]|U1 X Coordinate  :'+xpoint);
                     sprint('|U5[|U0Y|U5]|U1 Y Coordinate  :'+ypoint);
                  end
                  else begin
                     sprint('|U5[|U01|U5]|U1 Long descript :'+ldesc);
                     sprint('|U5[|U02|U5]|U1 Short descript:'+sdesc);
                  end;
                  sprint('|U5[|U03|U5]|U1 Cmd letters   :'+ckeys);
                  sprint('|U5[|U04|U5]|U1 ACS required  :"'+acs+'"');
                  sprint('|U5[|U05|U5]|U1 Cmdkeys       :'+cmdkeys);
                  sprint('|U5[|U06|U5]|U1 CString       :'+mstring);
                  s:='';
                  if (pull in commandflags) then s:='(P)ulldown';
                  if (hidden in commandflags) then begin
                     if (s<>'') then s:=s+', ';
                     s:=s+'(H)idden';
                  end;
                  if (unhidden in commandflags) then begin
                     if (s<>'') then s:=s+', ';
                     s:=s+'(U)nhidden';
                  end;
                  if (s='') then s:='None';
                  sprint('|U5[|U07|U5]|U1 Flags         :'+s);
                  sprint('|U5[|U0Q|U5]|U1 Quit');
               end;
               sprompt('|NL|U0Menu Editing |U5[|U1?/Help|U5]|U3 ');
               onek(c,'Q1234567[]FJL?XY'^M);
               redraw := true;
               nl;
               case c of
                  '1':begin
                     if (pulldown in menur.menuflags) then print('Normal Text Line:')
                     else print('New long description:');
                     instring(ldesc,70);
                     menuchanged:=true;
                  end;
                  '2':begin
                     if (pulldown in menur.menuflags) then print('Highlighted Text Line:')
                     else print('New short description: ');
                     instring(sdesc,70);
                     menuchanged:=true;
                  end;
                  'X':if (pulldown in menur.menuflags) then begin
                     prompt('Command X Point: ');
                     inu(xpnt);
                     cmdr[ii].xpoint:=cstr(xpnt);
                     menuchanged:=true;
                  end;
                  'Y':if (pulldown in menur.menuflags) then begin
                     prompt('Command Y Point: ');
                     inu(ypnt);
                     cmdr[ii].ypoint:=cstr(ypnt);
                     menuchanged:=true;
                  end;
                  '3':begin
                     prompt('New command letters: ');
                     input(s,14);
                     if (s<>'') then begin
                        ckeys:=s;
                        menuchanged:=TRUE;
                     end;
                  end;
                  '4':begin
                     prompt('New ACS: ');
                     inputwn(acs,20,menuchanged);
                  end;
                  '5':begin
                     prompt('New command type: ');
                     input(s,2);
                     if (length(s)=2) then begin
                        cmdkeys:=s;
                        menuchanged:=TRUE;
                     end;
                  end;
                  '6':begin
                     prompt('New CString: ');
                     inputwnwc(mstring,50,menuchanged);
                  end;
                  '7':begin
                     print('(P)ulldown - (H)idden command  -  (U)nhidden command');
                     sprompt('|NL|U1Choose|U9:|U3 ');
                     onek(c,'QHUP'^M);
                     bb:=menuchanged;
                     menuchanged:=TRUE;
                     case c of
                        'P':if (pull in commandflags) then commandflags:=commandflags-[pull]
                           else commandflags:=commandflags+[pull];
                        'H':if (hidden in commandflags) then commandflags:=commandflags-[hidden]
                           else commandflags:=commandflags+[hidden];
                        'U':if (unhidden in commandflags) then commandflags:=commandflags-[unhidden]
                           else commandflags:=commandflags+[unhidden];
                        else menuchanged:=bb;
                     end;
                     c:=#0;
                  end;
                  '[': if (ii>1) then dec(ii)
                     else c:=' ';
                  ']': if (ii<noc) then inc(ii)
                     else c:=' ';
                  'F': if (ii<>1) then ii:=1
                     else c:=' ';
                  'J':begin
                     prompt('Jump to entry: ');
                     input(s,3);
                     if (value(s)>=1) and (value(s)<=noc) then ii:=value(s)
                     else c:=' ';
                  end;
                  'L':if (ii<>noc) then ii:=noc
                     else c:=' ';
                  '?':;{ee_help;}
               end;
            end;
         until (c in ['Q','[',']','F','J','L']) or (hangup);
      end;
   end;
end;

end.
