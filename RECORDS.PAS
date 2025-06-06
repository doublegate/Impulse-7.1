Unit Records;

Interface

const
   progname      :string[7]='Impulse';
   acronym       :string[3]='imp';
   ver           :string[7]='7.1';
   copyright     :string[37]='(c)1993, 98 by Brandon Sneed (Nivenh)';
   verdate       :string[8]='7/27/98';
   maxboards     =254;   { 1 - x }
   maxconfs      =20;    { You cannot change this! }
   maxuboards    =254;   { 1 - x }
   maxprotocols  =20;    { 0 - x }
   maxevents     =10;    { 0 - x }   { #0 is Nightly Events (if active) }
   maxubatchfiles=20;
   maxmenucmds   =50;
   maxBufSize    = 4096;
   maxStrings    = 255;

type
  astr    =string[160];
  str80   =string[80];
  acstring=string[20];            { Access Condition String }
  acrq    ='@'..'Z';              { AR flags }

  uflags =
   (rlogon,                       { L - Restricted to one call a day }
    rchat,                        { C - Can't page the SysOp }
    rvalidate,                    { V - Posts marked unvalidated }
    rbackspace,                   { B - Can't do ^B/^N/etc in messages }
    ramsg,                        { A - Can't change the AutoMessage }
    rpostan,                      { * - Can't post anonymously }
    rpost,                        { P - Can't post at all }
    remail,                       { E - Can't send any e-mail }
    rvoting,                      { K - Can't vote }
    rmsg,                         { M - Force e-mail deletion }
    spcsr,                        { }
    onekey,                       { onekey input mode }
    avatar,                       { user has AVATAR }
    pause,                        { pause }
    novice,                       { user is at novice help level }
    ansi,                         { user has ANSI }
    color,                        { user has color }
    alert,                        { alert SysOp when user logs on }
    smw,                          { short-message waiting for user }
    nomail,                       { user mail-box is closed }
    fnodlratio,                   { 1 - No UL/DL ratio }
    fnopostratio,                 { 2 - No post/call ratio }
    fnofilepts,                   { 3 - No file points checking }
    fnodeletion);                 { 4 - Protection from deletion }

  flistflags =
   (fname,                        { filename }
    fsize,                        { file size }
    fpts,                         { file points }
    fdesc,                        { file desc. }
    extdesc,                      { extended desc. }
    whoul,                        { who uploaded it }
    dateul,                       { date uploaded }
    numdl);                       { num of d/l's. }


  anontyp =
   (atno,                         { No anonymous posts allowed }
    atyes,                        { Anonymous posts are allowed }
    atforced,                     { ALL posts are forced anonymous }
    atdearabby,                   { "Dear Abby" message base }
    atanyname);                   { User's can post as ANY name they want }

  clrs    = array[FALSE..TRUE,0..9] of byte; { color records }
  secrange= array[0..255] of integer;        { security tables }

  cpackdatetime=array[1..6] of byte;

  mzscanr = set of 1..maxboards;
  fzscanr = set of 0..maxuboards;
  mhireadr= array[1..maxboards] of cpackdatetime;

  dlnscan =set of 0..96;

  ConfRec=                       {CONF.DAT}
  Record
    ACSlevel : Array [1..maxconfs] of String[5];  { SL level required. }
    ConfName : Array [1..maxconfs] of String[20]; { Name of Conference }
  End;

  nuvrec=
  record
    unum      :integer;             { User number being voted on }
    handle    :string[80];          { Handle of user being voted on }
    no,                             { No votes }
    yes,                            { Yes votes }
    totvoted  :integer;             { # of users voted on him already }
    votedby   :array[1..50] of integer;{ #'s of users that have voted on him, so they can't vote more than once }
    exists    :boolean;             { Still exists to be voted upon? }
    commentptr:longint;             { Pointer to comments in comment file }
  end;

  nuvcomrec=
  record
    user        :string[40];        { User that left comment }
    comm        :string[80];        { Actual comment }
    nextcomment :longint;           { Pointer to next comment }
    exists      :boolean;           { Does comment actually exist }
  end;

  noderec=
  record
    desc    :string[40];            { Short node description (Subscriber line, 2400 line, HST line, etc) }
    acs     :acstring;              { ACS required to log in to that node }
    status  :byte;                  { What the node is doing at the moment }
    nodespd :string[6];             { What speed is the user logged in at }
    username:string[40];            { User that is on right now }
    usernum :integer;               { User # that is on right now }
  end;

  nodemessrec=
  record
    mess    :string[80];            { Message that is going to that node }
    deleted :boolean;               { It is deleted? }
    nodeto  :byte;                  { Which node is it going to? }
    nodefrom:byte;                  { Which node is it from? }
    namefrom:string[40];            { Username that it is from }
    numfrom :integer;               { User number that it is from }
  end;

  forumrec=
  record
    user   :string[80];             { Name of user in that slot }
    deleted:boolean;                { Is the slot record deleted? }
    node   :byte;                   { Node that they are on }
  end;


  smalrec=                          { NAMES.LST : Sorted names listing }
  record
    name   :string[36];             { user name }
    deleted:boolean;                { user is deleted }
    number :integer;                { user number }
  end;

  userrec=                            { USER.LST : User account records }
  record
    name         :string[36];              { user name        }
    realname     :string[36];              { real name        }
    pw           :string[20];              { user password    }
    ph           :string[12];              { user phone #     }
    bday         :string[8];               { user birthdate   }
    firston      :string[8];               { firston date     }
    x1xs         :array[1..2] of byte;
    laston       :string[8];               { laston date      }
    x2xs         :array[1..2] of byte;
    street       :string[30];              { mailing address  }
    citystate    :string[30];              { city, state      }
    zipcode      :string[10];              { zipcode          }
    unused       :array [1..31] of byte;
    autosig      :string[40];              { autosig          }
    unused2      :array [1..41] of byte;
    note         :string[39];              { SysOp note       }
    prompt       :byte;                    { Chosen Menu Prompt }
    lockedout    :boolean;                 { if locked out    }
    deleted      :boolean;                 { if deleted       }
    lockedfile   :string[8];               { lockout msg to print }
    novotes,
    yesvotes     :byte;                    { # of yes/no votes in NUV }
    ac           :set of uflags;           { user flags   }
    fflag        :set of flistflags;       { users configable file list }
    ar           :set of acrq;             { AR flags     }

    zzqscan      :array[1..64] of word;    { last read msg pointers }
    xqxxx        :array[1..64] of word;
    zzqscn       :array[1..64] of boolean; { scan boards flags  }
    zzdlnscn     :dlnscan;                 { scan uboards flags }

    unused3      :array[1..20] of byte;

    sex          :char;                    { user sex }

    ttimeon      :longint;                 { total mins spent on  }
    x1xx         :integer;
    uk           :longint;                 { UL k                 }
    x2xx         :integer;
    dk           :longint;                 { DL k                 }
    x3xx         :integer;

    uploads,
    downloads,                             { # of ULs / # of DLs  }
    loggedon,                              { # times logged on    }
    tltoday,                               { # min left today     }
    msgpost,                               { # public posts       }
    emailsent,                             { # email sent         }
    feedback,                              { # feedback sent      }
    forusr,                                { forward mail to user # }
    filepoints   :integer;                 { # of file points     }

    waiting,                               { mail waiting         }
    linelen,                               { line length (# cols) }
    pagelen,                               { page length (# rows) }
    ontoday,                               { # times on today     }
    illegal,                               { # illegal logon attempts }
    sl,dsl       :byte;                    { SL / DSL }

    cols         :clrs;                    { user colors }

    lastmsg,
    lastfil      :byte;                    { last msg/file areas   }
    credit       :longint;                 { $$$ credit in dollars }
    x4xx         :integer;
    timebank     :integer;                 { # mins in Time Bank   }
    boardsysop   :array[1..5] of byte;     { msg board SysOp       }

    trapactivity,                          { if trapping users activity }
    trapseperate :boolean;                 { if trap to seperate TRAP file }

    timebankadd  :integer;                 { time added to timebank TODAY }
    mpointer     :longint;                 { *REMOVED* }
    chatauto,                              { if auto chat trapping }
    chatseperate :boolean;                 { if seperate chat file to trap to }
    userstartmenu:string[8];               { menu to start user out on }
    slogseperate :boolean;                 { seperate SysOp log? }
    clsmsg       :byte;                    { 1 if clear-screen msg, 2 if not }
    flistopt     :byte;                    { type of file list type to use }
    msgorder     :byte;                    { 0:Chrono, 1:ReplyTree }
    avadjust     :byte;                    { AVATAR color adjust: 1=no, 2=yes }
  end;

  zscanrec=                       { ZSCAN.DAT : NewScan recs (file/msg) }
  record                          { ** b0..b3 }
    mhiread:mhireadr;             { NewScan high message pointers }
    mzscan:mzscanr;               { NewScan message bases }
    fzscan:fzscanr;               { NewScan file bases }
  end;

  msgindexstatr=
   (miexist,                      { does message actually exist? }
    miencrypted,                  { is it encrypted? }
    miunvalidated,                { is message unvalidated? }
    mipermanent,                  { is the message permanent? }
    miallowmci,                   { DID owner have access to MCI? }
    mithreads,                    { is message referenced? (threaded) }
    mimassmail,                   { is it private, mass mail? }
    miscanned);                   { is message scanned for FidoNet? }

  msgindexrec=                         { *.MIX : Message index records }
  record
    messagenum:word;                   { message number, tonum in EMAIL.MIX }
    hdrptr:longint;                    { pointer to message header }
    msgid:longint;                     { message ID (sequential) }
    isreplytoid:longint;               { ID of replied message }
    msgdate:cpackdatetime;              { message date/time (packed) }
    msgdowk:byte;                      { message day-of-week (0=Sun ...) }
    msgindexstat:set of msgindexstatr; { status flags }
    isreplyto:word;                    { reply this message is to (-1=None) }
    numreplys:word;                    { number of replies to THIS message }
  end;

  fromtoinfo=                  { from/to information for mheaderrec }
  record
    anon:byte;                 { anonymous type }
    usernum:word;              { user number }
    asn:string[42];            { given name for this case }
    real:string[36];           { user real name }
    alias:string[36];          { user alias }
  end;

  mheaderrec=                  { *.BRD : Message header records }
  record
    signature:longint;         { header signature - $FFFFFFFF }
    msgptr:longint;            { pointer to message text }
    msglength:longint;         { length of message text }
    fromi:fromtoinfo;          { from information }
    toi:fromtoinfo;            { to information }
    title:string[60];          { title of message }
    origindate:string[19];     { Echo/Group original msg date }
  end;

  zlogrec=                        { ZLOG.DAT : System log }
  record
    date:string[8];
    userbaud:array[0..4] of integer;
    active,calls,newusers,pubpost,privpost,fback,criterr:integer;
    uploads,downloads:integer;
    uk,dk:longint;
  end;

  modemrec=
  record
    waitbaud:longint;            { wait baud }
    comport:byte;                { comport number }
    (*
    inoutbuff:byte;              { modem in/out buffer size }
    irqnum:byte;                 { irq number 0-7 (255 for none) }
    *)
    init:string[80];             { initialization string }
    answer:string[40];           { answer string }
    hangup:string[40];           { hangup string }
    offhook:string[40];          { phone off-hook string }
    nocallinittime:byte;         { reinit modem after x mins of inactivity }
    (*
    arq9600rate:longint;         { baud rate to USE when 9600 ARQ result code }
    noforcerate:boolean;         { whether to force baud rate }
    *)
    hardware:boolean;            { use hardware handshaking }
  end;

  fstringrec=
  record
    ansiq,                            { system pw string }
    enterhandle,                      { Handle }
    enterpw,                          { Password }
    lprompt:string[80];               { File Listings prompt }
    echoc:char;                       { Echo char for PWs }
    engage,                           { "Engage Chat" }
    endchat,                          { "End Chat" }
    wait,                             { "SysOp Working" }
    pause,                            { "Pause" }
    entermsg1,                        { Join Conference }
    entermsg2,                        { Prompt Choice }
    newscan1,                         { "NewScan begin" }
    newscan2,                         { "NewScan done" }
    scanmessage,                      { Message scanning prompt }
    automsgt:string[80];              { Auto-Message title }
    autom:char;                       { Auto-Message border characters }

    shelldos1,                        { " >> SysOp shelling to DOS ..." }
    shelldos2,                        { " ... thank you for waiting." }
    chatcall1,                        { Chat call activated. [C] to turn off }
    chatcall2,                        { chat call deactivated }
    guestline,                        { "To be a guest ... " }
    namenotfound,                     { "Name NOT found in user list." }
    bulletinline,                     { Bulletin line }
    thanxvote,                        { "Thanks for voting" }

    listline,                         { "List files - P to pause" }
    newline,                          { "Search for new files -" }
    searchline,                       { "Search all dirs for file mask" }
    findline1,                        { "Search for descriptions... " }
    findline2,                        { "Enter the string to search for.." }
    downloadline,                     { "Download - " }
    uploadline,                       { "Upload - " }
    viewline,                         { "View interior files - " }
    nofilepts,                        { "Insufficient file points." }
    unbalance,                        { "Your UL/DL ratio is unbalanced" }

    pninfo,                           { "P to Pause, N for next dir" }
    gfnline1,                         { "[Enter]=All files" }
    gfnline2,                         { "File mask: " }
    batchadd:string[80];              { (borrowed) area changing }
    newuserp,                         { new user password                  }
    logonnew,                         { logon as new?                      }
    quotebegin,                       { begin quote                        }
    quoteend,                         { end quote                          }
    updatesys,                        { updating system status             }
    enteroneline,                     { enter a oneliner                   }
    protselect,                       { protocol selection                 }
    multitask,                        { (borrowed) Search [W/S]            }
    phonenum,                         { enter your phone #                 }
    bbsadd,                           { bbslist: bbs comment               }
    bbsphone,                         { bbslist: add bbs phone             }
    bbsname,                          { bbslist: add bbs name              }
    bbsspeed,                         { bbslist: max speed                 }
    bbstype,                          { bbslist: bbs type                  }
    leavefeedback,                    { leave feedback?                    }
    sysoppass,                        { sysop password                     }
    nuzipcode,                        { *borrowed* bbslist sysop           }
    nuaddress,                        { newuser address                    }
    nubday,                           { newuser birthday                   }
    nucity,                           { newuser where are you from?        }
    nuphone,                          { newuser phone                      }
    numisc1,                          {*borrowed* (A)dd to batch,(Q)uit.   }
    nuscrwide,                        { newuser screen width               }
    nuscrlines,                       { newuser screen lines               }
    nureal,                           { newuser real name                  }
    nuhandle,                         { newuser handle                     }
    nustate,                          { *borrowed* newuser info change     }
    nusex,                            { newuser sex                        }
    numisc2,                          { newuser misc2                      }
    numisc3,                          { newuser misc3                      }
    nuwords,                          { newuser words blinking?            }
    nuansi,                           { newuser use ansi?                  }
    nupause,                          { newuser use pause?                 }
    nuenterpw:string[80];             { newuser enter password             }
    postanon,                         { Post anonymously? }
    quote1,                           { starting at: }
    quote2,                           { Ending At: }
    enterunote,                       { enter user note }
    newusernote,                      { default user note }
    msgtitle,                         { title: }
    msgsendto,                        { send to: }
    uploadmsg,                        { upload msg? }
    fileaddedtobq,                    { file added to batch queue }
    fnewscan,                         { file newscan start }
    enterautosig,                     { prompt for autosig }
    enterfilenametoul,                { enter filename to ul/dl }
    deletereadmail,                   { delete read mail? }
    nomsgs,                           { no msgs on General }
    globalnewscan,                    { globalnewscan? }
    nomailwaiting,                    { no mail waiting }
    numbermsgs,                       { 102 msgs on General }
    startmsgread,                     { start msg reading }
    fseprompt,                        { FSE prompt }
    aborted,                          { Aborted... }
    msgsaved,                         { Msg posted on General }
    logoffafter,                      { logoff after transfer? }
    posthere,                         { Post on General? }
    turbologin,                       { turbo login? }
    readmailq,                        { read your email? }
    enteronl,                         { enter oneliner }
    fbasejoined,                      { filebase joined }
    mbasejoined,                      { msgbase joined }
    addrumor,                         { add a rumor }
    nuvote,                           { yna prompt for NUV }
    nuvvalidated,                     { Nuv User Validated }
    nuvdeleted,                       { Nuv User Deleted }
    fileprivate,                      { make uploaded file private? }
    ReEnterPW,                        { reenter pw for verification }
    rumorentered : string[80];        { rumor entered }
    yesbar,                           { YES/no }
    nobar  :String[39];               { yes/NO }
    localLogin,                       { local login string }
    hack      ,                       { attempted hack }
    areyousure,                       { are you sure? }
    wrongPassword,                    { entered the wrong password }
    currentPassword,                  { current password }
    wrongPhone,                       { wrong phone number }
    timeBankDeposit,                  { add time }
    timeBankWithDraw,                 { withdraw time }
    unKnownHandle : string[80];
  end;

{  typeStringRec = array[1..maxStrings] of string[80];}


  systatrec=
  record
    gfilepath:string[79];             { GFILES path }
    afilepath:string[79];             { AFILES path (text files path) }
    menupath:string[79];              { MENUS path  }
    trappath:string[79];              { LOG path (traps, chats, SysOp logs) }
    msgpath:string[79];               { MSG path (priv/pub mail path) }
    tfilepath:string[79];             { ISL script path }
    temppath:string[79];              { TEMP path - "temp" directory }
    usenuv:boolean;                   { Is new user voting active? }
    nuvyes,                           { Yes votes required for validation in NUV }
    nuvno:byte;                       { No votes required for validation in NUV }
    bbsname:string[80];               { BBS name }
    bbsphone:string[12];              { BBS phone number }
    sysopname:string[30];             { SysOp's full name or alias }
    regnum:string[10];                { BBS Registration Number }
    startdir:string[80];              { batch file to run instead of NScan }
    maxTimeBank,                      { max number of users system can have }
    lowtime,                          { SysOp begin minute (in minutes) }
    hitime,                           { SysOp end time }
    dllowtime,                        { normal downloading hours begin.. }
    dlhitime:integer;                 { ..and end }
    msgedit:boolean;                  { which editor to use, FSE/Line }
    shuttlelog,                       { is Shuttle Logon active? }
    lock300:boolean;                  { lock out 300 baud? }
    sysoppw,                          { SysOp PW }
    newuserpw,                        { newuser PW (or NULL if none) }
    shuttlepw:string[20];             { Shuttle PW (if Shuttle active) }
    b300lowtime,                      { 300 baud calling hours begin.. }
    b300hitime,                       { ..and end }
    b300dllowtime,                    { 300 baud downloading hours begin.. }
    b300dlhitime:integer;             { ..and end }
    closedsystem:boolean;             { DON'T allow new users? }
    swapshell:boolean;                { is swap shell function enabled? }
    eventwarningtime:integer;         { time before event warning }
    tfiledate:string[8];              { last date text-files were inserted }
    lastmsgid:longint;                { last-used message ID (sequential) }

    sop,                              { SysOp }
    csop,                             { Co-SysOp }
    msop,                             { Message SysOp }
    fsop,                             { File SysOp }
    spw,                              { SysOp PW at logon }
    seepw,                            { see SysOp PWs remotely  }
    normpubpost,                      { make normal public posts }
    normprivpost,                     { send normal e-mail }
    anonpubread,                      { see who posted public anon }
    anonprivread,                     { see who sent anon e-mail }
    anonpubpost,                      { make anon posts }
    anonprivpost,                     { send anon e-mail }
    seeunval,                         { see unvalidated files }
    dlunval,                          { DL unvalidated files }
    nodlratio,                        { no UL/DL ratio }
    nopostratio,                      { no post/call ratio }
    nofilepts,                        { no file points checking }
    ulvalreq:acstring;                { uploads require validation by SysOp }

    maxprivpost,                      { max e-mail can send per call }
    maxfback,                         { max feedback per call }
    maxpubpost,                       { max posts per call }
    maxchat,                          { max chat-pages per call }
    maxwaiting,                       { max mail in mail-box }
    csmaxwaiting,                     { max mail in mail-box for Co-SysOp + }
    maxlogontries,                    { tries allowed for PW's at logon }
    notUsed,
    maxlines,                         { max lines in message }
    csmaxlines,                       { max lines in message for Co-SysOp + }
    sysopcolor,                       { SysOp color in chat mode }
    usercolor : byte;                 { user color in chat mode }
    minspaceforpost,                  { minimum K drive space left to post }
    minspaceforupload:integer;        { minimum K drive space left to upload }
    backsysoplogs,                    { days to keep SYSOP##.LOG }
    wfcblanktime:byte;                { minutes after which to blank WFC menu }
    linelen,                          { default video line length }
    pagelen:byte;                     { default video page length }

    specialfx,                        { Use iCEcolor? }
    fossil,                           { make use of FOSSIL comm driver }
    allowalias,                       { allow alias's? (handles) }
    phonepw,                          { use phone number password in logon? }
    localsec,                         { is local security ON? }
    localscreensec,                   { is local screen-security ON? }
    globaltrap,                       { trap ALL USER'S activity? }
    autochatopen,                     { does chat buffer auto-open? }
    autominlogon,                     { Auto-Message in logon? }
    bullinlogon,                      { bulletins in logon? }
    lcallinlogon,                     { Last Callers list in logon? }
    yourinfoinlogon,                  { User Stats in logon? }
    multitask,                        { is BBS multitasking? }
    offhooklocallogon,                { take phone off-hook for local logons? }
    forcevoting,                      { is manditory logon voting active? }
    compressbases,                    { "compress" file/message base numbers? }
    searchdup:boolean;                { search for dup. filenames when UL? }
    slogtype:byte;                    { SysOp log type: File/Printer/Both }
    stripclog:boolean;                { strip colors from SysOp log output? }
    newapp,                           { user# to send new user application to }
    guestuser,                        { *borrowed* f2 status bar var }
    timeoutbell,                      { minutes before time-out bell }
    timeout:byte;                     { minutes before timeout (logoff) }
    usewfclogo:boolean;               { whether to use WFC menu logo }
    useems:boolean;                   { attempt to use EMS for overlay }
    usebios:boolean;                  { use ROM BIOS for local video output }
    useScrollBack:boolean;                  { suppress snow on CGA systems }

    filearccomment:array[1..3] of string[80]; { BBS comments for archives }
    ulcheckit,                        { Use Internal Upload Checker? }
    uldlratio,                        { are UL/DL ratios active? }
    fileptratio:boolean;              { is auto file-pt compensation active? }
    fileptcomp,                       { file point compensation ratio }
    fileptcompbasesize,               { file point "base compensation size" }
    ulrefund,                         { percent time refund for ULs }
    tosysopdir:byte;                  { "To SysOp" file base }
    validateallfiles:boolean;         { validate ALL FILES automatically? }
    remdevice:string[10];             { remote output device (GATEx,COMx,etc) }
    maxintemp,                        { max K allowed in TEMP\3\ }
    minresume:integer;                { min K to allow resume-later }
    maxdbatch,                        { max files in DL batch queue }
    maxubatch:byte;                   { max files in UL batch queue }

    newsl,                            { newuser SL }
    newdsl:byte;                      { newuser DSL }
    newar:set of acrq;                { newuser AR }
    newac:set of uflags;              { newuser AC }
    newfp:integer;                    { newuser file points }
    autosl,                           { auto-validation SL }
    autodsl:byte;                     { auto-validation DSL }
    autoar:set of acrq;               { auto-validation AR }
    autoac:set of uflags;             { auto-validation AC }

    allstartmenu:string[8];           { logon menu to start ALL users on }
    bulletprefix:string[8];           { default bulletins filename prefix }

    timeallow,                        { time allowance }
    callallow,                        { call allowance }
    dlratio,                          { # ULs/# DLs ratios }
    dlkratio,                         { DLk/ULk ratios }
    postratio:secrange;               { post/call ratios }

    lastdate:string[8];               { last system date }
    curwindow:byte;                   { type of SysOp window currently in use }
    istopwindow:boolean;              { is SysOp window on top of screen? }
    callernum:longint;                { total number of callers }
    numusers:integer;                 { number of users }

    todayzlog:zlogrec;                { TODAY's ZLOG record }

    postcredits:integer; {file points/upload credit compensation for posts}
    displayansimsg:boolean;           { display "ansi missing" messages? }
    scrpause:boolean;                 { disable/enable screen pauses }

    windowon:boolean;

    swappath:string[79];              { swap shell path }
  end;

  mbflags=
   (mbunhidden,                   { whether *VISIBLE* to users w/o access }
    mbrealname,                   { whether real names are forced }
    mbisdir,                      { if a message base directory }
    mbmsgpath,                    { if *.BRD file stored in MSGPATH }
    mbfilter,                     { whether to filter ANSI/8-bit ASCII }
    mbskludge,                    { strip IFNA kludge lines }
    mbsseenby,                    { strip SEEN-BY lines }
    mbsorigin,                    { strip origin lines }
    mbscenter,                    { strip centering codes }
    mbsbox,                       { strip box codes }
    mbmcenter,                    { center boxed/centered lines }
    mbaddtear,                    { add tear/origin lines }
    mbtopstar);                   { whether XBase for XMail base }

  boardrec=                       { BOARDS.DAT : Message base records }
  record
    name:string[40];              { message base description }
    filename:string[8];           { BRD/MIX data filename }
    lastmsgid:longint;            { last message ID number }
    msgpath:string[40];           { messages pathname   }
    acs,                          { access requirement }
    postacs,                      { post access requirement }
    mciacs:acstring;              { MCI usage requirement }
    maxmsgs:word;                 { max message count }
    anonymous:anontyp;            { anonymous type }
    password:string[20];          { base password }
    mbstat:set of mbflags;        { message base status vars }
    permindx:longint;             { permanent index # }
    mbtype:integer;               { base type (0=Local,1=Echo,2=XMail) }
    origin:string[50];            { origin line }
    text_color,                   { color of standard text }
    quote_color,                  { color of quoted text }
    tear_color,                   { color of tear line }
    origin_color:byte;            { color of origin line }
    zone,                         { alternate address     }
    net,                          { (zone:net/node.point) }
    node,
    point:integer;
    conf:byte;
    res:array[1..2] of byte;      { RESERVED }
  end;

  fbflags=
   (fbnoratio,                    { if <No Ratio> active }
    fbunhidden,                   { whether *VISIBLE* to users w/o access }
    fbdirdlpath,                  { if *.DIR file stored in DLPATH }
    fbisdir,                      { if a file base directory }
    fbusegifspecs,                { whether to use GifSpecs }
    fbnetlink);                   { whether Net-Linked to other Impulse BBS's }

  ulrec=                          { UPLOADS.DAT : File base records }
  record
    name:string[40];              { area description  }
    filename:string[12];          { filename + ".DIR" }
    dlpath,                       { download path     }
    ulpath:string[40];            { upload path       }
    maxfiles:integer;             { max files allowed }
    password:string[20];          { password required }
    arctype,                      { wanted archive type (1..max,0=inactive) }
    cmttype:byte;                 { wanted comment type (1..3,0=inactive) }
    fbdepth:integer;              { file base dir depth }
    fbstat:set of fbflags;        { file base status vars }
    acs,                          { access requirements }
    ulacs,                        { upload requirements }
    nameacs:acstring;             { see-names requirements }
    permindx:longint;             { permanent index # }
    Conf:byte;                    { fbase conference }
    res:array[1..5] of byte;      { RESERVED }
  end;

  filstat=
   (notval,                       { if file is NOT validated }
    isrequest,                    { if file is REQUEST }
    resumelater);                 { if file is RESUME-LATER }

  ulfrec=                         { *.DIR : File records }
  record
    filename:string[12];          { Filename }
    description:string[60];       { File description }
    filepoints:integer;           { File points }
    nacc:integer;                 { Number DLs }
    ft:byte;                      { File type (useless?) }
    blocks:integer;               { # 128 byte blks }
    owner:integer;                { ULer of file }
    stowner:string[36];           { ULer's name }
    date:string[8];               { Date ULed }
    daten:integer;                { Numeric date ULed }
    vpointer:longint;             { Pointer to verbose descr, -1 if none }
    filestat:set of filstat;      { File status }
    priv:boolean;                 { private for someone? }
    privfor:string[8];            { a short string of who its for }
    {res:array[1..10] of byte;     { RESERVED }
  end;

  verbrec=                        { VERBOSE.DAT : Verbose descriptions }
  record
    descr:array[1..20] of string[50];
  end;

  lcallers=                       { LASTON.DAT : Last few callers records }
  record
    callernum:integer;            { system caller number }
    name:string[36];              { user name of caller }
    number:integer;               { user number of caller }
    citystate:string[30];         { city/state of caller }
  end;

  eventrec=                       { EVENTS.DAT : Event records }
  record
    active:boolean;               { whether active }
    description:string[30];       { event description (for logs) }
    etype:char;                   { A:CS, C:hat, D:os call, E:xternal }
    execdata:string[20];          { errorlevel if "E", commandline if "D" }
    busytime:integer;             { off-hook time before; 0 if none }
    exectime:integer;             { time of execution }
    busyduring:boolean;           { busy phone DURING event? }
    duration:integer;             { length of time event takes }
    execdays:byte;                { bitwise execution days or day of month if monthly }
    monthly:boolean;              { monthly event? }
  end;

  mnuflags=
   (clrscrbefore,                 { C: clear screen before menu display }
    dontcenter,                   { D: don't center the menu titles! }
    nomenuprompt,                 { N: no menu prompt whatsoever? }
    forcepause,                   { F: force a pause before menu display? }
    pulldown,                     { P: pulldown flag. }
    autotime);                    { T: is time displayed automatically? }


  menurec=                        { *.MNU : Menu records }
  record
    menuname:array[1..3] of string[40];  { menu name }
    directive,                           { help file displayed }
    tutorial:string[12];                 { tutorial help file }
    menuprompt:string[120];              { menu prompt }
    acs:acstring;                        { access requirements }
    password:string[15];                 { password required }
    fallback:string[8];                  { fallback menu }
    forcehelplevel:byte;                 { forced help level for menu }
    gencols:byte;                        { generic menus: # of columns }
    gcol:array[1..3] of byte;            { generic menus: colors }
    menuflags:set of mnuflags;           { menu status variables }
  end;

  cmdflags=
   (hidden,                       { H: is command ALWAYS hidden? }
    pull,                         { P: is command flagged as Pulldown Active? }
    unhidden);                    { U: is command ALWAYS visible? }

  commandrec=                       { *.MNU : Command records }
  record
    ldesc:string[70];               { long/Normal Text command description }
    sdesc:string[70];               { short/Highlighted command description }
    xpoint,                         { the command's X position }
    ypoint:string[3];               { the command's Y position }
    ckeys:string[14];               { command-execution keys }
    acs:acstring;                   { access requirements }
    cmdkeys:string[2];              { command keys: type of command }
    mstring:string[50];             { MString: command data }
    commandflags:set of cmdflags;   { command status variables }
  end;

  xbflags=
   (xbactive,
    xbisbatch,
    xbisresume,
    xbxferokcode);

  protrec=
  record
    xbstat:set of xbflags;                       { protocol flags }
    ckeys:string[14];                            { command keys }
    descr:string[40];                            { description }
    acs:acstring;                                { access string }
    templog:string[25];                          { temp. log file }
    uloadlog,dloadlog:string[25];                { permanent log files }
    ulcmd,dlcmd:string[78];                      { UL/DL commandlines }
    ulcode,dlcode:array [1..6] of string[6];     { UL/DL codes }
    envcmd:string[60];                        {B}{ environment setup cmd }
    dlflist:string[25];                       {B}{ DL file lists }
    maxchrs:integer;                             { max chrs in cmdline }
    logpf,logps:integer;                      {B}{ pos in log file for data }
    permindx:longint;                            { permanent index # }
    res:array[1..11] of byte;                    { RESERVED }
  end;

  datetimerec=
  record
    day,hour,min,sec:longint;
  end;

  fidorec=                          { FIDONET.DAT : FidoNet information }
  record
    zone:integer;                   { FidoNet zone number }
    net:integer;                    { FidoNet net number }
    node:integer;                   { FidoNet node number }
    point:integer;                  { FidoNet point number }
    origin:string[50];              { origin line }
    text_color:byte;                { color of standard text }
    quote_color:byte;               { color of quoted text }
    tear_color:byte;                { color of tear line }
    origin_color:byte;              { color of origin line }
    skludge:boolean;                { strip kludge lines? }
    sseenby:boolean;                { strip SEEN-BY lines? }
    sorigin:boolean;                { strip origin line? }
    scenter:boolean;                { strip TG centering codes? }
    sbox:boolean;                   { strip TG box codes? }
    mcenter:boolean;                { center boxed/centered lines? }
    addtear:boolean;                { add tear/origin lines? }
    res:array[1..1978] of byte;     { RESERVED }
  end;
  promptrec = array[1..4] of record
    prompt : string[80];
    name   : string[20];
  end;
  areaRec = array[1..150] of byte;
  rumorRec = string[79];
  oneLineRec = string[70];
  ansiBufType = array[1..maxBufSize] of char;
   mciType = record
      isAction : boolean;
      action : procedure;
      code : array[1..2] of char;
      value : string[80];
   end;

   BBSrec=Record
      Name:String[30];
      Speed:String[5];
      Soft:String[8];
      Phone:String[12];
      Comment:String[30];
      Sysop:String[10];
   end;
   stringType = array[1..maxStrings] of string[80];

Implementation
End.
