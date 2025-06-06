(* Piece of Crap User Editor *)
{$A+,B+,F+,I+,L+,O+,R-,S-,V-,P-}
unit sysop3;

interface

uses
   Crt,      Dos,      myio,     common1,  common2,  common3,  common5,
   common,   records,  strproc,  nuv;

procedure restric_list;
procedure uedit1;
Procedure PullEdit;
procedure uedit(usern:integer);
procedure nukeuser(uname : string; unum : word);

implementation

uses
   mail0,    menus,    misc3,    misc4,    misc5,    cuser;

procedure killusermail(usern : word);
var pinfo:pinforec;
    mixr:msgindexrec;
    i,j:longint;
    s : string;
begin
   savepinfo(pinfo);
   initbrd(-1);
   for i:=0 to himsg do begin
      seek(mixf,i); blockread(mixf,mixr,1);
      j:=mixr.messagenum;
      if ((miexist in mixr.msgindexstat) and (j=usern)) then s:=rmail(i);
   end;
   loadpinfo(pinfo);
end;


Procedure NukeUser(uname : string; unum : word);
var U : UserRec;
    NuvFile : File Of NuvRec;
    ThisNuv : NuvRec;
    found : boolean;
begin
   dsr(uname);
   assign(nuvfile,status^.gfilepath+'nuv.dat');
   {$I-} reset(nuvfile); {$I+}
   if (ioresult<>0) then rewrite(nuvfile);
   found := false;
   repeat
      if (not eof(nuvfile)) then read(nuvfile, thisnuv);
      if thisnuv.handle = uname then found := true;
      if (found) then begin
         thisnuv.exists := false;
         seek(nuvFile,filePos(nuvFile)-1);
         write(nuvfile, thisnuv);
      end;
   until (found) or (eof(nuvfile));
   close(nuvfile);
   reset(uf);
   seek(uf,unum);
   read(uf,u);
   u.deleted:=TRUE;
   seek(uf,unum);
   write(uf,u);
   killusermail(unum);
   if (thisnuv.commentptr<>0) then deletecomments(thisnuv.commentptr);
end;

Procedure PullEdit;
Var List     : Array[1..255] of ^String;
    Cont     ,
    NumUsers ,
    sp       ,
    First    ,
    scp      ,
    page     ,
    cur      : word;
    C        : Char;
    s        : str80;
    C2       : byte;
    wind     : windowrec;
    go       : Boolean;

Procedure ShowUsers;
var u:userrec;
    s:astr;
    i:integer;
begin
   cls;
   reset(uf);
   i:=0;
   NumUsers:=1;
   while (i<filesize(uf)-1) do begin
      new(list[i+1]);
      list[i+1]^ := '';
      inc(i);
      seek(uf, i);
      read(uf,u);
      Inc(NumUsers);
      s := '*DELETED* '+caps(u.name);
      if u.deleted then list[i]^:=s
      else List[i]^:=caps(u.name);
   end;
   close(uf);
end;


  begin
    s := '*DELETED* ';
    reset(uf);
    if fileSize(uf) < 2 then begin
      close(uf);
      exit;
    end;
    close(uf);
    showUsers;
    go := true;
    first := 1;
    sp := 1;
    scp := 1;
    cur := 1;
    page := 0;
    prompt(expand(' ',78-whereX));
    cursoron(false);
    while go do begin
      cl(1);
      scp := 1;
      if numusers < 23 then begin
        for cont := 1 to numusers-1 do begin
          gotoxy(1,scp);
          sprint('['+mln(Cstr(Cont),3)+'] '+List[Cont]^);
          prompt(expand(' ',75-whereX));
{         for c2:=wherex to 75 do write(' ');}
          inc(scp);
        end;
      end
      else begin
        for cont := first to first + 22 do begin
          gotoxy(1,scp);
          sprint('['+mln(Cstr(Cont),3)+'] '+List[Cont]^);
          for c2:=wherex to 75 do write(' ');
          inc(scp);
        end;
      end;
      cl(6);
      gotoxy(1,sp);
      sprint('['+mln(Cstr(cur),3)+'] '+List[cur]^);
      getkey(c);
      c:=upcase(c);
      case c of
        'A','H': Begin { up arrow }
                   if sp >= 1 then dec(sp);
                   if cur > 1 then dec(cur);
                   if (sp = 0) AND (first > 1) then begin
                     dec(first);
                     sp := 1;
                   end
                   else if sp = 0 then sp := 1;
                end;
        'B','P': Begin { down arrow }
                   if sp <> numusers-1 then begin
                     if (sp <= 23) then inc(sp);
                     if cur <= numusers-1 then inc(cur);
                     if (sp = 24) AND (cur <= numusers-1) then begin
                       inc(first);
                       sp := 23;
                     end else if (sp=24) then begin
                       sp := 23;
                       cur := numusers-1;
                     end;
                   end;
                 end;
             ^M: begin { enter key }
                   setwindow(wind,1,1,80,25,7,0,0);
                   cursoron(true);
                   For scp := 1 to numusers-1 do dispose(list[scp]);
                   uedit(cur);
                   showusers;
                   cursoron(false);
                   removewindow(wind);
                 end;
            'Q': go := false;
        'D','S': begin
                   nukeuser(list[cur]^,cur);
                   if pos(s,list[cur]^) = 0 then insert(s,list[cur]^,1);
                 end;
      end;
    end;
    for scp := 1 to numusers-1 do dispose(list[scp]);
end;

procedure uedit1;
begin
  uedit(usernum);
end;

procedure restric_list;
begin
   sprint('|NLRestrictions:|NL');
   lcmds(27,3,'LCan logon ONLY once/day','CCan''t page SysOp');
   lcmds(27,3,'VPosts marked unvalidated','Back/downspacing restricted');
   lcmds(27,3,'ACan''t add to BBS list','*Can''t post/send anon.');
   lcmds(27,3,'PCan''t post at all','ECan''t send email');
   lcmds(27,3,'KCan''t vote','MAutomatic mail deletion');
   sprint('|NLSpecial:|NL');
   lcmds(27,3,'1No UL/DL ratio check','2No post/call ratio check');
   lcmds(27,3,'3No file points check','4Protection from deletion');
   nl;
end;

function spflags(u:userrec):astr;
var r:uflags;
    s:astr;
begin
   s:='';
   for r:=rlogon to rmsg do
      if r in u.ac then s:=s+copy('LCVBA*PEKM',ord(r)+1,1)
      else s:=s+'-';
   s:=s+'/';
   for r:=fnodlratio to fnodeletion do
      if r in u.ac then s:=s+copy('1234',ord(r)-19,1)
      else s:=s+'-';
   spflags:=s;
end;

procedure unwrite(n : integer; x, y : byte);
begin
   goxy(x,y);
   case n of
      0..9 : prompt(cstr(n)+'  ');
      10..99 : prompt(cstr(n)+' ');
      else prompt(cstr(n));
   end;
end;

procedure uwrite(s : string; x, y, max : byte);
var str : string;
begin
   goxy(x,y);
   prompt(s+expand(' ',max-length(s)));
end;

procedure showuiScr;
begin
   cls;
   nl;
   sprint('|U9ÚÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÂÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ¿');
   sprint('|U9³ |U0 '+acronym+' v'+ver+' User Editor     |U9³ |U3[|U1^|U3]'+
          ' |U0Status: |U7            |U3[|U1S|U3]|U0 SL:    |U3[|U1D|U3]|U0 DSL:    |U9³');
   sprint('|U9ÃÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÙ |U3[|U1P|U3]|U0 Phone Number: |U7              '+
          '               |U9³');
   sprint('|U9³ |U3[|U1N|U3]|U0 Handle:                                      '+
          '  |U3[|U13|U3]|U0 File Records        |U9³');
   sprint('|U9³ |U3[|U1R|U3]|U0 Real Name:                                   '+
          '  |U3[|U1#|U3]|U0 Points:             |U9³');
   sprint('|U9³ |U3[|U1A|U3]|U0 Address:                                     '+
          '      UL''s:               |U9³');
   sprint('|U9³ |U3[|U1C|U3]|U0 City/State:                                  '+
          '      DL''s:               |U9³');
   sprint('|U9³ |U3[|U1Z|U3]|U0 UserNote:                                    '+
          '  |U3[|U12|U3]|U0 Mail Records        |U9³');
   sprint('|U9³ |U3[|U1G|U3]|U0 Age/Sex:                                     '+
          '      Pub:                |U9³');
   sprint('|U9³ |U3[|U1L|U3]|U0 Last On:                                     '+
          '      Priv:               |U9³');
   sprint('|U9³ |U3[|U1$|U3]|U0 Password:                                    '+
          '      Fback:              |U9³');
   sprint('|U9³ |U3[|U1E|U3]|U0 AC Restrictions:                             '+
          '      Waiting:            |U9³');
   sprint('|U9³ |U3[|U1F|U3]|U0 AR Flags:                                    '+
          '  |U3[|U11|U3]|U0 Call Records        |U9³');
   sprint('|U9³ |U3[|U1''|U3]|U0 User Colors           |U3[|U1~|U3]|U0 Chat '+
          'Logging             Total:              |U9³');
   sprint('|U9³ |U3[|U1-|U3]|U0 Read Users Infoform   |U3[|U1@|U3]|U0 '+
          'Lockout / Un-Lockout     Time Left:          |U9³');
   sprint('|U9³ |U3[|U1_|U3]|U0 Read Other Infoforms  |U3[|U1*|U3]|U0'+
          ' AutoValidate         |U3[|U1&|U3]|U0 Time Bank:         |U9 ³');
   sprint('|U9ÀÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ'+
          'ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÙ');
   sprint('|U3[|U1Q|U3]|U0 Quit Editor  |U3[|U1[|U3]|U0 Back User  '+
          '      |U3[|U1]|U3]|U0 Forward User      |U3[|U1=|U3]|U0 Reload Old Data');
   sprint('|U3[|U1!|U3]|U0 Toggle Alert |U3[|U1O|U3]|U0 Search Options'+
          '   |U3[|U1{|U3]|U0 Search Backwards  |U3[|U1}|U3]|U0 Search Forwards');
   sprint('|U9ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ');
end;
procedure showuserinfo(typ,usern:integer; user1:userrec);
var i:integer;
    c:char;
    str : string;
begin
   with user1 do begin
      goxy(1,1);
      common.prompt('User #   ');
      unwrite(usern,7,1);
      common.prompt(' of ');
      unwrite(filesize(uf)-1,14,1);
      goxy(43,3);
      if (deleted) then common.prompt('Deleted    ')
      else if (lockedout) then common.prompt('Locked Out')
      else if (alert in ac) then common.prompt('Alert!     ')
      else common.prompt('Normal     ');
      unwrite(sl,63,3);
      unwrite(dsl,75,3);
      uwrite(caps(name),15,5,36);
      goxy(17,14);
      str := '';
      for c:='A' to 'Z' do if c in ar then str := str + c
      else str := str + '-';
      common.prompt(str);
      uwrite(ph,49,4,12);
      uwrite(realname,18,6,36);
      goxy(24,13);
      common.prompt(spflags(user1));
      uwrite(street,16,7,30);
      goxy(16,10);
      common.prompt(sex+cstr(ageuser(bday))+' ('+bday+')');
      uwrite(citystate,19,8,30);
      uwrite(note,17,9,37);
      uwrite(laston,16,11,8);
      unwrite(loggedon,66,15);
      uwrite(mn(tltoday,6),70,16,6);
      unwrite(timebank,70,17);
      unwrite(msgpost,64,10);
      unwrite(emailsent,65,11);
      unwrite(feedback,66,12);
      unwrite(waiting,68,13);
      unwrite(uploads,65,7);
      unwrite(downloads,65,8);
      unwrite(filepoints,67,6);
      if aacs(status^.seepw) then uwrite(pw,17,12,20)
      else uwrite(expand(strings^.echoc,length(pw)),17,12,20);
   end;
end;


procedure uedit(usern:integer);
type f_statusflagsrec=(fs_deleted,fs_trapping,fs_chatbuffer,
                       fs_lockedout,fs_alert,fs_slogging);
const autolist:boolean=TRUE;
      userinfotyp:byte=1;
      f_state:array[0..14] of boolean=
        (FALSE,FALSE,FALSE,FALSE,FALSE,FALSE,FALSE,FALSE,
         FALSE,FALSE,FALSE,FALSE,FALSE,FALSE,FALSE);
      f_gentext:string[30]='';
      f_acs:string[50]='';
      f_sl1:word=0; f_sl2:word=255;
      f_dsl1:word=0; f_dsl2:word=255;
      f_ar:set of acrq=[];
      f_ac:set of uflags=[];
      f_status:set of f_statusflagsrec=[];
      f_laston1:word=0; f_laston2:word=65535;
      f_firston1:word=0; f_firston2:word=65535;
      f_numcalls1:word=0; f_numcalls2:word=65535;
      f_age1:word=0; f_age2:word=65535;
      f_gender:char='M';
      f_postratio1:word=0; f_postratio2:word=65535;
      f_dlkratio1:word=0; f_dlkratio2:word=65535;
      f_dlratio1:word=0; f_dlratio2:word=65535;
var user,user1:userrec;
    r:uflags;
    f:file;
    ii,isn,s:astr;
    i,i1,x,oldusern:integer;
    byt:byte;
    c:char;
    bb,save,save1,abort,next,redraw:boolean;

  function unam:astr;
  begin
    unam:=caps(user.name)+' #'+cstr(usern);
  end;

  function searchtype(i:integer):string;
  var s:string;
  begin
    case i of
      0:s:='General text';           1:s:='Search ACS';
      2:s:='User SL';                3:s:='User DSL';
      4:s:='User AR flags';          5:s:='User AC flags';
      6:s:='User status';            7:s:='Days since last on';
      8:s:='Days since first on';    9:s:='Number of calls';
     10:s:='User age';               11:s:='User gender';
     12:s:='# 1/10''s call/post';    13:s:='#k DL/1k UL';
     14:s:='# DLs/1 UL';
    end;
    searchtype:=s;
  end;

  function find_fs:string;
  var fsf:f_statusflagsrec;
      s:string;
  begin
    s:='';
    for fsf:=fs_deleted to fs_slogging do
      if (fsf in f_status) then
        case fsf of
          fs_deleted   :s:=s+'deleted,';
          fs_trapping  :s:=s+'trapping,';
          fs_chatbuffer:s:=s+'chat buffering,';
          fs_lockedout :s:=s+'locked out,';
          fs_alert     :s:=s+'alert,';
          fs_slogging  :s:=s+'sep. SysOp Log,';
        end;
    if (s<>'') then s:=copy(s,1,length(s)-1) else s:='None.';
    find_fs:=s;
  end;

  procedure pcuropt;
  var r:uflags;
      s:string;
      i:integer;
      c:char;
      abort,next:boolean;
  begin
    nl; cls;
    sprint(#3#5+'< Search limiting options >');
    i:=-1;
    abort:=FALSE; next:=FALSE;
    while ((i<14) and (not abort) and (not hangup)) do begin
      inc(i);
      if (i in [0..9]) then c:=chr(i+48) else
        case i of 10:c:='A'; 11:c:='G'; 12:c:='P'; 13:c:='K'; 14:c:='N'; end;
      if (i=1) then cl(3);
      sprompt(c+'. '+#3#1+mln(searchtype(i),19)+': '); s:='';
      if (not f_state[i]) then
        s:='Inactive!'
      else begin
        case i of
          0:s:='"'+f_gentext+'"';
          1:s:='"'+f_acs+'"';
          2:s:=cstr(f_sl1)+' SL ... '+cstr(f_sl2)+' SL';
          3:s:=cstr(f_dsl1)+' DSL ... '+cstr(f_dsl2)+' DSL';
          4:for c:='A' to 'Z' do
              if (c in f_ar) then s:=s+c else s:=s+'-';
          5:begin
              for r:=rlogon to rmsg do
                if (r in f_ac) then s:=s+copy('LCVBA*PEKM',ord(r)+1,1)
                else s:=s+'-';
              s:=s+'/';
              for r:=fnodlratio to fnodeletion do begin
                if (r in f_ac) then s:=s+copy('1234',ord(r)-19,1)
                else s:=s+'-';
              end;
            end;
          6:s:=find_fs;
          7:s:=cstr(f_laston1)+' days ... '+cstr(f_laston2)+' days';
          8:s:=cstr(f_firston1)+' days ... '+cstr(f_firston2)+' days';
          9:s:=cstr(f_numcalls1)+' calls ... '+cstr(f_numcalls2)+' calls';
         10:s:=cstr(f_age1)+' years ... '+cstr(f_age2)+' years';
         11:s:=aonoff(f_gender='M','Male','Female');
         12:s:=cstr(f_postratio1)+' ... '+cstr(f_postratio2);
         13:s:=cstr(f_dlkratio1)+' ... '+cstr(f_dlkratio2);
         14:s:=cstr(f_dlratio1)+' ... '+cstr(f_dlratio2);
        end;
        cl(3);
      end;
      sprint(s);
      wkey(abort,next);
    end;
    nl;
  end;

  function okusr(x:integer):boolean;
  var fsf:f_statusflagsrec;
      u:userrec;
      i,j:longint;
      ok:boolean;

    function nofindit(s:string):boolean;
    begin
      nofindit:=(pos(strAllCaps(f_gentext),strAllCaps(s))=0);
    end;

  begin
    with u do begin
      seek(uf,x); read(uf,u); ok:=TRUE;
      i:=-1;
      while ((ok) and (i<14)) do begin
        inc(i);
        if (f_state[i]) then
          case i of
            0:if ((nofindit(name)) and (nofindit(realname)) and
                  (nofindit(street)) and (nofindit(citystate)) and
                  (nofindit(zipcode)) and
                  (nofindit(ph)) and (nofindit(note)))
                  then
                ok:=FALSE;
            1:if (not aacs1(u,x,f_acs)) then ok:=FALSE;
            2:if ((sl<f_sl1) or (sl>f_sl2)) then ok:=FALSE;
            3:if ((dsl<f_dsl1) or (dsl>f_dsl2)) then ok:=FALSE;
            4:if (not (ar>=f_ar)) then ok:=FALSE;
            5:if (not (ac>=f_ac)) then ok:=FALSE;
            6:for fsf:=fs_deleted to fs_slogging do
                if (fsf in f_status) then
                  case fsf of
                    fs_deleted   :if (not deleted) then ok:=FALSE;
                    fs_trapping  :if (not trapactivity) then ok:=FALSE;
                    fs_chatbuffer:if (not chatauto) then ok:=FALSE;
                    fs_lockedout :if (not lockedout) then ok:=FALSE;
                    fs_alert     :if (not (alert in ac)) then ok:=FALSE;
                    fs_slogging  :if (not slogseperate) then ok:=FALSE;
                  end;
            7:if ((daynum(laston)>daynum(date)-f_laston1) or
                  (daynum(laston)<daynum(date)-f_laston2)) then ok:=FALSE;
            8:if ((daynum(firston)>daynum(date)-f_firston1) or
                  (daynum(firston)<daynum(date)-f_firston2)) then ok:=FALSE;
            9:if ((loggedon<f_numcalls1) or (loggedon>f_numcalls2)) then ok:=FALSE;
           10:if (((ageuser(bday)<f_age1) or (ageuser(bday)>f_age2)) and
                  (ageuser(bday)<>0)) then
                ok:=FALSE;
           11:if (sex<>f_gender) then ok:=FALSE;
           12:begin
                j:=msgpost; if (j=0) then j:=1; j:=loggedon div j;
                if ((j<f_postratio1) or (j>f_postratio2)) then ok:=FALSE;
              end;
           13:begin
                j:=uk; if (j=0) then j:=1; j:=dk div j;
                if ((j<f_dlkratio1) or (j>f_dlkratio2)) then ok:=FALSE;
              end;
           14:begin
                j:=uploads; if (j=0) then j:=1; j:=downloads div j;
                if ((j<f_dlratio1) or (j>f_dlratio2)) then ok:=FALSE;
              end;
          end;
      end;
    end;
    okusr:=ok;
  end;

  procedure search(i:integer);
  var u:userrec;
      n:integer;
      c:char;
  begin
    n:=usern;
    repeat
      inc(usern,i);
      if (usern<=0) then usern:=filesize(uf)-1;
      if (usern>=filesize(uf)) then usern:=1;
    until ((okusr(usern)) or (usern=n));
  end;

  procedure clear_f;
  var i:integer;
  begin
    for i:=0 to 14 do f_state[i]:=FALSE;

    f_gentext:=''; f_acs:='';
    f_sl1:=0; f_sl2:=255; f_dsl1:=0; f_dsl2:=255;
    f_ar:=[]; f_ac:=[]; f_status:=[];
    f_laston1:=0; f_laston2:=65535; f_firston1:=0; f_firston2:=65535;
    f_numcalls1:=0; f_numcalls2:=65535; f_age1:=0; f_age2:=65535;
    f_gender:='M';
    f_postratio1:=0; f_postratio2:=65535; f_dlkratio1:=0; f_dlkratio2:=65535;
    f_dlratio1:=0; f_dlratio2:=65535;
  end;

  procedure stopt;
  var fsf:f_statusflagsrec;
      i,usercount:integer;
      c,ch:char;
      done:boolean;
      s:astr;

    procedure chbyte(var x:integer);
    var s:astr;
        i:integer;
    begin
      input(s,3); i:=x;
      if (s<>'') then i:=value(s);
      if ((i>=0) and (i<=255)) then x:=i;
    end;

    procedure chword(var x:word);
    var s:astr;
        w:word;
    begin
      input(s,5);
      if (s<>'') then begin
        w:=value(s);
        if ((w>=0) and (w<=65535)) then x:=w;
      end;
    end;

    procedure inp_range(var w1,w2:word; r1,r2:word);
    begin
      print('Range: '+cstr(r1)+'..'+cstr(r2));
      common.prompt('Lower limit ['+cstr(w1)+'] : '); chword(w1);
      common.prompt('Upper limit ['+cstr(w2)+'] : '); chword(w2);
    end;

    function get_f_ac:string;
    var r:uflags;
        s:string;
    begin
      for r:=rlogon to rmsg do
        if (r in f_ac) then s:=s+copy('LCVBA*PEKM',ord(r)+1,1)
        else s:=s+'-';
      s:=s+'/';
      for r:=fnodlratio to fnodeletion do begin
        if (r in f_ac) then s:=s+copy('1234',ord(r)-19,1)
        else s:=s+'-';
      end;
      get_f_ac:=s;
    end;

  begin
    done:=FALSE;
    pcuropt;
    repeat
      common.prompt('Change (?=help) : '); onek(c,'Q0123456789AGPKN?CLTU'^M);
      nl;
      case c of
        '0'..'9':i:=ord(c)-48;
        'A':i:=10; 'G':i:=11; 'P':i:=12; 'K':i:=13; 'N':i:=14;
      else
            i:=-1;
      end;
      if (i<>-1) then begin
        sprompt(#3#5+'[> '+#3#0);
        if (f_state[i]) then
          sprint(searchtype(i))
        else begin
          f_state[i]:=TRUE;
          sprint(searchtype(i)+' is now *ON*');
        end;
{        nl;}
      end;

      case c of
        '0':begin
              print('General text ["'+f_gentext+'"]:');
              input(s,30);
              if (s<>'') then f_gentext:=s;
            end;
        '1':begin
              common.prompt('Search ACS ["'+f_acs+'"]:');
              inputl(s,50);
              if (s<>'') then f_acs:=s;
            end;
        '2':begin
              common.prompt('Lower limit ['+cstr(f_sl1)+'] : ');
              chword(f_sl1);
              common.prompt('Upper limit ['+cstr(f_sl2)+'] : ');
              chword(f_sl2);
            end;
        '3':inp_range(f_dsl1,f_dsl2,0,255);
        '4':repeat
              common.prompt('Which AR flag? <CR>=Quit : ');
              onek(ch,^M'ABCDEFGHIJKLMNOPQRSTUVWXYZ');
              if (ch<>^M) then
                if (ch in ['A'..'Z']) then
                  if (ch in f_ar) then f_ar:=f_ar-[ch] else f_ar:=f_ar+[ch];
            until ((ch=^M) or (hangup));
        '5':begin
              repeat
                common.prompt('Restrictions ['+get_f_ac+'] [?]Help [Q]uit :');
                onek(c,'Q LCVBA*PEKM1234?'^M);
                case c of
                  ^M,' ','Q': ;
                  '?':restric_list;
                else
                      if (tacch(c) in f_ac) then f_ac:=f_ac-[tacch(c)]
                      else f_ac:=f_ac+[tacch(c)];
                end;
              until ((c in [^M,' ','Q']) or (hangup));
            end;
        '6':repeat
              s:=find_fs;
              sprint(#3#4+'Current flags: '+#3#3+s);
              common.prompt('Toggle (?=help) : '); onek(ch,'QACDLST? '^M);
              if (pos(ch,'ACDLST')<>0) then begin
                case ch of
                  'A':fsf:=fs_alert;
                  'C':fsf:=fs_chatbuffer;
                  'D':fsf:=fs_deleted;
                  'L':fsf:=fs_lockedout;
                  'S':fsf:=fs_slogging;
                  'T':fsf:=fs_trapping;
                end;
                if (fsf in f_status) then f_status:=f_status-[fsf]
                  else f_status:=f_status+[fsf];
              end else
                if (ch='?') then begin
                  nl;
                  lcmds(15,3,'Alert','Chat-buffering');
                  lcmds(15,3,'Deleted','Locked-out');
                  lcmds(15,3,'Seperate SysOp logging','Trapping');
                  nl;
                end;
            until ((ch in ['Q',' ',^M]) or (hangup));
        '7':inp_range(f_laston1,f_laston2,0,65535);
        '8':inp_range(f_firston1,f_firston2,0,65535);
        '9':inp_range(f_numcalls1,f_numcalls2,0,65535);
        'A':inp_range(f_age1,f_age2,0,65535);
        'G':begin
              common.prompt('Gender ['+f_gender+'] : ');
              onek(c,'QMF'^M); nl;
              if (c in ['F','M']) then f_gender:=c;
            end;
        'P':inp_range(f_postratio1,f_postratio2,0,65535);
        'K':inp_range(f_dlkratio1,f_dlkratio2,0,65535);
        'N':inp_range(f_dlratio1,f_dlratio2,0,65535);
        'C':if pynq('Are you sure? ',true) then clear_f;
        ^M,'L':pcuropt;
        'T':begin
              common.prompt('Which? '); onek(ch,'Q0123456789AGPKN'^M);
              case ch of
                '0'..'9':i:=ord(ch)-48;
                'A':i:=10; 'G':i:=11; 'P':i:=12; 'K':i:=13; 'N':i:=14;
              else
                    i:=-1;
              end;
              if (i<>-1) then begin
                f_state[i]:=not f_state[i];
                sprompt(#3#5+'[> '+#3#0+searchtype(i)+' is now *');
                if (f_state[i]) then print('ON*') else print('OFF*');
              end;
              nl;
            end;
        'U':begin
              abort:=FALSE; usercount:=0;
              for i:=1 to filesize(uf)-1 do begin
                if (okusr(i)) then begin
                  seek(uf,i); read(uf,user1);
                  printacr(#3#3+caps(user1.name)+' #'+cstr(i),abort,next);
                  inc(usercount);
                end;
                if (abort) then i:=filesize(uf)-1;
              end;
              if (not abort) then
                sprint('|NL^7 - '+#3#5+cstr(usercount)+' Users.|NL');
            end;
        'Q':done:=TRUE;
        '?':begin
              sprint(#3#3+'0-9,AGPKN'+#3#1+': Change option');
              lcmds(14,3,'List options','Toggle options on/off');
              lcmds(14,3,'Clear options','User''s who match');
              lcmds(14,3,'Quit','');
              nl;
            end;
      end;
      if (pos(c,'C0123456789AGPKN')<>0) then nl;
    until ((done) or (hangup));
  end;

  {
  procedure killusermail;
  var u:userrec;
      pinfo:pinforec;
      mixr:msgindexrec;
      i,j:longint;
  begin
    savepinfo(pinfo);
    initbrd(-1);
    for i:=0 to himsg do begin
      seek(mixf,i); blockread(mixf,mixr,1);
      j:=mixr.messagenum;
      if ((miexist in mixr.msgindexstat) and (j=usern)) then s:=rmail(i);
    end;
    loadpinfo(pinfo);
  end;
  }

  procedure delusr;
  var i:integer;
  begin
    if (not user.deleted) then begin
      save:=TRUE; user.deleted:=TRUE;
      nukeuser(user.name, usern);
      sysoplog('* Deleted user: '+caps(user.name)+' #'+cstr(usern));
      user.waiting:=0;
      {killusermail;}
    end;
  end;

  procedure renusr;
  begin
    if (user.deleted) then print('Can''t rename deleted users.')
    else begin
      nl; common.prompt('Enter new name: '); input(ii,36);
      if (ii<>'') and (ii[1] in ['A'..'Z','?']) then begin
        dsr(user.name); isr(ii,usern);
        user.name:=ii; save:=TRUE;
        if (usern=usernum) then thisuser.name:=ii;
      end;
    end;
  end;

  procedure chhflags;
  var done:boolean;
      c:char;
  begin
    nl;
    done:=FALSE;
    repeat
      common.prompt('Restrictions ['+spflags(user)+'] [?]Help [Q]uit :');
      onek(c,'Q LCVBA*PEKM1234?'^M);
      case c of
        ^M,' ','Q':done:=TRUE;
        '?':restric_list;
      else
            begin
              if (c='4') and (not so) then print('You can''t change that!')
              else begin
                acch(c,user);
                save:=TRUE;
              end;
            end;
      end;
    until (done) or (hangup);
    save:=TRUE;
  end;

  procedure chhsl;
  begin
    common.prompt('Enter new SL: '); ini(byt);
    if (not badini) then begin
      save:=TRUE;
      if (byt<thisuser.sl) or (usernum=1) then begin
        if (usernum=usern) and (byt<thisuser.sl) then
          if not pynq('Lower your own SL level? ',false) then exit;
        user.sl:=byt;
      end else begin
        sysoplog('UEDIT: Illegal SL change- '+caps(user.name)+' #'+cstr(usern)+
                 ' to '+cstr(byt));
        print('Access denied.'^G);
      end;
    end;
  end;

  procedure chhdsl;
  begin
    common.prompt('Enter new DSL: '); ini(byt);
    if (not badini) then begin
      save:=TRUE;
      if (byt<thisuser.dsl) or (usernum=1) then begin
        if (usernum=usern) and (byt<thisuser.sl) then
          if not pynq('Lower your own DSL level? ',false) then exit;
        user.dsl:=byt;
      end else begin
        sysoplog('UEDIT: Illegal DSL change- '+caps(user.name)+' #'+cstr(usern)+
                 ' to '+cstr(byt));
        print('Access denied.'^G);
      end;
    end;
  end;

  procedure chrecords(beg:byte);
  var onn:byte;
      done:boolean;
      c:char;
      i:integer;
      lng:longint;
  begin
    onn:=beg;
    done:=FALSE;
    with user do
      repeat
        nl;
        case onn of
          1:begin
              sprint(#3#5+'Call records:');
              print('(0)Total calls: '+mn(loggedon,5)+' (1)Total time on:   '+mn(trunc(ttimeon),8));
              print('(2)Calls today: '+mn(ontoday,5)+ ' (3)Time left today: '+mn(tltoday,5));
              print('(4)Illegal logon attempts: '+mn(illegal,5));
              nl;
              common.prompt('Select: (0-4) [M]ail [F]ile [Q]uit :');
              onek(c,'Q01234MF'^M);
            end;
          2:begin
              sprint(#3#5+'Mail records:');
              print('(0)Pub. posts: '+mn(msgpost,5)+' (1)Priv. posts:  '+mn(emailsent,5));
              print('(2)Fback sent: '+mn(feedback,5)+' (3)Mail waiting: '+mn(waiting,5));
              nl;
              common.prompt('Select: (0-3) [C]all [F]ile [Q]uit :');
              onek(c,'Q0123CF'^M);
            end;
          3:begin
              sprint(#3#5+'File records:');
              print('(0)# of DLs: '+mn(downloads,5)+' (1)DL k: '+cstr(trunc(dk)));
              print('(2)# of ULs: '+mn(uploads,5)+' (3)UL k: '+cstr(trunc(uk)));
              nl;
              common.prompt('Select: (0-3) [C]all [M]ail [Q]uit :');
              onek(c,'Q0123CM'^M);
            end;
        end;
        case c of
          'Q',^M:done:=TRUE;
          'C':onn:=1;
          'M':onn:=2;
          'F':onn:=3;
          '0'..'4':begin
            nl; common.prompt('New value: '); inl(lng);
            i:=lng;
            if (lng>99999) then lng:=99999;
            if not badini then
              case onn of
                1:case value(c) of
                    0:loggedon:=i; 1:ttimeon:=i; 2:ontoday:=i; 3:tltoday:=i;
                    4:illegal:=i;
                  end;
                2:case value(c) of
                    0:msgpost:=i; 1:emailsent:=i; 2:feedback:=i; 3:waiting:=i;
                  end;
                3:case value(c) of
                    0:downloads:=i; 1:dk:=lng; 2:uploads:=i; 3:uk:=lng;
                  end;
              end;
          end;
        end;
      until (done) or (hangup);
  end;

  function onoff(b:boolean; s1,s2:astr):astr;
  begin
    if b then onoff:=s1 else onoff:=s2;
  end;

  procedure lcmds3(len,c:byte; c1,c2,c3:astr);
  var s:astr;
  begin
    s:='';
    s:=s+#3#1+'('+#3+chr(c)+c1[1]+#3#1+')'+mln(copy(c1,2,lenn(c1)-1),len-1);
    if (c2<>'') then
      s:=s+#3#1+'('+#3+chr(c)+c2[1]+#3#1+')'+mln(copy(c2,2,lenn(c2)-1),len-1);
    if (c3<>'') then
      s:=s+#3#1+'('+#3+chr(c)+c3[1]+#3#1+')'+copy(c3,2,lenn(c3)-1);
    printacr(s,abort,next);
  end;

begin
 if (thisuser.ac=thisuser.ac+[pause]) then begin
                        thisuser.ac:=thisuser.ac-[pause];
                        bb:=true;
                      end;
  reset(uf);
  if ((usern<1) or (usern>filesize(uf)-1)) then begin close(uf); exit; end;
  if (usern=usernum) then begin
    user:=thisuser;
    seek(uf,usern); write(uf,user);
  end;
  seek(uf,usern); read(uf,user);

  clear_f;

  oldusern:=0;
  save:=FALSE;
  redraw := true;
  repeat
    abort:=FALSE;
    if (autolist) or (usern<>oldusern) or (c=^M) then begin
{      if (c=^M) then doansi;}
      goxy(1,1);
      if redraw then showuiscr;
      redraw := false;
      showuserinfo(userinfotyp,usern,user);
      oldusern:=usern;
    end;
    goxy(1,22);
    sprompt('|U0User Editing|U9:  ');
    goxy(15,22);
    onek(c,'Q?[]={}*^@!ACDEFGIKLMNOPRSTUWYZ$123''#&-_;:\~'^M);
    case c of
      '?':begin
            nl; sprint(#3#5+'User-list command help'); nl;
            print('<CR>Redisplay user');
            lcmds3(21,3,';New list mode',':Autolist mode toggle','');
            lcmds3(21,3,'[Back one user',']Forward one user','=Ooops (reload old data)');
            lcmds3(21,3,'{Search backward','}Search forward','*Auto-validate user');
            lcmds3(21,3,'~Trap/chat logging','@Lockout/Unlockout','!Toggle alert status');
            lcmds3(21,3,'Address','City and state','DSL');
            lcmds3(21,3,'ERestrictions','FAR flags','GSex/Age');
            lcmds3(21,3,'IAutoSig','KSysOp comments','Laston date');
            lcmds3(21,3,'Mailbox','Name or handle','OSearch options');
            lcmds3(21,3,'Phone number','Real user name','SL');
            lcmds3(21,3,'YMessage SysOp bases','Zip code','$Password');
            lcmds3(21,3,'1Call records','2Mail records','3File records');
            lcmds3(21,3,'''User colors','#File points','&Time bank');
            lcmds3(21,3,'^Delete/Restore user','-New user answers','_Other Q. answers');
            lcmds3(21,3,'\Show SysOp Log','UGoto user name/#','Quit editor');
            pausescr;
            {doansi;}
            save:=FALSE;
            redraw := true;
          end;
      '[',']','{','}','U','Q':begin
            if save then begin
              seek(uf,usern); write(uf,user);
              if usern=usernum then thisuser:=user;
              save:=FALSE;
            end;
            case c of
              '[':begin
                    dec(usern);
                    if (usern<=0) then usern:=filesize(uf)-1;
                  end;
              ']':begin
                    inc(usern);
                    if (usern>=filesize(uf)) then usern:=1;
                  end;
              '{':begin
                    search(-1);
                  end;
              '}':begin
                    search(1);
                  end;
              'U':begin
                    redraw := true;
                    common.prompt('Enter user name, #, or partial search string: ');
                    finduserws(i);
                    if (i>0) then begin
                      seek(uf,i); read(uf,user);
                      usern:=i;
                    {doansi;}
                    end;
                  end;
            end;
            seek(uf,usern); read(uf,user);
            if (usern=usernum) then thisuser:=user;
          end;
      '=': begin
             redraw := true;
            if pynq('|NL|U0Reload old user data? ',true) then begin
              seek(uf,usern); read(uf,user);
              if (usern=usernum) then thisuser:=user;
              save:=FALSE;
              sprint(#3#7+'Old data reloaded.');
            end;
          end;
      'O','-','_',';',':','\':
          begin
            redraw := true;
            case c of
              'O':stopt;
              '-':begin
                    readasw(usern,status^.afilepath+'newuser');
                    pausescr;
                  end;
              '_':begin
                    nl;
                    common.prompt('Print questionairre file: '); input(s,8); nl;
                    readasw(usern,status^.afilepath+s);
                    pausescr;
                    {doansi;}
                  end;
              ';':begin
                    nl;
                    common.prompt('(L)ong or (S)hort list mode : ');
                    onek(c,'QSL '^M);
                    case c of
                      'S':userinfotyp:=2;
                      'L':userinfotyp:=1;
                    end;
                    {doansi;}
                  end;
              ':':autolist:=not autolist;
              '\':begin
                    s:=status^.trappath+'slog'+cstr(usern)+'.log';
                    printf(s);
                    if (nofile) then print('"'+s+'": file not found.');
                    pausescr;
                    {doansi;}
                  end;
            end;
          end;
      '*','^','@','!','A','C','D','E','F','G','I','K','L','M',
      'N','P','R','S','Y','Z','$','1','2','3','''','#','&','~':
          begin
            redraw := true;
            if ((thisuser.sl<=user.sl) or (thisuser.dsl<=user.dsl)) and
               (usernum<>1) and (usernum<>usern) then begin
              sysoplog('UEDIT: Tried to modify '+
                       caps(user.name)+' #'+cstr(usern));
              print('Access denied.');
            end else begin
              save1:=save; save:=TRUE;
              case c of
                '*':autovalidate(user,usern);
                '^':if (user.deleted) then begin
                      print('User is currently deleted.');
                      nl;
                      if pynq('Restore this user? ',false) then begin
                        isr(user.name,usern);
                        user.deleted:=FALSE;
                      end else
                        save:=save1;
                    end else
                      if (fnodeletion in user.ac) then begin
                        print('Access denied - This user is protected from deletion.');
                        sysoplog('* Attempt to delete user: '+caps(user.name)+
                                 ' #'+cstr(usern));
                        nl; pausescr;
                        {doansi;}
                        save:=save1;
                      end else begin
                        print('User is not currently deleted.');
                        nl;
                        print('NOTE: If this user is deleted, ALL VOTING RECORDS,');
                        print('AND ANY EMAIL TO OR FROM THIS USER WILL BE DELETED.');
                        nl;
                        if pynq('*DELETE* this user? ',true) then delusr
                        else save:=save1;
                        {doansi;}
                      end;
                '@':begin
                      nl;
                      user.lockedout:=not user.lockedout;
                      if (user.lockedout) then begin
                        print('User is now LOCKED out.');
                        nl;
                        print('Each time the user logs on from now on, a text file will');
                        print('be displayed before connection is terminated.');
                        nl;
                        common.prompt('Enter lockout filename: ');
                        input(ii,8);
                        if (ii='') then user.lockedout:=FALSE
                        else begin
                          user.lockedfile:=ii;
                          sysoplog('UEDIT: Locked '+unam+' out: Lockfile "'+ii+'"');
                        end;
                      end;
                      if (not user.lockedout) then
                        print('User is no longer locked out of system.');
                      nl;
                      pausescr;
                      {doansi;}
                    end;
                '!':if (alert in user.ac) then user.ac:=user.ac-[alert]
                                else user.ac:=user.ac+[alert];
                'A':begin cstuff(1,3,user); {doansi;} end;
                'C':begin cstuff(4,3,user); {doansi;} end;
                'D':begin chhdsl; {doansi;} end;
                'E':begin chhflags; {doansi;} end;
                'F':begin
                      nl;
                      repeat
                        common.prompt('Which AR flag? <CR>=Quit : ');
                        onek(c,'ABCDEFGHIJKLMNOPQRSTUVWXYZ'^M);
                        if (c<>^M) then
                          if (not (c in thisuser.ar)) and (usernum<>1) then begin
                            sysoplog('UEDIT: Tried to give '+caps(user.name)+
                                     ' #'+cstr(usern)+' AR flag "'+c+'"');
                            print('Access denied.'^G)
                          end else
                            if (c in ['A'..'Z']) then
                              if (c in user.ar) then user.ar:=user.ar-[c]
                                                else user.ar:=user.ar+[c];
                      until (c=^M) or (hangup);
                      c:=#0;
                      {doansi;}
                    end;
                'G':begin
                      cstuff(2,3,user);
                      cstuff(12,3,user);
                      {doansi;}
                    end;
                'I':begin cstuff(6,3,user); {doansi;} end;
                'K':begin
                      nl;
                      print('New SysOp Note: ');
                      inputl(s,39);
                      if (s<>'') then user.note:=s;
                      {doansi;}
                    end;
                'L':begin
                      nl;
                      print('New Laston date, in the form MM/DD/YY:');
                      inputl(s,8);
                      if (s<>'') and (daynum(s)<>0) then user.laston:=s;
                      {doansi;}
                    end;
                'M':begin cstuff(15,3,user); {doansi;} end;
                'N':begin renusr; {doansi;} end;
                'P':begin sprint(user.ph); cstuff(8,3,user); {doansi;} end;
                'R':begin cstuff(10,3,user); {doansi;} end;
                'S':begin chhsl; {doansi;} end;
                'Y':begin
                      nl;
                      print('Message base SysOp:');
                      nl;
                      for i:=1 to 5 do common.prompt(cstr(i)+') '+cstr(user.boardsysop[i])+'  ');
                      nl; print('(-1 is inactive)');
                      nl;
                      common.prompt('Which number? (1-5) :'); ini(byt); i:=byt;
                      if (not badini) then begin
                        common.prompt('Which message base? (1-'+cstr(numboards)+') :'); ini(byt);
                        if (not badini) then
                          if ((byt>=1) and (byt<=numboards)) or (byt=-1) then
                            user.boardsysop[i]:=byt;
                      end;
                      {doansi;}
                    end;
                'Z':begin cstuff(6,3,user); {doansi;} end;
                '$':begin cstuff(9,3,user); {doansi;} end;
                '1'..'3':begin chrecords(value(c)); {doansi;} end;
                '''':begin cstuff(21,3,user); {doansi;} end;
                '#':begin
                      nl;
                      common.prompt('Enter new amount of file points.: ');
                      inu(i);
                      if (not badini) then user.filepoints:=i;
                      {doansi;}
                    end;
                '&':begin
                      nl;
                      common.prompt('Enter new amount of time in time bank: ');
                      inu(i);
                      if (not badini) then user.timebank:=i;
                      {doansi;}
                    end;
                '~':begin
                      repeat
                        nl;
                        sprint('1. Trapping status: '+
                          onoff(user.trapactivity,
                          #3#7+onoff(user.trapseperate,
                          'Trapping to TRAP'+cstr(usern)+'.MSG',
                          'Trapping to TRAP.MSG'),
                          'Off')+onoff(status^.globaltrap,#3#8+' <GLOBAL>',''));
                        sprint('2. Auto-chat state: '+onoff(user.chatauto,
                          onoff(user.chatseperate,
                          #3#7+'Output to CHAT'+cstr(usern)+'.MSG',
                          #3#7+'Output to CHAT.MSG'),'Off')+
                          onoff(status^.autochatopen,#3#8+' <GLOBAL>',''));
                        sprint('3. SysOp Log state: '+onoff(user.slogseperate,
                          #3#7+'Logging to SLOG'+cstr(usern)+'.LOG',
                          #3#3+'Normal output'));
                        nl;
                        common.prompt('Select (1-3,Q=Quit) : '); onek(c,'Q123'^M);
                        if (c in ['1'..'3']) then begin
                          nl;
                          case c of
                            '1':begin
                                  user.trapactivity:=
                                    pynq('Trap user activity? ['+
                                    syn(user.trapactivity)+'] : ',false);
                                  if (user.trapactivity) then begin
                                    user.trapseperate:=
                                      pynq('Log to seperate file? ['+
                                      syn(user.trapseperate)+'] : ',true);
                                  end else
                                    user.trapseperate:=FALSE;
                                end;
                            '2':begin
                                  user.chatauto:=
                                    pynq('Auto-chat buffer open? ['+
                                    syn(user.chatauto)+'] : ',false);
                                  if (user.chatauto) then begin
                                    user.chatseperate:=
                                      pynq('Seperate buffer file? ['+
                                      syn(user.chatseperate)+'] : ',true);
                                  end else
                                    user.chatseperate:=FALSE;
                                end;
                            '3':begin
                                  user.slogseperate:=
                                    pynq('Output SysOp Log seperately? ['+
                                    syn(user.slogseperate)+'] : ',true);
                                end;
                          end;
                        end;
                      until ((not (c in ['1'..'3'])) or (hangup));
                      c:=#0;
                    end;
                else
                  save:=save1;
              end;
            end;
          {doansi;}
          end;
    end;
    if (usern=usernum) then thisuser:=user;
  until (c='Q') or hangup;
  if (bb) then thisuser.ac:=thisuser.ac+[pause];
  {$I-} close(uf); {$I+}
  topscr;
end;

end.
