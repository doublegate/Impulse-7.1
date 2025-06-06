(* impDos - online dos emulator *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit impDos;

interface

uses
   Crt,      Dos,      common5,  records,  common,    file3,    common1,
   common2,  common3,  file0,    file1,    file2,    file4,    file8,
   file9,    file11,   execbat,  islu,     menus3,   cmd,strproc;

procedure miniDos;

implementation

uses ulcheck;
var xword:array[1..9] of astr;

procedure parse(s:astr);
var i,j,k:integer;
begin
   for i:=1 to 9 do xword[i]:='';
   i:=1;
   j:=1;
   k:=1;
   if (length(s)=1) then xword[1]:=s;
   while (i<length(s)) do begin
      inc(i);
      if ((s[i]=' ') or (length(s)=i)) then begin
         if (length(s)=i) then inc(i);
         xword[k]:=copy(s,j,(i-j));
         j:=i+1;
         inc(k);
      end;
   end;
end;

procedure minidos;
var curdir,s,s1:astr;
    abort,next,done,restr,nocmd,nospace:boolean;

   procedure versionInfo;
   begin
      sprint('|NLImpDOS(R) Version '+ver+'|NLCopyrighted by Tumyeto Software|NL');
   end;

   procedure docmd(cmd:astr);
   var fi:file of byte;
       f:file;
       ps,ns,es,op,np:astr;
       s1,s2,s3:astr;
       numfiles,tsiz:longint;
       retlevel,i,j:integer;
       b,ok,wasrestr:boolean;

      function restr1:boolean;
      begin
         restr1:=restr;
         if (restr) then wasrestr:=TRUE;
      end;

   begin
      wasrestr:=FALSE;
      abort:=FALSE;
      next:=FALSE;
      nocmd:=FALSE;
      for i:=1 to 9 do xword[i]:=strAllCaps(xword[i]);
      s:=xword[1];
      if ((pos('\',xword[2])<>0) or (pos('..',xword[2])<>0)) and (restr) then exit;
      if (s='DIR/W') then s:='DIR /W'
      else if (s='?') or (s='HELP') then printf('minidos')
      else if (s='EXIT') or (s='QUIT') then done:=TRUE
      else if ((s='DEL') or (s='DELETE')) and (not restr1) then begin
         if ((not exist(xword[2])) and (not iswildcard(xword[2]))) or (xword[2]='') then print('File not found.')
         else begin
            xword[2]:=fexpand(xword[2]);
            ffile(xword[2]);
            repeat
               if not ((dirinfo.attr and VolumeID=VolumeID) or (dirinfo.attr and Directory=Directory)) then begin
                  assign(f,dirinfo.name);
                  {$I-} erase(f); {$I+}
                  if (ioresult<>0) then print('"'+dirinfo.name+'": Could not delete!');
               end;
               nfile;
            until (not found) or (hangup);
         end;
      end
      else if (s='MEM') then begin
         cmdShowStackFree;
         cmdShowHeapFree;
      end
      else if (s='TYPE') then begin
         printfile(fexpand(xword[2]));
         if (nofile) then print('File not found.');
      end
      else if (s='SHOW') then begin
         showscr(fexpand(xword[2]));
         if (nofile) then print('File not found.');
      end
      else if (s='LADLE') then cmdGetSauce(xword[2],xword[3],xword[4],true)
      else if (s='ISLC') then islc(fexpand(xword[2]))
      else if (s='ISLD') then isld(fexpand(xword[2]))
      else if ((s='REN') or (s='RENAME')) then begin
         if ((not exist(xword[2])) and (xword[2]<>'')) then print('File not found.')
         else begin
            xword[2]:=fexpand(xword[2]);
            assign(f,xword[2]);
            {$I-} rename(f,xword[3]); {$I+}
            if (ioresult<>0) then print('File not found.');
         end;
      end
      else if (s='DIR') then begin
         b:=TRUE;
         for i:=2 to 9 do if (xword[i]='/W') then begin
            b:=FALSE;
            xword[i]:='';
         end;
         if (xword[2]='') then xword[2]:='*.*';
         s1:=curdir;
         xword[2]:=fexpand(xword[2]);
         fsplit(xword[2],ps,ns,es);
         s1:=ps;
         s2:=ns+es;
         if (s2='') then s2:='*.*';
         if (not iswildcard(xword[2])) then begin
            ffile(xword[2]);
            if ((found) and (dirinfo.attr=directory)) or ((length(s1)=3) and (s1[3]='\')) then begin   {* root directory *}
               s1:=bslash(TRUE,xword[2]);
               s2:='*.*';
            end;
         end;
         nl;
         dir(s1,s2,b);
         nl;
      end
      else if ((s='CD') or (s='CHDIR')) and (xword[2]<>'') and (not restr1) then begin
         xword[2]:=fexpand(xword[2]);
         {$I-} chdir(xword[2]); {$I+}
         if (ioresult<>0) then print('Invalid pathname.');
      end
      else if ((s='MD') or (s='MKDIR')) and (xword[2]<>'') and (not restr1) then begin
         {$I-} mkdir(xword[2]); {$I+}
         if (ioresult<>0) then print('Unable to create directory.');
      end
      else if ((s='RD') or (s='RMDIR')) and (xword[2]<>'') and (not restr1) then begin
         {$I-} rmdir(xword[2]); {$I+}
         if (ioresult<>0) then sprint('Invalid path, not directory,|NLor directory not empty');
      end
      else if (s='COPY') and (not restr1) then begin
         if (xword[2]<>'') then begin
            if (iswildcard(xword[3])) then print('Wildcards not allowed in destination parameter!')
         else begin
            if (xword[3]='') then xword[3]:=curdir;
            xword[2]:=bslash(FALSE,fexpand(xword[2]));
            xword[3]:=fexpand(xword[3]);
            ffile(xword[3]);
            b:=((found) and (dirinfo.attr and directory=directory));
            if ((not b) and (copy(xword[3],2,2)=':\') and (length(xword[3])=3)) then b:=TRUE;
            fsplit(xword[2],op,ns,es);
            op:=bslash(TRUE,op);
            if (b) then np:=bslash(TRUE,xword[3])
            else begin
               fsplit(xword[3],np,ns,es);
               np:=bslash(TRUE,np);
            end;
            j:=0;
            abort:=FALSE;
            next:=FALSE;
            ffile(xword[2]);
            while (found) and (not abort) and (not hangup) do begin
               if (not ((dirinfo.attr=directory) or (dirinfo.attr=volumeid))) then begin
                  s1:=op+dirinfo.name;
                  if (b) then s2:=np+dirinfo.name else s2:=np+ns+es;
                  prompt(s1+' -> '+s2+' :');
                  copyfile(ok,nospace,TRUE,s1,s2);
                  if (ok) then begin
                     inc(j);
                     nl;
                  end
                  else if (nospace) then sprompt(#3#7+' - *Insufficient space*')
                  else sprompt(#3#7+' - *Copy failed*');
                  nl;
               end;
               if (not empty) then wkey(abort,next);
               nfile;
            end;
            if (j<>0) then begin
               prompt('  '+cstr(j)+' file');
               if (j<>1) then prompt('s');
               print(' copied.');
            end;
         end;
      end;
      end
      else if (s='MOVE') and (not restr1) then begin
      if (xword[2]<>'') then begin
        if (iswildcard(xword[3])) then
          print('Wildcards not allowed in destination parameter!')
        else begin
          if (xword[3]='') then xword[3]:=curdir;
          xword[2]:=bslash(FALSE,fexpand(xword[2]));
          xword[3]:=fexpand(xword[3]);
          ffile(xword[3]);
          b:=((found) and (dirinfo.attr and directory=directory));
          if ((not b) and (copy(xword[3],2,2)=':\') and
              (length(xword[3])=3)) then b:=TRUE;

          fsplit(xword[2],op,ns,es);
          op:=bslash(TRUE,op);

          if (b) then
            np:=bslash(TRUE,xword[3])
          else begin
            fsplit(xword[3],np,ns,es);
            np:=bslash(TRUE,np);
          end;

          j:=0;
          abort:=FALSE; next:=FALSE;
          ffile(xword[2]);
          while (found) and (not abort) and (not hangup) do begin
            if (not ((dirinfo.attr=directory) or (dirinfo.attr=volumeid))) then
            begin
              s1:=op+dirinfo.name;
              if (b) then s2:=np+dirinfo.name else s2:=np+ns+es;
              prompt(s1+' -> '+s2+' :');
              movefile(ok,nospace,TRUE,s1,s2);
              if (ok) then begin
                inc(j);
                nl;
              end else
                if (nospace) then sprompt(#3#7+' - *Insufficient space*')
                else sprompt(#3#7+' - *Move failed*');
              nl;
            end;
            if (not empty) then wkey(abort,next);
            nfile;
          end;
          if (j<>0) then begin
            prompt('  '+cstr(j)+' file');
            if (j<>1) then prompt('s');
            print(' moved.');
          end;
        end;
      end;
    end
    else
    if (s='CLS') then cls
    else
    if (length(s)=2) and (s[1]>='A') and (s[1]<='Z') and
       (s[2]=':') and (not restr1) then begin
      {$I-} getdir(ord(s[1])-64,s1); {$I+}
      if (ioresult<>0) then print('Invalid drive.')
      else begin
        {$I-} chdir(s1); {$I+}
        if (ioresult<>0) then begin
          print('Invalid drive.');
          chdir(curdir);
        end;
      end;
    end
    else
    if (s='VIEW') then begin
      if (xword[2]='') then begin
        nl;
        print('Syntax is:   "IFL filename"');
        nl;
      end else begin
        s1:=xword[2];
        if (pos('.',s1)=0) then s1:=s1+'*.*';
        lfi(s1,abort,next);
      end;
    end
    else
    if (s='SEND') and (xword[2]<>'') then begin
      if exist(xword[2]) then unlisted_download(fexpand(xword[2]))
        else print('File not found.');
    end
    else
    if (s='VER') then versioninfo
    else if (s='FORMAT') then begin
       sprint('|NL'+strings^.hack+'|NL');
       hangup:=true;
    end
    else if (s='DIRSIZE') then begin
       nl;
      if (xword[2]='') then print('Needs a parameter.')
      else begin
        numfiles:=0; tsiz:=0;
        ffile(xword[2]);
        while (found) do begin
          inc(tsiz,dirinfo.size);
          inc(numfiles);
          nfile;
        end;
        if (numfiles=0) then print('No files found!')
          else print('"'+strAllCaps(xword[2])+'": '+cstrl(numfiles)+' files, '+
                     cstrl(tsiz)+' bytes.');
      end;
      nl;
    end
    else
    if (s='DISKFREE') then begin
      if (xword[2]='') then j:=exdrv(curdir) else j:=exdrv(xword[2]);
      nl;
      print(cstrl(freek(j)*1024)+' bytes free on '+chr(j+64)+':');
      nl;
    end
    else
    if (s='EXT') and (not restr1) then begin
      s1:=cmd;
      j:=pos('EXT',strAllCaps(s1))+3; s1:=copy(s1,j,length(s1)-(j-1));
      while (copy(s1,1,1)=' ') do s1:=copy(s1,2,length(s1)-1);
      if ((incom) or (outcom)) then
        s1:=s1+' >'+status^.remdevice+' < CON';
      if (length(s1)>127) then begin nl; print('Command too long!'); nl; end
      else
        shelldos(TRUE,s1,retlevel);
    end else begin
      nocmd:=TRUE;
      if (s<>'') then
    {    if (not wasrestr) then} print('Unknown Command "'+s+'"');
{        else print('Restricted command.');}
    end;
  end;

begin
   chdir(bslash(FALSE,status^.afilepath));
   restr:=(not cso);
   done:=FALSE;
   sprint('|NL|07Type "EXIT" to return to '+ProgName+'.|NL');
   versioninfo;
   repeat
      getdir(0,curdir);
      prompt(lowcase(curdir)+'>'); inputl(s1,128); parse(s1);
      docmd(s1);
      if (not nocmd) then sysoplog('> '+s1);
   until (done) or (hangup);
   chdir(start_dir);
end;
end.