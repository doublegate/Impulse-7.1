{$A+,B+,F+,I+,L-,O+,R-,S-,V-}
Unit Common5;

Interface

uses records;
function mso:boolean;
function fso:boolean;
function cso:boolean;
function so:boolean;
function timer:real;
procedure shelldos(bat:boolean; cl:string; var rcode:integer);
function lowcase(s:string):string;
procedure onek(var c:char; ch:string);
procedure checkhangup;
procedure goxy(x,y:integer);
procedure writehdr(s:string);
procedure choosaprompt;
function lenn(s:string):integer;
function mln(s:string; l:integer):string;
function mlnnomci(s:string; l:integer):string;
function mlnmci(s:string; l:integer):string;
function mrn(s:string; l:integer):string;
function mn(i,l:longint):string;
procedure dosansi(c:char);
procedure lpromptc(c:char);
function expand(s:string; len : byte) : string;
procedure cDelay(ms : Word);

Implementation

Uses AnsiDrv,  Crt,   Dos,  Common, Common1, common2, common3, common4,
     tmpcom, scrlbk, myio,exec;

function expand(s:string; len : byte) : string;
var i : byte;
    str : string;
begin
   str := '';
   for i := 1 to len do str := str + s;
   expand := str;
end;

procedure checkhangup;
begin
   if (not com_carrier) then
      if ((outcom) and (not hangup)) then begin
         hangup:=TRUE;
         hungup:=TRUE;
      end;
end;


function mso:boolean;
var i:byte;
    b:boolean;
begin
   b:=FALSE;
   for i:=1 to 5 do
      if (board=thisuser.boardsysop[i]) then b:=TRUE;
   mso := ((cso) or (aacs(status^.msop)) or (b));
end;

function fso:boolean;
begin
   fso := ((cso) or (aacs(status^.fsop)));
end;

function cso:boolean;
begin
   cso:=((so) or (aacs(status^.csop)));
end;

function so:boolean;
begin
  so:=(aacs(status^.sop));
end;

function timer:real;
var r:registers;
    h,m,s,t:real;
begin
   r.ax:=44*256;
   msdos(dos.registers(r));
   h:=(r.cx div 256); m:=(r.cx mod 256); s:=(r.dx div 256); t:=(r.dx mod 256);
   timer:=h*3600+m*60+s+t/100;
end;

procedure shelldos(bat:boolean; cl:string; var rcode:integer);
var t:text;
    anyerrors:word;
    speed:integer;
    wind:windowrec;
begin
  {$I-}
  nosound;
  setwindow(wind,1,1,80,25,7,0,0);
  if (bat) then begin
    assign(t,'temp.bat');
    rewrite(t);
    writeln(t,cl);
    close(t);
    cl:='temp.bat';
  end;
  if (cl<>'') then cl:='/c '+cl;    { if '', just a local shell to DOS }
  remove_port;
  textcolor(7);
  swapvectors;
  If (status^.swapshell) then
{     dos.exec(getenv('COMSPEC'),cl)}
      rcode:= do_exec(getenv('COMSPEC'), cl, USE_ALL, $FFFF, FALSE)
  else
    dos.exec(getenv('COMSPEC'), cl);
  swapvectors;
  if (bat) then begin
    assign(t,'temp.bat');
    erase(t);
  end;
  removewindow(wind);
  {$I+}
  if (spd=strings^.locallogin) then speed:=modemr.waitbaud else speed:=value(spd);
  iport;
  com_set_speed(speed);
end;


Procedure GoXY(X,Y:integer);
begin
   if (spd<>strings^.locallogin) then pr1(#27+'['+cstr(y)+';'+cstr(x)+'H');
   if (wantout) then gotoxy(x,y);
   pap:=0;
end;

procedure writehdr(s:string);
begin
   hdrstr:=s;
   printf('hdr');
end;

procedure choosaprompt;
var c : char;
    finished,
    done : boolean;
begin
   finished:=false;
   repeat
      cls;
      if (not exist(status^.afilepath+'prompt.*')) then begin
         writehdr('Prompt Select');
         nl;
         sprint('|U0Choose Your Prompt Type|NL');
         sprint(#3#3+'[1] '+#3#0+prmp^[1].name);
         sprint(#3#3+'[2] '+#3#0+prmp^[2].name);
         sprint(#3#3+'[3] '+#3#0+prmp^[3].name);
         sprint(#3#3+'[4] '+#3#0+prmp^[4].name);
         sprint(#3#3+'[5] '+#3#0+'MNU Default');
         nl;
      end
      else printf('prompt');
      sprompt(strings^.entermsg2);
      onek(c,'QV12345'^M);
      case c of
         'V':begin
             nl;
             sprompt('View Which: ');
             onek(c,'12345');
             case (c) of
                '1' : sprint(prmp^[1].prompt);
                '2' : sprint(prmp^[2].prompt);
                '3' : sprint(prmp^[3].prompt);
                '4' : sprint(prmp^[4].prompt);
                '5' : sprint(menur.menuprompt);
             end;
	     pauseScr;
         end;
         '1': begin
            thisuser.prompt:=1;
            finished:=true;
         end;
         '2': begin
            thisuser.prompt:=2;
            finished:=true;
         end;
         '3': begin
            thisuser.prompt:=3;
            finished:=true;
         end;
         '4': begin
            thisuser.prompt:=4;
            finished:=true;
         end;
         '5': begin
            thisuser.prompt:=5;
            finished:=true;
         end;
         'Q',^M :finished:=true;
      end;
   until (finished);
end;


function lenn(s:string):integer;
var i,len:integer;
begin
   len:=length(s);
   i:=1;
   while (i<=length(s)) do begin
      if (s[i] in [#3,'^']) then if (i<length(s)) then begin
         dec(len,2);
         inc(i);
      end;
      inc(i);
   end;
   lenn:=len;
end;

function mlnnomci(s:string; l:integer):string;
begin
   while (length(s)<l) do s:=s+' ';
   if (length(s)>l) then
      repeat
         s:=copy(s,1,length(s)-1)
      until (length(s)=l) or (length(s)=0);
   mlnnomci:=s;
end;

function mlnmci(s:string; l:integer):string;
begin
  while (pipeLen(s)<l) do s:=s+' ';
  if (pipeLen(s)>l) then
    repeat s:=copy(s,1,length(s)-1) until (pipelen(s)=l) or (length(s)=0);
  mlnmci:=s;
end;

function mln(s:string; l:integer):string;
begin
   while (lenn(s)<l) do s:=s+' ';
   if (lenn(s)>l) then
      repeat
         s:=copy(s,1,length(s)-1)
      until (lenn(s)=l) or (length(s)=0);
   mln:=s;
end;

function mrn(s:string; l:integer):string;
begin
  while lenn(s)<l do s:=' '+s;
  if lenn(s)>l then s:=copy(s,1,l);
  mrn:=s;
end;

function mn(i,l:longint):string;
begin
   mn:=mln(cstr(i),l);
end;

procedure dosansi(c:char);
begin
  cansi(c);
end;

       {
	     if ((outcom) and (okansi)) then
         pr1(getc(bb));}
       {curco:=bb; textattr:=bb;}

   (*
   if (okscrollback) and not (reading_a_msg) then begin   {Add it to the line before LineFeed Is Printed}
     if beforeview then begin
       dec(curscrlline);
       dec(amtscrllines);
       beforeview:=false;
     end;
     addtoscroll(wherey);
     myscrolly:=wherey;
     end;
   *)
procedure lpromptc(c:char);
var ss:string;
    bb:byte;
begin
   if (c=^G) then exit;
   case c of
      ^H:if (pap>0) then dec(pap);
      ^J:begin
            if ((not ch) and (not write_msg) and (not reading_a_msg)) then
	       if (not ctrljoff) then bb:=thisuser.cols[color in thisuser.ac][1]
       	       else lil:=0;
	    if (wantout) then write(^J);
	    inc(lil);
            if ((lil>=thisuser.pagelen) and (not write_msg) and (status^.scrpause)) or
               ((lil>=thisuser.pagelen) and (reading_a_msg)) then begin
	       lil:=0;
               pausescr;
            end;
	    exit;
         end;
      ^L:lil:=0;
      ^M:pap:=0;
      ^[:dosansion:=TRUE;
   end;
   if (okscrollback) and (myscrolly<>wherey) and not (reading_a_msg) then begin
      if beforeview then begin
         dec(curscrlline);
         dec(amtscrllines);
         beforeview:=false;
      end;
      addtoscroll(myscrolly);
      myscrolly:=wherey;
   end;
   if (wantout) then
      if (not dosansion) then write(c)
      else cansi(c);
end;

function lowcase(s:string):string;
var i:integer;
begin
  for i:=1 to length(s) do
    if (s[i] in ['A'..'Z']) then s[i]:=chr(ord(s[i])+32);
  lowcase:=s;
end;

procedure onek(var c:char; ch:string);
var s:string;
begin
   cl(3);
   repeat
      if (not (onekey in thisuser.ac)) then begin
         input(s,3);
         if length(s)>=1 then c:=s[1]
         else if (s='') and (pos(^M,ch)<>0) then c:=^M
         else c:=' ';
      end
      else getkey(c);
   until (pos(upcase(c),ch)>0) or (hangup);
   if (hangup) then c:=ch[1];
   if (onekey in thisuser.ac) then begin
      if (onekda) then begin
         if (c in [#13,#32..#255]) then begin
	    outkey(c);
	    if (trapping) then write(trapfile,c);
         end;
      end;
     if (onekcr) then nl;
   end;
   onekcr:=TRUE;
   onekda:=TRUE;
   c:=upcase(c);
end;
procedure cDelay(ms : Word); Assembler;
asm
   mov ax, 1000;
   mul ms;
   mov cx, dx;
   mov dx, ax;
   mov ah, $86;
   int $15;
end;

{procedure onek(var c:char; ch:string);
var s:string;
begin
  repeat
    if (not (onekey in thisuser.ac)) then begin
      input(s,3);
      if length(s)>=1 then c:=s[1] else
	if (s='') and (pos(^M,ch)<>0) then c:=^M else
	  c:=' ';
    end else getkey(c);
  until (pos(upcase(c),ch)>0) or (hangup);
  if (hangup) then c:=ch[1];
  if (onekey in thisuser.ac) then begin
    if (onekda) then
      if (c in [#13,#32..#255]) then begin
	outkey(c);
	if (trapping) then write(trapfile,c);
      end;
    if (onekcr) then nl;
  end;
  onekcr:=TRUE;
  onekda:=TRUE;
  c:=upcase(c);
end;}

end.
