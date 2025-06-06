{$A+,B+,D-,E+,F+,I+,L+,N-,O+,R-,S-,V-}
{ file system config }
unit sysop2f;

interface

uses
   Crt,      Dos,      common1,  common2,  common3,  common5,  common4,
   common;

procedure pofilesconfig;

implementation

procedure pofilesconfig;
var s:string[80];
    i:integer;
    c:char;
    b:byte;
    bb:byte;
    done,changed:boolean;
begin
   done:=FALSE;
   repeat
      with status^ do begin
         cls;
         header('File Area Configuration');
         top;
         if (not ulcheckit) then middle('A','Archive Scanner .BAT file      :'+startdir)
         else middle('A','Archive Scanner .BAT file      :None');
         middle('B','Use N-Scan Internal U/L Check  :'+aonoff(ulcheckit,'Active','In-Active'));
         middle('C','Upload/download ratio          :'+aonoff(uldlratio,'Active','In-active'));
         middle('D','Auto file point compensation   :'+aonoff(fileptratio,'Active','In-active'));
         middle('E','File point compensation ratio  :'+cstr(fileptcomp)+' to 1');
         middle('F','Base file size per 1 file point:'+cstr(fileptcompbasesize)+'k');
         middle('G','Upload time refund percent     :'+cstr(ulrefund)+'%');
         s:='Uploads File Base              :';
         if (tosysopdir=255) or (tosysopdir=0) then s:=s+'None'
         else s:=s+cstr(tosysopdir);
         middle('H',s);
         middle('I','Auto-validate ALL files ULed?  :'+syn(validateallfiles));
         middle('J','Remote DOS re-direction device :'+status^.remdevice);
         middle('K','Max k-bytes allowed in TEMP\3\ :'+cstr(status^.maxintemp));
         middle('L','Min k-bytes to save for resume :'+cstr(status^.minresume));
         middle('1','Comment                        :'+status^.filearccomment[1]);
         middle('2','Zip Comment File               :'+status^.filearccomment[2]);
         middle('3','BBS Ad. File                   :'+status^.filearccomment[3]);
         bottom;
         command('QABCDEFGIJKL123');
         onek(c,'QABCDEFGHIJKL123'^M);
         nl;
         case c of
            'Q':done:=TRUE;
            'A': if (ulcheckit) then begin
               prompt('Enter Batch File To run external Archive Scanner: ');
               inputl(s,40);
               if (s<>' ') then begin
                  status^.startdir:=s;
                  ulcheckit:=false;
               end;
            end
            else begin
               ulcheckit:=true;
               status^.startdir:='';
            end;
            'B': begin
               ulcheckit:=not ulcheckit;
               status^.startdir:='';
            end;
            'C': uldlratio:=not uldlratio;
            'D': fileptratio:=not fileptratio;
            'E'..'H':begin
               if (c<>'H') then print('Range (1-255)')
               else print('Range (1-'+cstr(maxulb)+')  (255 to disable)');
               prompt('New value: ');
               ini(b);
               if (not badini) then
                  case c of
                     'E':if (b>=1) and (b<=255) then fileptcomp:=b;
                     'F':fileptcompbasesize:=b;
                     'G':ulrefund:=b;
                     'H':if ((b>=1) and (b<=maxulb)) or (b=255) then tosysopdir:=b;
                  end;
            end;
            'I': validateallfiles:=not validateallfiles;
            'J':begin
               sprint('This should be "GATE1" (or "GATE2", etc) if you have it.');
               sprint('OTHERWISE, it should be "COM1" (or "COM2", etc).|NL');
               prompt('New device: ');
               inputwn(remdevice,10,changed);
            end;
            'K':begin
               prompt('New max k-bytes: ');
               inu(i);
               if (not badini) then status^.maxintemp:=i;
            end;
            'L':begin
               prompt('New min resume k-bytes: ');
               inu(i);
               if (not badini) then status^.minresume:=i;
            end;
            '1'..'3': begin
               bb:=ord(c)-48;
               Case C of
                  '1': prompt('Comment: ');
                  '2': prompt('Zip Comment: ');
                  '3': prompt('BBS Ad: ');
               End;
               inputwnwc(status^.filearccomment[bb],32,changed);
            end;
         end;
      end;
  until (done) or (hangup);
end;

end.
