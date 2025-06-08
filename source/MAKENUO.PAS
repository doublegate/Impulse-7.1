var
    inF : text;
    outF : file;
    b ,
    n : byte;
    nu : array[1..26] of byte;

begin
   fillchar(nu,sizeOf(nu),0);
   assign(inf,paramstr(1));
   reset(inf);
   assign(outF,'nuorder.dat');
   rewrite(outf,1);
   n := 1;
   while (not(eof(inf))) do begin
      readLn(inf,b);
      nu[n] := b;
      inc(n);
   end;
   blockwrite(outf,nu,sizeOf(nu));
   close(outf);
   close(inf);


end.