type Twfc = record
   x,
   y,
   n,
   h : byte;
end;
var wfc : Twfc;
    ft  : text;
    fi  ,
    fo  : file;
    buf : array[1..4000] of byte;
    fts,
    fis : string[80];
    ts : string[2];
    tw : word;
begin
   if paramcount <> 2 then begin
      writeln('Usage is : ');
      writeLn('"mkwfc <bin_file> <txt_file>"');
      exit;
   end;
   fis := paramstr(1);
   fts := paramstr(2);
   assign(fi,fis);
   reset(fi,1);
   assign(fo,'wfc.bin');
   rewrite(fo,1);
   blockread(fi,buf,4000);
   close(fi);
   assign(ft,fts);
   reset(ft);
   readln(ft,ts);
   val(ts,wfc.x,tw);
   readln(ft,ts);
   val(ts,wfc.y,tw);
   readln(ft,ts);
   val(ts,wfc.n,tw);
   readln(ft,ts);
   val(ts,wfc.h,tw);
   close(ft);
   blockwrite(fo,buf,4000);
   blockwrite(fo,wfc,sizeOf(wfc));
   close(fo);
end.
