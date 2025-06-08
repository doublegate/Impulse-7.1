(* Common Procedures *)
{$A+,B+,F+,I+,L-,O+,R-,S-,V-}
unit common;

interface

uses
   Crt,      Dos,      timejunk, records,  output;
const
   strLen=160;
   sepr2=#3#9+'\'+#3#0;
   ESCAPE = 27;
   F1 = 59;  F2 = 60;  F3 = 61;  F4 = 62;  F5 = 63;  F6 = 64;
   F7 = 65;  F8 = 66;  F9 = 67;  F10 = 68;
   SHIFT_F1 = 84;      SHIFT_F2 = 85;      SHIFT_F3 = 86;
   SHIFT_F4 = 87;      SHIFT_F5 = 88;      SHIFT_F6 = 89;
   SHIFT_F7 = 90;      SHIFT_F8 = 91;      SHIFT_F9 = 92;
   SHIFT_F10 = 93;
   CTRL_F1 = 94;       CTRL_F2 = 95;       CTRL_F3 = 96;
   CTRL_F4 = 97;       CTRL_F5 = 98;       CTRL_F6 = 99;
   CTRL_F7 = 100;      CTRL_F8 = 101;      CTRL_F9 = 102;
   CTRL_F10 = 103;
   CTRL_PRTSC = 114;
   ALT_F1 = 104;       ALT_F2 = 105;       ALT_F3 = 106;
   ALT_F4 = 107;       ALT_F5 = 108;       ALT_F6 = 109;
   ALT_F7 = 110;       ALT_F8 = 111;       ALT_F9 = 112;
   ALT_F10 = 113;
   ALT_Q = 16;         ALT_W = 17;         ALT_E = 18;
   ALT_R = 19;         ALT_T = 20;         ALT_Y = 21;
   ALT_U = 22;         ALT_I = 23;         ALT_O = 24;
   ALT_P = 25;         ALT_A = 30;         ALT_S = 31;
   ALT_D = 32;         ALT_F = 33;         ALT_G = 34;
   ALT_H = 35;         ALT_J = 36;         ALT_K = 37;
   ALT_L = 38;         ALT_Z = 44;         ALT_X = 45;
   ALT_C = 46;         ALT_V = 47;         ALT_B = 48;
   ALT_N = 49;         ALT_M = 50;
   ALT_1 = 120;        ALT_2 = 121;        ALT_3 = 122;
   ALT_4 = 123;        ALT_5 = 124;        ALT_6 = 125;
   ALT_7 = 126;        ALT_8 = 127;        ALT_9 = 128;
   ALT_0 = 129;
   ALT_MINUS = 130;    ALT_EQUAL = 131;    ARROW_HOME = 71;
   ARROW_UP = 72;      ARROW_PGUP = 73;    ARROW_LEFT = 75;
   ARROW_RIGHT = 77;   ARROW_END = 79;     ARROW_DOWN = 80;
   ARROW_PGDN = 81;    CTRL_HOME = 119;    CTRL_END = 117;

var uf:file of userrec;           { USER.LST                              }
    bf:file of boardrec;          { BOARDS.DAT                            }
    xf:file of protrec;           { PROTOCOL.DAT                          }
    ulf:file of ulrec;            { UPLOADS.DAT                           }
    ulff:file of ulfrec;          { *.DIR                                 }
    sf:file of smalrec;           { NAMES.LST                             }
    verbf:file of verbrec;        { VERBOSE.DAT                           }
    multinodef:file of noderec;
    clasicnode:astr;
    mixf:file;                    { *.MIX                                 }
    brdf:file;                    { *.BRD                                 }
    sysopf,                       { SYSOP.LOG                             }
    sysopf1,                      { SLOGxxxx.LOG                          }
    trapfile,                     { TRAP*.MSG                             }
    cf:text;                      { CHAT*.MSG                             }
    small:smalrec;
    status:^systatRec;
    strings:^fstringRec;
    modemr:modemrec;              { modem configuration                   }
    fidor:fidorec;                { FidoNet information                   }
    thisuser:userrec;             { user's account records                }
    zscanr:zscanrec;              { user's zscan records                  }

    { BRD files }
    msg_on:integer;               { current message being read            }

    { EVENTS }
    events:array[0..maxevents] of ^eventrec;
    numevents:integer;            { # of events                           }

    { PROTOCOLS }
    protocol:protrec;             { protocol in memory                    }
    numprotocols:integer;         { # of protocols                        }

    { FILE BASES }
    memuboard,tempuboard:ulrec;   { uboard in memory, temporary uboard    }
    readuboard,                   { current uboard # in memory            }
    maxulb,                       { # of file bases                       }
    fileboard:integer;            { file base user is in                  }

    { MESSAGE BASES }
    memboard:boardrec;            { board in memory                       }
    readboard,                    { current board # in memory             }
    numboards,                    { # of message bases                    }
    board:integer;                { message base user is in               }

    { FILE/MESSAGE BASE COMPRESSION TABLES }
    ccboards:array[0..1,1..maxboards] of byte;
    ccuboards:array[0..1,0..maxuboards] of byte;
    nuOrder:array[1..26] of byte;

    spd:string[80];                { current modem speed, "KB" for local   }

(*****************************************************************************)

    { message stuff }
    mintabloaded:word;            { minor table loaded }
    mintaboffset:longint;         { minor table file offset }
    mintab:array[0..99] of msgindexrec; { minor table }
    himsg:longint;                { highest message number }
    himintab:longint;             { highest minor table number }


    buf:string[255];              { macro buffer                          }

    sitedatetime:packdatetime;    { last time site compiled/changed status }


    cmdlist,                      { list of cmds on current menu          }
    irt,                          { reason for reply                      }
    lastname,                     { author of last message displayed      }
    lastuname,                    { last name, whether anon or not        }
    ll,                           { "last-line" string for word-wrapping  }
    start_dir:string;             { directory BBS was executed from       }

    tim,                          { time last keystroke entered           }
    timeon:datetimerec;           { time user logged on                   }

    choptime,                     { time to chop off for system events    }
    extratime,                    { extra time - given by F7/F8, etc      }
    freetime,                     { free time                             }
    oltime:real;

    answerbaud,                   { baud rate to answer the phone at      }
    exteventtime,                 { # minutes before external event       }

    chatt,                        { number chat attempts made by user     }
    etoday,                       { E-mail sent by user this call         }
    ftoday,                       { feedback sent by user this call       }
    lastprot,                     { last protocol #                       }
    ldate,                        { last daynum()                         }
    lil,                          { lines on screen since last pausescr() }
    mread,                        { # public messages has read this call  }
    pap,                          { characters on this line so far        }
    ptoday,                       { posts made by user this call          }
    realdsl,                      { real DSL level of user                }
    realsl,                       { real SL level of user (for F9)        }
    usernum:integer;              { user's user number                    }

    bread,                        { board loaded, or -1 for e-mail        }
    bwant:integer;

    readpmnu : char;              { 2nd char in command str for readp.mnu }
    filemnu  : boolean;
    filepmnu : char;              { 2nd char in command str for filep.mnu }
    nuvpmnu : char;

    chelplevel,                   { current help level                    }
    curco,                        { current ANSI color                    }
    elevel,                       { ERRORLEVEL to exit with               }
    tshuttlelogon:byte;           { type of special Shuttle Logon command }
    thisnode:integer;
    nodeinfo:noderec;

    { misc shit }
    lastcalled : string[36];

    { conf shit }
    mConf,
    fConf : byte;

    { New pointer stuff }
    prmp : ^promptRec;
    areanum : ^areaRec;
    mciList : array[1..255] of ^mciType;
{    strs : ^stringType;
    stringFile : file of stringType;}
    tasker : byte;                { multi tasker for use with time slices }
    osName : string[7];
const
    fuckedUp : boolean = false;

const
    numMcis : word = 0;
    MAXTRIES = 1;                 { how many times shout it try to init the modem? }
    inConfig:boolean=FALSE;       { are they in the config program?       }
    allowabort:boolean=TRUE;      { are aborts allowed?                   }
    echo:boolean=TRUE;            { is text being echoed? (FALSE=use echo chr)}
    hangup:boolean=TRUE;          { is user offline now?                  }
    nofile:boolean=TRUE;          { did last pfl() file NOT exist?        }
    onekcr:boolean=TRUE;          { does ONEK prints<CR> upon exit?       }
    onekda:boolean=TRUE;          { does ONEK display the choice?         }
    slogging:boolean=TRUE;        { are we outputting to the SysOp log?   }
    sysopon:boolean=TRUE;         { is SysOp logged onto the WFC menu?    }
    wantout:boolean=TRUE;         { output text locally?                  }
    wcolor:boolean=TRUE;          { in chat: was last key pressed by SysOp? }
    badfpath:boolean=FALSE;       { is the current DL path BAD?           }
    badufpath:boolean=FALSE;      { is the current UL path BAD?           }
    badini:boolean=FALSE;         { was last call to ini/inu value()=0, s<>"0"? }
    bchanged:boolean=FALSE;       { was BRD file changed?                 }
    bnp:boolean=FALSE;            { was file base name printed yet?       }
    cfo:boolean=FALSE;            { is chat file open?                    }
    ch:boolean=FALSE;             { are we in chat mode?                  }
    chatcall:boolean=FALSE;       { is the chat call "noise" on?          }
    pauseme:boolean=TRUE;         { allow screen pauses?                  }
    contlist:boolean=FALSE;       { continuous message listing mode on?   }
    croff:boolean=FALSE;          { are CRs turned off?                   }
    ctrljoff:boolean=FALSE;       { turn color to #1 after ^Js??          }
    cwindowon:boolean=FALSE;      { is SysOp window ON?                   }
    doneafternext:boolean=FALSE;  { offhook and exit after next logoff?   }
    doneday:boolean=FALSE;        { are we done now? ready to drop to DOS? }
    dosansion:boolean=FALSE;      { output chrs to ImpAnsi for ANSI codes?!!? }
    fastlogon:boolean=FALSE;      { if a FAST LOGON is requested          }
    hungup:boolean=FALSE;         { did user drop carrier?                }
    incom:boolean=FALSE;          { accepting input from com?             }
    inwfcmenu:boolean=FALSE;      { are we in the WFC menu?               }
    lan:boolean=FALSE;            { was last post/email anonymous/other?  }
    lastcommandgood:boolean=FALSE;{ was last command a REAL command?      }
    lastcommandovr:boolean=FALSE; { override PAUSE? (NO pause?)           }
    macok:boolean=FALSE;          { are macros OKay right now?            }
    mailread:boolean=FALSE;       { did user delete some e-mail?          }
    menuactiv:boolean=false;      { menus are active at the moment        }
    scanfilemsg:boolean=false;    { is user at a Newscan Prompt/Menu?     }
    localioonly:boolean=FALSE;    { local I/O ONLY?                       }
    packbasesonly:boolean=FALSE;  { pack message bases ONLY?              }
    newmenutoload:boolean=FALSE;  { menu command returns TRUE if new menu to load }
    nightly:boolean=FALSE;        { execute hard-coded nightly event?     }
    overlayinems:boolean=FALSE;   { is overlay file in EMS memory?        }
    outcom:boolean=FALSE;         { outputting to com?                    }
    printingfile:boolean=FALSE;   { are we printing a file?               }
    quitafterdone:boolean=FALSE;  { quit after next user logs off?        }
    reading_a_msg:boolean=FALSE;  { is user reading a message?            }
    readingmail:boolean=FALSE;    { reading private mail?                 }
    read_with_mci:boolean=FALSE;  { read message with MCI?                }
    shutupchatcall:boolean=FALSE; { was chat call "SHUT UP" for this call? }
    trapping:boolean=FALSE;       { are we trapping users text?           }
    useron:boolean=FALSE;         { is there a user on right now?         }
    wantfilename:boolean=FALSE;   { display message filename in scan?     }
    wasnewuser:boolean=FALSE;     { did a NEW USER log on?                }
    write_msg:boolean=FALSE;      { is user writing a message?            }
    telluserevent:byte=0;     { has user been told about the up-coming event? }
    exiterrors:byte=254;          { ERRORLEVEL for Critical Error exit    }
    exitnormal:byte=255;          { ERRORLEVEL for Normal exit            }
    cursorthere:boolean=TRUE;     { is cursor on? }
var
    first_time:boolean;           { first time loading a menu?            }
    menustack:array[1..8] of string[12]; { menu stack                     }
    menustackptr:integer;         { menu stack pointer                    }
    last_menu,                    { last menu loaded                      }
    curmenu:string;               { current menu loaded                   }
    menur:menurec;                { menu information                      }
    cmdr:array[1..50] of commandrec; { command information                }
    noc:integer;                  { # of commands on menu                 }
    fqarea,mqarea:boolean;        { file/message quick area changes       }
    doit,doitt:boolean;
    newdate:string[8];            { NewScan pointer date                  }
    lrn:integer;                  { last record # for recno/nrecno        }
    lfn:string;                     { last filename for recno/nrecno        }

    batchtime:real;               { }
    numbatchfiles:integer;        { # files in DL batch queue             }
    batch:array[1..20] of record
      fn:string[65];
      section:integer;
      pts:integer;
      blks:longint;
      tt:real;
    end;
    numubatchfiles:integer;       { # files in UL batch queue }
    ubatch:array[1..maxubatchfiles] of record
      fn:string[12];
      section:integer;
      description:string[40];
      vr:byte;
    end;
    ubatchv:array[1..maxubatchfiles] of ^verbrec;
    hiubatchv:integer;
    hdrstr:string[60];
    OldTempdir:string;
    tempdirchanged:boolean;
    changeablemci:string[30];

procedure loaduboard(i:integer);
procedure loadboard(i:integer);
procedure sprompt(s:string);
function fbaseac(b:byte):boolean;
function mbaseac(nb:integer):boolean;
procedure changefileboard(b:integer);
procedure changeboard(b:integer);
function freek(d:integer):longint;    (* See disk space *)
function totalk(d:integer):longint;   (* See total space *)
function okansi:boolean;
function okavatar:boolean;
function nsl:real;
function ageuser(bday:string):integer;     (* returns age of user by birthdate *)
{function allcaps(s:string):string;    (* returns a COMPLETELY capitalized string *)}
function caps(s:string):string;                (* returns a capitalized string.. *)
function cinkey:char;
function intime(tim:real; tim1,tim2:integer):boolean;
function getc(c:byte):string;
function stripcolor(o:string):string;
procedure sl1(s:string);
procedure sysoplog(s:string);
function tch(s:string):string;
function time:string;
function date:string;
function value(s:string):longint;
function cstr(i:longint):string;
function nam:string;
function leapyear(yr:integer):boolean;
function days(mo,yr:integer):integer;
function daycount(mo,yr:integer):integer;
function daynum(dt:string):integer;
function dat:string;
procedure getkey(var c:char);
procedure pr1(s:string);
procedure pr(s:string);
procedure setc(c:byte);
procedure cl(c:integer);
procedure prompt(s:string);
function sqoutsp(s:string):string;
function exdrv(s:string):byte;
procedure print(s:string);
procedure nl;
procedure prt(s:string);
procedure prestrict(u:userrec);
procedure loadurec(var u:userrec; i:integer);
procedure saveurec(u:userrec; i:integer);
function empty:boolean;
function inkey:char;
procedure outkey(c:char);
function checkeventday(i:integer; t:real):boolean;
function checkpreeventtime(i:integer; t:real):boolean;
function checkeventtime(i:integer; t:real):boolean;
function checkevents(t:real):integer;
procedure dm(i:string; var c:char);
Procedure ClearScreen(attr : Byte; ch : Char);
procedure cls;
procedure swac(var u:userrec; r:uflags);
function tacch(c:char):uflags;
procedure acch(c:char; var u:userrec);
procedure sprint(s:string);
procedure lcmds(len,c:byte; c1,c2:string);
procedure autovalidate(var u:userrec; un:integer);
function aonoff(b:boolean; s1,s2:string):string;
function onoff(b:boolean):string;
function syn(b:boolean):string;
procedure pyn(b:boolean);
function pynq(s:string;Default:boolean):boolean;
function centre(s:string):string;
procedure wkey(var abort,next:boolean);
function ctim(rl:real):string;
function tlef:string;
procedure printacr(s:string; var abort,next:boolean);
function longtim(dt:datetimerec):string;
function dt2r(dt:datetimerec):real;
procedure r2dt(r:real; var dt:datetimerec);
procedure timediff(var dt:datetimerec; dt1,dt2:datetimerec);
procedure getdatetime(var dt:datetimerec);
function cstrl(li:longint):string;
function cstrr(rl:real; base:integer):string;
procedure pfl(fn:string; var abort,next:boolean; cr:boolean);
procedure printfile(fn:string);
function exist(fn:string):boolean;
function aacs1(u:userrec; un:integer; s:string):boolean;
function aacs(s:string):boolean;
function strippipes(s : string) : string;
function pipelen(s : string) : byte;


implementation

uses myio    , tmpcom  , Script  , common5 , scrlbk  ,
     common1 , ansidrv , common2 , mail3   , common3 ,
     menus   , menus2  , strproc, sys;

procedure autovalidate(var u:userrec; un:integer);
var settings : set of uflags;
    b:boolean;
begin
   settings:=[rlogon,rchat,rvalidate,rbackspace,ramsg,rpostan,rpost,remail,
              rvoting,rmsg,fnodlratio,fnopostratio,fnofilepts,fnodeletion];
   with u do begin
      if (un=usernum) then begin
         realsl:=sl;
         realdsl:=dsl;
         newcomptables;
      end;
      sl:=status^.autosl;
      dsl:=status^.autodsl;
      ac:=ac-settings;
      ac:=ac+(status^.autoac*settings);
      (* do NOT modify user's personal settings, such as ANSI, color, etc.. *)
      ar:=status^.autoar;
      filepoints:=status^.newfp;
      tltoday:=status^.timeallow[sl];
   end;
end;

procedure loaduboard(i:integer);
var ulfo:boolean;
begin
   if (readuboard<>i) then begin
      ulfo:=(filerec(ulf).mode<>fmclosed);
      if (not ulfo) then reset(ulf);
      if ((i>=1) and (i<=filesize(ulf)-1)) then begin
         seek(ulf,i);
         read(ulf,memuboard);
      end
      else memuboard:=tempuboard;
      readuboard:=i;
      if (not ulfo) then close(ulf);
   end;
end;

procedure loadboard(i:integer);
var bfo:boolean;
begin
   if (readboard<>i) then begin
      bfo := (filerec(bf).mode<>fmclosed);
      if (not bfo) then reset(bf);
      if ((i-1<0) or (i-1>filesize(bf)-1)) then i:=1;
      seek(bf,i-1); read(bf,memboard);
      readboard:=i;
      if (not bfo) then close(bf);
   end;
end;

procedure lcmds(len,c:byte; c1,c2:string);
var s:string;
begin
   s:=copy(c1,2,lenn(c1)-1);
   if (c2<>'') then s:=mln(s,len-1);
   sprint('|U9[|U0'+c1[1]+'|U9]|U0'+s);
   if (c2<>'') then sprint('|U9[|U0'+c2[1]+'|U9]|U0'+copy(c2,2,lenn(c2)-1));
end;

function fbaseac(b:byte):boolean;
begin
   fbaseac:=FALSE;
   if ((b<0) or (b>maxulb)) then exit;
   loaduboard(b);
   fbaseac:=aacs(memuboard.acs);
end;

function mbaseac(nb:integer):boolean;
begin
   mbaseac:=FALSE;
   if ( (nb >= 1) OR (nb <= numboards) ) then begin
      loadboard(nb);
      mbaseac:=aacs(memboard.acs);
   end;
end;

procedure changefileboard(b:integer);
var s:string[20];
    go:boolean;
begin
   go:=FALSE;
   if (b>=1) and (b<=maxulb) then
      if (fbaseac(b)) then { fbaseac loads memuboard itself ... }
         if (memuboard.password='') then go:=TRUE
   else begin
      nl;
      sprint('File base '+cstr(ccuboards[1][b])+': '+#3#5+memuboard.name);
      prompt('Password: ');
      input(s,20);
      if (s=memuboard.password) then go:=TRUE
      else print('Wrong!');
   end;
   if (go) then begin
      fileboard:=b;
      thisuser.lastfil:=fileboard;
   end;
end;

procedure changeboard(b:integer);
var s:string[20];
    go:boolean;
begin
   go:=FALSE;
   if (b>=1) and (b<=numboards) then
      if (mbaseac(b)) then { mbaseac loads memboard itself ... }
         if (memboard.password='') then go:=TRUE
         else begin
            nl;
            sprint('Message base '+cstr(ccboards[1][b])+': '+memboard.name);
            prompt('Password: ');
            input(s,20);
            if (s=memboard.password) then go:=TRUE
            else print(strings^.wrongPassword);
         end;
         if (go) then begin
            board:=b;
            thisuser.lastmsg:=board;
         end;
end;

function freek(d:integer):longint;
begin
   freek := diskfree(d) div 1024;
end;

function totalk(d:integer):longint;
begin
   totalk := diskSize(d) div 1024;
end;

function okansi:boolean;
begin
   okansi := ((ansi in thisuser.ac) or (avatar in thisuser.ac));
end;

function okavatar:boolean;
begin
   okavatar := (avatar in thisuser.ac);
end;

function nsl:real;
var ddt,
    dt:datetimerec;
    beenon:real;
begin
   if ((useron) or (not inwfcmenu)) then begin
      getdatetime(dt);
      timediff(ddt,timeon,dt);
      beenon:=dt2r(ddt);
      nsl:=((thisuser.tltoday*60.0+extratime+freetime)-(beenon+choptime));
   end
   else nsl:=3600.0
end;

function waitackfile:boolean;
var rl:real;
begin
   rl:=timer;
   waitackfile:=TRUE;
   repeat
      if (not com_rx_empty) then begin
         case com_rx of
	    #6 : exit;                                  { ACK }
	    #21 : begin
               waitackfile:=FALSE;
               exit;
            end;  { NAK }
         end;
      end;
   until (timer-rl>10.0);
   waitackfile:=FALSE;
end;

procedure sendfilep(s:string);
var f:file of char;
    c:char;
begin
   assign(f,s);
   {$I-} reset(f); {$I+}
   if (ioresult<>0) then begin
      nl;
      print('"'+s+'": File not found.');
      nl;
   end
   else begin
      if (waitackfile) then begin
         while (not eof(f)) do begin
            read(f,c);
            com_tx(c);
         end;
         pr1(^Z^Z^Z);
      end;
     close(f);
   end;
end;

function cinkey:char;
var tar:array[1..20] of char;
    rl:real;
    tarc:integer;
    c:char;
begin
   if (recom1(c)) then begin
      cinkey:=c;
      if (c=^A) then begin
         tarc:=1;
         tar[1]:=^B;
         rl:=timer;
         repeat
	    if (recom1(c)) then begin
               tar[tarc]:=c;
               inc(tarc);
            end;
         until ((timer-rl>2.0) or (tarc>11) or (tar[1]<>^B));
      end;
   end
   else cinkey:=#0;
end;

function intime(tim:real; tim1,tim2:integer):boolean;
(* "tim" is seconds (timer) time; tim1/tim2 are minutes time. *)
begin
   intime:=TRUE;
   while (tim>=24.0*60.0*60.0) do tim:=tim-24.0*60.0*60.0;
   if (tim1<>tim2) then
      if (tim2>tim1) then
         if (tim<=tim1*60.0) or (tim>=tim2*60.0) then intime:=FALSE
   else if (tim<=tim1*60.0) and (tim>=tim2*60.0) then intime:=FALSE;
end;

procedure opensysopf;
begin
   assign(sysopf,status^.trappath+'sysop.'+cstr(thisnode));
   {$I-} append(sysopf); {$I+}
   if (ioresult<>0) then begin
      rewrite(sysopf);
      append(sysopf);
   end;
end;

function stripcolor(o:string):string;
var s:string;
    i:integer;
    lc:boolean;
begin
   s:='';
   lc:=FALSE;
   for i:=1 to length(o) do
      if (lc) then lc:=FALSE
      else if ((o[i]=#3) or (o[i]='^')) then lc:=TRUE else s:=s+o[i];
   stripcolor:=s;
end;

procedure sl1(s:string);
begin
   if (slogging) then begin
      s:=stripcolor(s);
      if (textrec(sysopf).mode<>fmoutput) then opensysopf;
      writeln(sysopf,s);
      if ((thisuser.slogseperate) and (textrec(sysopf1).mode=fmoutput)) then writeln(sysopf1,s);
   end;
end;

procedure sysoplog(s:string);
begin
   sl1('   '+s);
end;

function tch(s:string):string;
begin
   if (length(s)>2) then s:=copy(s,length(s)-1,2)
   else if (length(s)=1) then s:='0'+s;
   tch:=s;
end;

function time:string;
var h,m,s:string[3];
    hh,mm,ss,ss100:word;
begin
   gettime(hh,mm,ss,ss100);
   str(hh,h);
   str(mm,m);
   str(ss,s);
   time:= tch(h) + ':' + tch(m) + ':' + tch(s);
end;

function date:string;
var r:registers;
    y,m,d:string[3];
    yy,mm,dd,dow:word;
begin
   getdate(yy,mm,dd,dow);
   str(yy-1900,y);
   str(mm,m);
   str(dd,d);
   date:=tch(m)+'/'+tch(d)+'/'+tch(y);
end;

function value(s:string):longint;
var i:longint;
    j:integer;
begin
   val(s,i,j);
   if (j<>0) then begin
      s := copy(s,1,j-1);
      val(s,i,j)
   end;
   value:=i;
   if (s='') then value:=0;
end;

function cstr(i:longint):string;
var c:string[16];
begin
  str(i,c);
  cstr:=c;
end;

function nam:string;
begin
  nam := caps(thisuser.name)+' #'+cstr(usernum);
end;

function ageuser(bday:string):integer;
var i:integer;
begin
   i:=value(copy(date,7,2))-value(copy(bday,7,2));
   if (daynum(copy(bday,1,6)+copy(date,7,2))>daynum(date)) then dec(i);
   ageuser:=i;
end;

{function allcaps(s:string):string;
var i:integer;
begin
  for i:=1 to length(s) do s[i]:=upcase(s[i]);
  allcaps:=s;
end;}

function caps(s:string):string;
var i:integer;
begin
  for i:=1 to length(s) do
    if (s[i] in ['A'..'Z']) then s[i]:=chr(ord(s[i])+32);
  for i:=1 to length(s) do
    if (not (s[i] in ['A'..'Z','a'..'z'])) then
      if (s[i+1] in ['a'..'z']) then s[i+1]:=upcase(s[i+1]);
  s[1]:=upcase(s[1]);
  caps:=s;
end;

function leapyear(yr:integer):boolean;
begin
   leapyear:=(yr mod 4=0) and ((yr mod 100<>0) or (yr mod 400=0));
end;

function days(mo,yr:integer):integer;
var d:integer;
begin
   d:=value(copy('312831303130313130313031',1+(mo-1)*2,2));
   if ((mo=2) and (leapyear(yr))) then inc(d);
   days:=d;
end;

function daycount(mo,yr:integer):integer;
var m,t:integer;
begin
   t:=0;
   for m:=1 to (mo-1) do t:=t+days(m,yr);
   daycount:=t;
end;

function daynum(dt:string):integer;
var d,m,y,t,c:integer;
begin
   t:=0;
   m:=value(copy(dt,1,2));
   d:=value(copy(dt,4,2));
   
   { Fixed Y2K handling for 2-digit years:
     Years 00-79 are treated as 2000-2079
     Years 80-99 are treated as 1980-1999
     This allows the BBS to work with dates from 1980 through 2079 }
   y:=value(copy(dt,7,2));
   if y < 80 then 
      y:=y+2000 
   else 
      y:=y+1900;
   
   for c:=1985 to y-1 do
      if (leapyear(c)) then inc(t,366) else inc(t,365);
   t:=t+daycount(m,y)+(d-1);
   daynum:=t;
   
   { Allow dates from 1980 onwards instead of 1985 }
   if y<1980 then daynum:=0;
end;

function dat:string;
const mon : array [1..12] of string[3] =
	   ('Jan','Feb','Mar','Apr','May','Jun',
	    'Jul','Aug','Sep','Oct','Nov','Dec');
var ap : string;
    i:integer;
    year,
    month,
    day,
    dayofweek,
    hour,
    minute,
    second,
    sec100:word;
begin
   getdate(year,month,day,dayofweek);
   gettime(hour,minute,second,sec100);
   if (hour<12) then ap:='am'
   else begin
      ap := 'pm';
      if (hour>12) then dec(hour,12);
   end;
   if (hour=0) then hour:=12;
   dat:=cstr(hour)+':'+tch(cstr(minute))+' '+ap+'  '+
             copy('SunMonTueWedThuFriSat',dayofweek*3+1,3)+' '+
             mon[month]+' '+cstr(day)+', '+cstr(year);
end;

procedure pr1(s:string);
var i:integer;
begin
   for i:=1 to length(s) do sendcom1(s[i]);
end;

procedure pr(s:string);
begin
   pr1(s+#13);
end;

function getc(c:byte):string;
const xclr: array[0..7] of char=('0','4','2','6','1','5','3','7');
var s:string;
    b:boolean;
   procedure adto(ss:string);
   begin
      if (s[length(s)]<>';') and (s[length(s)]<>'[') then s:=s+';';
      s:=s+ss;
      b:=TRUE;
   end;

begin
   b:=FALSE;
   if ((curco and (not c)) and $88)<>0 then begin
      s:=#27+'[0';
      curco:=$07;
   end
   else s:=#27+'[';
   if (c and 7<>curco and 7) then adto('3'+xclr[c and 7]);
   if (c and $70<>curco and $70) then adto('4'+xclr[(c shr 4) and 7]);
   if (c and 128<>0) then adto('5');
   if (c and 8<>0) then adto('1');
   if (not b) then adto('3'+xclr[c and 7]);
   s:=s+'m';
   getc:=s;
end;

procedure setc(c:byte);
var s:string;
    i:integer;
begin
   if ((c<>curco) or (dosansion)) then begin
      s:=getc(c);
      curco:=c;
      if (okansi) then begin
         if (outcom) then pr1(s);
         if (wantout) then begin
            textattr:=c;
            if (dosansion) then begin
               s:=#27+'[0;'+copy(s,3,length(s)-2);
               for i:=1 to length(s) do dosansi(s[i]);
            end;
         end;
         textAttr := curco;
      end;
   end;
end;

procedure cl(c:integer);
begin
   if (c in [0..9]) then
      if (okansi) then setc(thisuser.cols[(color in thisuser.ac)][c]);
end;

function sqoutsp(s:string):string;
begin
   while (pos(' ',s)>0) do delete(s,pos(' ',s),1);
   sqoutsp:=s;
end;

function exdrv(s:string):byte;
begin
   s:=fexpand(s);
   exdrv:=ord(s[1])-64;
end;

procedure prompt(s:string);
var s1,s2:string;
    i:integer;
    bb:byte;
begin
   checkhangup;
   if (hangup) then exit;
   if (outcom) then begin
      s1:=s;
      while (pos(^J,s1)<>0) do begin
         i:=pos(^J,s1);
         s2:=copy(s,1,i-1);
         s1:=copy(s1,i+1,length(s1)-i);
         for i:=1 to length(s2) do sendcom1(s2[i]);
         if ((not ch) and (not write_msg) and (not reading_a_msg)) then
	    if (not ctrljoff) then begin
	       bb:=thisuser.cols[color in thisuser.ac][1];
	       if (okansi) then pr1(getc(bb));
	       curco:=bb;
	    end
            else lil:=0;
         sendcom1(^J);
      end;
      for i:=1 to length(s1) do sendcom1(s1[i]);
   end;
   for i:=1 to length(s) do begin
      if dosAnsiOn then cAnsi(s[i])
      else write(s[i]);
   end;
   if (trapping) then
      if (copy(s,length(s)-1,2)=^M^J) then writeln(trapfile,copy(s,1,length(s)-2))
      else write(trapfile,s);
end;

procedure print(s:string);
begin
   prompt(s+^M^J);
end;

procedure nl;
begin
   prompt(^M^J);
end;

procedure prt(s:string);
begin
   prompt(S);
end;

function substone(src,old,new:string):string;
var p:integer;
begin
   if (old<>'') then begin
      p:=pos(old,strAllCaps(src));
      if (p>0) then begin
         insert(new,src,p+length(old));
         delete(src,p,length(old));
      end;
   end;
   substone:=src;
end;

procedure sprompt(s:string);
var ss,sss:string;
    r,i,p1,p2,x,z:integer;
    c,mc:char;
    xx,b:boolean;
begin
   checkHangup;
   if (hangup) then exit;
   ss:=s;
   sss:='';
   b:=FALSE;
   if (pos('##',ss)=1) then begin
      if (not write_msg) and (not reading_a_msg) then begin
         ss:=copy(ss,3,length(ss));
         curmenu:=status^.menupath+ss+'.mnu';
         readin;
      end;
   end
   else if (pos('&&',ss)=1) then begin
      if (not write_msg) and (not reading_a_msg) then begin
         ss:=copy(ss,3,length(ss));
         doscript(ss);
      end;
   end
   else if (pos('~~',ss)=1) then begin
      if (not write_msg) and (not reading_a_msg) then begin
         ss:=copy(ss,3,length(ss));
         printf(ss);
      end;
   end
   else if (pos('|',ss)<>0) then oPrintMci(ss)
   else begin
      if (trapping) then write(trapfile,ss);
      if (not okansi) then ss:=stripcolor(ss)
      else while (ss<>'') and ((pos(#3,ss)<>0) or (pos('^',ss)<>0)) do begin
         p1:=pos(#3,ss);
         if (p1=0) then p1:=500;
         p2:=pos('^',ss);
         if (p2=0) then p2:=500;
         if (p2<p1) then p1:=p2;
         if (p1<>500) then begin
	    mc:=ss[p1+1];
            sss:=copy(ss,1,p1-1);
	    ss:=copy(ss,p1+2,length(ss)-(p1+1));
         end
         else begin
            sss:=ss;
            ss:='';
         end;
         if (outcom) then for i:=1 to length(sss) do sendcom1(sss[i]);
         for i:=1 to length(sss) do lpromptc(sss[i]);
         if ((mc>=#0) and (mc<=#9)) then cl(ord(mc))
         else if ((mc>='0') and (mc<='9')) then cl(ord(mc)-48);
      end;
      if (outcom) then for i:=1 to length(ss) do sendcom1(ss[i]);
      for i:=1 to length(ss) do lpromptc(ss[i]);
   end;
end;

procedure sprint(s:string);
begin
   sprompt(s+^M^J);
end;

procedure prestrict(u:userrec);
var r:uflags;
begin
   for r:=rlogon to rmsg do
      if (r in u.ac) then write(copy('LCVBA*PEKM',ord(r)+1,1))
      else write('-');
   writeln;
end;

function empty:boolean;
var e:boolean;
begin
   e:=(not keypressed);
   if ((incom) and (e)) then e:=(com_rx_empty);
   if (hangup) then begin
      com_flush_rx;
      e:=TRUE;
   end;
   empty:=e;
end;

function inkey:char;
var c:char;
begin
   c:=#0;
   inkey:=#0;
   checkhangup;
   if (keypressed) then begin
      c:=readkey;
      if ((c=#0) and (keypressed)) then begin
         c:=readkey;
         sysopKeys(c);
         if fuckedup then pauseScr;
         if (c=#68) then c:=#1
         else if((c<>'H') and (c<>'P')) then c:=#0;
         if (buf<>'') then begin
	    c:=buf[1];
	    buf:=copy(buf,2,length(buf)-1);
         end;
      end;
      inkey:=c;
   end
   else if (incom) then inkey:=cinkey;
end;

procedure docc2(c:char);
var i:integer;
begin
   case c of
      ^G: if (outcom) then for i:=1 to 4 do sendcom1(#0);
      ^J:begin
         if (wantout) then write(^J);
	 inc(pap);
      end;
      ^L:begin
         if (wantout) then clrscr;
	 lil:=0;
      end;
   end;
end;

procedure outkey(c:char);
begin
   if (c=#29) then exit;
{   if (not echo) then
      if ((status^.localsec) and (c in [#32..#255])) then c:=strings^.echoc;}
   if (c=#27) then dosansion:=TRUE;
   if (not (c in [^J,^L])) then
      if (not ((c=^G) and (incom))) then
         if ((c<>#0) and (wantout)) then
            if (not dosansion) then write(c) else dosansi(c);
   if ((not echo) and (c in [#32..#255])) then c:=strings^.echoc;
   if (c<#32) then docc2(c);
   if (outcom) then sendcom1(c);
end;

function checkeventday(i:integer; t:real):boolean;
var s:string;
    year,month,day,dayofweek:word;
    e:integer;
begin
   checkeventday:=FALSE;
   with events[i]^ do begin
      getdate(year,month,day,dayofweek);
      e:=0;
      if (timer+t>=24.0*60.0*60.0) then begin
         inc(dayofweek);
         e:=1;
         if (dayofweek>6) then dayofweek:=0;
      end;
      if (monthly) then begin
         if (value(copy(date,4,2))+e=execdays) then checkeventday:=TRUE;
      end
      else begin
         if ((1 shl (6-dayofweek)) and execdays<>0) then checkeventday:=TRUE;
      end;
   end;
end;

function checkpreeventtime(i:integer; t:real):boolean;
begin
   with events[i]^ do
      if (busytime=0) then checkpreeventtime:=FALSE
      else checkpreeventtime:=intime(timer+t,exectime-busytime,exectime);
end;

function checkeventtime(i:integer; t:real):boolean;
begin
   with events[i]^ do
      if (duration=0) then checkeventtime:=FALSE
      else checkeventtime:=intime(timer+t,exectime,exectime+duration);
end;

function checkevents(t:real):integer;
var i:integer;
begin
   for i:=0 to numevents do
      with events[i]^ do
         if (active) then
	    if (checkeventday(i,t)) then begin
	       checkevents:=i;
	       if (checkpreeventtime(i,t)) or (checkeventtime(i,t)) then begin
	          if (etype in ['D','E','P']) then exit;
	          if ((etype='A') and (not aacs(execdata)) and (useron)) then exit;
	       end;
	    end;
   checkevents:=0;
end;

procedure dm(i:string; var c:char);
begin
   buf:=i;
   if (buf<>'') then begin
      c:=buf[1];
      buf:=copy(buf,2,length(buf)-1);
   end;
end;

procedure getkey(var c:char);
var dt,ddt:datetimerec;
    aphase,e:integer;
    abort,next,b,tf,t1,bufalready:boolean;
begin
   lil:=0;
   if (buf<>'') then begin
      c:=buf[1];
      buf:=copy(buf,2,length(buf)-1);
   end
   else begin
      if (not empty) then begin
         if (ch) then c:=chinkey
         else c:=inkey;
      end
      else begin
         getdatetime(tim);
         t1:=FALSE;
         tf:=FALSE;
         c:=#0;
         if (alert in thisuser.ac) then aphase:=1
         else aphase:=0;
         while ((c=#0) and (not hangup)) do begin
            timeSlice;
	    if (aphase<>0) then begin
	       case aphase of
	          1: begin
                     sound(1000);
                     cdelay(35);
                  end;
	          2: begin
                     sound(1500);
                     cdelay(40);
                  end;
	          3: begin
                     sound(1900);
                     cdelay(45);
                  end;
	          4: begin
                     sound(2300);
                     cdelay(50);
                  end;
	          5: begin
                     sound(3400);
                     cdelay(55);
                  end;
	       end;
	       aphase:=aphase mod 5+1;
	    end;
	    if (ch) then c:=chinkey
            else c:=inkey;
	    getdatetime(dt);
	    timediff(ddt,tim,dt);
            if (status^.timeout<>-1) and (spd <> strings^.locallogin) and (not inwfcmenu) and
	        (dt2r(ddt)>status^.timeout*60) and (c=#0) then begin
	       printf('timedout');
	       if (nofile) then print('Time out has occurred.  Log off time was at '+time+'.');
	       hangup:=TRUE;
	       sysoplog(#3#7+'- Time-out at '+time+' -');
	    end;
            if (status^.timeoutbell<>-1) and (spd <> strings^.locallogin) and (not inwfcmenu) and
	        (dt2r(ddt)>status^.timeoutbell*60) and (not tf) and (c=#0) then begin
	       tf:=TRUE;
	       outkey(^G);
               outkey(^G);
	    end;
	    checkhangup;
         end;
         nosound;
      end;
   end;
end;

Procedure ClearScreen(attr : Byte; ch : Char);
begin
   Asm
      mov  ax, $b800
      mov  es, ax
      xor  di, di
      mov  cx, 80*24
      mov  ah, attr
      mov  al, &ch
      rep  stosw
   end;
   setc(attr);
   goxy(1,1);
end;

procedure cls;
begin
   if okscrollback then begin
      if beforeview then begin
         dec(curscrlline);
         dec(amtscrllines);
         beforeview:=false;
      end;
      addtoscroll(wherey);
   end;  {Scrollback Shit}
   if (okansi) then begin
      if (outcom) then pr(#27+'[2J');
      if (wantout) then clearscreen(7,' ');
   end
   else outkey(^L);
   if (trapping) then writeln(trapfile,^L);
   cl(1);
   lil:=0;
end;

procedure swac(var u:userrec; r:uflags);
begin
   if (r in u.ac) then u.ac:=u.ac-[r]
   else u.ac:=u.ac+[r];
end;

function tacch(c:char):uflags;
begin
   case c of
      'L':tacch:=rlogon;
      'C':tacch:=rchat;
      'V':tacch:=rvalidate;
      'B':tacch:=rbackspace;
      'A':tacch:=ramsg;
      '*':tacch:=rpostan;
      'P':tacch:=rpost;
      'E':tacch:=remail;
      'K':tacch:=rvoting;
      'M':tacch:=rmsg;
      '1':tacch:=fnodlratio;
      '2':tacch:=fnopostratio;
      '3':tacch:=fnofilepts;
      '4':tacch:=fnodeletion;
   end;
end;

procedure acch(c:char; var u:userrec);
begin
   swac(u,tacch(c));
end;

function aonoff(b:boolean; s1,s2:string):string;
begin
   if (b) then aonoff:=s1
   else aonoff:=s2;
end;

function onoff(b:boolean):string;
begin
   if (b) then onoff:='On '
   else onoff:='Off';
end;

function syn(b:boolean):string;
begin
   if (b) then syn:='Yes'
   else syn:='No ';
end;

procedure pyn(b:boolean);
begin
  print(syn(b));
end;

function strippipes(s : string) : string;
begin
   while (pos('|',s)<>0) do delete(s, pos('|',s), 3);
   strippipes := s;
end;

function pipelen(s : string) : byte;
var i : byte;
    len : byte;
begin
   len := length(strippipes(s));
   pipelen := len;
end;

function pynq(s:string;Default:boolean):boolean;
var c:char;
    xx,
    yy : byte;
    stop : boolean;
begin
   if (not hangup) then begin
      sprompt(s+' ');
      xx := whereX;
      yy := whereY;
      pynq := default;
      stop := false;
      repeat
         goXY(xx,yy);
         if (default) then sprompt(strings^.yesBar)
         else sprompt(strings^.noBar);
         repeat
            getkey(c);
            c:=upcase(c);
         until (c in ['K',' ','Y','N','M','P','C','D',^M,^N]) or (hangup);
         case c of
            ' ','C','M','D','K': default := not(default);
            'N':begin
               default := false;
               pynq:=false;
               stop := true;
            end;
            'Y': begin
               default := true;
               pynq:=true;
               stop := true;
            end;
            ^M : stop := true;
         end;
      until (stop) or (hangup);
      if (hangup) then pynq:=FALSE;
      pynq := default;
   end;
end;


function centre(s:string):string;
var i,j:integer;
begin
   if (pap<>0) then nl;
   if (s[1]=#2) then s:=copy(s,2,length(s)-1);
   i:=length(s); j:=1;
   while (j<=length(s)) do begin
      if s[j]=#3 then begin
         dec(i,2);
         inc(j);
      end;
      inc(j);
   end;
   if i<thisuser.linelen then
      s:=copy('                                               ',1,
              (thisuser.linelen-i) div 2)+s;
   centre:=s;
end;

procedure wkey(var abort,next:boolean);
var c:char;
begin
   if (empty) then exit;
   if ((abort) or (hangup)) then exit;
   getkey(c);
   case upcase(c) of
      ' ',^C,^Y,^K:abort:=TRUE;
      'N',^N: begin
         abort:=TRUE;
         next:=TRUE;
      end;
      'P',^S:getkey(c);
   end;
   if (not allowabort) then begin
      abort:=FALSE;
      next:=FALSE;
   end;
   if (abort) then begin
      com_purge_tx;
      nl;
      sprint(strings^.aborted);
   end;
end;

function ctim(rl:real):string;
var h,m,s:string;
begin
   s:=tch(cstr(trunc(rl-int(rl/60.0)*60.0)));
   m:=tch(cstr(trunc(int(rl/60.0)-int(rl/3600.0)*60.0)));
   h:=cstr(trunc(rl/3600.0));
   if (length(h)=1) then h:='0'+h;
   ctim:=h+':'+m+':'+s;
end;

function tlef:string;
begin
   tlef:=ctim(nsl);
end;

function longtim(dt:datetimerec):string;
var s:string;
    d:integer;

   procedure ads(comma:boolean; i:integer; lab:string);
   begin
      if (i<>0) then begin
         s:=s+cstrl(i)+' '+lab;
         if (i<>1) then s:=s+'s';
         if (comma) then s:=s+', ';
      end;
   end;

begin
   s:='';
   with dt do begin
      d:=day;
      if (d>=7) then begin
         ads(TRUE,d div 7,'week');
         d:=d mod 7;
      end;
      ads(TRUE,d,'day');
      ads(TRUE,hour,'hour');
      ads(TRUE,min,'minute');
      ads(FALSE,sec,'second');
   end;
   if (s='') then s:='0 seconds';
   if (copy(s,length(s)-1,2)=', ') then s:=copy(s,1,length(s)-2);
   longtim:=s;
end;

function dt2r(dt:datetimerec):real;
begin
   with dt do dt2r:=day*86400.0+hour*3600.0+min*60.0+sec;
end;

procedure r2dt(r:real; var dt:datetimerec);
begin
   with dt do begin
      day:=trunc(r/86400.0); r:=r-(day*86400.0);
      hour:=trunc(r/3600.0); r:=r-(hour*3600.0);
      min:=trunc(r/60.0); r:=r-(min*60.0);
      sec:=trunc(r);
   end;
end;

procedure timediff(var dt:datetimerec; dt1,dt2:datetimerec);
begin
   with dt do begin
      day:=dt2.day-dt1.day;
      hour:=dt2.hour-dt1.hour;
      min:=dt2.min-dt1.min;
      sec:=dt2.sec-dt1.sec;
      if (hour<0) then begin
         inc(hour,24);
         dec(day);
      end;
      if (min<0) then begin
         inc(min,60);
         dec(hour);
      end;
      if (sec<0) then begin
         inc(sec,60);
         dec(min);
      end;
   end;
end;

procedure getdatetime(var dt:datetimerec);
var w1,w2,w3,w4:word;
begin
   gettime(w1,w2,w3,w4);
   with dt do begin
      day:=daynum(date);
      hour:=w1;
      min:=w2;
      sec:=w3;
   end;
end;

function cstrl(li:longint):string;
var c:string;
begin
   str(li,c);
   cstrl:=c;
end;

function cstrr(rl:real; base:integer):string;
var i:integer;
    s:string;
    r1,r2:real;
begin
   if (rl<=0.0) then cstrr:='0'
   else begin
      r1:=ln(rl)/ln(1.0*base);
      r2:=exp(ln(1.0*base)*(trunc(r1)));
      s:='';
      while (r2>0.999) do begin
         i:=trunc(rl/r2);
         s:=s+copy('0123456789ABCDEF',i+1,1);
         rl:=rl-i*r2;
         r2:=r2/(1.0*base);
      end;
      cstrr:=s;
   end;
end;

{procedure printa1(s:string; var abort,next:boolean);
var s1,s2,ss,sss,ssss,tcode,mcix,mcixx:string;
    i,ls,p1,p2,p3:integer;
    c,mc:char;
    savcurco:byte;
    isansi,iscolor,ismci,istcode,usetcodes:boolean;

  procedure dotcode(c:char; var s:string);
  var s1,s2:string;
  begin
    case mc of
      'c':if (pos(';',s)<>0) then begin
            s1:=copy(s,1,pos(';',s)-1);
            delete(s,1,length(s1)+1);
          end;
      'C':begin
            if (okansi) then setc(ord(s[1]));
            delete(s,1,1);
          end;
    end;
  end;

  procedure sends(s:string);
  var i:word;
  begin
    i:=0;
    while (i<length(s)) do begin
      inc(i);
      sendcom1(s[i]);
    end;
  end;

  procedure locs(s:string);
  var i:integer;
  begin
    i:=0;
    while (i<length(s)) do begin
      inc(i);
      lpromptc(s[i]);
    end;
  end;

  procedure handlecolors;
  label goto1;
  begin
      goto1:  { ack! }
 {   mc:=ss[p1+1]; sss:=copy(ss,1,p1-1);
    ss:=copy(ss,p1+2,length(ss)-(p1+1));

    if (outcom) then sends(sss);
    locs(sss);

    if ((mc>=#0) and (mc<=#9)) then cl(ord(mc)) else
      if ((mc>='0') and (mc<='9')) then cl(ord(mc)-48);

    p1:=pos(#3,ss);
    if (p1<>0) then goto goto1;
  end;

  procedure handletcodes;
  label goto1;  { *ACK!* }
{  begin
      goto1:
    if ((p3<p2) and (p3<>0)) then begin
      istcode:=TRUE;
      p2:=p3;
    end else
      istcode:=FALSE;

    mc:=ss[p2+1]; sss:=copy(ss,1,p2-1);
    ss:=copy(ss,p2+2,length(ss)-(p2+1));

    if (outcom) then sends(sss);
    locs(sss);

    if (istcode) then dotcode(mc,ss);

    p2:=pos('õ',ss); p3:=pos(^T,ss);
    if (p2+p3>0) then goto goto1;
  end;

  procedure handletcodesc;
  label goto1;  { **ACK!!!*!*!***** }
{  begin
      goto1:
    if (p2<>500) then
      if (pos(ss[p2+1],mcixx)=0) then p2:=500;

    iscolor:=TRUE; istcode:=FALSE;
    if ((p2<p1) or (p3<p1)) then
      if ((p2<p1) and (p2<p3)) then begin p1:=p2; iscolor:=FALSE; end else
        if (p3<p1) then begin p1:=p3; iscolor:=FALSE; istcode:=TRUE; end;

    mc:=ss[p1+1]; sss:=copy(ss,1,p1-1);
    ss:=copy(ss,p1+2,length(ss)-(p1+1));

    if (outcom) then sends(sss);
    locs(sss);

    if (iscolor) then begin
      if ((mc>=#0) and (mc<=#9)) then cl(ord(mc)) else
        if ((mc>='0') and (mc<='9')) then cl(ord(mc)-48);
    end else
      if (istcode) then dotcode(mc,ss);

    p1:=pos(#3,ss); if (p1=0) then p1:=500;
    p2:=pos('õ',ss); if (p2=0) then p2:=500;
    p3:=pos(^T,ss); if (p3=0) then p3:=500;
    if (p1+p2+p3<1500) then goto goto1;
  end;

begin
  tcode:=''; ss:='';
  if (abort) then exit;
  doit:=TRUE; isansi:=FALSE;
  if (pos(^[,s)<>0) then begin
    lil:=0;
    isansi:=TRUE;
  end else
    if (s[1]='&') then begin
      if (thisuser.sl<value(copy(s,2,4))) then doit:=FALSE;
      s:=copy(s,5,length(s)-4);
    end;
  if ((hangup) or (not doit)) then begin abort:=TRUE; exit; end;
  ss:=s; sss:=''; i:=1;
  mcix:=''; mcixx:='';
  while (pos(#29,ss)<>0) do delete(ss,pos(#29,ss),1);
  if (not okansi) then ss:=stripcolor(ss);
  if (trapping) then write(trapfile,ss);

    p1:=pos(#3,ss); if (p1=0) then p1:=500;
    p2:=pos('õ',ss); if (p2=0) then p2:=500;
    p3:=pos(^T,ss); if (p3=0) then p3:=500;
    if (isansi) then begin
      p1:=500; p2:=500; p3:=500;
    end;

    if (((reading_a_msg) and (not read_with_mci)) and (p2+p3<>1000)) then
      begin p2:=500; p3:=500; end;

    if ((p2=500) and (p3=500)) then begin
      if (p1<>500) then handlecolors;
    end else
      if (p1=500) then handletcodes else handletcodesc;
    if (outcom) then sends(ss);
    locs(ss);
  wkey(abort,next);
end;}

procedure printacr(s:string; var abort,next:boolean);
var org:string;
    p,op,rp,rop,nca:integer;
    okdoit,sram,turnoff:boolean;

  procedure doboxedtitle(s:string);
  const B_UL='Ö'; B_UR='·'; B_LL='Ó'; B_LR='½';
        B_TOP='Ä'; B_BOT='Ä'; B_LFT='º'; B_RGT='º';
  var b:array[0..7] of char;
      x,numsp:integer;
      i:string;

    function ritr(c:char; l:integer):string;
    var s:string;
        i:integer;
    begin
      s:='';
      for i:=1 to l do s:=s+c;
      ritr:=s;
    end;

  begin
    i:=s;
    if (i[length(i)]=#1) then i:=copy(i,1,length(i)-1);
    if (okansi) then
      for x:=0 to 7 do
        case x of
          0:b[x]:=B_UL;   1:b[x]:=B_UR;   2:b[x]:=B_LL;  3:b[x]:=B_LR;
          4:b[x]:=B_TOP;  5:b[x]:=B_BOT;  6:b[x]:=B_LFT; 7:b[x]:=B_RGT;
        end
    else
      for x:=0 to 7 do
        case x of
          0:b[x]:='.';  1:b[x]:='.';  2:b[x]:='`';  3:b[x]:='''';
          4:b[x]:='-';  5:b[x]:='-';  6:b[x]:=':';  7:b[x]:=':';
        end;
    numsp:=(thisuser.linelen div 2)-((lenn(i)+4) div 2);
    printacr(#3#4+ritr(#32,numsp)+b[0]+ritr(b[4],lenn(i)+2)+b[1],abort,next);
    printacr(#3#4+ritr(#32,numsp)+b[6]+' '+#3#3+i+#3#4+' '+b[7],abort,next);
    printacr(#3#4+ritr(#32,numsp)+b[2]+ritr(b[5],lenn(i)+2)+b[3]+#3#1,abort,next);
  end;

begin
  if ((allowabort) and (abort)) then exit;

  if (s[length(s)]=#1) then
    if (copy(s,length(s)-1,1)<>#3) then s:=copy(s,1,length(s)-1);

  okdoit:=TRUE; abort:=FALSE; {nopfile:=FALSE;}
  turnoff:=(s[length(s)]=#29);

  if (copy(s,1,1)='&') then begin
    if (thisuser.sl<value(copy(s,2,4))) then exit;
    s:=copy(s,5,length(s)-4);
  end;
  checkhangup;


  if (pos(^[,s)>0) then begin
     sprompt(s);    {printa1(s,abort,next);}
    if ((not turnoff) and (not croff)) then begin
      nl;
      if (trapping) then writeln(trapfile);
    end;
    croff:=FALSE;
    exit;
  end else
  if (s[1]=#2) then begin
     sprompt(centre(s));    {printa1(centre(s),abort,next);}
    if (not turnoff) then nl;
    croff:=FALSE; exit;
  end else
  if (length(s)>=3) and (copy(s,1,3)='`#[') then begin
    doboxedtitle(copy(s,4,length(s)-3));
    croff:=FALSE; exit;
  end else begin
    sprompt(s);    {printa1(s,abort,next);}
    if (abort) then begin curco:=255-curco; cl(1); end;
    if ({(not nofeed) and} (doit) and (not croff) and (not turnoff)) then
      if (not abort) then nl;
    doit:=TRUE;
  end;
  croff:=FALSE;
end;

procedure pfl(fn:string; var abort,next:boolean; cr:boolean);
var fil:text;
    ofn:string;
    ls:string[255];
    ps:integer;
    c:char;
    oldpause,oaa:boolean;
begin
  printingfile:=TRUE;
  oaa:=allowabort;
  allowabort:=TRUE;
  abort:=FALSE; next:=FALSE;
  oldpause:=(pause in thisuser.ac);
  nofile:=FALSE;
  if (not hangup) then begin
{    assign(fil,sqoutsp(fn));}
    assign(fil,fn);
    {$I-} reset(fil); {$I+}
    if (ioresult<>0) then nofile:=TRUE
    else begin
      abort:=FALSE;
      while ((not eof(fil)) and (not nofile) and
             (not abort) and (not hangup)) do begin
        ps:=0;
        repeat
          inc(ps);
          read(fil,ls[ps]);
        until ((ls[ps]=^M) or (ps=255) or (eof(fil)) or (hangup));
        ls[0]:=chr(ps);
        if (ls[ps]=^M) then begin
          if (not eof(fil)) then read(fil,c);
          ls[0]:=chr(ps-1);
        end else
          croff:=TRUE;
        if (pos(^[,ls)<>0) then ctrljoff:=TRUE;
        printacr(ls,abort,next);
      end;
      close(fil);
    end;
  end;
  if (oldpause) then thisuser.ac:=thisuser.ac+[pause];
  allowabort:=oaa;
  printingfile:=FALSE; ctrljoff:=FALSE;
  curco:=255-curco; cl(1);
  redrawforansi;
end;

function exist(fn:string):boolean;
var srec:searchrec;
begin
   exist := true;
   findfirst(fn,anyfile,srec);
   if doserror<>0 then exist := false;
end;

procedure printfile(fn:string);
var abort,next:boolean;
begin
   fn:=strAllCaps(fn);
   pfl(fn,abort,next,TRUE);
end;

function aacs1(u:userrec; un:integer; s:string):boolean;
var s1,s2:string;
    p1,p2,i,j:integer;
    c,c1,c2:char;
    b:boolean;

  procedure getrest;
  begin
    s1:=c;
    p1:=i;
    if ((i<>1) and (s[i-1]='!')) then begin s1:='!'+s1; dec(p1); end;
    if (c in ['C','F','G','R','V','X']) then begin
      s1:=s1+s[i+1];
      inc(i);
    end else begin
      j:=i+1;
      repeat
	if (s[j] in ['0'..'9']) then begin
	  s1:=s1+s[j];
	  inc(j);
	end;
      until ((j>length(s)) or (not (s[j] in ['0'..'9'])));
      i:=j-1;
    end;
    p2:=i;
  end;

  function argstat(s:string):boolean;
  var vs:string;
      year,month,day,dayofweek,hour,minute,second,sec100:word;
      vsi:integer;
      boolstate,res:boolean;
  begin
    boolstate:=(s[1]<>'!');
    if (not boolstate) then s:=copy(s,2,length(s)-1);
    vs:=copy(s,2,length(s)-1); vsi:=value(vs);
    case s[1] of
      'A':res:=(ageuser(u.bday)>=vsi);
      'B':res:=((value(spd)>=value(vs+'00')) or (spd=strings^.locallogin));
      'C':res:=FALSE;   { conferences - not implemented yet }
      'D':res:=(u.dsl>=vsi);
      'F':res:=(upcase(vs[1]) in u.ar);
      'G':res:=(u.sex=upcase(vs[1]));
      'H':begin
          gettime(hour,minute,second,sec100);
          res:=(hour=vsi);
          end;
      'P':res:=(u.filepoints>=vsi);
      'R':res:=(tacch(upcase(vs[1])) in u.ac);
      'S':res:=(u.sl>=vsi);
      'T':res:=(trunc(nsl) div 60>=vsi);
      'U':res:=(un=vsi);
      'V':res:=((u.sl>status^.newsl) or (u.dsl>status^.newdsl) or
	   ((status^.newsl=status^.autosl) and (status^.newdsl=status^.autodsl)));
      'W':begin
	    getdate(year,month,day,dayofweek);
	    res:=(dayofweek=ord(s[1])-48);
	  end;
      'Y':res:=(trunc(timer) div 60>=vsi);
    end;
    if (not boolstate) then res:=not res;
    argstat:=res;
  end;

begin
  s:=strAllCaps(s);
  i:=0;
  while (i<length(s)) do begin
    inc(i);
    c:=s[i];
    if (c in ['A'..'Z']) and (i<>length(s)) then begin
      getrest;
      b:=argstat(s1);
      delete(s,p1,length(s1));
      if (b) then s2:='^' else s2:='%';
      insert(s2,s,p1);
      dec(i,length(s1)-1);
    end;
  end;
  s:='('+s+')';
  while (pos('&',s)<>0) do delete(s,pos('&',s),1);
  while (pos('^^',s)<>0) do delete(s,pos('^^',s),1);
  while (pos('(',s)<>0) do begin
    i:=1;
    while ((s[i]<>')') and (i<=length(s))) do begin
      if (s[i]='(') then p1:=i;
      inc(i);
    end;
    p2:=i;
    s1:=copy(s,p1+1,(p2-p1)-1);
    while (pos('|',s1)<>0) do begin
      i:=pos('|',s1);
      c1:=s1[i-1]; c2:=s1[i+1];
      s2:='%';
      if ((c1 in ['%','^']) and (c2 in ['%','^'])) then begin
	if ((c1='^') or (c2='^')) then s2:='^';
	delete(s1,i-1,3);
	insert(s2,s1,i-1);
      end else
	delete(s1,i,1);
    end;
    while(pos('%%',s1)<>0) do delete(s1,pos('%%',s1),1);   {leave only "%"}
    while(pos('^^',s1)<>0) do delete(s1,pos('^^',s1),1);   {leave only "^"}
    while(pos('%^',s1)<>0) do delete(s1,pos('%^',s1)+1,1); {leave only "%"}
    while(pos('^%',s1)<>0) do delete(s1,pos('^%',s1),1);   {leave only "%"}
    delete(s,p1,(p2-p1)+1);
    insert(s1,s,p1);
  end;
  aacs1:=(not (pos('%',s)<>0));
end;

function aacs(s:string):boolean;
begin
  aacs:=aacs1(thisuser,usernum,s);
end;

procedure loadurec(var u:userrec; i:integer);
var ufo:boolean;
begin
   ufo:=(filerec(uf).mode<>fmclosed);
   if (not ufo) then reset(uf);
   if (i<>usernum) then begin
      seek(uf,i);
      read(uf,u);
   end
   else u:=thisuser;
   if (not ufo) then close(uf);
end;

procedure saveurec(u:userrec; i:integer);
var ufo:boolean;
begin
   ufo:=(filerec(uf).mode<>fmclosed);
   if (not ufo) then reset(uf);
   seek(uf,i);
   write(uf,u);
   if (i=usernum) then thisuser:=u;
   if (not ufo) then close(uf);
end;

end.
