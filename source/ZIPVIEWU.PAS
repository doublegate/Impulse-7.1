{$A+,B+,F+,I-,L+,O+,R-,S+,V-}
UNIT zipviewu;

INTERFACE

USES  Dos,  Crt, Common, Common5,common2,sys;

PROCEDURE zipview(zipfile: STRING);

IMPLEMENTATION

CONST hexdigit : ARRAY[0..15] OF CHAR = '0123456789abcdef';

FUNCTION hexbyte(b: byte): STRING;                        (* Byte to Hexbyte *)
BEGIN
   hexbyte := hexdigit[b SHR 4]+hexdigit[b AND $f];
END;

FUNCTION hexlong(l: LONGINT): STRING;                  (* Longint to Hexlong *)
VAR n : ARRAY[1..4] OF BYTE ABSOLUTE l;
BEGIN
   hexlong := hexbyte(n[4])+hexbyte(n[3])+hexbyte(n[2])+hexbyte(n[1]);
END;

FUNCTION b2attr(a,g: BYTE): STRING;                     (* Byte to Attribute *)
VAR attr : STRING[5];
BEGIN
   attr := '--w- ';
   IF (g AND 1)=1 THEN attr[5]:='*';                          (* Encrypted? *)
   IF (a AND 1)=1 THEN attr[3]:='r';                          (* Read Only? *)
   IF (a AND 2)=2 THEN attr[2]:='h';                             (* Hidden? *)
   IF (a AND 4)=4 THEN attr[1]:='s';                             (* System? *)
   IF (a AND 8)=8 THEN attr[4]:='?';                (* Unknown at this time *)
   b2attr := attr;
END;

FUNCTION w2date(d: WORD): STRING;                            (* Word to Date *)
VAR s : STRING;
BEGIN
   s := tch(cstr((d SHR 5) AND 15 ))+'-'+                          (* Month *)
        tch(cstr((d      ) AND 31 ))+'-'+                            (* Day *)
        tch(cstr(((d SHR 9) AND 127)+80));                          (* Year *)
   w2date := s;
END;

FUNCTION w2time(t: WORD): STRING;                            (* Word to Time *)
VAR s : STRING;
BEGIN
   s := tch(cstr((t SHR 11) AND 31))+':'+                           (* Hour *)
        tch(cstr((t SHR  5) AND 63));                             (* Minute *)
   w2time := s;
END;

PROCEDURE zipview(zipfile: STRING);                     (* View the ZIP File *)
  CONST lsig = $04034B50;                                 (* Local Signature *)
        csig = $02014b50;                               (* Central Signature *)
  TYPE lheader = RECORD                                      (* Local Header *)
                   signature  : LONGINT;      (* local file header signature *)
                   version,                                (* version mad by *)
                   gpflag,                          (* general purpose flags *)
                   compress,                           (* compression method *)
                   time,date  : WORD;         (* last mod file time and date *)
                   crc32,                                          (* crc-32 *)
                   csize,                                 (* compressed size *)
                   usize      : LONGINT;                (* uncompressed size *)
                   fnamelen,                              (* filename length *)
                   extrafield : WORD;                  (* extra field length *)
                 END;
       cheader = RECORD                                    (* Central Header *)
                   signature  : LONGINT;    (* central file header signature *)
                   version    : WORD;                     (* version made by *)
                   vneeded    : WORD;           (* version needed to extract *)
                   gpflag     : ARRAY[1..2] OF BYTE;(* general purpose flags *)
                   compress   : WORD;                  (* compression method *)
                   time       : WORD;                  (* last mod file time *)
                   date       : WORD;                  (* last mod file date *)
                   crc32      : LONGINT;                           (* crc-32 *)
                   csize      : LONGINT;                  (* compressed size *)
                   usize      : LONGINT;                (* uncompressed size *)
                   fnamelen   : WORD;                     (* filename length *)
                   extrafield : WORD;                  (* extra field length *)
                   fcl        : WORD;                 (* file comment length *)
                   dns        : WORD;                   (* disk number start *)
                   ifa        : WORD;            (* internal file attributes *)
                   efa        : ARRAY[1..4] OF BYTE;   (* external file attr *)
                   roolh      : LONGINT;  (* relative offset of local header *)
                 END;

VAR z          : INTEGER;               (* Number of files processed counter *)
    tmpint,
    totalu,                              (* Total bytes that were compressed *)
    totalc     : LONGINT;          (* result of total bytes being compressed *)
    hdr        : ^cheader;            (* temporary cental header file record *)
    f          : FILE;                                           (* file var *)
    s          : STRING;                          (* archive filename string *)
    percent    : BYTE;           (* Temporary var holding percent compressed *)
    numfiles   : WORD;                         (* Number of files in archive *)

CONST comptypes : ARRAY[0..8] OF STRING[7] =            (* Compression Types *)
                  ('Stored ',                              (* Not Compressed *)
                   'Shrunk ',                                      (* Shrunk *)
                   'Reduce1',                                   (* Reduced 1 *)
                   'Reduce2',                                   (* Reduced 2 *)
                   'Reduce3',                                   (* Reduced 3 *)
                   'Reduce4',                                   (* Reduced 4 *)
                   'Implode',                                    (* Imploded *)
                   'NotSure',                        (* Unknown at this time *)
                   'DeflatN');                                   (* Deflated *)

   FUNCTION seekc(VAR f: FILE): BOOLEAN;
   VAR curpos  : LONGINT;                           (* current file position *)
       buf     : lheader;                   (* Temporary local header record *)
       ioerror : INTEGER;                       (* Temporary IOResult holder *)
       reslt   : WORD;                                   (* Blockread Result *)
   BEGIN
      seekc := FALSE;                                           (* init seekc *)
      curpos := 0;                              (* init current file position *)
      SEEK(f,0);                                        (* goto start of file *)
      BLOCKREAD(f,buf,SIZEOF(lheader),reslt);      (* Grab first local header *)
      ioerror := IORESULT;                                  (* Test for error *)
      WHILE (ioerror = 0) AND (buf.signature=lsig) DO begin(* Test if OK..continue *)
         INC(numfiles);                         (* Increment number of files *)
         WITH buf DO curpos := FILEPOS(f)+fnamelen+extrafield+csize; (* Find end of local header *)
         SEEK(f,curpos);                         (* Goto end of local header *)
         BLOCKREAD(f,buf,SIZEOF(lheader),reslt);   (* Grab next local header *)
         ioerror := IORESULT;                              (* Test for error *)
      END;
      IF ioerror<>0 THEN EXIT;               (* If error then exit function *)
      IF (buf.signature=csig) THEN begin(* Did we find the first central header? *)
         seekc := TRUE;                      (* Found first central header *)
         SEEK(f,curpos); (* Ensure we are at central headers file position *)
      END;
   END;

  VAR curpos : LONGINT;

BEGIN
   numfiles := 0;      (* Counter of Number of Files to Determine When Done *)
   z        := 0;                   (* Counter of Number of Files Processed *)
   totalu   := 0;                      (* Total Bytes of Uncompressed Files *)
   totalc   := 0;                      (* Total Size after being Compressed *)
   cls;
   NEW(hdr);        (* Dynamically Allocate Memory for a Temp Header Record *)
   ASSIGN(f,zipfile);                        (* Assign Filename to File Var *)
   {$I-}RESET(f,1); {$I+}                    (* Open Untyped File *)
   IF IORESULT<>0 THEN begin                 (* If we get an error, exit program *)
      print('Error - File not found.');
      Exit;
   END;
   IF NOT seekc(f) THEN begin           (* Skip Local Headers and goto first Central Header *)
                                        (* If we could not locate a Central Header *)
        CLOSE(f);                                      (* Close Untyped File *)
        Sprint('Error - Corrupted or Not a ZIP File.');
   END;
   sysMciSet('ZN',zipFile);
   printf('zipview.top');
   REPEAT
      FILLCHAR(s,SIZEOF(s),#0);                         (* Clear Name String *)
      BLOCKREAD(f,hdr^,SIZEOF(cheader));                 (* Read File Header *)
      BLOCKREAD(f,MEM[SEG(s):OFS(s)+1],hdr^.fnamelen);  (* Read Archive Name *)
      s[0] := CHR(hdr^.fnamelen);                 (* Get Archive Name Length *)
      IF (hdr^.signature=csig) THEN begin                          (* Is a header *)
         sysResetMciList;
         INC(z);                                  (* Increment File Counter *)
         sysMciSet('Z1',mrn(cstr(hdr^.usize),7));
         sysMciSet('Z2',mrn(comptypes[hdr^.compress],7));
         sysMciSet('Z3',mrn(cstr(hdr^.csize),8));
         if (hdr^.csize=0) or (hdr^.usize*100.0=0) or (hdr^.usize=0) then percent:=0
         else percent := ROUND(100.0-(hdr^.csize/hdr^.usize*100.0));
         sysMciSet('Z4',mrn(cstr(percent),4)+'%');
         sysMciSet('Z5',w2date(hdr^.date));
         sysMciSet('Z6',w2time(hdr^.time));
         sysMciSet('Z7',hexlong(hdr^.crc32));
         sysMciSet('Z8',b2attr(hdr^.efa[1],hdr^.gpflag[1]));
         sysMciSet('Z9',mln(s,13));
         sysMciSet('ZT',mrn(cstr(totalu),7));
         sysMciSet('ZC',mrn(cstr(totalc),7));
         sysMciSet('ZF',cstr(z));
         INC(totalu,hdr^.usize);             (* Increment size uncompressed *)
         INC(totalc,hdr^.csize);               (* Increment size compressed *)
         printf('zipview.mid');
      END;
      SEEK(f,FILEPOS(f)+hdr^.extrafield+hdr^.fcl);
   UNTIL (hdr^.signature<>csig) OR EOF(f) OR (z=numfiles); (* No more Files *)
   printf('ZIPVIEW.BOT');
   CLOSE(f);                                          (* Close Untyped File *)
   DISPOSE(hdr);                            (* Deallocate Header Var Memory *)
END;

END.
