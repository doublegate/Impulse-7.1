{ Main impulse code }

(* Compiler Switches *)
{$A+,B+,F+,I+,L-,O+,R-,S-,V-,Q-}

(* Memory Allocation (Stack Size, MinHeap, MaxHeap) *)
{$M 35520,0,205520}
Program Impulse;

{ Include Files }
Uses
   Checkpat, Crt,      Overlay,  OvrLay,   BpTrap,   InitP,    Menus2,
   Dos,      AnsiDrv,  Sysop7,   Sysop8,   Sysop9,   Sysop11,  Rumors,
   ScrlBk,   Mail0,    Mail1,    Mail2,    Mail3,    Mail4,    Mail5,
   Mail6,    Mail9,    File0,    File1,    File2,    File4,    File5,
   File6,    File8,    File9,    File10,   File11,   File12,   File13,
   File14,   Misc1,    WfcMenu,  Misc2,    Misc3,    Misc4,    Misc5,
   CUser,    Doors,    Menus3,   MyIO,     Logon1,   Logon2,   NewUsers,
   Common1,  Menus,    TimeJunk, Tmpcom,   MsgPack,  Common,   Common5,
   multinod, Common2,  Common3,  Ulcheck,  Common4,  Records,  cmd,
   sys,      outPut,   strProc,  strProc2, MModem,   Sysop3;

{ Overlayed Files }
   (* File System *)
   {$O File0   } {$O File1   } {$O File2   } {$O File3   } {$O File4   }
   {$O File5   } {$O File6   } {$O File8   } {$O File9   } {$O File10  }
   {$O File11  } {$O File12  } {$O File13  } {$O File14  }

   (* Logon/Logoff System *)
   {$O Logon1  } {$O Logon2  }

   (* Message/Email System *)
   {$O Mail0   } {$O Mail1   } {$O Mail2   } {$O Mail3   } {$O Mail4   }
   {$O Mail5   } {$O Mail6   } {$O Mail7   } {$O Mail9   }

   (* Misc Commands *)
   {$O Misc1   } {$O Misc2   } {$O Misc3   } {$O Misc4   } {$O Misc5   }

   (* Sysop Config System *)
   {$O Sysop1  } {$O Sysop2  } {$O Sysop21 } {$O Sysop2a } {$O Sysop2b }
   {$O Sysop2c } {$O Sysop2d } {$O Sysop2e } {$O Sysop2f } {$O Sysop2g }
   {$O Sysop2h } {$O Sysop2i } {$O Sysop2j } {$O Sysop2s } {$O Sysop2z }
   {$O Sysop3  } {$O Sysop7  } {$O Sysop7m } {$O Sysop8  } {$O Sysop9  }
   {$O Sysop11 }

   (* Misc Stuff *)
   {$O ansiedit} {$O Cmd     } {$O Conf    } {$O CUser   } {$O Doors   }
   {$O Execbat } {$O InitP   } {$O Islu    } {$O LZHview } {$O MModem  }
   {$O MsgPack } {$O Multinod} {$O NewUsers} {$O ULcheck } {$O Uconfig }
   {$O Rumors  } {$O Script  } {$O StrProc2} {$O Wfcmenu } {$O ZIPviewu}
   {$O Nuv     } {$O TimeBank} {$O impDos}
var
   ExecFirst  : boolean;
   NewMenuCmd : string[80];

Procedure MenuExec;
Var
   Cmd : str80;
   Done : boolean;
Begin
   done := true;
   If (ExecFirst) then Begin
      ExecFirst := FALSE;
      Cmd := NewMenuCmd;
      NewMenuCmd := '';
   End
   Else MainMenuHandle(Cmd);
   if ((copy(cmd,1,2)='\\') and (thisuser.sl=255)) then begin
      domenucommand(done,copy(cmd,3,length(cmd)-2),newmenucmd);
      if (newmenucmd<>'') then cmd:=newmenucmd
      else cmd:='';
   end;
   newmenucmd:='';
   if (cmd='') then
      repeat
         domenuexec('ENTER',newmenucmd)
      until (newmenucmd='');
   repeat
      domenuexec(cmd,newmenucmd);
   until (newmenucmd='');
end;

Procedure DoBBS;
Var
   OvrPath : String;
   I,
   RCode         : Integer;
   NeedToHangup    : Boolean;
   SyStatF         : File of systatrec;
   ExitingBBS : Boolean;
Begin
   ExitingBBS:=False;
   ovrpath:=fsearch('IMP.OVR',getenv('PATH'));
   checksnow:=FALSE;
   directvideo:=TRUE;
   useron:=FALSE; usernum:=0;
   getdir(0,start_dir);
   new(status);
   Assign(Systatf, 'STATUS.DAT');
   {$I-}Reset(Systatf);{$I+}
   if (iOresult<>0) then begin
      writeln;
      writeln('Unable to access STATUS.DAT');
      writeln;
      halt(exiterrors);
   end
   else {$I-} read(systatf,status^); {$I+}
   close(systatf);
   usefossil:=true;
   okscrollback:=false;  {Scrollback is off initially}
   beforeview:=false;    {Another Scrollback Boolean}
   OvrInit('IMP.OVR');
   If (OvrResult <> OvrOk) Then Begin
      writeln('Critical error: Overlay manager error.');
      halt(1);
   End
   Else If (status^.useems) then Begin
      OvrInitXms;
      If (OvrResult <> OvrOk) Then OvrInitEms;
      overlayinems:=true;
   End;
   cls;
   if (status^.specialfx) then begin
      highbackground(true);
      textcolor($1F);
   end
   else begin
      highbackground(false);
      textattr := 30;
   end;
   gotoxy(1,1);
   write(expand(' ',80));
   gotoXy(1,1);
   cwritecentered(1,ProgName+' v'+ver+' Startup Sequence');
   window(1,2,80,25);
   textcolor(7);
   initBBs;
   if (packbasesonly) then begin
      wfcmdefine;
      doshowpackbases;
      thisuser.ac:=thisuser.ac-[pause];
      nl;
      sprint(#3#5+'Message bases have been packed.');
      cursoron(TRUE);
      ExitingBBS:=true;
      halt(exitNormal);
   end;
   mailread:=FALSE;
   needtohangup:=false;
   trap;
   while (not doneday) do begin
      write_msg:=FALSE;
      sysopon:=not status^.localsec;
      wantout:=not status^.localscreensec;
      if status^.useScrollBack then begin
         if okscrollback then begin      {Get rid of scrollback shit from memory}
            okscrollback:=false;
            dispose(scrlpt);
            dispose(scrlatpt);
         end;
      end;
      wfcmenus(needtohangup);
      needtohangup:=FALSE;
      useron:=FALSE;
      usernum:=0;
      if (not doneday) then
         while (not hangup) do begin
            if (getuser) then newuser;
            if (not hangup) then begin
               macok:=TRUE;
               if (not hangup) then logon;
               if (not hangup) then begin
                  with thisuser do begin
                     newdate:=laston;
                     if (not mbaseac(lastmsg)) then lastmsg:=1;
                     if (not fbaseac(lastfil)) then lastfil:=1;
                     if lastmsg=0 then lastmsg:=1;
                     if lastfil=0 then lastmsg:=1;
                     board:=lastmsg;
                     fileboard:=lastfil;
                  end;
                  batchtime:=0.0;
                  numbatchfiles:=0;
                  numubatchfiles:=0;
                  hiubatchv:=0;
                  newcomptables;
                  menustackptr:=0;
                  for i:=1 to 8 do menustack[i]:='';
                  last_menu:=status^.allstartmenu+'.MNU';
                  if (not exist(status^.menupath+last_menu)) then begin
                     sysoplog('"'+status^.menupath+last_menu+'" is MISSING.');
                     last_menu:=status^.allstartmenu;
                  end;
                  curmenu:=status^.menupath+last_menu;
                  readin;
                  if (novice in thisuser.ac) then chelplevel:=3
                  else chelplevel:=1;
               end;
               newmenucmd:='';
               i:=1;
               while ((i<=noc) and (newmenucmd='')) do begin
                  if (cmdr[i].ckeys='FIRSTCMD') then
                     if (aacs(cmdr[i].acs)) then newmenucmd:='FIRSTCMD';
                  inc(i);
               end;
               execfirst:=(newmenucmd='FIRSTCMD');
               while (not hangup) do menuexec;    {*** main BBS loop ***}
            end;
         logoff;
         if (quitafterdone) then begin
            elevel:=exitnormal;
            hangup:=TRUE;
            doneday:=TRUE;
            needtohangup:=TRUE;
         end;
         if (not doneday) then sl1(#3#3+'Logoff '+#3#5+'['+dat+']');
         if (textrec(sysopf1).mode=fmoutput) then begin
            {$I-} close(sysopf1); {$I+}
            if (ioresult<>0) then writeln('Errors closing SLOGxxxx.LOG');
         end;
         if ((com_carrier) and (not doneday)) then
            if (spd<>strings^.locallogin) then needtohangup:=TRUE;
      end;
   end;
   if (needtohangup) then dophonehangup(TRUE);
   if (exist('bbsdone.bat')) then shelldos(FALSE,'bbsdone.bat',rcode);
   exitingbbs := true;
   reset(sysopf);
   close(sysopf);
   term_ready(TRUE);
   remove_port;
   makenode(0);
   makenodeon('Nobody',0);
   exitingbbs := true;
   exitbbs(elevel);
end;

Begin
   filemode:=66;                                 { sets file sharing on }
   doBbs;                                        { the main bbs loop    }
End.