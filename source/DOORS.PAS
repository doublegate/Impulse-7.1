{ Doors }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit doors;

interface

uses
   Crt,      Dos,      common5,  records,  execbat,  tmpcom,   common1,
   common2,  common3,  common,   strProc;

function process_door(s:astr):astr;
procedure dodoorfunc(kind:char; cline:astr);

implementation

function timestr:astr;
var i:astr;
begin
   i:=cstrr(nsl/60,10);
   timestr:=i;
end;

function process_door(s:astr):astr;
var i:integer;
    sda,namm:astr;
    sdoor:string[255];
begin
   namm:=caps(thisuser.realname);
   sdoor:='';
   for i:=1 to length(s) do begin
      if copy(s,i,1)='@' then begin
         sda:='';
         case upcase(s[i+1]) of
            'B':if spd<>strings^.locallogin then sda:=spd
               else sda:='0';
            'D':begin
               loaduboard(fileboard);
               sda:=memuboard.dlpath;
            end;
            'F':sda:=copy(namm,1,pos(' ',namm)-1);
            'G':if okansi then sda:='1'
               else sda:='0';
            'I':begin
               loaduboard(fileboard);
               sda:=status^.gfilepath;
               if (copy(sda,length(sda),1)<>'\') then sda:=sda+'\';
               sda:=sda+memuboard.filename+'.DIR';
            end;
            'L':if (pos(' ',namm)=0) then sda:=namm
               else sda:=copy(namm,pos(' ',namm)+1,length(namm));
            'N':sda:=caps(thisuser.name);
            'T':sda:=timestr;
            'R':sda:=(copy(nam,pos('#',nam)+1,length(nam)));
         end;
         sdoor:=sdoor+sda;
         inc(i);
      end
      else sdoor:=sdoor+copy(s,i,1);
   end;
   process_door:=sdoor;
end;

procedure imp_drop_file;
var fp:text;
begin
   assign(fp,ProgName+'.IDF');
   rewrite(fp);
   with thisuser do begin
      if (usefossil) then writeln(fp,'COMDRIVER:    FOSSIL')
      else if (spd=strings^.locallogin) then writeln(fp,'COMDRIVER:    LOCAL')
      else writeln(fp,'COMDRIVER:    INTERNAL');
      writeln(fp,'COMPORT:      '+cstr(modemr.comport));
      {writeln(fp,'COMIRQ:       '+cstr(modemr.irqnum));}
      {writeln(fp,'COMBUFFER:    '+cstr(modemr.inoutbuff));}
      writeln(fp,'BPSRATE:      '+spd);
      writeln(fp,'VERSION:      v'+ver);
      writeln(fp,'BBSNAME:      '+status^.bbsname);
      writeln(fp,'SYSOPNAME:    '+status^.sysopname);
      writeln(fp,'HANDLE:       '+caps(name));
      writeln(fp,'REALNAME:     '+realname);
      writeln(fp,'PASSWORD:     '+pw);
      writeln(fp,'SLEVEL:       '+cstr(sl));
      writeln(fp,'DLEVEL:       '+cstr(dsl));
      writeln(fp,'FILEPOINTS:   '+cstr(filepoints));
      writeln(fp,'PROMPT#:      '+cstr(prompt));
      if (thisuser.ac=thisuser.ac+[avatar]) then
      writeln(fp,'EMULATION:    AVATAR') else
      writeln(fp,'EMULATION:    ANSI');
      writeln(fp,'UPLOADS:      '+cstr(uploads));
      writeln(fp,'DOWNLOADS:    '+cstr(downloads));
      writeln(fp,'UPLOADED-K:   '+cstr(uk));
      writeln(fp,'DOWNLOADED-K: '+cstr(dk));
      writeln(fp,'CURRENT-TIME: '+timestr);
      writeln(fp,'TIME-LEFT:    '+cstr(tltoday));
   end;
   close(fp);
end;

procedure write_dorinfo1_def(rname:boolean);  (* RBBS-PC's DORINFO1.DEF *)
var fp:text;
    first,last:astr;
    s:astr;
begin
   assign(fp,'dorinfo1.def');
   rewrite(fp);
   writeln(fp,stripcolor(status^.bbsname));
   first:=copy(status^.sysopname,1,pos(' ',status^.sysopname)-1);
   last:=copy(status^.sysopname,length(first)+2,length(status^.sysopname));
   writeln(fp,first);
   writeln(fp,last);
   if spd=strings^.locallogin then writeln(fp,'COM0') else writeln(fp,'COM'+cstr(modemr.comport));
   if spd=strings^.locallogin then s:='0' else s:=spd;
   writeln(fp,s+' BAUD,N,8,1');
   writeln(fp,'0');
   if (rname) then begin
      if pos(' ',thisuser.realname)=0 then begin
         first:=thisuser.realname;
         last:='';
      end
      else begin
         first:=copy(thisuser.realname,1,pos(' ',thisuser.realname)-1);
         last:=copy(thisuser.realname,length(first)+2,length(thisuser.realname));
      end;
      first:=strallcaps(first);
      last:=strallcaps(last);
   end
   else begin
      if pos(' ',thisuser.name)=0 then begin
         first:=thisuser.name;
         last:='';
      end else begin
         first:=copy(thisuser.name,1,pos(' ',thisuser.name)-1);
         last:=copy(thisuser.name,length(first)+2,length(thisuser.name));
      end;
   end;
   writeln(fp,caps(first));
   writeln(fp,caps(last));
   writeln(fp,thisuser.citystate);
   if (ansi in thisuser.ac) then writeln(fp,'1')
   else writeln(fp,'0');
   writeln(fp,thisuser.sl);
   s:=timestr;
   if length(s)>3 then s:='999';
   writeln(fp,s);
   writeln(fp,'0');
   close(fp);
end;

procedure write_door_sys(rname:boolean);    (* GAP's DOOR.SYS *)
var fp:text;
    i:integer;
    s:astr;
begin
  assign(fp,'door.sys');
  rewrite(fp);
  if spd<>strings^.locallogin then writeln(fp,'COM'+cstr(modemr.comport)+':') else writeln(fp,'COM0:');
  if spd<>strings^.locallogin then writeln(fp,spd) else writeln(fp,cstr(modemr.waitbaud));
	writeln(fp,' 8');
	writeln(fp,' 1');
	writeln(fp,' N');
	if wantout then writeln(fp,' Y') else writeln(fp,' N');
	writeln(fp,' Y');
        if (so) then writeln(fp,' Y') else writeln(fp,' N');
	if alert in thisuser.ac then writeln(fp,' Y') else writeln(fp,' N');
  if (rname) then writeln(fp,thisuser.realname) else writeln(fp,thisuser.name);
  writeln(fp,thisuser.citystate);
  writeln(fp,copy(thisuser.ph,1,3)+' '+copy(thisuser.ph,5,8));
  writeln(fp,copy(thisuser.ph,1,3)+' '+copy(thisuser.ph,5,8));
  writeln(fp,thisuser.pw);
  writeln(fp,cstr(thisuser.sl));
  writeln(fp,cstr(thisuser.loggedon));
  writeln(fp,thisuser.laston);
  writeln(fp,cstrl(trunc(nsl)));
  writeln(fp,cstr(trunc(nsl) div 60));
  if okansi then writeln(fp,'GR') else writeln(fp,'NG');
  writeln(fp,cstr(thisuser.pagelen));
	if novice in thisuser.ac then writeln(fp,' N') else writeln(fp,' Y');
  s:='';
  for i:=1 to 7 do
    if chr(i+64) in thisuser.ar then s:=s+cstr(i);
  writeln(fp,s);
  writeln(fp,'7');
	writeln(fp,'12/31/99');
	writeln(fp,' '+cstr(usernum));
	writeln(fp,' X');
	writeln(fp,' '+cstr(thisuser.uploads));
	writeln(fp,' '+cstr(thisuser.downloads));
	writeln(fp,' '+cstr(trunc(thisuser.dk)));
	writeln(fp,' 999999');
  close(fp);
end;

procedure write_chain_txt;
var fp:text;
    ton,tused:real;
    s:string[20];

   function bo(b:boolean):astr;
   begin
     if b then bo:='1'
     else bo:='0';
   end;

begin
   assign(fp,'chain.txt');
   rewrite(fp);
   with thisuser do begin
      writeln(fp,usernum);                      { user number        }
      writeln(fp,name);                         { user name          }
      writeln(fp,realname);                     { real name          }
      writeln(fp,'');                           { "call sign" ?      }
      writeln(fp,ageuser(bday));                { age                }
      writeln(fp,sex);                          { sex                }
      str(credit:7,s);
      writeln(fp,s+'.00');     { credit             }
      writeln(fp,laston);                       { laston date        }
      writeln(fp,linelen);                      { # screen columns   }
      writeln(fp,pagelen);                      { # screen rows      }
      writeln(fp,sl);                           { SL                 }
      writeln(fp,bo(so));                       { is he a SysOp?     }
      writeln(fp,bo(cso));                      { is he a CoSysOp?   }
      writeln(fp,bo(okansi));                   { is graphics on?    }
      writeln(fp,bo(incom));                    { is remote?         }
      str(nsl:10:2,s);
      writeln(fp,s);           { time left (sec)    }
      writeln(fp,status^.gfilepath);             { gfiles path        }
      writeln(fp,status^.gfilepath);             { data path          }
      writeln(fp,'SYSOP.LOG');                  { SysOp log filespec }
      if (spd=strings^.locallogin) then s:='0'           { baud rate          }
      else s := spd;
      writeln(fp,s);
      writeln(fp,modemr.comport);               { COM port           }
      writeln(fp,stripcolor(status^.bbsname));   { system name        }
      writeln(fp,status^.sysopname);             { SysOp's name       }
      with timeon do begin
         ton:=hour*3600.0+min*60.0+sec;
         tused:=timer-ton;
         if (tused<0) then tused:=tused+3600.0*24.0;
      end;
      writeln(fp,trunc(ton));                   { secs on f/midnight }
      writeln(fp,trunc(tused));                 { time used (sec)    }
      writeln(fp,uk);                           { upload K           }
      writeln(fp,uploads);                      { uploads            }
      writeln(fp,dk);                           { download K         }
      writeln(fp,downloads);                    { downloads          }
      writeln(fp,'8N1');                        { COM parameters     }
   end;
   close(fp);
end;

procedure dodoorfunc(kind:char; cline:astr);
var doorstart,doorend,doortime:datetimerec;
    s,cline2:astr;
    retcode,savsl,savdsl:integer;
    realname:boolean;
begin
   realname:=FALSE;
   if ((sqoutsp(cline)='') and (incom)) then begin
      print('This command is inoperative!');
      if (cso) then print('(A CString of "" will shell to DOS LOCALLY!)');
      exit;
   end;

   if ((realsl<>-1) and (realdsl<>-1)) then begin
      savsl:=thisuser.sl;
      savdsl:=thisuser.dsl;
      thisuser.sl:=realsl;
      thisuser.dsl:=realdsl;
      saveurec(thisuser,usernum);
   end;

   cline2:=cline;
   if copy(strallcaps(cline2),1,2)='R;' then begin
      realname:=TRUE;
      cline2:=copy(cline2,3,length(cline2)-2);
   end;
   s:=process_door(cline2);
   case kind of
      'I':begin
         commandline('Running An Impulse Utiltiy/External Program');
         commandline('Creating IMPULSE.IDF');
         imp_drop_file;
      end;
      'C':begin
         commandline('Outputting CHAIN.TXT ...');
         write_chain_txt;
      end;
      'D':begin
         commandline('Outputting DORINFO1.DEF ...');
         write_dorinfo1_def(realname);
      end;
      'G':begin
         commandline('Outputting DOOR.SYS ...');
         write_door_sys(realname);
      end;
   end;
   commandline('Now running "'+s+'"');
   sysoplog('>> '+date+' '+time+'- Door "'+s+'"');
   close(sysopf);
   getdatetime(doorstart);
   shel('');
   shelldos(FALSE,s,retcode);
   shel2;
   getdatetime(doorend);
   timediff(doortime,doorstart,doorend);
   chdir(start_dir);
   append(sysopf);
   if ((realsl<>-1) and (realdsl<>-1)) then begin
      reset(uf);
      seek(uf,usernum);
      read(uf,thisuser);
      close(uf);
      thisuser.sl:=savsl;
      thisuser.dsl:=savdsl;
   end;
   com_flush_rx;
   getdatetime(tim);
   sysoplog('>> '+date+' '+time+'- Returned (spent '+longtim(doortime)+')');
end;

end.
