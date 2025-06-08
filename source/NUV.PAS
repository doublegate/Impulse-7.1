(* New User Voting *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-,T+}
unit Nuv;
Interface
procedure deletecomments(commptr:integer);
procedure readcomments;
procedure newUserVote;

Implementation
uses
   records,  common,   crt,      common5,  sysop3,   common3,  sys,
   misc4,    common2,  menus,    menus2;

var thisnuv : nuvrec;

procedure deletecomments(commptr:integer);
var cf:file of nuvcomrec;
    thiscomm:nuvcomrec;
begin
   if not exist(status^.gFilePath+'nuvcmnt.dat') then exit;
   assign(cf,status^.gfilepath+'nuvcmnt.dat');
   reset(cf);
   while(commptr<>0) do begin
      seek(cf,commptr);
      read(cf,thiscomm);
      thiscomm.exists:=FALSE;
      seek(cf,commptr);
      write(cf,thiscomm);
      commptr:=thiscomm.nextcomment;
   end;
   close(cf);
end;

procedure readcomments;
var cf:file of nuvcomrec;
    thiscomm:nuvcomrec;
    x:byte;
begin
   assign(cf,status^.gfilepath+'nuvcmnt.dat');
   reset(cf);
   x := whereX;
   while(thisNuv.commentptr<>0) do begin
      seek(cf,thisNuv.commentptr);
      read(cf,thiscomm);
      sprint('|U1'+caps(thiscomm.user)+'|U9: |U2'+thiscomm.comm);
      goxy(x,wherey);
      thisNuv.commentptr:=thiscomm.nextcomment;
   end;
   close(cf);
end;

procedure newUserVote;
var nuvfile:file of nuvrec;
    i,max,nuf:integer;
    s:astr;
    done:boolean;
    vote:char;
    u:userrec;

   function checkvote:boolean;
   var i1:integer;
      checkstat:boolean;
   begin
      checkstat:=FALSE;
      for i1:=1 to thisnuv.totvoted do if ((thisnuv.votedby[i1]=usernum) AND (thisNuv.exists)) then checkstat:=TRUE;
      checkvote:=checkstat;
   end;

   function nuvreal:integer;
   var sofar:integer;
   begin
      seek(nuvfile,0);
      sofar:=0;
      while(not eof(nuvfile)) do begin
         read(nuvfile,thisnuv);
         if (thisnuv.exists) then inc(sofar);
      end;
      nuvreal:=sofar;
   end;

   procedure valuser;
   begin
      cls;
      sprint(strings^.nuvvalidated);
      reset(uf);
      seek(uf,thisnuv.unum);
      read(uf,u);
      autovalidate(u,thisnuv.unum);
      seek(uf,thisnuv.unum);
      thisnuv.exists:=FALSE;
      write(uf,u);
      close(uf);
      if(thisnuv.commentptr<>0) then deletecomments(thisnuv.commentptr);
   end;

   procedure deluser;
   begin
      cls;
      sprint(strings^.nuvdeleted);
      nukeuser(thisnuv.handle, thisnuv.unum);
      thisnuv.exists := false;
    end;

   procedure chkforcmnt;
   var cf:file of nuvcomrec;
       thiscomm:nuvcomrec;
       newcommptr,at,commat,lastat:longint;
       done:boolean;
   begin
      cls;
      if(pynq('Would you like to add a comment? ',false)) then begin
         assign(cf,status^.gfilepath+'nuvcmnt.dat');
         {$I-} reset(cf); {$I+}
         if(ioresult<>0) then begin
	    rewrite(cf);
	    thiscomm.comm:='Dummy comment';
	    thiscomm.user:='NOBODY';
	    thiscomm.exists:=TRUE;
	    thiscomm.nextcomment:=0;
            write(cf,thiscomm);
	    reset(cf);
         end;
         newcommptr:=filesize(cf);
         at:=0;
         done:=FALSE;
         while(not eof(cf)) and (not done) do begin
	    read(cf,thiscomm);
	    if(not thiscomm.exists) then begin
	       done:=TRUE;
               newcommptr:=at;
	    end;
	    inc(at);
         end;
         thiscomm.user:=thisuser.name;
         thiscomm.nextcomment:=0;
         thiscomm.exists:=TRUE;
         nl;
         sprint('Enter your comment on this user');
         sprompt('|U2þ |U3');
         inputl(thiscomm.comm,77);
         if(thiscomm.comm<>'') then begin
	    seek(cf,newcommptr);
	    write(cf,thiscomm);
	    if(thisnuv.commentptr<>0) then begin
	       commat:=thisnuv.commentptr;
	       repeat
	          lastat:=commat;
	          seek(cf,commat);
	          read(cf,thiscomm);
	          commat:=thiscomm.nextcomment;
	       until(commat=0);
	       thiscomm.nextcomment:=newcommptr;
	       seek(cf,lastat);
	       write(cf,thiscomm);
	    end
            else thisnuv.commentptr:=newcommptr;
         end;
         close(cf);
      end;
   end;
var nuvp : string;
    didNuvP : boolean;
begin
   didNuvP := false;
   if (thisuser.sl<=status^.newsl) then exit;
   assign(nuvfile,status^.gfilepath+'nuv.dat');
   {$I-} reset(nuvfile); {$I+}
   if(ioresult<>0) then rewrite(nuvfile);
   max:=nuvreal;
   if(max=0) then begin
      writehdr('New User Voting');
      sprint('No users to vote on.');
   end
   else begin
      done:=FALSE;
      seek(nuvfile,0);
      for i:=1 to max do begin
         nuf:=0;
         read(nuvfile,thisnuv);
         if (not checkvote) then begin
            sprint(caps(thisnuv.handle));
            seek(nuvfile,filepos(nuvfile)-1);
            sysMciSet('NM',caps(thisnuv.handle));
            sysMciSet('YV',cstr(thisnuv.yes));
            sysMciSet('NV',cstr(thisnuv.no));
            sysMciSet('VN',cstr(status^.nuvno));
            sysMciSet('VY',cstr(status^.nuvyes));
            sysMciSet('CM','');
            mciList[numMcis]^.isAction := true;
            mciList[numMcis]^.action := readComments;
            readasw(thisnuv.unum,'newuser');
            printf('nuvote');
            sysResetMciList;
{            printnuv('nuvote');}
{            sprompt(strings^.nuvote);}
            if (pos('MATRIX',curmenu) <> 0) then last_menu := status^.menupath+status^.allStartMenu
            else last_menu := curmenu;
            curmenu := status^.menupath+'NUVP.MNU';
            didNuvP := true;
            readin;
            nuvp := '';
            mainMenuHandle(nuvp);
            {        if (fso) then onek(vote,'YNAVD')
            else onek(vote,'YNA');}
	    case nuvpmnu of
	       'Y':begin
	          inc(thisnuv.totvoted);
		  thisnuv.votedby[thisnuv.totvoted]:=usernum;
		  inc(thisnuv.yes);
		  if(thisnuv.yes>=status^.nuvyes) then valuser
		  else chkforcmnt;
	       end;
	       'N':begin
	          inc(thisnuv.totvoted);
		  thisnuv.votedby[thisnuv.totvoted]:=usernum;
		  inc(thisnuv.no);
		  if (thisnuv.no>=status^.nuvno) then deluser
                  else chkforcmnt;
	       end;
	       'V':if (fso) then valuser;
	       'D':if (fso) then deluser;
	    end;
	    write(nuvfile,thisnuv);  { Write changes to disk }
	 end;
      end;
      close(nuvfile);
      reset(nuvfile);
      max:=nuvreal;  { Recalculate # of valid users to vote on }
      if (didNuvP) then begin
        curmenu := last_menu;
        readin;
      end;
   end;
   close(nuvfile);
end;
end.