{ Upload Checker }
{$A+,B+,E+,F+,I+,L+,N-,O+,R-,S-,V-}
unit ulcheck;

interface

procedure ulexamine(s,f:string);
Procedure External_ULcheck(fname:string);

implementation

uses
   Crt,      myio,     Dos,      common,   common1,  common2,  common3,
   Common5,  ansidrv,  records,  cmd,      file1;

Procedure External_ULcheck(fname:string);
var ret:integer;

   function parse(s:astr):astr;
   var i:integer;
      sda,namm:astr;
      sdoor:string;
   begin
      namm:=caps(thisuser.realname);
      sdoor:='';
      for i:=1 to length(s) do begin
         if copy(s,i,1)='@' then begin
            sda:='';
            case upcase(s[i+1]) of
               'B':if spd<>strings^.locallogin then sda:=spd
                  else sda:='0';
               'F':sda:=fname;
            end;
            sdoor:=sdoor+sda;
            inc(i);
         end
         else sdoor:=sdoor+copy(s,i,1);
      end;
      parse:=sdoor;
   end;

begin
   fname:=sqoutsp(fname);
   shelldos(FALSE,parse(status^.startdir),ret);
end;

procedure ulexamine(s,f:string);
Var RCode:Integer;

    CCurPosY:Array [1..5] Of Byte;
    CCurPosX:Array [1..5] Of Byte;
    CC:Byte;

    C2CurPosY:Array [1..5] Of Byte;
    C2CurPosX:Array [1..5] Of Byte;
    C2C:Byte;

    StXPos:Array [1..5] Of byte;
    StYPos:Array [1..5] Of byte;
    STC:Byte;

    EndXPos:Array [1..5] Of byte;
    EndYPos:Array [1..5] Of byte;
    EndC:Byte;

    DoneX : Byte;
    DoneY : Byte;

Procedure GetC(Num:Byte);
Begin
   CCurPosX[Num]:=WhereX;
   CCurPosY[Num]:=WhereY;
   CC:=TextAttr;
End;

Procedure GetC2(Num:Byte);
Begin
   C2CurPosY[Num]:=WhereX;
   C2CurPosX[Num]:=WhereY;
   C2C:=TextAttr;
End;

Procedure GetStart(Num:Byte);
Begin
   STXPos[Num]:=WhereX;
   STYPos[Num]:=WhereY;
   STC:=TextAttr;
End;
{#27+'['+cstr(y)+';'+cstr(x)+'H'}
Procedure GetEnd(Num:Byte);
Begin
   EndXPos[Num]:=WhereX;
   EndYPos[Num]:=WhereY;
   EndC:=TextAttr;
End;

Procedure GetDone;
begin
   DoneX := WhereX;
   DoneY := WhereY;
end;

Function EndIt(Num:Byte):String;
Begin
   Case Num Of
      1:Endit:='Passed              ';
      2:Endit:='Done                ';
      3:Endit:='Added               ';
      4:Endit:='Added               ';
      5:Endit:='Done                ';
   End;
End;

Procedure ShowMsg(X,Y:Byte; Msg:String; Color:Byte);
Begin
   Goxy(X,Y);
   SetC(color);
   Prompt(Msg);
End;

Function Start(Num:byte):String;
Begin
   Case Num Of
      1:Start:='Testing Zip Interior';
      2:Start:='Removing Files      ';
      3:Start:='Adding Zip Comment  ';
      4:Start:='Adding Zip Ad       ';
      5:Start:='Getting Description ';
   End;
End;
procedure showscr(fn:string);
var f:file;
    oaa,
    abort,
    next : boolean;
    c2,
    c,
    k : char;
    xpoint,
    ypoint : byte;
    tmp:string;
    i:integer;
    pub:boolean;
    ansiBuf: ^ansiBufType;
    sread,
    tRead : longint;
    code : byte;
    read : integer;
    x    : integer;

begin
   assign(f,fn);
   if (not exist(fn)) then begin
      noansi(fn);
      exit;
   end;
   new(ansiBuf);
   reset(f,1);
   sread := fileSize(f);
   tread := 0;
   printingfile:=TRUE;
   oaa:=allowabort;
   allowabort:=TRUE;
   next:=FALSE;
   nofile:=false;
   abort:=false;
   while (tRead < sRead) and (not Abort) do begin
      BlockRead(F,ansiBuf^,maxBufSize,Read);
      Inc(tRead,Read);
      if tRead > sRead then Dec(Read,tRead-sRead);
      x := 1;
      while x <= Read do begin
         k := ansiBuf^[x];
         inc(x);
         if k='%' then begin
            c2 := ansiBuf^[x];
            inc(x);
            c := ansiBuf^[x];
            inc(x);
            case (c2) of
            'D' : begin
            case (c) of
            '1' : getC(1);
            '2' : getC(2);
            '3' : getC(3);
            '4' : getC(4);
            '5' : getC(5);
            'N' : getDone;
            else  prompt('%D'+c);
            end;
            end;
            'E' : begin
            case (c) of
            '1' : getEnd(1);
            '2' : getEnd(2);
            '3' : getEnd(3);
            '4' : getEnd(4);
            '5' : getEnd(5);
            else  prompt('%E'+c);
            end;
            end;
            'P' : begin
            case (c) of
            '1' : getC2(1);
            '2' : getc2(2);
            '3' : getc2(3);
            '4' : getc2(4);
            '5' : getc2(5);
            else  prompt('%P'+c);
            end;
            end;
            'S' : begin
            case (c) of
            '1' : getStart(1);
            '2' : getStart(2);
            '3' : getStart(3);
            '4' : getStart(4);
            '5' : getStart(5);
            else  prompt('%S'+c);
            end;
            end;
         else prompt('%'+c2+c);
         end;
    end
    else begin
         if (outcom) then pr1(k);
         lpromptc(k);
    end;
    wkey(abort,next);
    end;
   end;
   close(f);
   allowabort:=oaa;
   printingfile:=FALSE;
   ctrljoff:=FALSE;
   curco:=255-curco;
   cl(1);
   dispose(ansiBuf);
end;

procedure printul(fn:string);
var i:integer;
    s:string;
begin
   fn:=status^.afilepath+fn;
   if (pos('.',fn)<>0) then showscr(fn)
   else if exist(fn+'.1') then begin
      randomize;
      repeat
         i:=random(9);
         showscr(fn+'.'+cstr(i));
      until (exist(fn+'.'+cstr(i)));
   end
   else if thisuser.ac=thisuser.ac-[color] then showscr(fn+'.ASC')
   else showscr(fn+'.ANS');
end;

{   procedure zipdiz(fn:string);
   begin
      fn:=sqoutsp(fn);
      cmdGetSauce(fn,status^.afilePath+'SAUCE.TMP',status^.tempPath,false);
      if (pos('.ZIP',fn)<>0) then shelldos(false,'Pkunzip -o '+fn+' file_id.diz '+status^.temppath+' > nul', rcode);
      if (pos('.LZH',fn)<>0) then shelldos(false,'lha e f '+fn+' file_id.diz '+status^.temppath+' > nul', rcode);
      if (pos('.ARJ',fn)<>0) then shelldos(false,'arj e f '+fn+' file_id.diz '+status^.temppath+' > nul', rcode);
   end;}

var ok:boolean;
    wind:windowrec;
begin
   pauseme:=false;
   PrintUl('ULCHECK');
   ok:=true;
   f:=sqoutsp(f);
   {begin test #1}
   {$I-}
   ShowMsg(CCurPosX[1],CCurPosY[1],'þ',CC);
   ShowMsg(StXPos[1],StYPos[1],Start(1),STC);
   if (pos('.ZIP',f)<>0) then shelldos(false, 'pkunzip -t '+s+f,rcode);
   if (pos('.LZH',f)<>0) then shelldos(false, 'lha t '+s+f, rcode);
   if (pos('.ARJ',f)<>0) then shelldos(false, 'arj t '+s+f, rcode);
   ShowMsg(C2CurPosX[1],C2CurPosY[1],'þ',C2C);
   ShowMsg(EndXPos[1],EndYPos[1],Endit(1),EndC);
   {$I+}
   {end test #1}

   {begin test #3}
   ShowMsg(CCurPosX[2],CCurPosY[2],'þ',CC);
   ShowMsg(StXPos[2],StYPos[2],Start(2),STC);
   if exist(status^.gfilepath+'remove.txt') then begin
      {$I-}if (pos('.ZIP',f)<>0) then shelldos(false,'pkzip -d '+s+f+' @'+status^.gfilepath+'REMOVE.TXT',rcode);
      if (pos('.LZH',f)<>0) then shelldos(false,'lha d '+s+f+' @'+status^.gfilepath+'REMOVE.TXT',rcode);
      if (pos('.ARJ',f)<>0) then shelldos(false,'arj d '+s+f+' @'+status^.gfilepath+'REMOVE.TXT',rcode);{$I+}
   end;
   ShowMsg(C2CurPosX[2],C2CurPosY[2],'þ',C2C);
   ShowMsg(EndXPos[2],EndYPos[2],Endit(2),EndC);
   {end test #3}

   {begin test #4}
   ShowMsg(CCurPosX[3],CCurPosY[3],'þ',CC);
   ShowMsg(StXPos[3],StYPos[3],Start(3),STC);
   if (length(status^.filearccomment[2])<>0) and (exist(status^.filearccomment[2])) then
      if (pos('.ZIP',f)<>0) then {$I-} shelldos(false,'Pkzip -z '+s+f+' < '+status^.filearccomment[2],rcode);{$I+}
   ShowMsg(C2CurPosX[3],C2CurPosY[3],'þ',C2c);
   ShowMsg(EndXPos[3],EndYPos[3],Endit(3),EndC);
   {end test #4}

   {begin test #5}
   ShowMsg(CCurPosX[4],CCurPosY[4],'þ',CC);
   ShowMsg(StXPos[4],StYPos[4],Start(4),STC);
   if (length(status^.filearccomment[3])<>0) and (exist(status^.filearccomment[3])) then begin
      {$I-} if (pos('.ZIP',f)<>0) then shelldos(false,'Pkzip -a '+s+f+' '+status^.filearccomment[3],rcode);
      if (pos('.LZH',f)<>0) then shelldos(false,'lha a '+s+f+' '+status^.filearccomment[3],rcode);
      if (pos('.ARJ',f)<>0) then shelldos(false,'arj a '+s+f+' '+status^.filearccomment[3],rcode);{$I+}
   end;
   ShowMsg(C2CurPosX[4],C2CurPosY[4],'þ',C2C);
   ShowMsg(EndXPos[4],EndYPos[4],Endit(4),EndC);
   {end test #5}

   ShowMsg(CCurPosX[5],CCurPosY[5],'þ',CC);
   ShowMsg(StXPos[5],StYPos[5],Start(5),STC);
   zipdiz(s+f);
   ShowMsg(C2CurPosX[5],C2CurPosY[5],'þ',C2C);
   ShowMsg(EndXPos[5],EndYPos[5],Endit(5),EndC);
   GoXY(DoneX, DoneY);
   pauseme:=true;
end;

end.
