{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
{ main configuration }
unit sysop2b;


interface

uses
   Crt,      Dos,      common1,  common2,  common3,  sysop3,   sysop2j,
   common4,  records,  common5,  common;

procedure pofile;

implementation

function wantit(s : string):boolean;
begin
   changeableMci := s;
   wantit:=pynq('|NL'+strings^.areyousure,true);
end;

procedure noch;
begin
  print('No change.');
end;

function phours(s:astr; lotime,hitime:integer):astr;
begin
  if (lotime<>hitime) then
    phours:=tch(cstr(lotime div 60))+':'+tch(cstr(lotime mod 60))+'...'+
            tch(cstr(hitime div 60))+':'+tch(cstr(hitime mod 60))
  else
    phours:=s;
end;

procedure gettimerange(s:astr; var st1,st2:integer);
var t1,t2,t1h,t1m,t2h,t2m:integer;
begin
   if pynq(s,true) then begin
      sprint('|NL|NLAll entries in 24 hour time.  Hour: (0-23), Minute: (0-59)|NL');
      prompt('Starting time:');
      prompt('  Hour   : ');
      inu(t1h);
      if (t1h<0) or (t1h>23) then t1h:=0;
      prompt('                Minute : '); inu(t1m);
      if (t1m<0) or (t1m>59) then t1m:=0;
      nl;
      prompt('Ending time:  ');
      prompt('  Hour   : ');
      inu(t2h);
      if (t2h<0) or (t2h>23) then t2h:=0;
      prompt('                Minute : ');
      inu(t2m);
      if (t2m<0) or (t2m>59) then t2m:=0;
      t1:=t1h*60+t1m;
      t2:=t2h*60+t2m;
   end
   else begin
      t1:=0;
      t2:=0;
   end;
   nl;
   prompt('Hours: '+phours('Undeclared',t1,t2));
   if (wantit('change time range')) then begin
      st1:=t1;
      st2:=t2;
   end;
end;

procedure pofile;
var s   : str80;
    i   : integer;
    c   : char;
    barg,
    done: boolean;
begin
  done:=FALSE;
  repeat
    with status^ do begin
      cls;
      header('BBS Configuration');
      top;
      middle('A','BBS name & number  :'+bbsname+' ('+bbsphone+')');
      middle('B','SysOp''s name/alias :'+sysopname);
      middle('C','SysOp chat hours   :'+mln(phours('*None*',lowtime,hitime),16));
      middle('D','Regular DL hours   :'+mln(phours('Always allowed',dllowtime,dlhitime),16));
      middle('E','Matrix Logon       :'+mln(aonoff(shuttlelog,'Active','In-active'),16));
      middle('F','BBS Passwords                       ');
      middle('G','9600- baud hours   :'+phours('Always allowed',b300lowtime,b300hitime));
      middle('H','9600- baud DL hours:'+phours('Always allowed',b300dllowtime,b300dlhitime));
      middle('I','New users are      :'+aonoff(closedsystem,'Rejected','Accepted'));
      middle('J','Swap shell function:'+aonoff(swapshell,'Active','In-active'));
      middle('K','Pre-event warning  :'+cstr(eventwarningtime)+' seconds');
      middle('L','Directories                         ');
      middle('M','Multi-Node Editing                  ');
      middle('N','Use New User voting:'+aonoff(usenuv,'Active','In-active'));
      middle('!','Lockout 9600- Baud :'+syn(lock300)+'             ');
      bottom;
      command('QABCDEFGHIJK!LMNR');
      onek(c,'QABCDEFGHIJK!LMNR'); nl;
      case c of
        'Q':done:=TRUE;
        'A':begin
              print('New BBS name');
              prompt(':');
              inputwc(s,80);
              if (s<>'') then bbsname:=s else noch;
              nl;
              print('New BBS phone number, entered in the following format:');
              print('214-555-5555');
              prompt(':');
              input(s,12);
              if (s<>'') then bbsphone:=s else noch;
            end;
        'B':begin
              prompt('New SysOp name: ');
              inputl(s,30);
              if (s<>'') then sysopname:=s else noch;
            end;
        'M':ponode;
        'N':begin
	     usenuv:=pynq('Use new user voting? ',true);
	    end;

        'C':if (incom) then
              sprint(#3#7+'This can only be changed locally.')
            else
              gettimerange('Do you want to declare sysop hours? ',
                            lowtime,hitime);
        'D':gettimerange('Do you want to declare download hours? ',
                          dllowtime,dlhitime);
        'E':shuttlelog:=pynq('Use Matrix Login? ',true);
        '!':begin
              sprint('If callers below 9600 are locked out, "NO9600.MSG"');
              sprint('will be printed each time a caller under 9600 connects,');
              sprint('and the user will be hung up.');
              nl;
              lock300:=pynq('Should callers below 9600 baud be LOCKED OUT? ',true);
            end;
        'F':begin
              print('System Passwords:');
              print('  A. SysOp password        :'+sysoppw);
              print('  B. New user password     :'+newuserpw);
              print('  C. System Password       :'+shuttlepw);
              nl;
              prompt('Change (A-C) : '); onek(c,'QABC'^M);
              if (c in ['A'..'C']) then begin
                case c of
                  'A':prompt('New SysOp password: ');
                  'B':prompt('New new-user password: ');
                  'C':prompt('New System Password: ');
                end;
                 input(s,20);
                nl; print('New password: "'+s+'"');
                if (wantit('change password')) then
                  case c of
                    'A':sysoppw:=s;
                    'B':newuserpw:=s;
                    'C':shuttlepw:=s;
                  end;
              end;
            end;
        'G':gettimerange('Do you want to declare 9600 baud & below hours? ',
                          b300lowtime,b300hitime);
        'H':gettimerange('Do you want to declare 9600 baud & below hours? ',
                          b300dllowtime,b300dlhitime);
        'I':closedsystem:=pynq('Should new users be REJECTED? ',false);
        'J':swapshell:=pynq('Should the swap shell function be used? ',true);
        'K':begin
              prompt('New pre-event warning time ['+cstr(eventwarningtime)+'] : ');
              inu(i);
              if (not badini) then eventwarningtime:=i;
            end;
        'L':begin
             cls; barg:=false;
             repeat
              header('Directories');
              top;
              middle('0','Swap shell directory     ['+swappath+']');
              middle('1','Main data files dir.     ['+gfilepath+']');
              middle('2','Alternate text file dir. ['+afilepath+']');
              middle('3','Priv/pub msgs directory  ['+msgpath+']');
              middle('4','Menu file directory      ['+menupath+']');
              middle('5','Script (IPL) directory   ['+tfilepath+']');
              middle('6','Trap/log/chat directory  ['+trappath+']');
              middle('7','Temporary directory      ['+temppath+']');
              bottom;
              command('01234567Q');
              onek(c,'01234567Q');
              case c of
              '0'..'7':begin
              prompt('Enter new ');
               case c of
                 '1':prompt('DATA');   '2':prompt('ANSI');
                 '3':prompt('EMAIL');    '4':prompt('MENUS');
                 '5':prompt('SCRIPT');   '6':prompt('TRAP');
                 '7':prompt('TEMP');     '0':prompt('SWAP');
               end;
              print(' path:');
              input(s,79);
              if (s<>'') then begin
                if (copy(s,length(s),1)<>'\') then s:=s+'\';
                if (wantit('change directory')) then
                  case c of
                    '1':gfilepath:=s;     '2':afilepath:=s;
                    '3':msgpath:=s;       '4':menupath:=s;
                    '5':tfilepath:=s;     '6':trappath:=s;
                    '7':temppath:=s;      '0':swappath:=s;
                  end;
              end
              else noch;
             end;
             'Q':barg:=true;
            end;
            until ((barg) or (hangup));
            end;
      end;
    end;
  until (done) or (hangup);
end;

end.
