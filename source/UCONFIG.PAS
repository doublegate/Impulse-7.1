{ OBV/2 Style User Config }
{$A+,B+,E-,F+,I+,L+,N-,O+,R-,S-,V-}
Unit UConfig;

Interface

Uses
   Crt,      AnsiDrv,  Cuser,    Common,   Dos,      common1,  common2,
   common3,  records,  common5;

Procedure ChangeUConfig;

Implementation

Procedure ChangeUConfig;
Var OPosX:Array [0..10] of byte;
    OPosY:Array [0..10] of byte;

var c:char; finished,done:boolean; s:string;
    TempV:Integer;
    user:userrec;

procedure showcfg;
var tf:text;
    oaa,abort,next,gone:boolean;
    c,k:char;
    deux:char;
    sin:string[2];
    s:string;
    i:integer;
begin
   printingfile:=TRUE;
   oaa:=allowabort;
   allowabort:=TRUE;
   abort:=FALSE;
   next:=FALSE;
   nofile:=true;
   gone:=true;

   assign(tf,status^.afilepath+'NEWINF.ANS');
   {$I-} reset(tf); {$I+}
   if ioresult<>0 then exit;
   nofile:=false;
   cls;
   while not eof(tf) do
      begin
      if (abort) then exit;
      deux:=k;
      read (tf,k);
      if k='%' then
      begin
	 read(tf,c);
	 sin:=c;
	 read(tf,c);
	 sin:=sin+c;
	 s:=upcase(sin[1])+upcase(sin[2]);
            if s='OP' then begin
              oposx[0]:=wherex;
              Oposy[0]:=wherey;
            end else
            if s='OA' then begin
              Oposx[1]:=wherex; Oposy[1]:=wherey;
              Prompt(Caps(thisuser.name));
            end else
            if s='OB' then begin
              Oposx[2]:=wherex; Oposy[2]:=wherey;
              Prompt(cstr(thisuser.pagelen));
            end else
            if s='OC' then begin
              Oposx[3]:=wherex; Oposy[3]:=wherey;
              Prompt(thisuser.note);
            end else
            if s='OD' then begin
              Oposx[4]:=wherex; Oposy[4]:=wherey;
              if (ansi in thisuser.ac) then
                Prompt('True ')
                  else Prompt('False');
            end else
            if s='OE' then begin
              Oposx[5]:=wherex; Oposy[5]:=wherey;
              if (onekey in thisuser.ac) then prompt('False')
              else prompt('True ');
            end else
            if s='OF' then begin
              Oposx[6]:=wherex; Oposy[6]:=wherey;
              if (pause in thisuser.ac) then prompt('False')
              else prompt('True ');
            end else
            if s='OG' then begin
              Oposx[7]:=wherex; Oposy[7]:=wherey;
              Prompt(thisuser.autosig);
            end else
            if s='OH' then begin
              Oposx[8]:=wherex; Oposy[8]:=wherey;
              if (novice in thisuser.ac) then prompt('False')
              else prompt('True ');
            end else
            if s='OI' then begin
              Oposx[9]:=wherex; Oposy[9]:=wherey;
              For I:=1 to Length(thisuser.PW) do prompt(strings^.echoc);
            end else
            if s='OJ' then begin
              Oposx[10]:=wherex; Oposy[10]:=wherey;
              prompt(prmp^[thisuser.prompt].name);
            end;
      end else
       begin
       if (outcom) then
        pr1(k);
        cansi(k);
       end;
   end;
  close(tf);
  allowabort:=oaa;
  printingfile:=FALSE; ctrljoff:=FALSE;
  curco:=255-curco; cl(1);
end;

Procedure EditOp(C:Char);
Var Tmp:string;
    I:Integer;
Begin
  Case C Of
  'A':Begin
        GoXy(Oposx[1],Oposy[1]);
        cstuff(7,1,thisuser);
        GoXy(Oposx[1],Oposy[1]);
        Prompt(thisuser.name);
      End;
  'B':Begin
        GoXy(Oposx[2],Oposy[2]);
        cstuff(11,1,thisuser);
        GoXy(Oposx[2],Oposy[2]);
{        user.pagelen:=value(s);}
        Prompt(cstr(thisuser.pagelen));
      End;
  'C':Begin
        GoXy(Oposx[3],Oposy[3]);
        cstuff(6,1,thisuser);
        GoXy(Oposx[3],Oposy[3]);
        Prompt(thisuser.note);
      End;
  'D':Begin
        GoXy(Oposx[4],Oposy[4]);
        if (ansi in thisuser.ac) then Begin
          thisuser.ac:=thisuser.ac+[ansi];
          Prompt('ANSI');
        end
        else begin
          Prompt('None');
          thisuser.ac:=thisuser.ac-[ansi];
        end;
      End;
  'E':Begin
        GoXy(Oposx[5],Oposy[5]);
        if (onekey in thisuser.ac) then begin
          thisuser.ac:=thisuser.ac-[onekey];
          prompt('False');
        end else begin
          thisuser.ac:=thisuser.ac+[onekey];
          prompt('True ');
        end;
      End;
  'F':Begin
        GoXy(Oposx[6],Oposy[6]);
        if (pause in thisuser.ac) then begin
          thisuser.ac:=thisuser.ac-[pause];
          print('False');
        end else begin
          thisuser.ac:=thisuser.ac+[pause];
          print('True ');
        end;
      End;
  'G':Begin
        GoXy(Oposx[7],Oposy[7]);
        cstuff(13,1,thisuser);
        GoXy(Oposx[7],Oposy[7]);
        Prompt(user.autosig);
      End;
  'H':Begin
        GoXy(OPosx[8],Oposy[8]);
        if (novice in thisuser.ac) then begin
          thisuser.ac:=thisuser.ac-[novice];
          chelplevel:=1;
          print('False');
        end else begin
          thisuser.ac:=thisuser.ac+[novice];
          chelplevel:=3;
          print('True ');
       end;
      End;
  'I':Begin
        GoXy(Oposx[9],Oposy[9]);
        cstuff(9,1,thisuser);
        GoXy(Oposx[9],Oposy[9]);
        For I:=1 to Length(user.PW) do prompt(strings^.echoc);
      end;
  'J':begin
        inc(user.prompt);
        if (user.prompt>5) then user.prompt:=1;
        GoXy(oposx[10],oposy[10]);
        prompt(prmp^[user.prompt].name);
      end;
  end;
End;

Begin
   if not(hangup) then begin
      fillchar(oposx,sizeOf(oposx),1);
      fillchar(oposy,sizeOf(oposx),1);
      done:=false;
      while (not done) and (not hangup) do begin
         ShowCfg;
         goxy(oposx[0],oposy[0]);
         sprompt(strings^.nustate);
         getkey(c);
         c:=upcase(c);
         Case c of
            'Q':done:=true;
            else if c in ['A','B','C','D','E','F','G','H','I','J'] then EditOp(c);
         end;
      end;
   end;
End;

End.
