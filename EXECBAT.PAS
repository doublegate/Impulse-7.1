{$A+,B+,F+,I+,L+,O+,R-,S+,V-}
unit execbat;

interface

uses
   Crt,      Dos,      common5,  records,  common,   common1,  common2,
   common3,  myio;

var
  wind:windowrec;
  sx,sy:integer;
  wascls,savtw:boolean;
  savcurwind:integer;

procedure execbatch(var ok:boolean; showit:boolean;
                    bfn,tfn,dir,batline:astr; oklevel:integer);
procedure pexecbatch(showit:boolean; bfn,tfn,dir,batline:astr;
                     var retlevel:integer);
procedure shel(s:astr);
procedure shel2;

implementation

procedure execbatch(var ok:boolean;     { result                     }
                    showit:boolean;     { show working on user side  }
                    bfn:astr;           { .BAT filename              }
                    tfn:astr;           { temporary testing file     }
                    dir:astr;           { directory takes place in   }
                    batline:astr;       { .BAT file line to execute  }
                    oklevel:integer);   { DOS errorlevel for success }
var bfp:text;
    odir,todev:astr;
    i,rcode:integer;
begin
   todev:=' ';
   if ((showit) and (incom)) then todev:=' > '+status^.remdevice+' < con '
   else if (wantout) then todev:=''; {' >con';}
   getdir(0,odir);
   dir:=fexpand(dir);
   while copy(dir,length(dir),1)='\' do dir:=copy(dir,1,length(dir)-1);
   assign(bfp,bfn);
   rewrite(bfp);
   writeln(bfp,'echo off');
   writeln(bfp,chr(exdrv(dir)+64)+':');
   writeln(bfp,'cd '+dir);
   writeln(bfp,batline+todev);
   writeln(bfp,':done');
   writeln(bfp,chr(exdrv(odir)+64)+':');
   writeln(bfp,'cd '+odir);
   writeln(bfp,'exit');
   close(bfp);
   if (wantout) then begin
      textAttr := 30;
      clrEol;
      writeLn(batLine);
      textAttr := 7;
   end;
   shelldos(FALSE,bfn+todev,rcode);
   chdir(odir);
   {$I-} erase(bfp); {$I+}
   if (oklevel<>-1) then ok:=(rcode=oklevel)
   else ok:=TRUE;
end;

procedure pexecbatch(showit:boolean;     { show working on user side  }
                     bfn:astr;           { .BAT filename              }
                     tfn:astr;           { UNUSED -----------         }
                     dir:astr;           { directory takes place in   }
                     batline:astr;       { .BAT file line to execute  }
                 var retlevel:integer);  { DOS errorlevel returned    }
var tfp,bfp:text;
    odir,todev:astr;
begin
   {todev:=' > nul';}
   if (showit) and (incom) then todev:=' > '+status^.remdevice+' < con '
   else if (wantout) then todev:=' >con';
   getdir(0,odir);
   dir:=fexpand(dir);
   while copy(dir,length(dir),1)='\' do dir:=copy(dir,1,length(dir)-1);
   assign(bfp,bfn);
   rewrite(bfp);
   writeln(bfp,'echo off');
   writeln(bfp,chr(exdrv(dir)+64)+':');
   writeln(bfp,'cd '+dir);
   writeln(bfp,batline+todev);
   writeln(bfp,':done');
   writeln(bfp,chr(exdrv(odir)+64)+':');
   writeln(bfp,'cd '+odir);
   writeln(bfp,'exit');
   close(bfp);

   if (wantout) then begin
      textColor(15);
      textbackground(1);
      clreol;
      write(batline);
      clreol;
      textColor(7);
      textbackground(0);
      writeln;
   end;
   if (todev=' >con') then todev:=''
   else todev:=' >nul';
   shelldos(FALSE,bfn+todev,retlevel);
   chdir(odir);
   {$I-} erase(bfp); {$I+}
end;

procedure shel(s:astr);
begin
   wascls:=FALSE;
   savcurwind:=status^.curwindow;
   if (s<>'') then begin
      wascls:=TRUE;
      sx:=wherex;
      sy:=wherey;
      setwindow(wind,1,1,80,25,7,0,0);
      cls;
      textbackground(1);
      textColor(15);
      clreol;
      write(s);
      textbackground(0);
      textcolor(7);
      writeln;
   end
   else if (savcurwind<>0) then sclearwindow;
   {if (not status^.istopwindow) then sclearwindow;}
end;

procedure shel2;
begin
   if (wascls) then begin
      cls;
      removewindow(wind);
      gotoxy(sx,sy);
      topscr;
   end
   else if (savcurwind<>0) then schangewindow(TRUE,savcurwind);
end;

end.