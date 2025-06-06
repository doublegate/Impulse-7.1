{ MENU EDITOR }
{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit sysop7;

interface

uses
   Crt,      Dos,      common5,  records,  common,   common1,  common2,
   common3,  sysop7m,  file9,    menus2,   strproc,  file2;

procedure menu_edit;

implementation

var menuchanged:boolean;
    filv:file;

function readin:boolean;                    (* read in the menu file curmenu *)
begin
   noc:=0;
   assign(filv,curmenu);
   {$I-} reset(filv,1); {$I+}
   if ioresult<>0 then begin
      print(curmenu+' does not exist.');
      readin:=FALSE;
   end
   else begin
      menuchanged:=FALSE;
      blockread(filv,menur,sizeof(menur));
      if fileSize(filv) >= sizeOf(menur) + sizeOf(cmdr[noc]) then begin
         repeat
            inc(noc);
            blockread(filv,cmdr[noc],sizeof(cmdr[noc]));
         until (eof(filv));
      end;
      close(filv);
      readin:=TRUE;
   end;
end;

procedure menu_edit;
const showcmdtype:integer=0;
      menudata:boolean=FALSE;
var nocsave,i,i1,i2,ii:integer;
    c:char;
    abort:boolean;
    s,scurmenu:astr;

   procedure makenewfile(fn:astr);                 (* make a new command list *)
   var f:file;
       m : menurec;
       c : commandrec;
   begin
      assign(f,fn);
      {$I-} rewrite(f,1); {$I+}
      if (ioresult=0) then begin
         with m do begin
            menuname[1] := 'New Menu';
            menuname[2] := '';
            menuname[3] := '';
            directive := '';
            tutorial := '';
            menuprompt := 'Command? ';
            acs := '';
            password := '';
            fallback := status^.allstartmenu;
            forcehelplevel := 0;
            gencols := 4;
            gcol[1] := 9;
            gcol[2] := 0;
            gcol[3] := 1;
            menuflags := [];
         end;
         with c do begin
            ldesc := '';
            sdesc := '';
            xpoint := '';
            ypoint := '';
            ckeys := '';
            acs := '';
            cmdkeys := '';
            mstring := '';
            commandflags := [];
         end;
         blockwrite(f,m,sizeof(m));
         blockwrite(f,c,sizeof(c));
         close(f);
      end;
   end;

   procedure newcmd(n:integer);                          { new command stuff }
   begin
      with cmdr[n] do begin
         ldesc:='[N] New Command';
         sdesc:='';
         xpoint:='0';
         ypoint:='0';
         ckeys:='N';
         acs:='';
         cmdkeys:='';
         mstring:='';
         if (pullDown in menuR.menuFlags) then commandFlags := +[pull]
         else commandflags:=[];
      end;
   end;

   procedure moveinto(i1,i2:integer);
   begin
      cmdr[i1]:=cmdr[i2];
   end;

   procedure mes;
   var i:integer;
   begin
      rewrite(filv,1);
      blockwrite(filv,menur,sizeof(menur));
      for i:=1 to noc do blockwrite(filv,cmdr[i],sizeof(cmdr[i]));
      close(filv);
      sysoplog('* Saved menu file: '+scurmenu);
   end;

   procedure med;
   begin
      prompt('Delete menu file: ');
      input(s,8);
      s:=status^.menupath+strallcaps(s)+'.MNU';
      assign(filv,s);
      {$I-} reset(filv,1); {$I+}
      if (ioresult=0) then begin
         close(filv);
         sprint('|NLMenu file: "'+s+'"');
         changeablemci := 'delete the menu';
         if pynq(strings^.areyousure,false) then begin
            sysoplog('* Deleted menu file: "'+s+'"');
            erase(filv);
         end;
      end;
   end;

   procedure mei;
   begin
      prompt('Insert menu file: ');
      input(s,8);
      s:=status^.menupath+strallcaps(s)+'.MNU';
      assign(filv,s);
      {$I-} reset(filv,1); {$I+}
      if (ioresult=0) then close(filv)
      else begin
         sysoplog('Inserted new menu file: "'+s+'"');
         makenewfile(s);
      end;
   end;

   procedure mec;
   var str : string;
       ok,
       noSpace : boolean;
   begin
      prompt('Copy menu file from: ');
      input(s,8);
      s:=status^.menupath+strallcaps(s)+'.MNU';
      prompt('Copy menu file to: ');
      input(str,8);
      str := status^.menupath+strallcaps(str)+'.MNU';
      copyFile(ok,noSpace,false,s,str);
   end;

   procedure mem;
   var i,j,k:integer;
       c:char;
       b:byte;
       bb:boolean;

      procedure memd(i:integer);                   (* delete command from list *)
      var x:integer;
      begin
         if (i>=1) and (i<=noc) then begin
            for x:=i+1 to noc do cmdr[x-1]:=cmdr[x];
            dec(noc);
         end;
      end;

      procedure memi(i:integer);             (* insert a command into the list *)
      var x:integer;
         s:astr;
      begin
         if (i>=1) and (i<=noc+1) and (noc<50) then begin
            inc(noc);
            if (i<>noc) then for x:=noc downto i do cmdr[x]:=cmdr[x-1];
            newcmd(i);
         end;
      end;

      procedure memp;
      var i,j,k:integer;
      begin
         prompt('Move which command? (1-'+cstr(noc)+') : ');
         inu(i);
         if ((not badini) and (i>=1) and (i<=noc)) then begin
            prompt('Move before which command? (1-'+cstr(noc+1)+') : ');
            inu(j);
            if ((not badini) and (j>=1) and (j<=noc+1) and (j<>i) and (j<>i+1)) then begin
               memi(j);
               if j>i then k:=i
               else k:=i+1;
               cmdr[j]:=cmdr[k];
               if j>i then memd(i)
               else memd(i+1);
               menuchanged:=TRUE;
            end;
         end;
      end;

      function sfl(b:boolean; c:char):char;
      begin
         if (b) then sfl:=c
         else sfl:='-';
      end;

   begin
      prompt('Modify menu file: ');
      input(s,8);
      assign(filv,status^.menupath+s+'.MNU');
      {$I-} reset(filv,1); {$I+}
      if ioresult=0 then begin
         close(filv);
         scurmenu:=s;
         curmenu:=status^.menupath+scurmenu+'.MNU';
         if readin then begin
            menuchanged:=FALSE;
            repeat
               if (c<>'?') then begin
                  cls;
                  abort:=FALSE;
                  if (menudata) then begin
                     sprint('|U5þ |U1Menu filename: |U0'+scurmenu);
                     if (not abort) then begin
                        with menur do begin
                        sprint('|U5[|U01|U5]|U1 Menu titles   : '+menuname[1]);
                        sprint('|U5[|U02|U5]|U1 Help files    : '+
                               aonoff((directive=''),'*Generic*',directive)+' / '+
                               aonoff((tutorial=''),'*Generic*',tutorial));
                        sprompt('|U5[|U03|U5]|U1 Prompt        : ');
                        print(menuprompt);
                        print('('+menuprompt+')');
                        sprint('|U5[|U04|U5]|U1 ACS required  : "'+acs+'"');
                        sprint('|U5[|U05|U5]|U1 Password      : '+
                               aonoff((password=''),'*None*',password));
                        sprint('|U5[|U06|U5]|U1 Fallback menu : '+
                               aonoff((fallback=''),'*None*',fallback));
                        sprint('|U5[|U07|U5]|U1 Forced ?-level: '+
                               aonoff((forcehelplevel=0),'None',cstr(forcehelplevel)));
                        { sprint('|U5[|U08|U5]|U1 Generic info  : '+cstr(gencols)+' cols - '+
                          cstr(gcol[1])+'/'+cstr(gcol[2])+'/'+cstr(gcol[3]));}
                        sprint('|U5[|U09|U5]|U1 Flags         : '+
                               sfl((clrscrbefore in menuflags),'C')+
                               sfl((dontcenter in menuflags),'D')+
                               sfl((nomenuprompt in menuflags),'N')+
                               sfl((forcepause in menuflags),'F')+
                               sfl((pulldown in menuflags),'P')+
                               sfl((autotime in menuflags),'T'));
                        sprint('|U5[|U0Q|U5]|U1 Quit');
                     end;
                  end;
                  end
                  else showcmds(showcmdtype);
               end;
               nl;
               sprompt('|15Menu Editing |08[|07?/Help|08]:|03 ');
               onek(c,'ABQDILMPSTX12345679?'^M);
               case c of
                  'A':begin
                     prompt('Enter the Top Left X point (1-80) : ');
                     ini(b);
                     if (not badini) and (b>0) and (b<=80) then begin
                        menur.menuname[2]:=cstr(b);
                        menuchanged:=TRUE;
                     end;
                  end;
                  'B':begin
                     prompt('Enter the Top Left Y point (1-25) : ');
                     ini(b);
                     if (not badini) and (b>0) and (b<=25) then begin
                        menur.menuname[3]:=cstr(b);
                        menuchanged:=TRUE;
                     end;
                  end;
                  '?':begin
                     sprompt('|NL|U9[|U0CR|U9]|U0 Redisplay screen|NL');
                     lcmds(20,3,'Delete command','PMove commands');
                     lcmds(20,3,'Insert command','Toggle display type');
                     lcmds(20,3,'Modify commands','XMenu data/command data');
                     lcmds(20,3,'Quit and save','');
                  end;
                  'D':begin
                     prompt('Delete which command? (1-'+cstr(noc)+') : ');
                     ini(b);
                     if (not badini) and (b>=1) and (b<=noc) then begin
                        memd(b);
                        menuchanged:=TRUE;
                     end;
                  end;
                  'I':if (noc<50) then begin
                     prompt('Insert before which command? (1-'+cstr(noc+1)+') : ');
                     inu(i);
                     if (not badini) and (i>=1) and (i<=noc+1) then begin
                        prompt('Insert how many commands? (1-'+cstr(50-noc)+') [1] : ');
                        inu(j);
                        if (badini) then j:=1;
                        if (j>=1) and (j<=50-noc) then begin
                           for k:=1 to j do memi(i);
                           menuchanged:=TRUE;
                        end;
                     end;
                  end
                  else begin
                     sprint('You already have 50 commands, delete some to make room.|NL');
                     pausescr;
                  end;
                  'M': memm(scurmenu,menuchanged);
                  'P': memp;
                  'T': begin
                     if (showCmdType = 0) then showCmdType := 1
                     else ShowCmdType := 0;
                  end;
                  'X': menudata:=not menudata;
                  '1':begin
                     sprompt('|NL|NLNew menu title: ');
                     inputwnwc(menur.menuname[1],100,menuchanged);
                  end;
                  '2':begin
                     sprompt('|NL|NL|U5New file displayed for help: ');
                     inputwn(menur.directive,12,menuchanged);
                     menur.directive:=strallcaps(menur.directive);
                     sprompt('|NLNew tutorial file? [default ext=".TUT"] : ');
                     inputwn(menur.tutorial,12,menuchanged);
                     menur.tutorial:=strallcaps(menur.tutorial);
                     nl;
                  end;
                  '3':begin
                     nl;
                     prompt('New menu prompt: ');
                     inputwnwc(menur.menuprompt,120,menuchanged);
                  end;
                  '4':begin
                     nl;
                     prompt('New menu ACS: ');
                     inputwn(menur.acs,20,menuchanged);
                  end;
                  '5':begin
                     nl;
                     prompt('New password: ');
                     inputwn1(menur.password,15,'u',menuchanged);
                  end;
                  '6':begin
                     nl;
                     prompt('New fallback menu: ');
                     inputwn1(menur.fallback,8,'u',menuchanged);
                  end;
                  '7':begin
                     nl;
                     prompt('New forced menu help-level (1-3,0=None) ['+
                            cstr(menur.forcehelplevel)+'] : ');
                     ini(b);
                     if ((not badini) and (b in [0..3])) then begin
                       menuchanged:=TRUE;
                       menur.forcehelplevel:=b;
                     end;
                  end;
                  {            '8':begin
                  repeat
                    nl;
                    print('C. Generic columns  :'+cstr(menur.gencols));
                    print('1. Bracket color    :'+cstr(menur.gcol[1]));
                    print('2. Command color    :'+cstr(menur.gcol[2]));
                    print('3. Description color:'+cstr(menur.gcol[3]));
                    print('S. Show menu');
                    nl;
                    prompt('Select (CS,1-3,Q=Quit) : '); onek(c,'QCS123'^M);
                    nl;
                    if (c='S') then genericmenu(2);
                    if (c in ['C','1'..'3']) then begin
                      case c of
                        'C':prompt('New number of generic columns (2-7) ['+
                                cstr(menur.gencols)+'] : ');
                      else
                            prompt('New generic menu color '+c+' (0-9) ['+
                                cstr(menur.gcol[ord(c)-48])+'] : ');
                      end;
                      ini(b);
                      if (not badini) then
                        case c of
                          'C':if (b in [2..7]) then begin
                                menuchanged:=TRUE;
                                menur.gencols:=b;
                              end;
                        else
                              if (b in [0..9]) then begin
                                menuchanged:=TRUE;
                                menur.gcol[ord(c)-48]:=b;
                              end;
                        end;
                    end;
                  until ((not (c in ['C','S','1'..'3'])) or (hangup));
                  c:=#0;
                end;}
                  '9':begin
                     nl;
                     print('(C)lear screen before menu  -  (D)on''t center the menu titles');
                     print('(N)o menu prompt displayed  -  (F)Force pause before menu display');
                     print('(T)Auto-time display        -  (P)ulldown menu toggle');
                     nl;
                     prompt('Choose : ');
                     onek(c,'QCDFNPT'^M);
                     bb:=menuchanged;
                     menuchanged:=TRUE;
                     with menur do
                        case c of
                           'C':if (clrscrbefore in menuflags) then menuflags:=menuflags-[clrscrbefore]
                              else menuflags:=menuflags+[clrscrbefore];
                           'D':if (dontcenter in menuflags) then menuflags:=menuflags-[dontcenter]
                              else menuflags:=menuflags+[dontcenter];
                           'N':if (nomenuprompt in menuflags) then menuflags:=menuflags-[nomenuprompt]
                              else menuflags:=menuflags+[nomenuprompt];
                           'F':if (forcepause in menuflags) then menuflags:=menuflags-[forcepause]
                              else menuflags:=menuflags+[forcepause];
                           'P':if (pulldown in menuflags) then menuflags:=menuflags-[pulldown]
                              else menuflags:=menuflags+[pulldown];
                           'T':if (autotime in menuflags) then menuflags:=menuflags-[autotime]
                              else menuflags:=menuflags+[autotime];
                           else menuchanged:=bb;
                        end;
                     c:=#0;
                  end;
               end;
            until ((c='Q') or (hangup));
            if (menuchanged) then begin
               sprint('Saving Menu.');
               mes;
            end;
         end;
      end;
   end;

begin
   nocsave:=noc;
   noc:=0;
   repeat
      abort:=FALSE;
      if (c<>'?') then begin
         cls;
         writehdr('Menu Editing');
         nl;
         dir(status^.menupath,'*.mnu',FALSE);
      end;
      sprompt('|NL|U1Menu Editor |U9[|U1?\Help|U9]:|U3 ');
      onek(c,'CQDIM?'^M);
      case c of
         '?':begin
            sprompt('|NL|U9[|U0CR|U9]|U0 Redisplay screen|NL');
            lcmds(17,3,'Delete menu file','Insert menu file');
            lcmds(17,3,'Modify menu file','Copy menu file');
            lcmds(17,3,'Quit and save','');
          end;
          'D':med;
          'I':mei;
          'M':mem;
          'C':mec;
      end;
   until (c='Q') or (hangup);
   noc:=nocsave;
end;

end.
