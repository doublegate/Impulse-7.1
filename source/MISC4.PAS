{ Misc Stuff 4 }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit misc4;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  records,  doors,
   misc3,    common5,  common,   strproc;

procedure readq(filen:astr; infolevel:integer);
procedure readasw(usern:integer; fn:astr);
procedure readasw1(fn:astr);

implementation

procedure readq(filen:astr; infolevel:integer);
var infile,outfile,outfile1:text;
    ps,ns,es,infilename,outfilename:astr;
    level0name,
    s : string;
    resp:string[79];
    len:string[2];
    time,x,y,i:integer;
    outp:char;

   procedure dumptofile;
   begin
      reset(outfile1);
      {$I-} append(outfile); {$I+}
      if (ioresult<>0) then rewrite(outfile);
      while (not eof(outfile1)) do begin
         readln(outfile1,s);
         writeln(outfile,s);
      end;
      close(outfile1);
      close(outfile);
      erase(outfile1);
   end;

begin
   level0name := '';
   infilename:=filen;
   if (not exist(infilename)) then begin
      fsplit(infilename,ps,ns,es);
      infilename:=ps+ns+'.INF';
      if (not exist(infilename)) then begin
         infilename:=status^.afilepath+ns+'.INF';
         if (not exist(infilename)) then begin
            sysoplog('þ InfoForm not found: "'+filen);
            print('þ InfoForm not found: "'+filen);
            exit;
         end;
      end;
   end;

   assign(infile,infilename);
   {$I-} reset(infile); {$I+}
   if (ioresult<>0) then begin
      sysoplog('þ InfoForm not found: "'+filen+'"');
      print('þ InfoForm not found: "'+filen+'"');
      exit;
   end;

   fsplit(infilename,ps,ns,es);
   outfilename:=status^.afilepath+ns+'.ASW';

   assign(outfile1,status^.afilepath+'TEMP$'+cstr(infolevel)+'.ASW');
   if (infolevel=0) then begin
      level0name:=outfilename;
      assign(outfile,outfilename);
      sysoplog('Ä Answered InfoForm "'+filen+'"');
      rewrite(outfile1);
      writeln(outfile1,'User: '+nam);
      writeln(outfile1,'Date: '+dat);
      writeln(outfile1);
   end
   else begin
      sysoplog('þ Answered InfoForm "'+filen+'"');
      rewrite(outfile1);
      assign(outfile,level0name);
   end;

   printingfile:=TRUE;
   reset(infile);

   while (not eof(infile)) do begin
      read(infile,outp);
      time:=0;
      if outp='%' then begin
         time:=1;
         len:='';
         read(infile,outp);
         len:=outp;
         read(infile,outp);
         len:=len+outp;
         x:=wherex;
         y:=wherey;
         inputl(resp,value(len));
         write(outfile1,resp);
         goxy(x+1,y);
      end;
      if outp='*' then begin
         x:=wherex;
         y:=wherey;
         inputl(resp,79);
         write(outfile1,resp);
         goxy(x+1,y);
      end;
      if ((outp<>'*') and (outp<>'%')) and (time<=0) then begin
         dosansi(outp);
         if (outcom) then pr1(outp);
         write(outfile1,outp);
      end;
   end;

   if (hangup) then begin
      writeln(outfile1);
      writeln(outfile1,'þ HUNG UP þ');
   end;

   close(outfile1);
   dumptofile;
   close(infile);
   printingfile:=FALSE;
end;

procedure readasw(usern:integer; fn:astr);
var qf:text;
    user:userrec;
    ps,ns,es:astr;
    qs:string[255];
    i,userntimes:integer;
    abort,next,userfound,usernfound,ufo:boolean;

   procedure exactmatch;
   begin
      reset(qf);
      repeat
         readln(qf,qs);
         if (copy(qs,1,6)='User: ') then begin
            i:=value(copy(qs,pos('#',qs)+1,length(qs)-pos('#',qs)));
            if (i=usern) then begin
               inc(userntimes);
               usernfound:=TRUE;
               if (strAllCaps(qs)=strAllCaps('User: '+user.name+' #'+cstr(usern))) then userfound:=TRUE;
            end;
         end;
         if (not empty) then wkey(abort,next);
      until (eof(qf)) or (userfound) or (abort);
   end;

   procedure usernmatch;
   begin
      sprompt(#3#7+'No exact user name matches; user number was found ');
      if (userntimes=1) then sprompt('once.')
      else sprompt(cstr(userntimes)+' times.');
      nl;
      reset(qf);
      repeat
         readln(qf,qs);
         if (copy(qs,1,6)='User: ') then begin
            i:=value(copy(qs,pos('#',qs)+1,length(qs)-pos('#',qs)));
            if (i=usern) then
               if (userntimes=1) then userfound:=TRUE
               else begin
                  sprompt(#3#4+'User: '+#3#3+copy(qs,7,length(qs)-6));
                  userfound:=pynq('  -- Is this right?',false);
               end;
         end;
         if (not empty) then wkey(abort,next);
      until (eof(qf)) or (userfound) or (abort);
      nl;
   end;

begin
   ufo:=(filerec(uf).mode<>fmclosed);
   if (not ufo) then reset(uf);
   if ((usern>=1) and (usern<=filesize(uf)-1)) then begin
      seek(uf,usern);
      read(uf,user);
   end
   else begin
      print('Invalid user number: '+cstr(usern));
      exit;
   end;
   if (not ufo) then close(uf);
   nl;
   abort:=FALSE;
   next:=FALSE;
   fn:=strAllCaps(fn);
   fsplit(fn,ps,ns,es);
   fn:=strAllCaps(status^.afilepath+ns+'.ASW');
   if (not exist(fn)) then begin
      fn:=strAllCaps(status^.gfilepath+ns+'.ASW');
      if (not exist(fn)) then begin
         print('InfoForm answer file not found: "'+fn+'"');
         exit;
      end;
   end;
   assign(qf,fn);
   {$I-} reset(qf); {$I+}
   if (ioresult<>0) then print('"'+fn+'": unable to open.')
   else begin
      userfound:=FALSE;
      usernfound:=FALSE;
      userntimes:=0;
      exactmatch;
      if (not userfound) and (usernfound) and (not abort) then usernmatch;
      if (not userfound) and (not abort) then print('Questionairre answers not found.')
      else begin
         sprint(qs); (*(#3#4+'User: '+#3#3+caps(user.name)+' #'+cstr(usern));*)
         repeat
            readln(qf,qs);
            if (copy(qs,1,6)<>'User: ') then begin {printacr(qs,abort,next)}
               for i:=1 to length(qs) do dosansi(qs[i]);
               if outcom then pr1(qs);
            end
            else userfound:=FALSE;
         until eof(qf) or (not userfound) or (abort);
      end;
      close(qf);
   end;
end;

procedure readasw1(fn:astr);
var ps,ns,es:astr;
    usern:integer;
begin
   nl;
   print('Read InfoForm answers -');
   nl;
   if (fn='') then begin
      prompt('Enter filename: ');
      input(fn,8);
      nl;
      if (fn='') then exit;
   end;
   fsplit(fn,ps,ns,es);
   fn:=strAllCaps(status^.gfilepath+ns+'.ASW');
   if (not exist(fn)) then begin
      fn:=strAllCaps(status^.afilepath+ns+'.ASW');
      if (not exist(fn)) then begin
         print('InfoForm answer file not found: "'+fn+'"');
         exit;
      end;
   end;
   print('Enter user number, user name, or partial search string:');
   finduserws(usern);
   if (usern<>0) then readasw(usern,fn)
   else begin
      nl;
      if pynq('List entire answer file? ',false) then begin
         nl;
         printf(ns+'.ASW');
      end;
   end;
end;

end.
