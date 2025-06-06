{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit file2;

interface

uses
   Crt,   Dos,  records,
  execbat, file0,
  common,common5,file4;

procedure copyfile(var ok,nospace:boolean; showprog:boolean;
                   srcname,destname:astr);
procedure movefile(var ok,nospace:boolean; showprog:boolean;
                   srcname,destname:astr);

implementation

procedure copyfile(var ok,nospace:boolean; showprog:boolean;srcname,destname:astr);
var buffer: ^ansiBufType;
    fs,dfs:longint;
    nrec,i:integer;
    src,dest:file;

   procedure dodate;
   var r:registers;
       od,ot,ha:integer;
   begin
      srcname:=srcname+#0;
      destname:=destname+#0;
      with r do begin
         ax:=$3d00; ds:=seg(srcname[1]); dx:=ofs(srcname[1]); msdos(dos.registers(r));
         ha:=ax; bx:=ha; ax:=$5700; msdos(dos.registers(r));
         od:=dx; ot:=cx; bx:=ha; ax:=$3e00; msdos(dos.registers(r));
         ax:=$3d02; ds:=seg(destname[1]); dx:=ofs(destname[1]); msdos(dos.registers(r));
         ha:=ax; bx:=ha; ax:=$5701; cx:=ot; dx:=od; msdos(dos.registers(r));
         ax:=$3e00; bx:=ha; msdos(dos.registers(r));
      end;
   end;

begin
   ok:=TRUE;
   nospace:=FALSE;
   assign(src,srcname);
   {$I-} reset(src,1); {$I+}
   if (ioresult<>0) then begin
      ok:=FALSE;
      exit;
   end;
   dfs:=freek(exdrv(destname));
   fs:=trunc(filesize(src)/1024.0)+1;
   if (fs>=dfs) then begin
      close(src);
      nospace:=TRUE;
      ok:=FALSE;
      exit;
   end
   else begin
      assign(dest,destname);
      {$I-} rewrite(dest,1); {$I+}
      if (ioresult<>0) then begin
         ok:=FALSE;
         exit;
      end;
      if (showprog) then begin
         cl(7);
         prompt(expand('.',(fs div 16)));
         prompt(expand(^H,(fs div 16)));
         cl(5);
      end;
      new(buffer);
      repeat
         blockread(src,buffer^,maxBufSize,nrec);
         blockwrite(dest,buffer^,nrec);
         if (showprog) then prompt('o');
      until (nrec<maxBufSize);
      disPose(buffer);
      if (showprog) then begin
         prompt(expand(^H,(fs div 16)+1));
         prompt(expand(' ',(fs div 16)+1));
         prompt(expand(^H,(fs div 16)+1));
         sprompt('^7-^5 DONE ^7-');
      end;
      close(dest);
      close(src);
      dodate;
   end;
end;

procedure movline(var src:astr; s1,s2:astr);
begin
   src:=substall(src,'@F',s1);
   src:=substall(src,'@I',s2);
end;

procedure movefile(var ok,nospace:boolean; showprog:boolean;
                   srcname,destname:astr);
var f:file;
    opath:astr;
begin
   ok:=TRUE;
   nospace:=FALSE;
   getdir(0,opath);
   copyfile(ok,nospace,showprog,srcname,destname);
   if (ok) then begin
      {$I-} erase(f); {$I+}
   end;
   chdir(opath);
end;

end.
