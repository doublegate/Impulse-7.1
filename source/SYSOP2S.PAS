{ String Editor }
{$A+,B+,D-,F+,I+,L+,O+,R-,S-,V-}
unit sysop2s;

interface

uses
   cmd,      common,   common2,  common3,  common5,  crt,      dos,
   output,   records,  strProc,  strProc2;

procedure postring;
implementation

const
  totalStrings=115;
  totalPages=20;
  sysStringsPerpage = 6;

var isError : boolean;
    eMsg    : string[40];

procedure command2;
begin
   if isError then sprint('|12'+eMsg)
   else nl;
   sprint('|U9[|U1Q|U0/Quit, |U1S|U0/Search, |U1R|U0/Repeat, |U1J|U0/Jump, |U1?|U0/Help|U9]');
   sprompt('|U0Command|U9: |U1');
end;

procedure middle2(c,t,s:string);
begin
   sprint('|U0'+c+' |U9('+'|U0'+t+'|U9) |U1 - '+s);
end;

procedure instring(p:astr; var v:astr; len:integer);
var changed:boolean;
begin
   print('Enter new "'+p+'" string:');
   if (incom) then inputwnwc(v,len,changed)
   else v:=NInput(WhereX,WhereY,v,len);
end;

function lookupPage(count : integer) : byte;
begin
   lookupPage := ((count-1) div 6)+1;
end;

function strSearch(startPos : byte; searchTxt : string;old : boolean) : integer;
var isIn,
    count,
    hold : byte;
    go    : boolean;
begin
   if searchTXT = '' then begin
      go := false;
      isError := true;
      eMsg := '|U1String Not Found!';
      strSearch := StartPos;
   end
   else go := true;
   if not(old) then sprompt(searchTXT);
   count := ((startPos) * sysStringsPerPage)-(sysStringsPerPage-1);
   hold := count;
   while go do begin
      isIn := pos(strAllCaps(searchTxt),strAllCaps(strlookupString(count)));
      if isIn <> 0 then begin
         isError := FALSE;
         go := false;
         strSearch := lookupPage(count);
         exit;
      end; { IF ISIN <> 0 }
      if count >= (totalStrings) then count := 1
      else inc(count);
      if count = hold then begin
         go := false;
         strSearch := startPos;
         isError := TRUE;
         eMsg := '|U1String Not Found!';
         exit;
      end; { IF COUNT = HOLD }
   end; { WHILE GO }
end;

procedure postring;
var fstringf:file of fstringrec;
    s : string[1];
    onpage:integer;
    c:char;
    abort,next,done:boolean;

  procedure showstrings;
  const globalKeys='QSROGJ?[]';
  begin
    abort:=FALSE; next:=FALSE;
    with strings^ do
      case onpage of
        1:begin
            middle2('1','1','System Password'); oPrintLn(ansiq);
            middle2('2','2','Handle Prompt'); oPrintLn(enterhandle);
            middle2('3','3','Password Prompt'); oPrintLn(enterpw);
            middle2('4','4','File Listing'); oPrintLn(lprompt);
            middle2('5','5','FSE Editor Prompt'); oPrintLn(fseprompt);
            middle2('6','6','Echo chr'); oPrintLn(echoc);
            command2;
            onek(c,globalKeys+'123456');
          end;
        2:begin
            middle2('1','7','Quote Starting At'); oPrintLn(quote1);
            middle2('2','8','Quote Ending At'); oPrintLn(quote2);
            middle2('3','9','Engage chat'); oPrintLn(engage);
            middle2('4','10','Exit chat'); oPrintLn(endchat);
            middle2('5','11','Sysop working'); oPrintLn(wait);
            middle2('6','12','Pause screen '); oPrintLn(pause);
            command2;
            onek(c,globalKeys+'123456');
          end;
        3:begin
            middle2('1','13','Join Conference'); oPrintLn(entermsg1);
            middle2('2','14','Prompt Choose (V/View)'); oPrintLn(entermsg2);
            middle2('3','15','NewScan start    '); oPrintLn(newscan1);
            middle2('4','16','NewScan done     '); oPrintLn(newscan2);
            middle2('5','17','Read msgs prompt '); oPrintLn(scanmessage);
            middle2('6','18','Start msg reading'); oPrintLn(startmsgread);
            command2;
            onek(c,globalKeys+'123456');
          end;
        4:begin
            middle2('1','19','Automessage by   '); oPrintLn(automsgt);
            middle2('2','20','Auto border char.'); oPrintLn(autom);
            middle2('3','21','Sysop Turbo Login'); oPrintLn(turbologin);
            middle2('4','22','Shell to DOS L#1 '); oPrintLn(shelldos1);
            middle2('5','23','Shell to DOS L#2 '); oPrintLn(shelldos2);
            middle2('6','24','Logoff after transfer?'); oPrintLn(logoffafter);
            command2;
            onek(c,globalKeys+'123456');
          end;
        5:begin
            middle2('1','25','Enter Chat reason'); oPrintLn(chatcall1);
            middle2('2','26','Paging Sysop     '); oPrintLn(chatcall2);
            middle2('3','27','Guest user info  '); oPrintLn(guestline);
            middle2('4','28','Name not found   '); oPrintLn(namenotfound);
            middle2('5','29','Bulletin line    '); oPrintLn(bulletinline);
            middle2('6','30','Add Rumor        '); oPrintLn(addrumor);
            command2;
            onek(c,globalKeys+'123456');
          end;
        6:begin
            middle2('1','31','List line        '); oPrintLn(listline);
            middle2('2','32','File NewScan line'); oPrintLn(newline);
            middle2('3','33','Search line      '); oPrintLn(searchline);
            middle2('4','34','Find Descrip. L#1'); oPrintLn(findline1);
            middle2('5','35','Find Descrip. L#2'); oPrintLn(findline2);
            middle2('6','36','Enter Filename to UL/DL');
                        oPrintLn(enterfilenametoul);
            command2;
            onek(c,globalKeys+'123456');
          end;
        7:begin
            middle2('1','37','Download line    '); oPrintLn(downloadline);
            middle2('2','38','Upload line - Type ''AFTER'' to batch upload');
              oPrintLn(uploadline);
            middle2('3','39','View content line'); oPrintLn(viewline);
            middle2('4','40','Insuff. file pts '); oPrintLn(nofilepts);
            middle2('5','41','Bad UL/DL ratio  '); oPrintLn(unbalance);
            middle2('6','42','File Newscan'); oPrintLn(fnewscan);
            command2;
            onek(c,globalKeys+'123456');
          end;
        8:begin
            middle2('1','43','P/N file info    '); oPrintLn(pninfo);
            middle2('2','44','Get filespec L#1 '); oPrintLn(gfnline1);
            middle2('3','45','Get filespec L#2 '); oPrintLn(gfnline2);
            middle2('4','46','Area Change (?/Help)'); oPrintLn(batchadd);
            middle2('5','47','New User Password'); oPrintLn(newuserp);
            middle2('6','48','Aborted'); oPrintLn(aborted);
            command2;
            onek(c,globalKeys+'123456');
          end;
        9:begin
            middle2('1','49','Quote Begin      '); oPrintLn(quotebegin);
            middle2('2','50','Quote End        '); oPrintLn(quoteend);
            middle2('3','51','Updating system  '); oPrintLn(updatesys);
            middle2('4','52','Enter Oneliner?  '); oPrintLn(enteroneline);
            middle2('5','53','Protocol Selection'); oPrintLn(protselect);
            middle2('6','54','Global Search?'); oPrintLn(globalnewscan);
            command2;
            onek(c,globalKeys+'123456');
          end;
       10:begin
           middle2('1','55','File Search [W/S] '); oPrintLn(multitask);
           middle2('2','56','Login Phone Number'); oPrintLn(phonenum);
           middle2('3','57','BBS Comment       '); oPrintLn(bbsadd);
           middle2('4','58','BBS Phone Number  '); oPrintLn(bbsphone);
           middle2('5','59','BBS Name          '); oPrintLn(bbsname);
           middle2('6','60','File added to Batch'); oPrintLn(fileaddedtobq);
           command2;
           onek(c,globalKeys+'123456');
          end;
       11:begin
           middle2('1','61','BBS Speed         '); oPrintLn(bbsspeed);
           middle2('2','62','BBS Type          '); oPrintLn(bbstype);
           middle2('3','63','BBS Sysop');          oPrintLn(nuzipcode);
           middle2('4','64','Leave Feedback?   '); oPrintLn(leavefeedback);
           middle2('5','65','Sysop Password    '); oPrintLn(sysoppass);
           middle2('6','66','Post Anonymous?'); oPrintLn(postanon);
           command2;
           onek(c,globalKeys+'123456');
          end;
       12:begin
           middle2('1','67','New User Address        '); oPrintLn(nuaddress);
           middle2('2','68','New User From           '); oPrintLn(nucity);
           middle2('3','69','New User Voice Phone #  '); oPrintLn(nuphone);
           middle2('4','70','New User Enter Handle   '); oPrintLn(nuhandle);
           middle2('5','71','New User Info Prompt    '); oPrintLn(nustate);
           middle2('6','72','New User Enter Note'); oPrintLn(enterunote);
           command2;
           onek(c,globalKeys+'123456');
           end;
       13:begin
           middle2('1','73','New User Screen Width   '); oPrintLn(nuscrwide);
           middle2('2','74','New User Screen Length  '); oPrintLn(nuscrlines);
           middle2('3','75','New User Real Name      '); oPrintLn(nureal);
           middle2('4','76','New User Sex (M/F/L/N)  '); oPrintLn(nusex);
           middle2('5','77','New User Words Blink?   '); oPrintLn(nuwords);
           middle2('6','78','New User Enter Autosig'); oPrintLn(enterautosig);
           command2;
           onek(c,globalKeys+'123456');
          end;
       14:begin
           middle2('1','79','New User Use ANSI?      '); oPrintLn(nuansi);
           middle2('2','80','New User Use Pause?     '); oPrintLn(nupause);
           middle2('3','81','New User Enter Password '); oPrintLn(nuenterpw);
           middle2('4','82','New User Birthday       '); oPrintLn(nubday);
           middle2('5','83','Logon as New?     '); oPrintLn(logonnew);
           middle2('6','84','Default User Note'); oPrintLn(newusernote);
           command2;
           onek(c,globalKeys+'123456');
          end;
       15:begin
           middle2('1','85','Enter msg title'); oPrintLn(msgtitle);
           middle2('2','86','Send msg to'); oPrintLn(msgsendto);
           middle2('3','87','Upload msg?'); oPrintLn(uploadmsg);
           middle2('4','88','Delete Read Mail?'); oPrintLn(deletereadmail);
           middle2('5','89','No mail waiting'); oPrintLn(nomailwaiting);
           middle2('6','90','Post on Msgbase?'); oPrintLn(posthere);
           command2;
           onek(c,globalKeys+'123456');
          end;
       16:begin
           middle2('1','91','Message posted on Msgbase'); oPrintLn(msgsaved);
           middle2('2','92','# Total msgs on Msgbase'); oPrintLn(numbermsgs);
           middle2('3','93','Read Your Email?'); oPrintLn(readmailq);
           middle2('4','94','Add to batch, Quit'); oPrintLn(numisc1);
           middle2('5','95','Scanning Uploaded Files'); oPrintLn(numisc2);
           middle2('6','96','Enter your Oneliner'); oPrintLn(enteronl);
           command2;
           onek(c,globalKeys+'123456');
          end;
       17:begin
           middle2('1','97','"MBASENAME" Joined'); oPrintLn(mbasejoined);
           middle2('2','98','"FBASENAME" Joined'); oPrintLn(fbasejoined);
           middle2('3','99','NUV Y/es N/o A/bstain'); oPrintLn(nuvote);
           middle2('4','100','NUV User Validated'); oPrintLn(nuvvalidated);
           middle2('5','101','NUV User Deleted'); oPrintLn(nuvdeleted);
           middle2('6','102','File private for'); oPrintLn(fileprivate);
           command2;
           onek(c,globalKeys+'123456');
          end;
       18:begin
           middle2('1','103','Re-Enter Password'); oPrintLn(reenterpw);
           middle2('2','104','Rumor Added'); oPrintLn(rumorentered);
           middle2('3','105','YES/no Prompt'); oPrintLn(yesbar);
           middle2('4','106','yes/NO Prompt'); oPrintLn(nobar);
           middle2('5','107','Local Login String'); oPrintLn(locallogin);
           middle2('6','108','Hack Attempt'); oPrintLn(hack);
           command2;
           onek(c,globalKeys+'123456');
          end;
       19:begin
           middle2('1','109','Are you sure?');
           oPrintLn(areyousure);
           middle2('2','110','Incorrect Password');
           oPrintLn(wrongPassword);
           middle2('3','111','Current Password');
           oPrintLn(currentPassword);
           middle2('4','112','Wrong Phone Number');
           oPrintLn(wrongPhone);
           middle2('5','113','Time Bank Deposit');
           oPrintLn(timeBankDeposit);
           middle2('6','114','Time Bank Withdraw');
           oPrintLn(timeBankWithDraw);
           command2;
           onek(c,globalKeys+'123456');
          end;
       20:begin
           middle2('1','115','Unknown Handle');
           oPrintLn(unKnownHandle);
           command2;
           onek(c,globalKeys+'1');
          end;
      end;
  end;

  procedure dostringstuff;
  var str,
      lu : string[80];
      jt : integer;
  begin
    str := '';
    case c of
      '?':printf('mcihelp.ans');
      'S':begin
            str := '';
            instring('search',str,79);
            onpage := strSearch(onpage,str,false);
          end;
      'R': if (not isError) and (str <> '') then begin
         if (onPage = TotalPages) then onPage := strSearch(1,str,true)
         else onpage := strSearch(onpage+1,str,true);
      end;
      'J':begin
            lu := '';
            instring('Jump to string',lu,79);
            jt := strStringToWord(lu);
            write(jt);
            if (jt >= 1) AND (jt <= totalStrings) then begin
              onpage := lookupPage(jt);
              isError := FALSE;
            end
            else begin
              eMsg := '|U1Invalid String Number!';
              isError := TRUE;
            end;
          end;
      'Q':done:=TRUE;
      'O':onpage := totalPages;
      'G':onpage := 1;
      '[':if onPage > 1 then dec(onpage);
      ']':if onPage < totalPages then inc(onpage);
    end;
    with strings^ do
      case onpage of
        1:case c of
            '1':instring('System Password Entry',ansiq,79);
            '2':instring('Handle Prompt',enterhandle,79);
            '3':instring('Password Prompt',enterpw,79);
            '4':instring('File List Prompt',lprompt,79);
            '5':instring('FSE Editor Prompt',fseprompt,79);
            '6':begin
                  prompt('Enter new echo character: ');
                  inputl(s,1);
                  if (s<>'') then echoc:=s[1];
                end;
           end;
        2:case c of
            '1':instring('Quote Starting At',quote1,79);
            '2':instring('Quote Starting At',quote2,79);
            '3':instring('Engage chat',engage,79);
            '4':instring('End chat',endchat,79);
            '5':instring('SysOp working',wait,79);
            '6':instring('Pause',pause,79);
          end;
        3:case c of
            '1':instring('Join Conference',entermsg1,79);
            '2':instring('Prompt Choice (V/View)',entermsg2,79);
            '3':instring('NewScan line 1',newscan1,79);
            '4':instring('NewScan line 2',newscan2,79);
            '5':instring('Read message prompt',scanmessage,79);
            '6':instring('Start msg reading',startmsgread,79);
           end;
        4:case c of
            '1':instring('Auto message title',automsgt,79);
            '2':begin
                  print('Enter new auto message border character:');
                  inputl(s,1);
                  if (s<>'') then autom:=s[1];
                end;
            '3':instring('Turbo Logon',turbologin,79);
            '4':instring('Shell to DOS line 1',shelldos1,79);
            '5':instring('Shell to DOS line 2',shelldos2,79);
            '6':instring('Logoff after Transfer',logoffafter,79);
           end;
        5:case c of
            '1':instring('Enter Chat reason',chatcall1,79);
            '2':instring('Paging Sysop',chatcall2,79);
            '3':instring('Guest user info at logon prompt',guestline,79);
            '4':instring('Name not found line during logon',namenotfound,79);
            '5':instring('Bulletins prompt line',bulletinline,79);
            '6':instring('Add a Rumor',addrumor,79);
          end;
        6:case c of
            '1':instring('List line',listline,79);
            '2':instring('File NewScan line',newline,79);
            '3':instring('Search line',searchline,79);
            '4':instring('Find description line 1',findline1,79);
            '5':instring('Find description line 2',findline2,79);
            '6':instring('Filename to UL/DL',enterfilenametoul,79);
          end;
        7:case c of
            '1':instring('Download line',downloadline,79);
            '2':instring('Upload line - Type ''AFTER'' to batch Upload',
                 uploadline,79);
            '3':instring('View interior contents line',viewline,79);
            '4':instring('Insufficient file points',nofilepts,79);
            '5':instring('Upload/Download ratio unbalanced',unbalance,79);
            '6':instring('File Newscan',fnewscan,79);
          end;
        8:case c of
            '1':instring('P / N file information',pninfo,79);
            '2':instring('Get filespec line 1',gfnline1,79);
            '3':instring('Get filespec line 2',gfnline2,79);
            '4':instring('Area Change (?/Help)',batchadd,79);
            '5':instring('New User Password',newuserp,79);
            '6':instring('Aborted',aborted,79);
          end;
       9:case c of
            '1':instring('Quote Begin',quotebegin,79);
            '2':instring('Quote End',quoteend,79);
            '3':instring('Updating system',updatesys,79);
            '4':instring('Enter Oneliner',enteroneline,79);
            '5':instring('Protocol Selection',protselect,79);
            '6':instring('Global Search',globalnewscan,79);
          end;
      10:case c of
           '1':instring('File Search [W/S]',multitask,79);
           '2':instring('Login Phone Number',phonenum,79);
           '3':instring('BBS Add to it?',bbsadd,79);
           '4':instring('BBS Phone Number',bbsphone,79);
           '5':instring('BBS Name',bbsname,79);
           '6':instring('File Added to batch',fileaddedtobq,79);
         end;
      11:case c of
           '1':instring('BBS Speed',bbsspeed,79);
           '2':instring('BBS Type',bbstype,79);
           '3':instring('BBS Sysop',nuzipcode,79);
           '4':instring('Leave Feedback?',leavefeedback,79);
           '5':instring('Sysop Password',sysoppass,79);
           '6':instring('Post Anonymously',postanon,79);
         end;
      12:case c of
           '1':instring('New User Address',nuaddress,79);
           '2':instring('New User From',nucity,79);
           '3':instring('New User Voice Phone #',nuphone,79);
           '4':instring('New User Enter Handle',nuhandle,79);
           '5':instring('New User Info Change Prompt',nustate,79);
           '6':instring('Enter Usernote',enterunote,79);
           end;
      13:case c of
           '1':instring('New User Screen Width',nuscrwide,79);
           '2':instring('New User Screen Length',nuscrlines,79);
           '3':instring('New User Real Name',nureal,79);
           '4':instring('New User Sex (M/F/L/N)',nusex,79);
           '5':instring('New User Words Blink?',nuwords,79);
           '6':instring('Enter Autosig',enterautosig,79);
         end;
      14:case c of
           '1':instring('New User Use ANSI Question',nuansi,79);
           '2':instring('New User Use Pause Question',nupause,79);
           '3':instring('New User Enter Password',nuenterpw,79);
           '4':instring('New User Birthday',nubday,79);
           '5':instring('Logon As New Question',logonnew,79);
           '6':instring('Default Usernote',newusernote,79);
          end;
       15:case c of
           '1':instring('Enter msg title',msgtitle,79);
           '2':instring('Send msg to',msgsendto,79);
           '3':instring('Upload msg?',uploadmsg,79);
           '4':instring('Delete Read Mail?',deletereadmail,79);
           '5':instring('No mail waiting',nomailwaiting,79);
           '6':instring('Post on Msgbase?',posthere,79);
          end;
       16:case c of
           '1':instring('Msg Posted on Msgbase',msgsaved,79);
           '2':instring('# Total Msgs on Msgbase',numbermsgs,79);
           '3':instring('Read your Email?',readmailq,79);
           '4':instring('(A)dd to batch, (Q)uit',numisc1,79);
           '5':instring('Scanning Uploaded Files',numisc2,79);
           '6':instring('Enter your Oneliner',enteronl,79);
          end;
       17:case c of
           '1':instring('"MBASENAME" Joined',mbasejoined,79);
           '2':instring('"FBASENAME" Joined',fbasejoined,79);
           '3':instring('NUV Y/es N/o A/bstain',nuvote,79);
           '4':instring('NUV User Validated',nuvvalidated,79);
           '5':instring('NUV User Deleted',nuvdeleted,79);
           '6':instring('File Private for',fileprivate,79);
          end;
       18:case c of
           '1':instring('Re-Enter Password',reenterpw,79);
           '2':instring('Rumor Added',rumorentered,79);
           '3':instring('YES/no Prompt',yesbar,79);
           '4':instring('yes/NO Prompt',nobar,79);
           '5':instring('Local Login String',localLogin,79);
           '6':instring('Hack Attempt',hack,79);
          end;
       19:case c of
           '1': inString('Are you sure?',areyousure,79);
           '2': inString('Incorrect Password',wrongPassword,79);
           '3': inString('Current Password',currentPassword,79);
           '4': inString('Wrong Phone Number',wrongPhone,79);
           '5': inString('Time Bank Deposit',timeBankDeposit,79);
           '6': inString('Time Bank WithDraw',timeBankWithdraw,79);
          end;
       20 : case c of
          '1' : inString('UnKnown Handle',unKnownHandle,79);
       end;
      end;
  end;

begin
  onpage:=1;
  done:=FALSE;
  isError := FALSE;
  repeat
     cls;
     sprint(#3#5+'String configuration - page '+cstr(onpage)+' of '+cstr(totalPages));
     nl;
     showstrings;
     nl;
     dostringstuff;
  until ((done) or (hangup));
  assign(fstringf,status^.gfilepath+'string.dat');
  reset(fstringf);
  write(fstringf,strings^);
  close(fstringf);
end;
end.
