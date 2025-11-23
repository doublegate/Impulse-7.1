# Pascal Global State Analysis

**Files with global constants:** 33
**Files with global variables:** 90

## Overview

Pascal units can export global constants and variables through their INTERFACE section.
These represent shared state that needs careful handling during Rust conversion.

## Global Constants

Global constants are safe and typically converted to Rust `const` or `static` values.

### Files with Global Constants

#### ANSIDRV.PAS
```pascal
ControlCh: Set of Char = ['A','B','C','D','f','s','u','H','J','K','m',';','r'];
MaxParms = 200;
CONST OrTable : ARRAY[30..47] OF BYTE =
```

#### ANSIEDIT.PAS
```pascal
maxlines = 101;
```

#### ASMSAUCE.PAS
```pascal
SAUCE_ID      : Char5 = 'SAUCE';
SAUCE_Version : Char2 = '00';
CMT_ID        : Char5 = 'COMNT';
MaxCMT        = 10;
```

#### BPTRAP.PAS
```pascal
addrsave:Pointer=NIL;
codesave:Word=0;
```

#### COMMON.PAS
```pascal
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
```

#### COMMON1.PAS
```pascal
lastc:byte=0;
```

#### COMMON2.PAS
```pascal
menuchoice : Array [1..6] of common4.menurec =
```

#### COMMON4.PAS
```pascal
Esc         = Chr(27);
Ansi_Letter = 'JDCABHmfsuK'+Chr(13);
```

#### EXEC.PAS
```pascal
RC_PREPERR   = $0100;
RC_NOFILE    = $0200;
RC_EXECERR   = $0300;
RC_ENVERR    = $0400;
RC_SWAPERR   = $0500;
RC_REDIRERR  = $0600;
USE_EMS      =  $01;
USE_XMS      =  $02;
USE_FILE     =  $04;
EMS_FIRST    =  $00;
```

#### EXECOLD.PAS
```pascal
RC_PREPERR   = $0100;
RC_NOFILE    = $0200;
RC_EXECERR   = $0300;
RC_ENVERR    = $0400;
RC_SWAPERR   = $0500;
RC_REDIRERR  = $0600;
USE_EMS      =  $01;
USE_XMS      =  $02;
USE_FILE     =  $04;
EMS_FIRST    =  $00;
```

#### FILE0.PAS
```pascal
ulffopen1:boolean=TRUE;   { whether ulff has been opened before }
```

#### FILE13.PAS
```pascal
maxsortrec=2000;   (* maximum size of directory which can be processed *)
```

#### FILE6.PAS
```pascal
lastpos:integer=-1;
```

#### INITP.PAS
```pascal
configonly:boolean=FALSE;
```

#### INSTALL.PAS
```pascal
SCREEN_SIZE=1440;
INSTALL_VER='1.0';
```

#### MAIL0.PAS
```pascal
_brd_opened:boolean=FALSE;  { has brdf been opened yet? }
oldnummsgs:integer=0;       { old number of messages }
gotlastmheader:boolean=FALSE;
```

#### MAIL4.PAS
```pascal
hellfreezesover=FALSE;
```

#### MENUS.PAS
```pascal
numenters : byte = 0;
```

#### MENUS2.PAS
```pascal
spacestr='                                               ';
```

#### MYIO.PAS
```pascal
infield_seperators:set of char=[' ','\','.'];
vidseg:word=$B800;
ismono:boolean=FALSE;
```

## Global Variables

Global mutable variables are problematic for Rust due to:
- Thread safety concerns
- Ownership model
- Lack of compile-time guarantees

### Files with Global Variables

#### ANSIDRV.PAS
```pascal
AvState : Word;
AvAttr : Byte;
CheckPositions : Boolean;
AnsiParm : Array [1..MaxParms] of Byte;
AnsiParmNo : Byte;
SaveX : Byte;
SaveY : Byte;
XC, YC : ShortInt;
```

#### ANSIEDIT.PAS
```pascal
rightmargin,savedx,savedy,topscrn:integer;
insertmode,msgdone,ansimode:boolean;
badnum:boolean;
hdrlen:integer;
```

#### ASMSAUCE.PAS
```pascal
SAUCE    : SAUCERec;
CMT      : CMTBlock;
```

#### BPTRAP.PAS
```pascal
exitsave,trapaddr:Pointer;
trapsp,trapbp:Word;
```

#### CMD.PAS
```pascal
ft:file of oneLineRec;
kn:byte;
r:oneLineRec;
sp:array[1..15] of oneLineRec;
for i:= 1 to 15 do sp[i] := '';
i := 0;
if (i>0) then for kn:=1 to i do sprint(sp[kn])
sysoplog('ONELINER: '+thisuser.name+' entered: "'+r+'"');
for kn := 1 to i do write(ft,sp[kn]);
for kn := 2 to 15 do write(ft,sp[kn]);
```

#### COMMON.PAS
```pascal
uf:file of userrec;           { USER.LST                              }
bf:file of boardrec;          { BOARDS.DAT                            }
xf:file of protrec;           { PROTOCOL.DAT                          }
ulf:file of ulrec;            { UPLOADS.DAT                           }
ulff:file of ulfrec;          { *.DIR                                 }
sf:file of smalrec;           { NAMES.LST                             }
verbf:file of verbrec;        { VERBOSE.DAT                           }
multinodef:file of noderec;
clasicnode:astr;
mixf:file;                    { *.MIX                                 }
```

#### COMMON1.PAS
```pascal
s:string[20];
checkpw:=TRUE;
echo:=FALSE;
echo:=TRUE;
checkpw:=FALSE;
```

#### COMMON2.PAS
```pascal
f2key:byte;
```

#### COMMON3.PAS
```pascal
s:string[5];
badini:=FALSE;
i:=value(s);
if (s='') then badini:=TRUE;
```

#### COMMON4.PAS
```pascal
fb,b,b2,i:integer;
acc:boolean;
b:=1;
acc:=false;
fb:=1;
acc:=fbaseac(b); { fbaseac will load memuboard }
b2:=ccuboards[1][b];
areanum^[fb]:=b2;
```

#### COMMON5.PAS
```pascal
i : byte;
str : string;
str := '';
for i := 1 to len do str := str + s;
expand := str;
```

#### CONF.PAS
```pascal
ConfDat:file of confrec;
Confr:confrec;
```

#### CUSER.PAS
```pascal
done,done1:boolean;
tryc:integer;
s:astr;
i:integer;
cnt : byte;
```

#### DOORS.PAS
```pascal
i:astr;
i:=cstrr(nsl/60,10);
timestr:=i;
```

#### EXEC.PAS
```pascal
progpars: string): integer;
cmdbat      1: Normal EXE/COM file
2: Executing BAT file via COMMAND.COM
3: Executing COMMAND.COM (or equivalent)
swapping    < 0: Exec, don't swap
0: Spawn, don't swap
> 0: Spawn, swap
CAUTION: If swapping is > 0, the routine may not modify the
spawn_check_proc = function (cmdbat: integer; swapping: integer;
var execfn: string; var progpars: string)
```

#### EXECBAT.PAS
```pascal
wind:windowrec;
sx,sy:integer;
wascls,savtw:boolean;
savcurwind:integer;
```

#### EXECOLD.PAS
```pascal
progpars: string): integer;
cmdbat      1: Normal EXE/COM file
2: Executing BAT file via COMMAND.COM
3: Executing COMMAND.COM (or equivalent)
swapping    < 0: Exec, don't swap
0: Spawn, don't swap
> 0: Spawn, swap
CAUTION: If swapping is > 0, the routine may not modify the
spawn_check_proc = function (cmdbat: integer; swapping: integer;
var execfn: string; var progpars: string)
```

#### FILE0.PAS
```pascal
dirinfo:searchrec;
found:boolean;
```

#### FILE1.PAS
```pascal
locbatup:boolean;
dizthere:boolean;
```

#### FILE10.PAS
```pascal
ff:file;
f:ulfrec;
v:verbrec;
s,s1,s2,fl,fn:astr;
x,i:longint;
pl,rn,dbn,oldfileboard:integer;
c:char;
espace,nospace,done,abort,next,ok:boolean;
gfn(fn); abort:=FALSE; next:=FALSE;
lastcommandovr:=TRUE;
```

## Rust Migration Strategy

### Constants
```rust
// Pascal: const MaxUsers = 1000;
// Rust:
const MAX_USERS: usize = 1000;
```

### Global Variables

**Option 1: Encapsulate in Struct**
```rust
struct BbsState {
    current_user: Option<User>,
    // other state
}

impl BbsState {
    fn new() -> Self { ... }
}
```

**Option 2: Thread-Local Storage**
```rust
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<BbsState> = RefCell::new(BbsState::new());
}
```

**Option 3: Static with Mutex (thread-safe)**
```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BBS_STATE: Mutex<BbsState> = Mutex::new(BbsState::new());
}
```

**Recommendation:** Encapsulate global state in a BbsState struct passed as parameter
or stored in async context (Tokio task-local storage).