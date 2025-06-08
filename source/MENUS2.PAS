(* Menuing System - 2 *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit menus2;

interface

uses
   Crt,      Dos,      common5,  records,  file4,    Common1,  Common2,
   Common3,  common,   strproc;

procedure readin;
procedure showcmds(listtype:integer);
function oksecurity(i:integer; var cmdnothid:boolean):boolean;
{procedure genericmenu(t:integer);}
procedure showthismenu;
procedure dopulls(var inp : string);

var gotcommand:boolean;

implementation

uses menus, menus3;

procedure dopulls(var inp : string);
var cc,c:char;
    newcmd,s:string;
    ckey,rep,ex,x,numberofchoices:integer;
    pp,qq,choice,tries,zz,xx,yy,i:integer;
    execnum:array [1..50] of integer;
    done:boolean;

begin
  done:=false;
  newcmd:='';
  if menur.directive <> '' then printf(menur.directive);
  ex:=1;
  thisuser.ac:=thisuser.ac+[ansi];
  while (not done) and (not hangup) do begin
    tries:=0;
    xx:=1;
    yy:=1;
    pp:=1;
    qq:=1;
    ex:=1;
    for i:=1 to noc do begin
      if (pull in cmdr[i].commandflags) then
      begin
        execnum[ex]:=i;
        inc(ex);
      end;
    end; {for i:=1 to noc}
    numberofchoices:=ex-1;
    choice:=1;

    for rep:=1 to numberofchoices do begin
      xx := value(cmdr[execnum[rep]].xpoint);
      if xx = 0 then xx := wherex;
      yy := value(cmdr[execnum[rep]].ypoint);
      if yy = 0 then yy := wherey;
      goxy(xx, yy);
      sprompt(cmdr[execnum[rep]].ldesc);
    end; {rep:=1 to numberofchoices}

    xx := value(cmdr[execnum[1]].xpoint);
    if xx = 0 then xx := wherex;
    yy := value(cmdr[execnum[1]].ypoint);
    if yy = 0 then yy := wherey;
    goxy(xx, yy);
    sprompt(cmdr[execnum[1]].sdesc);

    while (not done) and (not hangup) do
    begin
      getkey(c);
      c:=upcase(c);
      if (c in [^[,'[',']',^M, ^N]) then
      begin
      if c <> ^M then
        begin
        getkey(c);
        c := upcase(c);
        end;
      if (c in ['[','A','B','C','D','P','H','K','M',^M,^N]) then
      begin
      if c = '[' then
        getkey(c);

      c := upcase(c);

      if (filemnu) and (c in ['B','A']) then
        begin
        inp := '['+c;
        c := 'Z';
        done := true;
        end;

      if ((c='K') or (c='A') or (c='D')) then begin   {Cursor Up}  { ESC[A is cursor up }
        inp := c;
        xx:=value(cmdr[execnum[choice]].xpoint);
        if xx = 0 then xx := wherex;
        yy:=value(cmdr[execnum[choice]].ypoint);
        if yy = 0 then yy := wherey;
        goxy(xx,yy);
        sprompt(cmdr[execnum[choice]].ldesc);                            { terms send for up  }
        dec(choice);
        if choice < 1 then choice := numberofchoices;
        xx:=value(cmdr[execnum[choice]].xpoint);
        if xx = 0 then xx := wherex;
        yy:=value(cmdr[execnum[choice]].ypoint);
        if yy = 0 then yy := wherey;
        goxy(xx,yy);
        sprompt(cmdr[execnum[choice]].sdesc);
      end; {c='A', c='D'}

      if ((c='M') or (c='B') or (c='C')) then begin   {Cursor Down}
        xx:=value(cmdr[execnum[choice]].xpoint);
        if xx = 0 then xx := wherex;
        yy:=value(cmdr[execnum[choice]].ypoint);
        if yy = 0 then yy := wherey;
        goxy(xx,yy);
        sprompt(cmdr[execnum[choice]].ldesc);
        inc(choice);
        if choice > numberofchoices then choice := 1;
        xx:=value(cmdr[execnum[choice]].xpoint);
        if xx = 0 then xx := wherex;
        yy:=value(cmdr[execnum[choice]].ypoint);
        if yy = 0 then yy := wherey;
        goxy(xx,yy);
        sprompt(cmdr[execnum[choice]].sdesc);
      end; {c='B', c='C'}

      if c=#13 then begin
        {goxy(1,23);}
        inp := ^M;
        nl;
        gotcommand:=true;
        done:=true;
        cc:=cmdr[execnum[choice]].ckeys[1];
        for ckey:=1 to noc do begin
          if (cmdr[ckey].ckeys[1]=cc) then
          begin
            done:=true;
            newcmd:='';
            domenucommand(done,cmdr[ckey].cmdkeys+cmdr[ckey].mstring,newcmd);
          end;
        end;  {ckey:=1 to noc}
      end; {c=#13}
    end;
    end
  else {if (c in [#27,'[',']','C','D','A','B',^M,^N]) then}
  begin
    inp := ^M;
    for ckey:=1 to noc do
    begin
      if (cmdr[ckey].ckeys[1]=c) then
      begin
        done:=true;
        setc(7);
        domenucommand(done,cmdr[ckey].cmdkeys+cmdr[ckey].mstring,newcmd);
      end;
    end; {ckey=1 to noc}
  end; {else begin}
 end; {while (not done) and (not hangup) and (not getout) do}
 end; {while (not done) and (not hangup) do begin}
end; {begin}

procedure readin;
var infile        : file;
    s             ,
    lcmdlistentry : string;
    i             ,
    j             : byte;
    d             : menurec;
    newNoc        : byte;
begin
   cmdlist:='';
   noc:=0;
   assign(infile,curmenu);
   {$I-} reset(infile,1); {$I-}
   if (ioresult<>0) then begin
      assign(infile,curmenu);
      {$I-} reset(infile,1); {$I-}
      if (ioresult<>0) then begin
         sysoplog('"'+curmenu+'" is MISSING.');
         print('"'+curmenu+'" is MISSING.  Please inform SysOp.');
         print('Dropping back to fallback menu...');
         curmenu:=status^.menupath+menur.fallback+'.mnu';
         if (not exist(curmenu)) then curmenu := last_menu;
         assign(infile,curmenu);
         {$I-} reset(infile,1); {$I-}
         if (ioresult<>0) then begin
            sysoplog('"'+curmenu+'" is MISSING - Hung user up.');
            sprint('Fallback menu is *also* MISSING.  Please inform SysOp.');
            sprint('Critical error.');
            curmenu := last_menu;
         end;
      end;
   end;
   if (not hangup) then begin
      blockread(infile,menur,sizeof(menur));
      repeat
         inc(noc);
         blockread(infile,cmdr[noc],sizeof(cmdr[noc]));
      until eof(infile);
      mqarea:=FALSE;
      fqarea:=FALSE;
      lcmdlistentry:='';
      j:=0;
      for i:=1 to noc do begin
         if (cmdr[i].ckeys<>lcmdlistentry) then begin
            if (aacs(cmdr[i].acs)) then begin
               inc(j);
               if ((cmdr[i].ckeys<>'FIRSTCMD') and (cmdr[i].ckeys<>'GTITLE')) then begin
                  if (j<>1) then cmdlist:=cmdlist+',';
                  cmdlist:=cmdlist+cmdr[i].ckeys;
               end
               else dec(j); { if not firstcmd }
            end; { if b is true }
            lcmdlistentry:=+cmdr[i].ckeys;
         end; { if command key hasn't been used }
         if (cmdr[i].cmdkeys='M#') then mqarea:=TRUE;
         if (cmdr[i].cmdkeys='F#') then fqarea:=TRUE;
      end; { for i = 1 to total command }
      close(infile);
   end; { if not hangup }
   if not(pulldown in menur.menuflags) then begin
      lcmdlistentry := '';
      newNoc := noc;
      assign(infile,status^.menupath + 'global.mnu');
      {$I-} reset(infile,1); {$I+}
      if ((ioResult = 0) AND (not hangup)) then begin
         if (fileSize(infile) < sizeOf(menur) + sizeOf(cmdr[1])) then begin
            close(infile);
            exit;
         end;
         blockread(infile,d,sizeof(d));
         repeat
            inc(noc);
            blockread(infile,cmdr[noc],sizeof(cmdr[noc]));
         until eof(infile);
         j := 1;
         for i:=newNoc to noc do begin
            if (cmdr[i].ckeys<>lcmdlistentry) then begin
               if (aacs(cmdr[i].acs)) then begin
                  inc(j);
                  if ((cmdr[i].ckeys<>'FIRSTCMD') and (cmdr[i].ckeys<>'GTITLE')) then begin
                     if (j<>1) then cmdlist:=cmdlist+',';
                     cmdlist:=cmdlist+cmdr[i].ckeys;
                  end
                  else dec(j); { if not firstcmd }
               end; { if b is true }
               lcmdlistentry:=cmdr[i].ckeys;
            end; { if command key hasn't been used }
            if (cmdr[i].cmdkeys='M#') then mqarea:=TRUE;
            if (cmdr[i].cmdkeys='F#') then fqarea:=TRUE;
         end; { for i = 1 to total command }
         close(infile);
      end; { if not hangup }
   end;
end;

procedure showcmds(listtype:integer);
var i,j,numrows:integer;
    s,s1:astr;
    abort,next:boolean;

  function type1(i:integer):astr;
  begin
    type1:=mn(i,3)+mlnnomci(cmdr[i].ckeys,3)+mlnnomci(cmdr[i].cmdkeys,4)+
           mlnnomci(cmdr[i].mstring,15);
  end;

  function sfl(b:boolean; c:char):char;
  begin
    if (b) then sfl:=c else sfl:='-';
  end;

begin
  abort:=FALSE; next:=FALSE;
  if (noc<>0) then begin
    case listtype of
      0:begin
          printacr(#3#0+'# '+sepr2+'Command       '+sepr2+'Fl'+sepr2+
                   'ACS      '+sepr2+'Cmd'+sepr2+'MString',abort,next);
          printacr(#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴컴',abort,next);
          i:=1;
          while (i<=noc) and (not abort) and (not hangup) do begin
            print{acr}({#3#0+}mn(i,2)+' '+{#3#3+}mlnnomci(cmdr[i].ckeys,14)+' '+
                     sfl(hidden in cmdr[i].commandflags,'H')+
                     sfl(unhidden in cmdr[i].commandflags,'U')+' '+
                     {#3#9+}mlnnomci(cmdr[i].acs,10)+' '+
                     {#3#3+}mlnnomci(cmdr[i].cmdkeys,2)+' '+
                     cmdr[i].mstring);{,abort,next);}
            inc(i);
          end;
        end;
      1:begin
          numrows:=(noc+2) div 3;
          i:=1;
          s:=#3#0+'# '+sepr2+'KK-Typ-MString        ';
          s1:=#3#9+'컴컴컴컴컴컴컴컴컴컴컴컴컴';
          while (i<=numrows) and (i<3) do begin
            s:=s+' # '+sepr2+'KK-Typ-MString        ';
            s1:=s1+'컴컴컴컴컴컴컴컴컴컴컴컴컴';
            inc(i);
          end;
          printacr(s,abort,next);
          printacr(s1,abort,next);
          i:=0;
          repeat
            inc(i);
            s:=type1(i);
            for j:=1 to 2 do
              if i+(j*numrows)<=noc then
                s:=s+' '+type1(i+(j*numrows));
            print{acr}({#3#1+}s{,abort,next});
          until ((i>=numrows) or (abort) or (hangup));
        end;
    end;
  end
  else print('- No Commands on this menu -');
end;

function oksecurity(i:integer; var cmdnothid:boolean):boolean;
begin
   oksecurity:=FALSE;
   if (unhidden in cmdr[i].commandflags) then cmdnothid:=TRUE;
   if (not aacs(cmdr[i].acs)) then exit;
   oksecurity:=TRUE;
end;

{type
  bstr = string[160];

procedure genericmenu(t:integer);
var glin:array [1..maxmenucmds] of bstr;
    s,s1:bstr;
    gcolors:array [1..3] of byte;
    onlin,i,j,colsiz,numcols,numglin,maxright:integer;
    abort,next,b,cmdnothid:boolean;

  function gencolored(keys,desc:bstr; acc:boolean):bstr;
  begin
    s:=desc;
    j:=pos(strAllCaps(keys),strAllCaps(desc));
    if (j<>0) and (pos(#3,desc)=0) then begin
      insert(#3+chr(gcolors[3]),desc,j+length(keys)+1);
      insert(#3+chr(gcolors[1]),desc,j+length(keys));
      if (acc) then insert(#3+chr(gcolors[2]),desc,j);
      if (j<>1) then
        insert(#3+chr(gcolors[1]),desc,j-1);
    end;
    gencolored:=#3+chr(gcolors[3])+desc;
  end;

  function semicmd(s:string; x:integer):string;
  var i,p:integer;
  begin
    i:=1;
    while (i<x) and (s<>'') do begin
      p:=pos(';',s);
      if (p<>0) then s:=copy(s,p+1,length(s)-p) else s:='';
      inc(i);
    end;
    while (pos(';',s)<>0) do s:=copy(s,1,pos(';',s)-1);
    semicmd:=s;
  end;

  procedure newgcolors(s:string);
  var s1:string;
  begin
    s1:=semicmd(s,1); if (s1<>'') then gcolors[1]:=value(s1);
    s1:=semicmd(s,2); if (s1<>'') then gcolors[2]:=value(s1);
    s1:=semicmd(s,3); if (s1<>'') then gcolors[3]:=value(s1);
  end;

  procedure gen_tuto;
  var i,j:integer;
      b:boolean;
  begin
    numglin:=0; maxright:=0; glin[1]:='';
    for i:=1 to noc do begin
      b:=oksecurity(i,cmdnothid);
      if (((b) or (unhidden in cmdr[i].commandflags)) and
          (not (hidden in cmdr[i].commandflags))) then
        if (cmdr[i].ckeys='GTITLE') then begin
          inc(numglin); glin[numglin]:=cmdr[i].ldesc;
          j:=lenn(glin[numglin]); if (j>maxright) then maxright:=j;
          if (cmdr[i].mstring<>'') then newgcolors(cmdr[i].mstring);
        end else
          if (cmdr[i].ldesc<>'') then begin
            inc(numglin);
            glin[numglin]:=gencolored(cmdr[i].ckeys,cmdr[i].ldesc,b);
            j:=lenn(glin[numglin]); if (j>maxright) then maxright:=j;
          end;
    end;
  end;

  procedure stripc(var s1:bstr);
  var s:bstr;
      i:integer;
  begin
    s:=''; i:=1;
    while (i<=length(s1)) do begin
      if (s1[i]=#3) then inc(i) else s:=s+s1[i];
      inc(i);
    end;
    s1:=s;
  end;

  procedure fixit(var s:bstr; len:integer);
  var s1:bstr;
  begin
    s1:=s;
    stripc(s1);
    if (length(s1)<len) then
      s:=s+copy('                                        ',1,len-length(s1))
    else
      if (length(s1)>len) then s:=s1;
  end;

  procedure gen_norm;
  var s1:bstr;
      i,j:integer;
      b:boolean;
  begin
    s1:=''; onlin:=0; numglin:=1; maxright:=0; glin[1]:='';
    for i:=1 to noc do begin
      b:=oksecurity(i,cmdnothid);
      if (((b) or (unhidden in cmdr[i].commandflags)) and
          (not (hidden in cmdr[i].commandflags))) then begin
        if (cmdr[i].ckeys='GTITLE') then begin
          if (onlin<>0) then inc(numglin);
          glin[numglin]:=#2+cmdr[i].ldesc;
          inc(numglin); glin[numglin]:='';
          onlin:=0;
          if (cmdr[i].mstring<>'') then newgcolors(cmdr[i].mstring);
        end else begin
          if (cmdr[i].sdesc<>'') then begin
            inc(onlin); s1:=gencolored(cmdr[i].ckeys,cmdr[i].sdesc,b);
            if (onlin<>numcols) then fixit(s1,colsiz);
            glin[numglin]:=glin[numglin]+s1;
          end;
          if (onlin=numcols) then begin
            j:=lenn(glin[numglin]); if (j>maxright) then maxright:=j;
            inc(numglin); glin[numglin]:=''; onlin:=0;
          end;
        end;
      end;
    end;
    if (onlin=0) then dec(numglin);
  end;

  function tcentered(c:integer; s:bstr):bstr;
  const spacestr='                                               ';
  begin
    c:=(c div 2)-(lenn(s) div 2);
    if (c<1) then c:=0;
    tcentered:=copy(spacestr,1,c)+s;
  end;

  procedure dotitles;
  var i:integer;
      b:boolean;
  begin
    b:=FALSE;
    if (clrscrbefore in menur.menuflags) then begin
      cls;
    end;
    for i:=1 to 3 do
      if (menur.menuname[i]<>'') then begin
        if (not b) then begin nl; b:=TRUE; end;
        if (dontcenter in menur.menuflags) then
          printacr(menur.menuname[i],abort,next)
        else
          printacr(tcentered(maxright,menur.menuname[i]),abort,next);
      end;
  end;

begin
if (not (pulldown in menur.menuflags)) then begin
  for i:=1 to 3 do gcolors[i]:=menur.gcol[i];
  numcols:=menur.gencols;
  case numcols of
    2:colsiz:=39; 3:colsiz:=25; 4:colsiz:=19;
    5:colsiz:=16; 6:colsiz:=12; 7:colsiz:=11;
  end;
  if (numcols*colsiz>=thisuser.linelen) then
    numcols:=thisuser.linelen div colsiz;
  abort:=FALSE; next:=FALSE;
  if (t=2) then gen_norm else gen_tuto;
  dotitles;
  for i:=1 to numglin do
    if (glin[i]<>'') then
      if (glin[i][1]<>#2) then
        printacr(glin[i],abort,next)
      else
        printacr(tcentered(maxright,copy(glin[i],2,length(glin[i])-1)),
                 abort,next);
  end;
end;}

procedure showthismenu;
{var s:bstr;}
begin
   if (cHelpLevel = 3) then begin
      printF(menuR.directive);
      if not(novice in thisUser.ac) then cHelpLevel := 1;
   end;
{if (not (pulldown in menur.menuflags)) then begin
  case chelplevel of
    2:begin
        nofile:=TRUE; s:=menur.directive;
        if (s<>'') then begin
          if (pos('@S',s)<>0) then
            printf(substall(s,'@S',cstr(thisuser.sl)));
          if (nofile) then printf(substall(s,'@S',''));
        end;
      end;
    3:begin
        nofile:=TRUE; s:=menur.tutorial;
        if (s<>'') then begin
          if (pos('.',s)=0) then s:=s+'.tut';
          if (pos('@S',s)<>0) then
            printf(substall(s,'@S',cstr(thisuser.sl)));
          if (nofile) then printf(substall(s,'@S',''));
        end;
      end;
  end;
  if ((nofile) and (chelplevel in [2,3])) then genericmenu(chelplevel);
 end;}
end;

end.
