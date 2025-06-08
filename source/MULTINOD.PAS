{$A+,B+,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit multinod;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  records,
   common,   timejunk, strproc,  common5;

procedure makenode(stat:byte);
procedure makenodeon(name:astr; num:integer);
procedure whoson;
procedure sendnodemessage(node:byte; message:astr);
procedure readnodemessages;
procedure multichat(forum:byte);
procedure savenode;
procedure enternodemessage;
procedure entermultichat;

implementation

function pad(topad:astr; len:integer):astr;
var ps:astr;
begin
   if(length(topad)<len) then
      ps:=topad+copy('                                                                            ',1,len-length(topad))
   else ps:=copy(topad,1,len);
   pad:=ps;
end;

function choosenode:byte;
var i:integer;
    node:noderec;
    gotvalnum:boolean;
begin
   assign(multinodef,status^.gFilePath+'nodes.dat');
   reset(multinodef);
   nl;
   sprint(#3#4+'                                                         ');
   seek(multinodef,0);
   for i:=1 to filesize(multinodef) do begin
      read(multinodef,node);
      sprint(#3#4+' '+#3#0+pad(cstr(i),3)+#3#4+'ù'+#3#0+pad(node.desc,20)+#3#4+
             'ù'+#3#0+pad(node.username+' #'+cstr(node.usernum),30)+#3#4+' ');
   end;
   sprint(#3#4+'                                                         ');
   gotvalnum:=FALSE;
   while(not gotvalnum) and (not hangup) do begin
      nl;
      prompt(#3#4+'Node number: ');
      inu(i);
      if(i>0) and (i<=filesize(multinodef)) then begin
         gotvalnum:=TRUE;
         choosenode:=i;
      end
      else begin
         nl;
         sprint('Enter a number from 1-'+cstr(filesize(multinodef))+'.');
      end;
   end;
   close(multinodef);
end;

procedure makenode(stat:byte);
begin
   assign(multinodef,status^.gFilePath+'nodes.dat');
   reset(multinodef);
   nodeinfo.status:=stat;
   seek(multinodef,thisnode-1);
   {$I-}
   repeat
      write(multinodef,nodeinfo);
   until ioresult = 0;
   {$I+}
   close(multinodef);
end;

procedure makenodeon(name:astr; num:integer);
begin
   assign(multinodef,status^.gFilePath+'nodes.dat');
   reset(multinodef);
   nodeinfo.username:=name;
   nodeinfo.usernum:=num;
   nodeinfo.nodespd:=spd;
   seek(multinodef,thisnode-1);
   {$I-}
   repeat
      write(multinodef,nodeinfo);
   until (ioresult=0);
   {$I+}
   close(multinodef);
end;

procedure whoson;
var i:integer;
    node:noderec;
    s:astr;
begin
   assign(multinodef,status^.gFilePath+'nodes.dat');
   reset(multinodef);
   nl;
   sprint(#3#0+'Node                 Speed   User                    Status');
   sprint(#3#5+'ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ ÄÄÄÄÄÄÄ ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ');
   for i:=0 to filesize(multinodef)-1 do begin
      seek(multinodef,i);  {(was 0) Node 1 }
      read(multinodef,node);
      case node.status of
         0:s:='Waiting for Caller';
         1:s:='Offline';
         2:s:='Logging in';
         3:s:='Menuing';
         4:s:='Uploading';
         5:s:='Downloading';
         6:s:='Entering a message';
         7:s:='Reading a message';
         11..20:s:='Chat forum #'+cstr(node.status-10);
      end;
      sprint(#3#0+pad(node.desc,20)+' '+#3#2+pad(spd,7)+' '+#3#1+pad(caps(node.username)+' #'+cstr(node.usernum),23)
             +' '+#3#0+s);
   end;
   nl;
   close(multinodef);
end;

procedure sendnodemessage(node:byte; message:astr);
var nmf:file of nodemessrec;
    at:integer;
    nm:nodemessrec;
begin
   assign(nmf,status^.gFilePath+'nodemess.dat');
   {$I-} reset(nmf); {$I+}
   at:=0;
   if(ioresult<>0) then rewrite(nmf)
   else begin
      read(nmf,nm);
      while(not eof(nmf)) and (not nm.deleted) do begin
         inc(at);
         read(nmf,nm);
      end;
      if(not nm.deleted) then inc(at);
   end;
   seek(nmf,at);
   with nm do begin
      mess:=message;
      deleted:=FALSE;
      nodeto:=node;
      nodefrom:=thisnode;
      namefrom:=thisuser.name;
      numfrom:=usernum;
   end;
   {$I-}
   repeat
      write(nmf,nm);
   until ioresult=0;
   {$I+}
   close(nmf);
end;

procedure readnodemessages;
var nmf:file of nodemessrec;
    nm:nodemessrec;
begin
   assign(nmf,status^.gFilePath+'nodemess.dat');
   {$I-} reset(nmf); {$I+}
   if(ioresult=0) then begin
      while(not eof(nmf)) do begin
         read(nmf,nm);
         if(not nm.deleted) and (nm.nodeto=thisnode) then begin
            nl;
            sprint('|U5'+nm.namefrom+' #'+cstr(nm.numfrom)+'|U4 on node #|U3'+cstr(nm.nodefrom)+'|U5 says,');
            sprint('|U4"|U3'+nm.mess+'|U4"');
            nm.deleted:=TRUE;
            seek(nmf,filepos(nmf)-1);
            write(nmf,nm);
         end;
      end;
      close(nmf);
   end;
end;

procedure multichat(forum:byte);
var forumf:file of forumrec;
    jaf:file of byte;
    ourfile,theirfile:text;
    theirnames:array[1..25] of astr;
    theirflags:array[1..25] of boolean;
    theirat:array[1..25] of longint;
    theirnodes:array[1..25] of integer;
    jafat:longint;
    numtheir,i,i1,good,jafr,sp:byte;
    ourforum,theirforum:forumrec;
    said,leftover,garb:astr;
    done,donesaid:boolean;
    fdt,sdt:ldatetimerec;
    c:char;
begin
   cls;
   sprint('Entering forum #'+cstr(forum)+'...');
   sprint('/Q = Quit  -  /W = Who''s on  -  /S = Send node message');
   nl;
   makenode(forum+10);
   assign(ourfile,'multicht.'+cstr(thisnode));
   rewrite(ourfile);
   close(ourfile);
   append(ourfile);

   assign(forumf,'forum'+cstr(forum)+'.gen');
   {$I-} reset(forumf); {$I+}
   if(ioresult<>0) then begin
      rewrite(forumf);
      close(forumf);
      reset(forumf);
   end;

   if(filesize(forumf)>=25) then begin
      sprint(#3#4+'þ '+#3#1+'Forum full!');
      close(ourfile);
      close(forumf);
      exit;
   end;

   good:=255;
   if(filesize(forumf)>0) then begin
      for i:=0 to filesize(forumf)-1 do begin
         if(good=255) then begin
            seek(forumf,i);
            read(forumf,theirforum);
            if(theirforum.deleted) then good:=i;
         end;
      end;
   end;
   if(good=255) then good:=filesize(forumf);

   ourforum.user:=thisuser.name;
   ourforum.deleted:=FALSE;
   ourforum.node:=thisnode;

   seek(forumf,good);
   write(forumf,ourforum);
   close(forumf);
   reset(forumf);

   assign(jaf,'forum'+cstr(forum)+'.jaf');
   {$I-} reset(jaf); {$I+}
   if(ioresult<>0) then begin
      rewrite(jaf);
      close(jaf);
      reset(jaf);
   end;
   seek(jaf,filesize(jaf));
   i:=1;
   write(jaf,i);
   write(jaf,good);
   jafat:=filesize(jaf);
   close(jaf);

   for i:=0 to 25 do begin
      theirflags[i]:=FALSE;
   end;
   for i:=0 to filesize(forumf)-1 do begin
      read(forumf,theirforum);
      if(not theirforum.deleted) and (theirforum.node<>thisnode) then begin
         theirflags[i]:=TRUE;
         theirnames[i]:=theirforum.user;
         theirnodes[i]:=theirforum.node;
         assign(theirfile,'multicht.'+cstr(theirforum.node));
         reset(theirfile);
         theirat[i]:=0;
         while(not eof(theirfile)) do begin
            readln(theirfile,garb);
            inc(theirat[i]);
         end;
         close(theirfile);
      end;
   end;
   close(forumf);

   done:=FALSE;
   leftover:='';

   while(not done) and (not hangup) do begin
      readnodemessages;
      {$I-}reset(jaf);
      seek(jaf,jafat);
      while(not eof(jaf)) do begin
         jafat:=jafat+2;
         read(jaf,jafr);
         reset(forumf);
         if(jafr=0) then begin
            read(jaf,jafr);
            sprint('|U4þ |U1'+theirnames[jafr]+' left.');
            theirflags[jafr]:=FALSE;
         end
         else begin
            read(jaf,jafr);
            seek(forumf,jafr);
            read(forumf,theirforum);
            sprint('|U4þ |U1'++theirforum.user+' joined the forum.');
            theirflags[jafr]:=TRUE;
            theirnames[jafr]:=theirforum.user;
            theirnodes[jafr]:=theirforum.node;
            assign(theirfile,'multicht.'+cstr(theirforum.node));
            reset(theirfile);
            theirat[jafr]:=0;
            while(not eof(theirfile)) do begin
               inc(theirat[jafr]);
               readln(theirfile,garb);
            end;
            close(theirfile);
         end;
         close(forumf);
      end;
      close(jaf);{$I+}
      for i:=1 to 25 do begin
         if(theirflags[i]) then begin
            {$I-}assign(theirfile,'multicht.'+cstr(theirnodes[i]));
            reset(theirfile);
            for i1:=1 to theirat[i] do readln(theirfile,garb);
            while(not eof(theirfile)) do begin
               readln(theirfile,said);
               sprint(#3#3+theirnames[i]+#3#0+'> '+#3#4+said);
               inc(theirat[i]);
            end;
            close(theirfile);{$I+}
         end;
      end;

      if(leftover='') then begin
         c:=#0;
         if(keypressed) then c:=inkey
         else if(incom) then c:=cinkey;
         if(c in [' '..'~']) then leftover:=c;
      end;

      if(leftover='/') then begin
         leftover:='';
         sprompt(#3#3+'Multinode Chat Command'+#3#0+'> '+#3#4);
         getkey(c);
         c:=upcase(c);
         case c of
            'Q':begin
               sprint('Quit');
               done:=TRUE;
            end;
            'W':begin
               sprint('Who''s on');
               whoson;
            end;
            'S':begin
               sprint('Send node message');
               enternodemessage;
            end;
            else prompt(expand(^H+' '+^H,80));
         end;
      end;

    if(leftover<>'') then begin
      sprompt('|U3'+thisuser.name+'|U0> |U4'+leftover);
      said:=leftover;
      donesaid:=FALSE;
      repeat
        getkey(c);
        case c of
          ' '..'~':begin
                     if(length(said)>75-length(thisuser.name)) then begin
                       donesaid:=TRUE;
                       sp:=0;
                       for i:=1 to length(said) do begin
                         if(said[i]=' ') then sp:=i;
                       end;
                       if(sp<>0) then begin
                         leftover:=copy(said,sp+1,length(said)-sp)+c;
                         said:=copy(said,1,sp-1);
                         for i:=1 to length(leftover) do begin
                           sprompt(^H+' '+^H);
                         end;
                       end else begin
                         leftover:='';
                         said:=said+c;
                         sprompt(c);
                       end;
                     end else begin
                       sprompt(c);
                       said:=said+c;
                     end;
                   end;
          ^H:begin
               if(length(said)>0) then begin
                 sprompt(^H+' '+^H);
                 said:=copy(said,1,length(said)-1);
               end;
             end;
          ^M:begin
               donesaid:=TRUE;
               leftover:='';
             end;
        end;
      until(donesaid) or (hangup);
      nl;
      {$I-}
      repeat
      writeln(ourfile,said);
      until ioresult=0;
      {$I+}
      close(ourfile);
      append(ourfile);
    end;
  end;

  ourforum.deleted:=TRUE;
  reset(forumf);
  seek(forumf,good);
  write(forumf,ourforum);
  close(forumf);
  i:=0;
  reset(jaf);
  seek(jaf,jafat);
  write(jaf,i);
  write(jaf,good);
  close(jaf);

  close(ourfile);

  cls;
  sprint(#3#4+'þ '+#3#1+'Forum abandoned');
end;

procedure savenode;
begin
   reset(multinodef);
   seek(multinodef,thisnode-1);
   {$I-}
   repeat
      write(multinodef,nodeinfo);
   until ioresult=0;
   {$I+}
   close(multinodef);
end;

procedure enternodemessage;
var node,i:integer;
    mess:astr;
begin
   if (thisuser.sl=255) AND (pYnQ('Send message to all nodes?',false)) then node:=1001
   else node:=choosenode;
   nl;
   sprint('Enter message to send');
   prompt('þ ');
   inputl(mess,75);
   if(node<>1001) then sendnodemessage(node,mess)
   else begin
      assign(multinodef,status^.gFilePath+'nodes.dat');
      reset(multinodef);
      for i:=1 to filesize(multinodef) do begin
         sendnodemessage(i,mess);
      end;
      close(multinodef);
   end;
end;

procedure entermultichat;
var i:integer;
begin
   sprint('Chat forum to enter?  (1-10)');
   prompt('þ ');
   inu(i);
   if(i>0) and (i<11) then multichat(i)
   else sprint(#3#3+'Aborted.');
end;

end.

