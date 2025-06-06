{$A+,B+,F+,I+,L+,O+,R-,S+,V-}
unit file4;

interface

uses
   Crt,      Dos,      common5,  file0,    file14,   records,  common;

function substall(src,old,new:astr):astr;
procedure abend(var abort,next:boolean; message:string);
procedure lfi(fn:astr; var abort,next:boolean);
procedure lfin(rn:integer; var abort,next:boolean);
procedure lfii;

implementation

uses
   zipviewu, lzhview;

var
  accum_csize:longint;    {* compressed size accumulator *}
  accum_usize:longint;    {* uncompressed size accumulator *}
  files:integer;          {* number of files *}
  level:integer;          {* output directory level *}
  filetype:integer;       {* file type (1=ARC,2=ZIP,3=ZOO,4=LZH) *}
  aborted:boolean;

function substall(src,old,new:astr):astr;
var p:integer;
begin
   p:=1;
   while p>0 do begin
      p:=pos(old,src);
      if p>0 then begin
         insert(new,src,p+length(old));
         delete(src,p,length(old));
      end;
   end;
   substall:=src;
end;

function mnz(l:longint; w:integer):astr;
var s:astr;
begin
   s:=cstrl(l);
   while length(s) < w do s:='0'+s;
   mnz:=s;
end;

function mnr(l:longint; w:integer):astr;
begin
   mnr:=mrn(cstrl(l),w);
end;

{*------------------------------------------------------------------------*}

procedure abend(var abort,next:boolean; message:string);
begin
   nl;
   sprompt(#3#7+'** '+#3#5+message+#3#7+' **');
   nl;
   aborted:=TRUE;
   abort:=TRUE;
   next:=TRUE;
end;

procedure lfi(fn:astr; var abort,next:boolean);
var t:text;
    i:integer;
begin
   fn:=sqoutsp(fn);
   if (pos('.ZIP',fn)<>0) then zipview(fn)
   else if (pos('.LZH',fn)<>0) then lzhv(fn)
   else if (pos('.ARJ',fn)<>0) then begin
      exec(getenv('Comspec'),'/C arj v '+fn+' > '+status^.temppath+'tmpview.txt');
      printfile(status^.temppath+'tmpview.txt');
   end
   else sprompt('Unsupported Archive Format');
end;

procedure lfin(rn:integer; var abort,next:boolean);
var f:ulfrec;
begin
   seek(ulff,rn);
   read(ulff,f);
   lfi(memuboard.dlpath+f.filename,abort,next);
end;

procedure lfii;
var f:ulfrec;
    fn:astr;
    pl,rn:integer;
    abort,next,lastarc,lastgif,isgif:boolean;
begin
   nl;
   fn:='';
   sprint(strings^.viewline);
   nl;
   gfn(fn);
   abort:=FALSE;
   next:=FALSE;
   nl;
   if (fn='*.*') or (fn='') then exit;
   recno(fn,pl,rn);
   if (baddlpath) then exit;
   lastarc:=fALSE;
   lastgif:=FALSE;
   while ((rn<>0) and (not abort)) do begin
      seek(ulff,rn);
      read(ulff,f);
      isgif:=isgifext(f.filename);
      if (isgif) then begin
         lastarc:=FALSE;
         if (not lastgif) then begin
            lastgif:=TRUE;
            nl;
            nl;
            printacr(#3#0+'Filename.Ext '+sepr2+' Resolution '+sepr2+
                     ' Num Colors '+sepr2+' Signat.',abort,next);
            printacr(#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴�',abort,next);
         end;
         dogifspecs(sqoutsp(memuboard.dlpath+f.filename),abort,next);
      end
      else begin
         lastgif:=FALSE;
         if (not lastarc) then begin
            lastarc:=TRUE;
            nl;
         end;
         lfin(rn,abort,next);
      end;
      nrecno(fn,pl,rn);
      if (next) then abort:=FALSE;
      next:=FALSE;
   end;
   {$I-} close(ulff); {$I+}
end;

end.
