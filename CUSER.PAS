(* Change User Settings *)
{$A+,B+,E-,F+,I+,L+,N-,O+,R-,S-,V-}
unit cuser;

interface

uses
   Crt,      Dos,      misc5,    common5,  records,  common,   common1,
   common2,  common3,  strproc;

function cstuff(which,how:byte; var user:userrec) : boolean;

implementation

function cstuff(which,how:byte; var user:userrec) : boolean;
var done,done1:boolean;
    tryc:integer;
    s:astr;
    i:integer;
    cnt : byte;

   procedure doautosig;
   begin
      sprompt(strings^.enterautosig);
      inputl(s,40);
      if (s<>'') then user.autosig:=s;
      done1:=TRUE;
   end;

   procedure doaddress;
   begin
      sprompt(strings^.nuaddress);
      if (how=3) then inputl(s,30)
      else inputcaps(s,30);
      if (s<>'') then begin
         user.street:=s;
         done1:=TRUE;
      end;
   end;

{   procedure doZipCode;
   begin}


   procedure doansi;
   begin
      if pynq(strings^.nuansi,true) then begin
         user.ac := user.ac+[ansi];
         pr(#27+'[0;1;5;33;40mANSI');
         textColor(142);
         sprint('ANSI test');
         if pynq(strings^.nuwords,true) then user.ac:=user.ac+[color];
      end;
      done1:=TRUE;
   end;

   procedure docitystate;
   begin
      s := '';
      while (not hangup) and (s='') do begin
         sprompt(strings^.nucity);
         inputl(s,30);
         user.citystate:=s;
         done1:=TRUE;
      end;
   end;

   procedure doname;
   var i:integer;
       s1:astr;
       sfo:boolean;
       sr:smalrec;
   begin
      s := '';
      while not(hangup) and (s='') do begin
        if (status^.allowalias) then sprompt(strings^.nuhandle)
        else sprompt(strings^.nureal);
        inputl(s,36);
        s:=strAllCaps(s);
      end;
      if (blacklist(s)) then begin
         printf('blacklst');
         hangup:=true;
         exit;
      end;
      done1:=TRUE;
      {if ((not (s[1] in ['A'..'Z','?'])) or (s='')) then done1:=FALSE;}
      sfo:=(filerec(sf).mode<>fmclosed);
      if (not sfo) then reset(sf);
      for i:=1 to filesize(sf)-1 do begin
         seek(sf,i);
         read(sf,sr);
         if (sr.name=s) then begin
            done1:=FALSE;
            sprint(#3#7+'That name is already being used.');
         end;
      end;
      if (not sfo) then close(sf);
      if (not done1) and (not hangup) then begin
         sprint('Sorry, can''t use that name.');
         inc(tryc);
         sl1('Unacceptable name : '+s);
      end;
      if (tryc>=3) then hangup:=TRUE;
      if (done1) then user.name:=s;
      if ((done) and (how=1) and (not status^.allowalias)) then user.realname:=caps(s);
   end;

   procedure dopw;
   var s1:astr;
       d2 : boolean;
   begin
      d2 := true;
      done1:=false;
      case how of
         1:begin
            s:='';
            While (not done1) and (Not Hangup) Do Begin
               while d2 do begin
                  sprompt(strings^.nuenterpw);
                  inputl(s,20);
                  s:=strAllCaps(s);
                  if s <> '' then d2 := false;
               end;
               sprompt(strings^.reenterpw);
               inputl(s1,20);
               s1:=strAllCaps(s1);
               if (s=s1) then begin
                  user.pw:=s;
                  done1:=true;
               end
               else sprompt(strings^.wrongPassword);
            End;
            user.pw:=s;
            done1:=true;
         end;
         2:begin
            done1 := true;
            sprompt(strings^.currentPassword);
            inputl(s,20);
            s:=strAllCaps(s);
            if (s<>user.pw) then sprompt(strings^.wrongPassword)
            else begin
               s := '';
               sprompt(strings^.nuEnterPw);
               inputl(s,20);
               s:=strAllCaps(s);
               if (s = '') then sprint(strings^.aborted)
               else begin
                  sprompt(strings^.reEnterPw);
                  inputl(s1,20);
                  s1 := StrAllCaps(s1);
                  if (s1 <> s) then sprint(strings^.aborted)
                  else user.pw := s;
               end;
            end;
         end;
         3:begin
            sprompt(strings^.nuEnterPw);
            inputl(s,20);
            s:=strAllCaps(s);
            if (s<>'') then begin
               done1:=TRUE;
               user.pw:=s;
            end;
         end;
      end;
   end;

   procedure dorealname;
   begin
      if ((how=1) and (not status^.allowalias)) then begin
         user.realname:=caps(user.name);
         done1:=TRUE;
         exit;
      end;
      sprompt(strings^.nureal);
      inputl(s,36);
      if (s='=') then s:=caps(user.name);
      if (s<>'') then begin
         user.realname:=s;
         done1:=TRUE;
      end;
   end;

   procedure doscreen;
   var bb:byte;
   begin
      if (how=1) then begin
         user.linelen:=80;
         user.pagelen:=24;
      end;
      sprompt(strings^.nuscrwide);
      ini(bb);
      if (not badini) then user.linelen:=bb;
      sprompt(strings^.nuscrlines);
      ini(bb);
      if (not badini) then user.pagelen:=bb;
      done1:=TRUE;
   end;

   procedure dosex;
   var c:char;
   begin
      while (not hangup) do begin
         sprompt(strings^.nusex);
         onek(c,'MF'^M);
         if (c in ['M','F']) then begin
            user.sex:=c;
            done1:=TRUE;
            exit;
         end;
      end;
   end;

   procedure forwardmail;
   var u:userrec;
       s:astr;
       i:integer;
       b,ufo:boolean;
   begin
      nl;
      print('If you forward your mail, all mail');
      print('addressed to you will go to that person');
      print('Now enter the user''s number, or just');
      print('hit <CR> to deactivate mail forwarding: ');
      inputl(s,4);
      s:=strAllCaps(s);
      i:=value(s);
      nl;
      if (i=0) then begin
         user.forusr:=0;
         print('Forwarding deactivated.');
      end
      else begin
         ufo:=(filerec(uf).mode<>fmclosed);
         if (not ufo) then reset(uf);
         b:=TRUE;
         if (i>=filesize(uf)) then b:=FALSE
         else begin
            seek(uf,i);
            read(uf,u);
            if (u.deleted) or (nomail in u.ac) then b:=FALSE;
         end;
         if (i=usernum) then b:=FALSE;
         if (b) then begin
            user.forusr:=i;
            print('Forwarding set to: '+caps(u.name)+' #'+cstr(i));
            sysoplog('Started forwarding mail to '+caps(u.name)+' #'+cstr(i));
         end
         else print('Sorry, can''t forward to that user.');
         if (not ufo) then close(uf);
      end;
   end;

   procedure mailbox;
   begin
      if (nomail in user.ac) then begin
         user.ac:=user.ac-[nomail];
         sprint(#3#5+'Mailbox now open.');
         sysoplog('Opened mailbox.');
      end
      else if (user.forusr<>0) then begin
         user.forusr:=0;
         print('Mail no longer forwarded.');
         sysoplog('Stopped forwarding mail.');
      end
      else begin
         if pynq('Do you want to close your mailbox? ',false) then begin
            user.ac:=user.ac+[nomail];
            sprint(#3#5+'Mailbox now closed.');
            sprint(#3#5+'You >CAN NOT< recieve mail now.');
            sysoplog('Closed mailbox.');
         end
         else if pynq('|NLDo you want your mail forwarded? ',false) then forwardmail;
      end;
      done1:=TRUE;
   end;

   procedure dophone;
   var S : String[12];
       stop : Boolean;
       P : byte;
       C : Char;
   begin
      stop := False;
      sprompt(strings^.nuphone);
      P := 1;
      s := '';
      repeat
         getkey(c);
         case C of
            '0'..'9' : case P of
                  1,2,5,6,9,10,11,12: begin
                     prompt(C);
                     s := s + c;
                     Inc(P);
                  end;
                  3,7 : begin
                     prompt(C+'-');
                     s := s + c + '-';
                     Inc(P,2);
                  end;
            end;
            #13 : if P = 13 then stop := True;
            #8  : case P of
               5,9 : begin
                  prompt(^H+' '+^H^H+' '+^H);
                  delete(s,length(s)-1,2);
                  Dec(P,2);
               end;
               2,3,6,7,10,11,12,13 : begin
                  prompt(^H+' '+^H);
                  delete(s,length(s),1);
                  Dec(P);
               end;
            end;
         end;
      until (HangUp) or (stop);
      user.ph := s;
      nl;
      done1 := true;
   end;

   procedure dobday;
   var s,ts:astr;
       done:boolean;
       c:char;
   begin
      ts:='';
      sprompt(strings^.nubday);
      done:=FALSE;
      while (not done) and (not hangup) do Begin
         getkey(c);
         case c of
            '0'..'9':begin
	       if(length(ts)<6) then begin
	          ts:=ts+c;
	          prompt(c);
	          if ( (length(ts)=2) OR (length(ts)=4) ) then prompt('/');
	       end;
	    end;
            #27:done:=true;
            ^H:begin
	       if(length(ts)>0) then begin
	          ts:=copy(ts,1,length(ts)-1);
	          prompt(^H+' '+^H);
	          if((length(ts)=1) or (length(ts)=3)) then prompt(^H+' '+^H);
	       end;
	    end;
            ^M:if(length(ts)=6) then done:=TRUE;
         end;
      end;
      s:=copy(ts,1,2)+'/'+copy(ts,3,2)+'/'+copy(ts,5,2);
      if not (c=#27) then user.bday:=s;
      nl;
      done1:=true;
   end;


   procedure tog_ansi;
   var c:char;
   begin
      if ansi in user.ac then begin
         user.ac := user.ac -[ansi];
         print('Ansi Off');
      end
      else begin
         user.ac := user.ac + [ansi];
         print('Ansi On');
      end;
      done1:=TRUE;
   end;

   procedure tog_color;
   begin
      if (color in user.ac) then begin
         user.ac:=user.ac-[color];
         print('ANSI color disabled.');
      end
      else begin
         user.ac:=user.ac+[color];
         print('ANSI color activated.');
      end;
      done1:=TRUE;
   end;

   procedure tog_pause;
   begin
      if (pause in user.ac) then begin
         user.ac:=user.ac-[pause];
         print('No pause on screen.');
      end
      else begin
         user.ac:=user.ac+[pause];
         print('Pause on screen active.');
      end;
      done1:=TRUE;
   end;

   procedure tog_input;
   begin
      if (onekey in user.ac) then begin
         user.ac:=user.ac-[onekey];
         print('Full line input.');
      end
      else begin
         user.ac:=user.ac+[onekey];
         print('One key input.');
      end;
      done1:=TRUE;
    end;

   {procedure tog_avadj;
   begin
      if (user.avadjust=2) then begin
         user.avadjust:=1;
         print('Rip Graphics adjustment disabled.');
      end
      else begin
         user.avadjust:=2;
         print('Rip Graphics adjustment enabled.');
      end;
      done1:=TRUE;
   end;}

   procedure dounote;
   begin
      s:='';
      sprompt(strings^.enterunote);
      inputl(s,39);
      if (s='') then s:=strings^.newusernote;
      user.note:=s;
      done1:=true;
   end;

   procedure tog_expert;
   begin
      if (novice in user.ac) then begin
         user.ac:=user.ac-[novice];
         chelplevel:=1;
         print('Expert mode ON.');
      end
      else begin
         user.ac:=user.ac+[novice];
         chelplevel:=3;
         print('Expert mode OFF.');
      end;
      done1:=TRUE;
   end;

   procedure conffile;
   var c : char;
       y : byte;
      procedure writecheck(x:integer);
      var i : byte;
      begin
         if x = 0 then begin
            for i := 1 to 8 do begin
               goxy(38,7+i);
               sprompt('|08û');
            end;
         end
         else begin
            goxy(38,x);
            sprompt('|08û');
         end;
      end;

      procedure removecheck(x:integer);
      var i : byte;
      begin
         if x = 0 then begin
            for i := 1 to 8 do begin
               goxy(38,7+i);
               prompt(' ');
            end;
         end
         else begin
            goxy(38,x);
            prompt(' ');
         end;
      end;

   begin
      cls;
      writehdr('FileList Config');
      sprint('|U0File Listing |U1-|U0 Choose Your Setup');
      sprint('|U5ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ');
      sprint('|U01. File Name');
      sprint('|U02. File Size');
      sprint('|U03. File Points');
      sprint('|U04. File Description');
      sprint('|U05. Extended Description');
      sprint('|U06. Who Uploaded it');
      sprint('|U07. Date Uploaded');
      sprint('|U08. Number for Downloads');
      sprint('|U0N. None');
      sprint('|U0A. All');
      sprint('|U0Q. Quit');
      sprint('|U5ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ');
      y := whereY;
      if user.fflag=user.fflag+[fname] then writecheck(8);
      if user.fflag=user.fflag+[fsize] then writecheck(9);
      if user.fflag=user.fflag+[fpts] then writecheck(10);
      if user.fflag=user.fflag+[fdesc] then writecheck(11);
      if user.fflag=user.fflag+[extdesc] then writecheck(12);
      if user.fflag=user.fflag+[whoul] then writecheck(13);
      if user.fflag=user.fflag+[dateul] then writecheck(14);
      if user.fflag=user.fflag+[numdl] then writecheck(15);
      while (c<>'Q') or (not hangup) do begin
         gotoXY(1,y);
         sprompt('|U0Toggle Which|U9: |U3  '^H);
         onek(c,'12345678NAQ');
         case c of
            'Q': begin
               done1:=true;
               exit;
            end;
            '1': begin
               if (fname in user.fflag) then begin
                  user.fflag:=user.fflag-[fname];
                  removecheck(8);
               end
               else begin
                  user.fflag:=user.fflag+[fname];
                  writecheck(8)
               end;
            end;
            '2': begin
               if (fsize in user.fflag) then begin
                  user.fflag:=user.fflag-[fsize];
                  removeCheck(9);
               end
               else begin
                  user.fflag:=user.fflag+[fsize];
                  writeCheck(9);
               end;
            end;
            '3': begin
               if (fpts in user.fflag) then begin
                  user.fflag:=user.fflag-[fpts];
                  removeCheck(10);
               end
               else begin
                  user.fflag:=user.fflag+[fpts];
                  writeCheck(10);
               end;
            end;
            '4': begin
               if (fdesc in user.fflag) then begin
                  user.fflag:=user.fflag-[fdesc];
                  removecheck(11);
               end
               else begin
                  user.fflag:=user.fflag+[fdesc];
                  writecheck(11);
               end;
            end;
            '5': begin
               if (extDesc in user.fflag) then begin
                  user.fflag:=user.fflag-[extdesc];
                  removecheck(12);
               end
               else begin
                  user.fflag:=user.fflag+[extdesc];
                  writecheck(12);
               end;
            end;
            '6': begin
               if (whoUl in user.fflag) then begin
                  user.fflag:=user.fflag-[whoul];
                  removeCheck(13);
               end
               else begin
                  user.fflag:=user.fflag+[whoul];
                  writeCheck(13);
               end;
            end;
            '7': begin
               if (dateUl in user.fflag) then begin
                  user.fflag:=user.fflag-[dateul];
                  removeCheck(14);
               end
               else begin
                  user.fflag:=user.fflag+[dateul];
                  writeCheck(14);
               end;
            end;
            '8': begin
               if (numDl in user.fflag) then begin
                  user.fflag:=user.fflag-[numdl];
                  removeCheck(15);
               end
               else begin
                  user.fflag:=user.fflag+[numdl];
                  writeCheck(15);
               end;
            end;
            'N': begin
               with user do begin
                  if (fName in user.fflag) then fflag := fflag - [fname];
                  if (fSize in user.fflag) then fflag := fflag - [fsize];
                  if (fPts in user.fflag) then fflag := fflag - [fpts];
                  if (fDesc in user.fflag) then fflag := fflag - [fdesc];
                  if (extDesc in user.fflag) then fflag := fflag - [extdesc];
                  if (whoUl in user.fflag) then fflag := fflag - [whoul];
                  if (dateUl in user.fflag) then fflag := fflag - [dateUl];
                  if (numDl in user.fflag) then fflag := fflag - [numdl];
               end;
               removecheck(0);
            end;
            'A': begin
               with user do begin
                  if not (fName in user.fflag) then fflag := fflag + [fname];
                  if not (fSize in user.fflag) then fflag := fflag + [fsize];
                  if not (fPts in user.fflag) then fflag := fflag + [fpts];
                  if not (fDesc in user.fflag) then fflag := fflag + [fdesc];
                  if not (extDesc in user.fflag) then fflag := fflag + [extdesc];
                  if not (whoUl in user.fflag) then fflag := fflag + [whoul];
                  if not (dateUl in user.fflag) then fflag := fflag + [dateUl];
                  if not (numDl in user.fflag) then fflag := fflag + [numdl];
               end;
               writecheck(0);
            end;
         end;
      end;
      done1:=true;
   end;

   procedure chcolors;
   var c,c1:integer;
      ch:char;
      mcol,ocol:byte;
      ctyp,done:boolean;

      function colo(n:integer):astr;
      begin
         case n of
            0:colo:='Black';
            1:colo:='Blue';
            2:colo:='Green';
            3:colo:='Cyan';
            4:colo:='Red';
            5:colo:='Magenta';
            6:colo:='Yellow';
            7:colo:='White';
         end;
      end;

      function dt(n:integer):astr;
      begin
         s:=colo(n and 7)+' on '+colo((n shr 4) and 7);
         if (n and 8)<>0 then s:=s+', High Intensity';
         if (n and 128)<>0 then s:=s+', Blinking';
         dt:=s;
      end;

      function stf(n:integer):astr;
      begin
         case n of
            0:s:='Other';
            1:s:='Default';
            2:s:='Unused';
            3:s:='Yes/No';
            4:s:='Prompts';
            5:s:='Note';
            6:s:='Inverse';
            7:s:='Y/N question';
            8:s:='Blinking';
            9:s:='Other';
         end;
         stf:=cstr(n)+'. '+mln(s,20);
      end;

      procedure liststf;
      var c:integer;
      begin
         nl;
         for c:=0 to 9 do begin
            setc(7);
            prompt(stf(c));
            setc(user.cols[ctyp][c]);
            print(dt(user.cols[ctyp][c]));
         end;
      end;

   begin
      ctyp:=color in user.ac;
      setc(7);
      if (ctyp) then print('Set multiple colors.')
      else print('Set B&W colors.');
      ch:='?';
      done:=FALSE;
      while (not done) and (not hangup) do begin
         case ch of
            'Q':done:=TRUE;
            'L':liststf;
            '0'..'9':begin
               nl;
               setc(7);
               print('Current:');
               nl;
               c1:=value(ch);
               setc(7);
               prompt(stf(c1));
               setc(user.cols[ctyp][c1]);
               print(dt(user.cols[ctyp,c1]));
               nl;
               setc(7);
               print('Colors:');
               nl;
               for c:=0 to 7 do begin
                  setc(7);
                  prompt(cstr(c)+'. ');
                  setc(c);
                  prompt(mln(colo(c),12));
                  setc(7);
                  prompt(mrn(cstr(c+8),2)+'. ');
                  setc(c+8);
                  print(mln(colo(c)+'!',9));
               end;
               ocol:=user.cols[ctyp][c1];
               nl;
               prompt('|15F|07oreground: ');
               input(s,2);
               if (s='') then mcol:=ocol and 7
               else mcol:=value(s);
               prompt('Background: ');
               input(s,2);
               if (s='') then mcol:=mcol or (ocol and 112)
               else mcol:=mcol or (value(s) shl 4);
               if pynq('Blinking? ',false) then mcol:=mcol or 128;
               nl;
               setc(7);
               prompt(stf(c1));
               setc(mcol);
               print(dt(mcol));
               nl;
               if pynq('Is this correct? ',true) then user.cols[ctyp][c1]:=mcol;
            end;
         end;
         if (not done) then begin
            nl;
            prompt('Colors: (0-9) (L)ist (Q)uit :');
            onek(ch,'QL0123456789');
         end;
      end;
      done1:=TRUE;
   end;

   procedure checkwantpause;
   begin
      if pynq(strings^.nupause,true) then  user.ac:=user.ac+[pause]
      else user.ac:=user.ac-[pause];
      done1:=TRUE;
   end;

   procedure ww(www:integer);
   begin
      case www of
         1:doaddress;   2:dobday;       3:doansi;
         4:docitystate; 5:conffile;     6:dounote;
         7:doname;      8:dophone;      9:dopw;
         10:dorealname; 11:doscreen;    12:dosex;
         13:doautosig;  14:done1:=true; 15:mailbox;
         16:tog_ansi;   17:tog_color;   18:tog_pause;
         19:tog_input;  20:done1:=true; 21:chcolors;
         22:tog_expert; 23:done1:=true; 24:checkwantpause;
         {25:tog_avadj;}25:done1:=true;
         26: begin
            choosaprompt;
            done1:=true;
         end;
      end;
   end;

begin
   tryc:=0;
   cnt := 0;
   done1:=FALSE;
   case how of
      1: while (not Hangup) and (not Done1) do ww(which);
      2,3:begin
         ww(which);
         if not done1 then sprint(strings^.aborted);
      end;
   end;
   cstuff := done1;
end;

end.
