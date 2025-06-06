(* Time Bank *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
Unit timeBank;

Interface

Uses
   common,   sys,      common2,  common3;

procedure timeBankDeposit(time : integer);           { deposit time }
procedure timeBankWithDraw(time : integer);              { withdraw time }

Implementation

(* Make sure they don't have too much time, OR negative time *)
procedure checkTimeBank;
begin
   if thisUser.timebank > status^.maxTimeBank then thisUser.timeBank := status^.maxTimeBank;
   if thisUser.timeBank < 0 then thisUser.timeBank := 0;
end;

(* Deposit time in the time bank *)
procedure timeBankDeposit(time : integer);
var num : integer;
begin
   checkTimeBank;
   if (time <> 0) then begin
      if (time <= thisUser.tlToday) then begin
         inc(thisUser.timeBank,time);
         dec(thisUser.tlToday,time);
      end;
   end
   else begin
      sprompt(strings^.timeBankDeposit);
      inu(num);
      if badIni then sprint(strings^.aborted);
      if (num <= thisUser.tlToday) then begin
         inc(thisUser.timeBank,num);
         dec(thisUser.tlToday,num);
      end;
      checkTimeBank;
   end;
end;

(* Withdraw time from the time bank *)
procedure timeBankWithDraw(time : integer);
var num : integer;
begin
   checkTimeBank;
   if (time <> 0) then begin
      if (time <= thisUser.timeBank) then begin
         inc(thisUser.tlToday,time);
         dec(thisUser.timeBank,time);
      end;
   end
   else begin
      sprompt(strings^.timeBankWithDraw);
      inu(num);
      if badIni then sprint(strings^.aborted);
      if (num <= thisUser.timeBank) then begin
         inc(thisUser.tlToday,num);
         dec(thisUser.timeBank,num);
      end;
      checkTimeBank;
   end;
end;

{procedure TimeBank(s:astr);
var lng,
    maxperday,
    maxever:longint;
    zz:integer;
    oc:string[10];
    c:char;

  function cantdeposit:boolean;
  begin
    cantdeposit:=TRUE;
    if ((thisuser.timebankadd>=maxperday) and (maxperday<>0)) then exit;
    if ((thisuser.timebank>=maxever) and (maxever<>0)) then exit;
    cantdeposit:=FALSE;
  end;

begin
  maxperday:=value(s);
  maxever:=0;
  if (pos(';',s)<>0) then maxever:=value(copy(s,pos(';',s)+1,length(s)));
  if ((maxever<>0) and (thisuser.timebank>maxever)) then thisuser.timebank:=maxever;
  nl; nl;
  sprint('^5Time Bank ');
  nl;
  if (not cantdeposit) then
    sprint('^3A^1)dd time to your account.');
  sprint('^3G^1)oodbye, log off now.');
  sprint('^3Q^1)uit to BBS.');
  if (choptime=0.0) then
    sprint('^3W^1)ithdraw time from your account.');
  nl;
  if (choptime<>0.0) then
    sprint(#3#7+'You cannot withdraw time during this call.');
  if (cantdeposit) then begin
    if ((thisuser.timebankadd>=maxperday) and (maxperday<>0)) then
      sprint(#3#7+'You cannot add any more time to your account today.');
    if ((thisuser.timebank>=maxever) and (maxever<>0)) then
      sprint(#3#7+'You cannot add any more time to your account!');
  end;
  nl;
  sprompt(#3#5+'In your account: '+#3#3+cstr(thisuser.timebank)+
          #3#5+'   Time left online: '+#3#3+cstr(trunc(nsl) div 60));
  if (thisuser.timebankadd<>0) then
    sprompt('   ^5Deposited today: ^3'+cstr(thisuser.timebankadd));
  nl;
  sprompt(#3#5+'Account limits: '+#3#3);
  if (maxever<>0) then sprompt(cstr(maxever)+' max')
    else sprompt('No max limit');
  if (maxperday<>0) then sprompt(' / '+cstr(maxperday)+' per day');
  nl; nl;
  prompt('Time Bank :');
  oc:='QG';
  if (choptime=0.0) then oc:=oc+'W';
  if (not cantdeposit) then oc:=oc+'A';
  onek(c,oc);
  case c of
    'A':begin
          prompt('Add how many minutes? '); inu(zz); lng:=zz;
          nl;
          if (not badini) then
            if (lng>0) then
              if (lng>trunc(nsl) div 60) then
                sprint(#3#7+'You don''t have that much time left to deposit!')
              else
                if (lng+thisuser.timebankadd>maxperday) and (maxperday<>0) then
                  sprint(#3#7+'You can only add '+cstr(maxperday)+' minutes to your account per day!')
                else
                  if (lng+thisuser.timebank>maxever) and (maxever<>0) then
                    sprint(#3#7+'Your account deposit limit is '+cstr(maxever)+' minutes!')
                  else begin
                    inc(thisuser.timebankadd,lng);
                    inc(thisuser.timebank,lng);
                    dec(thisuser.tltoday,lng);
                    sprint('^5In your account: ^3'+cstr(thisuser.timebank)+
                           '^5   Time left online: ^3'+cstr(trunc(nsl) div 60));
                    sysoplog('TimeBank: Deposited '+cstr(lng)+' minutes.');
                  end;
              end;
          'G':hangup:=TRUE;
          'W':begin
                prompt('Withdraw how many minutes? '); inu(zz); lng:=zz;
                nl;
                if (not badini) then
                  if (lng>thisuser.timebank) then
                    sprint(#3#7+'You don''t have that much time left in your account!')
                  else
                    if (lng>0) then begin
                      dec(thisuser.timebankadd,lng);
                      if (thisuser.timebankadd<0) then thisuser.timebankadd:=0;
                      dec(thisuser.timebank,lng);
                      inc(thisuser.tltoday,lng);
                      sprint('^5In your account: ^3'+cstr(thisuser.timebank)+
                             '^5   Time left online: ^3'+cstr(trunc(nsl) div 60));
                      sysoplog('TimeBank: Withdrew '+cstr(lng)+' minutes.');
                    end;
                  end;
  end;
end;}

end.
