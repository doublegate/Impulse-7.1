(* Misc Commands/Functions *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}

unit misc1;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  ansidrv,  common5,
   common,   records;

procedure HighBackground(Onn: boolean);          { toggle ice color }
procedure userlist;                              { user list }
procedure lastcaller;                            { last callers }
procedure reqchat(x:astr);                       { chat request }
{procedure TimeBank(s:astr);                      { time bank }

implementation

uses
   mail2,    sys;

(* toggle ice color *)
procedure HighBackground(Onn: boolean);
var DisplayRegs    : Registers;
Begin
   With DisplayRegs Do Begin
      AX := $1003;
      BL := Ord(Not Onn);
      Intr($10,DisplayRegs);
   End;
end;

(* Reqest sysop chat *)
procedure reqchat(x:astr);
var i:integer;
    s:string[70];
Begin
   (* If they have reached the maximum chat attempts *)
   if (chatt>status^.maxchat) then begin
      printf('goaway');
      irt:='Tried chatting (more than '+cstr(status^.maxchat)+' times!)';
      sysoplog('Tried chatting more than '+cstr(status^.maxchat)+' times');
      imail(value(x));
      chatcall:=false;
      exit;
   end;

   (* If the chat caller is still on, turn it off *)
   if (chatcall) then begin
      chatcall:=false;
      exit;
   end;

   (* Get the new chat reason *)
   sprompt(strings^.chatcall1);
   inputl(s,70);
   inc(chatt);                                   { update number of chat attempts }
   chatcall:=true;                               { turn on the chat caller }
   sprint(strings^.guestline);                   { let user know chat is activate }
end;


(* User list *)
procedure userlist;
Var RecF:File of userrec;
    RecFi:userRec;
    pos:byte;

Begin
   (* Initialze the user file *)
   Assign(Recf,status^.gfilepath+'USER.LST');
   reset(recf);

   read(recf,recfi);                             { kill the dummy 1st account }
   printf('userlist.TOP');                       { print the header ansi }
   pos := 1;

   (* Print all the users *)
   while (not eof(recf)) do begin
      read(recf,recfi);
      (* only print if the user isn't deleted *)
      if (not recfi.deleted) then begin
         sysMciSet('UN',recfi.name);
         sysMciSet('RN',recfi.realname);
         sysMciSet('SN',recfi.note);
         sysMciSet('LO',recfi.laston);
         sysMciSet('SX',recfi.sex);
         sysMciSet('NU',cstr(pos));
         printf('userlist.mid');
         inc(pos);
         sysResetMciList;                           { reset the mci lsit }
      end;
   end; {eof(recf}
   close(recf);
   printf('userlist.BOT');                       { print the footer ansi }
   pauseScr;
End;

(* Print the last callers *)
procedure lastcaller;
Var RecF:File of lcallers;
    RecFi:lcallers;
    pos:byte;
Begin
   (* Initialize the file *)
   Assign(Recf,status^.gfilepath+'laston.dat');
   reset(recf);
   pos := 1;
   printf('lastcall.TOP');                       { print the header ansi }

   (* print all the last callers *)
   while (not eof(recf)) do begin
      read(recf,recfi);
      sysMciSet('CN',cstr(recfi.callernum));
      sysMciSet('UN',caps(recfi.name));
      sysMciSet('NM',cstr(recfi.number));
      printf('lastcall.mid');
      inc(pos);
      sysResetMciList;
   end; {eof(recf}
   close(recf);
   printf('lastcall.BOT');                      { print the footer ansi }
   pauseScr;
End;
end.
