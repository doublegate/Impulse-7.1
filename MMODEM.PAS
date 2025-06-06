{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit mmodem;

interface

uses
   Crt,      Dos,      common,   common5,  tmpcom,   records,  myio;

procedure outmodemstring1(s:astr);
procedure outmodemstring000(s:astr; showit:boolean);
procedure outmodemstring(s:astr);
procedure dophonehangup(showit:boolean);
procedure dophoneoffhook(showit:boolean);

implementation

uses common1,common2,common3;

procedure outmodemstring1(s:astr);
var i:integer;
begin
   for i:=1 to length(s) do begin
      com_tx(s[i]);
      cdelay(2);
   end;
   if (s<>'') then com_tx(^M);
end;

procedure outmodemstring000(s:astr; showit:boolean);
var i:integer;
begin
   for i:=1 to length(s) do
      case s[i] of
         '~':cdelay(500);
         '|':com_tx(^M);
      else begin
         com_tx(s[i]);
         cdelay(2);
      end;
   end;
   com_tx(^M);
end;

procedure outmodemstring(s:astr);
begin
   outmodemstring000(s,TRUE);
end;

procedure dophonehangup(showit:boolean);
var rl:real;
    tryc,rcode:integer;
    c:char;

   procedure dely(r:real);
   var r1:real;
   begin
      r1:=timer;
      while abs(timer-r1)<r do;
   end;

begin
   if (spd<>strings^.locallogin) then begin
      tryc:=0;
      while ((tryc<6) and (com_carrier) and (not keypressed)) do begin
         term_ready(FALSE);
         dely(2.0);
         term_ready(TRUE);
         com_flush_rx;
         outmodemstring000(modemr.hangup,showit);
         rl:=timer;
         while (c<>'0') and (abs(timer-rl)<2.0) do c:=cinkey;
         inc(tryc);
      end;
      term_ready(TRUE);
      if (keypressed) then c:=readkey;
   end;
   if (exist('plogoff.bat')) then shelldos(FALSE,'plogoff.bat',rcode);
end;

procedure dophoneoffhook(showit:boolean);
var rl1:real;
    c:char;
    done:boolean;
begin
   cdelay(300);
   com_flush_rx;
   outmodemstring000(modemr.offhook,showit);
   com_flush_rx;
   rl1:=timer;
   done:=FALSE;
   c:=#0;
   repeat
      c:=cinkey;
      if (c<>#0) then begin
         if (c=^M) then done:=TRUE;
      end;
   until ((abs(timer-rl1)>1.0) or (done)) or (keypressed);
   cdelay(50);
   com_flush_rx;
   textColor(15);
end;

end.
