{ Misc. Stuff 3 }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit misc3;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common5,  common,
   records;

procedure finduserws(var usernum:integer);

implementation

procedure finduserws(var usernum:integer);
var user:userrec;
    sr:smalrec;
    nn:astr;
    gg:integer;
    c:char;
    sfo,ufo,done,asked:boolean;
begin
   ufo:=(filerec(uf).mode<>fmclosed);
   if (not ufo) then reset(uf);
   input(nn,36);
   usernum:=value(nn);
{   if (nn='SYSOP') then begin
      userNum := 1;
      exit;
   end;}
   if (usernum>0) then begin
      if (usernum>filesize(uf)-1) then begin
         print('Unknown User.');
         usernum:=0;
      end
      else begin
         seek(uf,usernum);
         read(uf,user);
      end;
   end
   else if (nn<>'') then begin
      sfo:=(filerec(sf).mode<>fmclosed);
      if (not sfo) then reset(sf);
      done:=FALSE;
      asked:=FALSE;
      gg:=-1;
      while ((gg<filesize(sf)-1) and (not done)) do begin
         inc(gg);
         seek(sf,gg);
         read(sf,sr);
         if (pos(nn,sr.name)<>0) then
            if (sr.name=nn) then usernum:=sr.number
            else begin
               if (not asked) then begin
                  nl;
                  asked:=TRUE;
               end;
               sprint(#3#1+'Incomplete match --> '+#3#3+caps(sr.name)+' #'+cstr(sr.number));
               sprompt(#3#7+'Is this correct? (Y/N,Q=Quit) : ');
               onek(c,'QYN'^M);
               done:=TRUE;
               case c of
                  'Q':usernum:=0;
                  'Y':usernum:=sr.number;
                  else done:=FALSE;
               end;
            end;
      end;
      if (usernum=0) then print('User not found.');
      if (not sfo) then close(sf);
   end;
   if (not ufo) then close(uf);
end;

end.
