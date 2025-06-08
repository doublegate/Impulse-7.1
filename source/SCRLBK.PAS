{$G+,O+}
unit SCRLBK;

interface

uses  Crt,  myio,common;

type
  scrlptrtype=^scroll;
  scrlattrpttype=^scroll;
  scroll=array[1..819,1..80] of byte;  {80 characters}

var
  scrlpt:scrlptrtype;             {The actual scroll Text buffer}
  scrlatpt:scrlattrpttype;        {The Scroll Attributes Buffer}
  curscrlline:integer;            {Current line in the buffer}
  topscrline:integer;             {Line In Buffer At Top Of Screen}
  amtscrllines:integer;           {Amount OF Scrollines}
  bottombuffer,topbuffer:integer; {Top and bottom of the buffer}
  myscrolly:byte;
  okscrollback:boolean;
  beforeview:boolean;

procedure initializescrlback;     {Callled At Logon, along with getmems}
Procedure viewscrollback;
procedure addtoscroll(CurrentY:Byte);

implementation

uses common5, common2;

var
  row,column:byte;

procedure initializescrlback;
{Copies The First 25Lines Of The Text And Colors To The Arrays}
var
  i,j:longint;
begin
  fillchar(scrlatpt^, sizeOf(scroll), 0);
  fillchar(scrlpt^, sizeOf(scroll), 0);
  for i:=1 to 819 do
    for j:=1 to 80 do begin
      scrlpt^[i][j]:=0;
      scrlatpt^[i][j]:=0;
    end;
  for row:=1 to 24 do
    for column:=1 to 80 do begin
      scrlpt^[row][column]:=Mem[$B800 : 160*(Row-1)+2*(Column-1)];
      scrlatpt^[row][column]:=Mem[$B800 : 160*(Row-1)+2*(Column-1)+1];
    end;
  curscrlline:=24;
  amtscrllines:=24;
  topbuffer:=1;
  bottombuffer:=24;
  topscrline:=1;
end;

procedure myview;
var
  i,b:integer;
begin
  i:=0;
  b:=curscrlline;
  for row:=24 downto 1 do begin
    for column:=1 to 80 do begin
      Mem[$B800 : 160*(Row-1)+2*(Column-1)]:=scrlpt^[b][column];
      Mem[$B800 : 160*(Row-1)+2*(Column-1)+1]:=scrlatpt^[b][column];
    end;
    if b=1 then b:=819 else dec(b);
    inc(i);
  end;
  topscrline:=b+1;
end;

procedure upline;
begin
   if topscrline<>topbuffer then begin
      if (topScrLine <> bottomBuffer) then dec(topScrLine);
      if (curScrlLine <> bottomBuffer) then dec(curScrlLine);
      myview;
   end;
end;

procedure downline;
begin
   if curscrlline<>bottombuffer then begin
      if (topScrLine <> 819) then inc(topScrLine);
      if (curScrlLine <> 819) then inc(curScrlLine);
      myview;
   end;
end;

procedure pagemeup;
var i:byte;
begin
  if topscrline<>topbuffer then begin
    for i:=1 to 24 do
      if topscrline<>topbuffer then begin
        if topscrline=1 then topscrline:=819 else dec(topscrline);
        if curscrlline=1 then curscrlline:=819 else dec(curscrlline);
      end;
    myview;
  end;
end;

procedure pagemedown;
var i:byte;
begin
  if curscrlline<>bottombuffer then begin
    for i:=1 to 24 do
      if curscrlline<>bottombuffer then begin
        if (topscrline=819) then topscrline:=1 else inc(topscrline);
        if(curscrlline=819) then curscrlline:=1 else inc(curscrlline);
      end;
    myview;
  end;
end;

procedure viewscrollback;
var
  i:longint;
  abort:boolean;
  k:char;
  x,y:byte;
  windr : windowrec;
  oldF2 : byte;

  {
  procedure restoremyscreen;
  var I,b:integer;
  begin
    clrscr;
    curscrlline:=bottombuffer;
    i:=0;
    b:=curscrlline;
    for row:=y downto 1 do begin
      for column:=1 to 80 do begin
        Mem[$B800 : 160*(row-1)+2*(Column-1)]:=scrlpt^[b][column];
        Mem[$B800 : 160*(Row-1)+2*(Column-1)+1]:=scrlatpt^[b][column];
      end;
      if b=1 then b:=819 else dec(b);
    end;
    gotoxy(x,y);
  end;
  }

begin
  x:=wherex;
  y:=wherey;
  savescreen(windr, 1, 1, 80, 25);

  clrscr;

  if amtscrllines<=24 then begin
    for row:=curscrlline downto 1 do
      for column:=1 to 80 do begin
        Mem[$B800 : 160*(row-1)+2*(Column-1)]:=scrlpt^[row][column];
        Mem[$B800 : 160*(Row-1)+2*(Column-1)+1]:=scrlatpt^[row][column];
      end;
    topscrline:=1;
  end else myview;
  {Initial Setup}

  oldf2 := f2key;
  f2key := 254;
  schangewindow(TRUE,1);

  abort:=false;

  cursoron(false);

  while not abort do begin
    k:=readkey;
    if (k=#0) then begin
      k:=readkey;
      case k of
        #72,'A': upline;
        #80,'B': downline;
        #73    : pagemeup;
        #81    : pagemedown;
        #27    : abort:=true;
      end;
    end else if (k=#27) or (K=^C) then abort:=true;
  end;

  removewindow(windr);
  gotoxy(x,y);

  f2key := oldf2;

  if f2key <> 0 then schangewindow(TRUE,1) else
    sclearwindow;

  cursoron(true);
end;

procedure addtoscroll(CurrentY:byte);
{Copies the current line of text on the screen to scrollback buffer}
var I:integer;

  procedure copyfrommem;
  begin
    for column:=1 to 80 do begin
      scrlpt^[curscrlline][column]:=mem[$B800 : 160*(CurrentY-1)+2*(Column-1)];
      scrlatpt^[curscrlline][column]:=mem[$B800 : 160*(CurrentY-1)+2*(Column-1)+1];
    end;
  end;

begin


  if amtscrllines<819 then begin
    inc(Curscrlline);
    copyfrommem;
    inc(amtscrllines);
    inc(bottombuffer);
  end else begin
    if bottombuffer=819 then begin
      bottombuffer:=1;
      topbuffer:=2;
    end else
      if (topbuffer<819) and (bottombuffer<819) then begin
        inc(bottombuffer);
        inc(topbuffer);
      end else
        if (topbuffer=819) then begin
          topbuffer:=1;
          bottombuffer:=819;
        end;
    curscrlline:=bottombuffer;
    copyfrommem;
  end;
end;

end.



