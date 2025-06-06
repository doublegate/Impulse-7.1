{ Validation Levels }
{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit sysop2g;

interface

uses
   Crt,      Dos,      common5,  records,  sysop3,   common1,  common2,
   common3,  common4,  common;

procedure ponewauto;

implementation

function show_arflags(ss:integer):string;
var c:char;
    s:string[26];
begin
   s:='';
   for c:='A' to 'Z' do
      case ss of
         1:if c in status^.newar then s:=s+c else s:=s+'-';
         2:if c in status^.autoar then s:=s+c else s:=s+'-';
      end;
   show_arflags:=s;
end;

function show_restric(ss:integer):string;
var r:uflags;
    s:string[15];
begin
   s:='';
   for r:=rlogon to rmsg do
      case ss of
         1: if r in status^.newac then s:=s+copy('LCVBA*PEKM',ord(r)+1,1)
         else s:=s+'-';
         2: if r in status^.autoac then s:=s+copy('LCVBA*PEKM',ord(r)+1,1)
         else s:=s+'-';
      end;
   s:=s+'/';
   for r:=fnodlratio to fnodeletion do
      case ss of
         1: if r in status^.newac then s:=s+copy('1234',ord(r)-19,1)
         else s:=s+'-';
         2:if r in status^.autoac then s:=s+copy('1234',ord(r)-19,1)
         else s:=s+'-';
      end;
   show_restric:=s;
end;

procedure autoswac(var u:systatrec; r:uflags);
begin
   if r in u.autoac then u.autoac:=u.autoac-[r]
   else u.autoac:=u.autoac+[r];
end;

procedure autoacch(c:char; var u:systatrec);
begin
   case c of
      'L':autoswac(u,rlogon);
      'C':autoswac(u,rchat);
      'V':autoswac(u,rvalidate);
      'B':autoswac(u,rbackspace);
      'A':autoswac(u,ramsg);
      '*':autoswac(u,rpostan);
      'P':autoswac(u,rpost);
      'E':autoswac(u,remail);
      'K':autoswac(u,rvoting);
      'M':autoswac(u,rmsg);
      '1':autoswac(u,fnodlratio);
      '2':autoswac(u,fnopostratio);
      '3':autoswac(u,fnofilepts);
      '4':autoswac(u,fnodeletion);
   end;
end;

procedure zswac(var u:systatrec; r:uflags);
begin
   if (r in u.newac) then u.newac:=u.newac-[r]
   else u.newac:=u.newac+[r];
end;

procedure zacch(c:char; var u:systatrec);
begin
   case c of
      'L':zswac(u,rlogon);
      'C':zswac(u,rchat);
      'V':zswac(u,rvalidate);
      'B':zswac(u,rbackspace);
      'A':zswac(u,ramsg);
      '*':zswac(u,rpostan);
      'P':zswac(u,rpost);
      'E':zswac(u,remail);
      'K':zswac(u,rvoting);
      'M':zswac(u,rmsg);
      '1':zswac(u,fnodlratio);
      '2':zswac(u,fnopostratio);
      '3':zswac(u,fnofilepts);
      '4':zswac(u,fnodeletion);
   end;
end;

procedure ponewauto;
var done:boolean;
    c:char;
    b:byte;
    i:integer;
begin
   done:=FALSE;
   repeat
      cls;
      header('New user configuration              Auto-validation command');
      top;
      middle('A','SL : '+mln(cstr(status^.newsl),28)+'F  SL :'+cstr(status^.autosl));
      middle('B','DSL: '+mln(cstr(status^.newdsl),28)+'G  DSL:'+cstr(status^.autodsl));
      middle('C','AR : '+mln(show_arflags(1),28)+'H  AR :'+show_arflags(2));
      middle('D','AC : '+mln(show_restric(1),28)+'I  AC :'+show_restric(2));
      middle(' ','     '+mln(' ',28)+'E  #FP: '+cstr(status^.newfp));
      bottom;
      command('QABCDEFGHI');
      onek(c,'QABCDEFGHI');
      nl;
      case c of
         'A':begin
            prompt('Enter new user SL: ');
            ini(b);
            if not badini then status^.newsl:=b;
         end;
         'B':begin
            prompt('Enter new user DSL: ');
            ini(b);
            if not badini then status^.newdsl:=b;
         end;
         'C':repeat
            prompt('Toggle AR Flag? (A-Z) <CR>=Quit ['+show_arflags(1)+'] : ');
            onek(c,'ABCDEFGHIJKLMNOPQRSTUVWXYZ'^M);
            if c in ['A'..'Z'] then
               if c in status^.newar then status^.newar:=status^.newar-[c]
               else status^.newar:=status^.newar+[c];
         until (c=^M) or (hangup);
         'D':repeat
            prompt('Restrictions [?]Help <CR>=Quit ['+show_restric(1)+'] : ');
            onek(c,'Q?LCVBA*PEKM1234'^M);
            case c of
               'Q',^M:c:='Q';
               '?':restric_list;
               else zacch(c,status^);
            end;
         until (c='Q') or (hangup);
         'E': begin
            prompt('Enter auto validation file points: ');
            inu(i);
            if not badini then status^.newfp:=i;
         end;
         'F': begin
            prompt('Enter auto validation SL: ');
            ini(b);
            if not badini then status^.autosl:=b;
         end;
         'G': begin
            prompt('Enter auto validation DSL: ');
            ini(b);
            if not badini then status^.autodsl:=b;
         end;
         'H': repeat
             prompt('Toggle AR Flag? (A-Z) <CR>=Quit ['+show_arflags(2)+'] : ');
             onek(c,'ABCDEFGHIJKLMNOPQRSTUVWXYZ'^M);
             if c in ['A'..'Z'] then
                if c in status^.autoar then status^.autoar:=status^.autoar-[c]
                else status^.autoar:=status^.autoar+[c];
         until (c=^M) or (hangup);
         'I': repeat
             prompt('Restrictions ['+show_restric(2)+'] [?]Help [Q]uit : ');
             onek(c,'Q?LCVBA*PEKM1234'^M);
             case c of
                'Q',^M:c:='Q';
                '?':restric_list;
                else autoacch(c,status^);
             end;
         until (c='Q') or (hangup);
         'Q': done:=TRUE;
      end;
   until (done) or (hangup);
end;

end.
