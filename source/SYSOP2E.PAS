{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
{ Setup Flags }
unit sysop2e;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common5,  common4,
   common;

procedure poflagfunc;

implementation

procedure poflagfunc;
var s:string[80];
    c,cc:char;
    nuu,i:integer;
    done:boolean;
begin
   done:=FALSE;
   repeat
      with status^ do begin
         cls;
         header('Special Options');
         top;
         middle('A','Enable iCE Color         :'+onoff(specialfx)+
                '  B  Use EMS/XMS for OVR file  :'+onoff(useems));;
         middle('C','Handles allowed on system:'+onoff(allowalias)+
                '  D  Phone number in logon     :'+onoff(phonepw));
         middle('E','Local security protection:'+onoff(localsec)+
                '  F  Local screen security     :'+onoff(localscreensec));
         middle('G','Global activity trapping :'+onoff(globaltrap)+
                '  H  Auto chat buffer open     :'+onoff(autochatopen));
         middle('I','*NOT USED*                '+
                '     J  *NOT USED*');
         middle('K','*NOT USED*                '+
                '     L  *NOT USED*');
         middle('M','Blacklist Deleted Users  :'+onoff(multitask)+
                '  N  Offhook in local logon    :'+onoff(offhooklocallogon));
         middle('O','Mandatory voting         :'+onoff(forcevoting)+
                '  P  Compress file/msg numbers :'+onoff(compressbases));
         middle('R','UL duplicate file search :'+onoff(searchdup)+
                '  S  *NOT USED*');
         middle('T','Strip color off SysOp log:'+onoff(stripclog)+
                '  U  *NOT USED*');
         middle('V','Use BIOS for video output:'+onoff(usebios)+
                '  W  Use scroll back?          :'+onoff(useScrollBack));
         s:='New user message sent to :';
         if (newapp=-1) then s:=s+'Off'
         else s:=s+mn(newapp,3);
         s:=s+'  2  Guest user number         :';
         if (guestuser=-1) then s:=s+'Off'
         else s:=s+mn(guestuser,3);
         middle('1',s);
         s:='Mins before timeout bell :';
         if (timeoutbell=-1) then s:=s+'Off'
         else s:=s+mn(timeoutbell,3);
         s:=s+'  4  Mins before timeout       :';
         if (timeout=-1) then s:=s+'Off'
         else s:=s+mn(timeout,3);
         middle('3',s);
         bottom;
         command('QABCDEFGHMNOPRTVWX1234');
         onek(c,'QABCDEFGHMNOPRTVW1234'^M);
         nl;
         case c of
            'Q':done:=TRUE;
            'A':specialfx:=not specialfx;
            'B':useems:=not useems;
            'C':allowalias:=not allowalias;
            'D':phonepw:=not phonepw;
            'E':localsec:=not localsec;
            'F':localscreensec:=not localscreensec;
            'G':globaltrap:=not globaltrap;
            'H':autochatopen:=not autochatopen;
{        'I':autominlogon:=not autominlogon;
        'J':bullinlogon:=not bullinlogon;
        'K':lcallinlogon:=not lcallinlogon;
        'L':yourinfoinlogon:=not yourinfoinlogon;}
            'M':multitask:=not multitask;
            'N':offhooklocallogon:=not offhooklocallogon;
            'O':forcevoting:=not forcevoting;
            'P':begin
               compressbases:=not compressbases;
               nl;
               if (compressbases) then print('Compressing bases...')
               else print('De-compressing bases...');
               newcomptables;
            end;
            'R':searchdup:=not searchdup;
            'T':stripclog:=not stripclog;
{        'U':usewfclogo:=not usewfclogo;}
            'V': usebios:=not usebios;
              {directvideo:=not usebios;}
            'W':useScrollBack := not useScrollBack;
            '1'..'4': begin
               prompt('[E]nable [D]isable this function: ');
               onek(cc,'Q ED'^M);
               if cc in ['E','D'] then begin
                 badini:=FALSE;
                 case cc of
                    'D':i:=-1;
                    'E':begin
                       prompt('Range ');
                       case c of
                          '1','2':begin
                             reset(uf);
                             nuu:=filesize(uf)-1;
                             close(uf);
                             print('(1-'+cstr(nuu)+')');
                          end;
                          '3','4':print('(1-20)');
                          else print('(0-32767)');
                       end;
                       sprompt('|NLEnter value for this function: ');
                       inu(i);
                    end;
                 end;
                 if (not badini) then
                    case c of
                       '1':if ((i>=1) and (i<=nuu)) or (cc='D') then newapp:=i;
                       '2':if ((i>=1) and (i<=nuu)) or (cc='D') then guestuser:=i;
                       '3':if ((i>=1) and (i<=20)) or (cc='D') then timeoutbell:=i;
                       '4':if ((i>=1) and (i<=20)) or (cc='D') then timeout:=i;
                    end;
               end
               else print('No change.');
            end;
         end;
      end;
   until (done) or (hangup);
end;

end.
