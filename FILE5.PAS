(* File System 5 *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit file5;

interface

uses
   Crt,      Dos,      common5,  records,  common,   file3,    common1,
   common2,  common3,  file0,    file1,    file2,    file4,    file8,
   file9,    file11,   execbat;

procedure uploadall;

implementation

procedure uploadall;
var bn,savflistopt:integer;
    addall,rr,bb,abort,next,sall:boolean;
    maxlen:integer;

   procedure uploadfiles(b:integer; var abort,next:boolean);
   var fi:file of byte;
      f:ulfrec;
      v:verbrec;
      fn:astr;
      convtime:real;
      oldboard,pl,rn,gotpts,i:integer;
      choice,c:char;
      ulcheck,decide,hangafter,ok,convt,firstone:boolean;

   begin
      addall:=false;
      oldboard:=fileboard;
      firstone:=TRUE;
      decide:=false;
      ulcheck:=status^.ulcheckit;
      status^.ulcheckit:=true;

      if (fileboard<>b) then changefileboard(b);
      if (fileboard=b) then begin
         loaduboard(fileboard);
         nl;
         bb:=false;
         sprint('Scanning '+#3#5+memuboard.name+#3#1+' ('+memuboard.dlpath+')');
         ffile(memuboard.dlpath+'*.*');
         if (exist(memuboard.dlpath+'FILES.BBS')) then AddAll:=Pynq('Import FILES.BBS for file descs?',True);
         while (found) do begin
            if not ((dirinfo.attr and VolumeID=VolumeID) or (dirinfo.attr and Directory=Directory)) then begin
               fn:=align(dirinfo.name);
               recno(fn,pl,rn); { loads memuboard again .. }
               if (rn=0) then begin
                  assign(fi,memuboard.dlpath+fn);
                  {$I-} reset(fi); {$I+}
                  if (ioresult=0) then begin
                     f.blocks:=trunc((filesize(fi)+127.0)/128.0);
                     close(fi);
                     if (firstone) then writehdr('Adding Files');
                     if (firstone) then sprint('Area: '+memuboard.name+'|NL');
                     firstone:=FALSE;
                     f.filename:=fn;
                     nl;
                     sprint(#3#3+fn+' '+#3#4+cstr(f.blocks div 8)+'kb');
                     f.filename:=fn;
                     if (not addall) then
                        repeat
                           f.filename:=fn;
                           sprompt('|U3Add File? |U1[A/Y/N/D/J/?/Q] |U2');
                           onek(choice,'YNDSJ?QA'^M);
                           case choice of
                              ^M,'Y':begin
                                 v.descr[1]:='';
                                 f.description:='';
                                 if (pynq('Import FILE_ID.DIZ? ',true)) then begin
                                    hangafter:=false;
                                    dodescrs(f,v,pl,bb,hangafter,memuboard.ulpath,true);
                                    if (v.descr[1]<>'') then f.vpointer:=nfvpointer;
                                 end
                                 else begin
                                    sprompt('|U2Enter Description: ');
                                    repeat
                                       inputl(f.description,55);
                                    until ((f.description<>'') or (fso) or (hangup));
                                 end;
                                 ok:=true;
                                 decide:=true;
                              end;
                              '?':begin
                                 sprint('|U9[|U0A|U9]|U2 Add All Files');
                                 lcmds(15,3,'Yes, Add file','?Help');
                                 lcmds(15,3,'Delete file','Jump to next area');
                                 lcmds(15,3,'No, don''t add','Quit adding');
                                 decide:=false;
                              end;
                              'D': begin
                                 {$I-}erase(fi);{$I+}
                                 i:=ioresult;
                                 ok:=FALSE;
                                 decide:=true;
                              end;
                              'J': begin
                                 next:=TRUE;
                                 decide:=true;
                                 exit;
                              end;
                              'N': begin
                                 ok:=FALSE;
                                 decide:=true;
                              end;
                              'Q':begin
                                 decide:=true;
                                 abort:=true;
                                 exit;
                              end;
                              'A': begin
                                 addall:=true;
                                 v.descr[1]:='';
                                 f.description:='';
                                 hangafter:=false;
                                 sprint('|U1All files being marked NON-Private.');
                                 dodescrs(f,v,pl,bb,hangafter,memuboard.ulpath,false);
                                 if (v.descr[1]<>'') then f.vpointer:=nfvpointer;
                                 ok:=true;
                                 decide:=true;
                              end;
                           end;
                        until (decide) or (addall)
                     else begin
                        thisuser.ac:=thisuser.ac-[pause];
                        f.filename:=fn;
                        f.description:='';
                        v.descr[1]:='';
                        rr:=false;
                        hangafter:=false;
                        dodescrs(f,v,pl,bb,hangafter,memuboard.ulpath,(not addall));
                        if (v.descr[1]<>'') then f.vpointer:=nfvpointer;
                        ok:=true;
                     end;
                     if (v.descr[1]='') then f.vpointer:=-1;
                     doffstuff(f,fn,gotpts);
                     if (ok) then begin
                        newff(f,v);
                        sysoplog(#3#3+'Upload "'+sqoutsp(fn)+'" on '+memuboard.name);
                     end;
                  end;
               end;
            end;
            nfile;
         end;
      end;
      fileboard:=oldboard;
      close(ulff);
      reset(ulff);
      status^.ulcheckit:=ulcheck;
   end;

begin
   nl;
   print('Upload files into directories -'); nl;
   abort:=FALSE;
   next:=FALSE;
   sall:=pynq(strings^.globalnewscan,true);
   nl;
   if (sall) then begin
      bn:=1;
      while (not abort) and (bn<=maxulb) and (not hangup) do begin
         if (fbaseac(bn)) then uploadfiles(bn,abort,next);
         inc(bn);
         wkey(abort,next);
         if (next) then abort:=FALSE;
      end;
   end
   else uploadfiles(fileboard,abort,next);
end;

end.
