{ MISC. Commands }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}

unit cmd;

interface

procedure cmdShowStackFree;
procedure cmdShowHeapFree;
procedure cmdOneLiners;
procedure cmdGetSauce(ansFile,template,dizPath : string;showStuff : boolean);

implementation

uses common, common1, common2, common3, records, asmSauce;

procedure cmdShowStackFree;
begin
   sprint(^M^J'Stack space free : ' + cStr(sPtr) + ' bytes');
end;

procedure cmdShowHeapFree;
begin
   sprint(^M^J'Heap space free : ' + cStr(memAvail) + ' bytes');
end;

procedure cmdOneliners;
var ft:file of oneLineRec;
    i,
    kn:byte;
    r:oneLineRec;
    sp:array[1..15] of oneLineRec;
begin
   for i:= 1 to 15 do sp[i] := '';
   i := 0;
   assign(ft,status^.gfilepath+'oneline.dat');
   {$I-} reset(ft); {$I+}
   if ioResult <> 0 then rewrite(ft);
   while (i<fileSize(ft)) AND (i < 15) do begin
      read(ft,sp[i+1]);
      inc(i);
   end;
   Cls;
   printf('oneline.top');
   if (i>0) then for kn:=1 to i do sprint(sp[kn])
   else sprint('No Onliners Exist!');
   printf('oneline.bot');
   if (not pynq(strings^.enteroneline,false)) then begin
      close(ft);
      exit;
   end;
   rewrite(ft);
   sprompt(strings^.enteronl);
   inputl(r,70);
   sysoplog('ONELINER: '+thisuser.name+' entered: "'+r+'"');
   if r <> '' then begin
      if (i < 15) then begin
         for kn := 1 to i do write(ft,sp[kn]);
         write(ft,r);
      end
      else begin
         for kn := 2 to 15 do write(ft,sp[kn]);
         write(ft,r);
      end;
      sprint(#3#3'Your Oneliner was Added.');
   end
   else for kn := 1 to i do write(ft,sp[kn]);
   close(Ft);
   pauseScr;
end;

procedure cmdGetSauce(ansFile,template,dizPath : string;showStuff : boolean);
Var TempFile,
    FileDiz:Text;
    Tmp:String;
    EndTmp:Boolean;

   Function Filter(S:String):String;
   Var i:integer;
       tmp:string[10];
   Begin
      if (pos('|ST',s)<>0) then begin
         i:=pos('|ST',s);
         delete(s,pos('|ST',s),3);
         insert(sauce.title,s,i);
      end;
      if (pos('|SA',s)<>0) then begin
         i:=pos('|SA',s);
         delete(s,pos('|SA',s),3);
         insert(sauce.author,s,i);
      end;
      if (pos('|SG',s)<>0) then begin
         i:=pos('|SG',s);
         delete(s,pos('|SG',s),3);
         insert(sauce.group,s,i);
      end;
      if (pos('|SV',s)<>0) then begin
         i:=pos('|SV',s);
         delete(s,pos('|SV',s),3);
         insert(sauce.version,s,i);
      end;
      if (pos('|SD',s)<>0) then begin
         i:=pos('|SD',s);
         delete(s,pos('|SD',s),3);
         tmp:=copy(sauce.date,5,2)+'/'+copy(sauce.date,7,2)+'/'+copy(sauce.date,1,4);
         insert(tmp,s,i);
      end;
      filter:=s;
   end;

Begin
   if (showStuff) then begin
      nl;
      print('LADLE - v3.01 Sauce Extraction.');
      print('Copyright in 1995(98) by Tumyeto Software.');
      nl;
   end;
   EndTmp:=False;
   If (ansFile<>'') and (tempLate<>'') then begin
      If dizPath = '' then DizPath:='';
      If (Not Exist(Template)) then begin
        if (showStuff) then begin
           print(template+' not found!');
           nl;
        end;
        exit;
      end;
      if (getsauce(ansFile)) then begin
         Assign(FileDiz,dizpath+'FILE_ID.DIZ');
         Rewrite(FileDiz);
         Assign(TempFile,template);
         Reset(TempFile);
         repeat
            readln(tempFile,Tmp);
            writeln(filediz,filter(tmp));
            if (eof(tempFile)) then EndTmp:=True;
         until (EndTmp);
         Close(TempFile);
         Close(Filediz);
      end;
   end
   else begin
      if (showStuff) then begin
         print(' LADLE <ansifile> <template> <diz path>');
         print(' DizPath -MUST- include ending \ mark.');
         nl;
      end;
   end;
end;
end.