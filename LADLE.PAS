{$A+,B+,D-,E+,F+,I+,L-,N-,O-,R-,S-,V-,Q-,X-}

Program Ladle; {Extracts sauce descriptions to a diz.}

Uses AsmSauce;

Var Template,
    FileDiz:Text;
    TempPath,
    DizPath:String;
    Tmp:String;
    EndTmp:Boolean;

Function Filter(S:String):String;
Var i:integer;
    tmp:string[10];
Begin
  if (pos('|ST',s)<>0) then
  begin
    i:=pos('|ST',s);
    delete(s,pos('|ST',s),3);
    insert(sauce.title,s,i);
  end;
  if (pos('|SA',s)<>0) then
  begin
    i:=pos('|SA',s);
    delete(s,pos('|SA',s),3);
    insert(sauce.author,s,i);
  end;
  if (pos('|SG',s)<>0) then
  begin
    i:=pos('|SG',s);
    delete(s,pos('|SG',s),3);
    insert(sauce.group,s,i);
  end;
  if (pos('|SV',s)<>0) then
  begin
    i:=pos('|SV',s);
    delete(s,pos('|SV',s),3);
    insert(sauce.version,s,i);
  end;
  if (pos('|SD',s)<>0) then
  begin
    i:=pos('|SD',s);
    delete(s,pos('|SD',s),3);
    tmp:=copy(sauce.date,5,2)+'/'+copy(sauce.date,7,2)+'/'+copy(sauce.date,1,4);
    insert(tmp,s,i);
  end;
  filter:=s;
end;

Function Exist(S:String):Boolean;
Var F:File;
Begin
  Assign(F,S);
  {$I-} Reset(F); {$I+}
  If (ioresult<>0) then Exist:=False
    else Begin
      Exist:=True;
      Close(F);
    end;
End;
Begin
   writeLn;
   writeln('LADLE.EXE - v3.01 Sauce Extraction.');
   writeln('Copyright in 1995(98) by Tumyeto Software.');
   writeln;
   EndTmp:=False;
   If (ParamStr(1)<>'') and (paramstr(2)<>'') then begin
      If paramstr(3)<>'' then dizpath:=paramstr(3)
      else DizPath:='';
      Temppath:=ParamStr(2);
      If (Not Exist(Temppath)) then begin
        writeln(temppath+' not found!');
        writeln;
        halt(2);
      end;
      if (getsauce(paramstr(1))) then begin
        Assign(FileDiz,dizpath+'FILE_ID.DIZ');
        Rewrite(FileDiz);
        Assign(Template,temppath);
        Reset(Template);
        repeat
          readln(template,Tmp);
          writeln(filediz,filter(tmp));
          if (eof(template)) then EndTmp:=True;
        until (EndTmp);
        Close(Template);
        Close(Filediz);
      end;
   end
   else begin
      writeln(' LADLE.EXE <ansifile> <template> <diz path>');
      writeln(' DizPath -MUST- include ending \ mark.');
      writeln;
   end;
end.
