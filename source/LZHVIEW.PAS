{ View the internals of a LZH Compressed File }
{$A+,B+,F+,I-,L+,O+,R-,S+,V-}
Unit LZHview;

Interface

Uses
   Dos,      Common,   Crt,      records,  strProc,  common2,  sys;

Procedure LZHV(SS:String);

Implementation

Procedure LZHV(SS:String);
Type LZHHead = Record
                 HSize      : Byte;
                 Fill1      : Byte;
                 Method     : Array[1..5] of Char;
                 CompSize   : LongInt;
                 UCompSize  : LongInt;
                 Dos_DT     : LongInt;
                 Fill2      : Word;
                 FileNameLen: Byte;
                 FileName   : Array[1..12] of Char;
               end;

Var LZH1       : LZHHead;
    DT         : DateTime;
    FSize,L,C  : LongInt;
    F          : File;
    BUFF       : Array[1..maxBufSize] of Byte;
    DATE       : String[8];                { formatted date as YY/MM/DD }
    TIME       : String[6];                {     "     time as HH:MM }
    RES        : Word;
    DIR        : DirStr;
    FNAME      : NameStr;
    EXT        : ExtStr;
    LZHString,
    SName      : String;
    QUIT       : Boolean;
    SW         : Pointer;

   Function ord_to_str(i:LongInt;j:Byte):String;
   Var c:String;
   begin
      str(i,c);
      While length(c)<j do c:=' '+c;
      ord_to_str:=c;
   end;

   Procedure FDT(LI:LongInt); { Format Date/Time (time With AM PM) fields }
   Var t_ext : String;
   begin
      UnPackTime (LI,DT);
      DATE := ord_to_str(DT.Month,2)+'/'+ord_to_str(DT.Day,2)+'/'+ord_to_str(DT.Year mod 100,2);
      if DATE[1] = ' ' then DATE[1] := '0';
      if DATE[4] = ' ' then DATE[4] := '0';
      if DATE[7] = ' ' then DATE[7] := '0';
      if DT.Hour in [0..11] then t_ext:='a'
      else t_ext:='p';
      if DT.Hour in [13..24] then Dec(DT.Hour,12);
      TIME := ord_to_str(DT.Hour,2)+':'+ord_to_str(DT.Min,2);
      if TIME[1] = ' ' then TIME[1] := '0';
      if TIME[4] = ' ' then TIME[4] := '0';
      TIME:=TIME+t_ext;
   end;  { FDT }

   Procedure GET_LZH_ENTRY;
   begin
      FillChar(LZH1,SizeOf(LZHHead),#0);
      FillChar (DT,SizeOf(DT),#0);
      L := SizeOf(LZHHead);
      Seek (F,C);
      BlockRead (F,BUFF,L,RES);
      Move (BUFF[1],LZH1,L);
      With LZH1 do
         if HSize > 0 then begin
            Move (FileNameLen,SNAME,FileNameLen+1);
            UnPackTime (Dos_DT,DT);
            FSize := CompSize
         end
         else QUIT := True
   end;  { GET_LZH_ENTRY }

   Procedure DO_LZH (FN : String);
   Var fnstr, LZHMeth : String;
       fls,totu,totc : LongInt;
   begin
      totu:=0;
      totc:=0;
      fls:=0;
      Assign (F,FN);
      {$I-} Reset (F,1); {$I+}
      if Ioresult<>0 then begin
         Writeln(strAllCaps(FN)+' not found');
         Exit;
      end;
      FSize := FileSize(F);
      C := 0;
      QUIT := False;
      cls;
      printf('lzhview.top');
      sysMciSet('LN',fn);               { lzh file name }
      Repeat
         sysResetMciList;
         GET_LZH_ENTRY;
         if not QUIT then begin
            FSplit (SNAME,DIR,FNAME,EXT);
            fnstr:=FNAME+EXT;
            While length(fnstr)<12 do insert(' ',fnstr,length(fnstr)+1);
            FDT(LZH1.Dos_DT);
            inc(totu,lzh1.ucompsize);
            inc(totc,lzh1.compsize);
            inc(fls);
            sysMciSet('L2',fnstr);
            sysMciSet('L3',cstr(lzh1.uCompSize));
            sysMciSet('L4',cstr(lzh1.compSize));
            sysMciSet('L5',cstr(fls));
            sysMciSet('L6',date);
            sysMciSet('L7',time);
            sysMciSet('L8',cstr(totU));
            sysMciSet('L9',cstr(totC));
            Case LZH1.Method[4] of       {normally only 0,1 or 5}
               '0' : sysMciSet('L1','Stored');
               '1' : sysMciSet('L1','Frozen 1');
               '2' : sysMciSet('L1','Frozen 2');
               '3' : sysMciSet('L1','Frozen 3');
               '4' : sysMciSet('L1','Frozen 4');
               '5' : sysMciSet('L1','Frozen 5');
               else sysMciSet('L1','Unknown');
            end;
            printf('lzhview.mid');
         end;
         Inc (C,FSize+LZH1.HSize+2)
      Until QUIT;
      Close (F);
      printf('lzhview.bot');
      sysResetMciList;
   end;  { DO_LZH }
begin
   do_lzh(SS);
end;

end.

{
Note the changes in the date processing and compression method display.
Thanks again For the code.
}
