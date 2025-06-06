{$A+,B+,F+,I+,L-,O+,R-,S-,V-}
unit output;
interface
procedure oPrint(str : string);
procedure oPrintMci(str : string);
procedure oPrintLn(str : string);
procedure oPrintMciLn(str : string);

implementation
uses
   common,   common1,  common2,  common5,  crt,      records,
   strProc,  dos;


procedure oPrintMci(str : string);
var
   x: integer;
   pub:boolean;
   xpoint, ypoint : byte;
   tmp : string[2];
   i : integer;
   c,
   c2 : char;
begin
   x:=1;
   randomize;
   while x <= length(str) do begin
      if str[x] = '|' then begin
         inc(x);
         c2 := upcase(str[x]);
         inc(x);
         c := upcase(str[x]);
         inc(x);
         case (c2) of
            '0' : begin
               case (c) of
                  '0'..'9' : setc(value(c));
                  else  prompt('|0'+c);
               end;
            end;
            '1' : begin
               case (c) of
                  '0'..'5' : setc(value('1'+c));
                  else prompt('|1'+c);
               end;
            end;
            'A' : begin
               case (c) of
                  'D' : prompt(thisuser.street);
                  else prompt('|A'+c);
               end;
            end;
            'B' : begin
               case (c) of
                  '0'..'9': setc(textAttr+(16*(value(c))));
                  'A'..'F': setc(textAttr+(16*(ord(c) - 55)));
                  'M' : prompt(cStr(status^.maxTimeBank));
                  'N' : prompt(status^.bbsname);
                  'T' : prompt(cStr(thisUser.timeBank));
                  else prompt('|B'+c);
               end;
         end;
         'C' : begin
            case (c) of
            'F' : prompt(memuboard.name);
            'L' : cls;
            'M' : begin
               pub:=(bread<>-1);
               prompt(aonoff(pub,memboard.name,'Private Mail'));
            end;
            'S' : prompt(cstr(thisUser.loggedon));
            'T' : prompt(cstr(status^.callernum));
            else prompt('|C'+c);
            end;
         end;
         'D' : begin
            case (c) of
            'F' : begin
               case str[x] of
                  'K' : prompt(strInsertComma(cStrL(diskFree(exdrv(paramStr(0)))div 1024)));
                  'M' : prompt(strInsertComma(cStrL(diskFree(exdrv(paramStr(0))) div (1024*1024))));
                  else sprint(strInsertComma(cStrL(diskFree(exdrv(paramStr(0))))));
               end;
               inc(x);
            end;
            'K' : prompt(cstrl(thisuser.dk));
            'N' : prompt(cstr(thisuser.downloads));
            'T' : prompt(status^.todayzlog.date);
            'S' : begin
               case str[x] of
                  'K' : prompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0)))div 1024)));
                  'M' : prompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0))) div (1024*1024))));
                  else prompt(strInsertComma(cStrL(diskSize(exdrv(paramStr(0))))));
               end;
               inc(x);
            end;
            else prompt('|D'+c);
            end;
         end;
         'F' : begin
            case (c) of
            'L' : prompt(cstr(thisuser.dsl));
            'P' : prompt(cstr(thisuser.filepoints));
            'T' : prompt(cstr(status^.todayzlog.uploads));
            else prompt('|F'+c);
            end;
         end;
         'H' : begin
            case (c) of
            'M' : prompt(cstr(himsg+1));
            'S' : prompt(hdrStr);
            else prompt('|H'+c);
            end;
         end;
         'L' : begin
            case (c) of
            'C' : prompt(lastcalled);
            'D' : prompt(thisUser.lastOn);
            'O' : prompt(thisUser.cityState);
            'P' : prompt(cstr(modemR.waitBaud));
            'S' : begin
               prompt(expand(str[x],length(hdrStr)+1));
               inc(x);
            end;
            else prompt('|L'+c);
            end;
         end;
         'M' : begin
            case (c) of
            'N' : prompt(cstr(msg_on+1));
            else prompt('|M'+c);
            end;
         end;
         'N' : begin
            case (c) of
            'F' : prompt(cstr(status^.todayzlog.uploads));
            'L' : nl;
            'M' : if (menuactiv) then prompt(menur.menuname[1])
               else prompt('Main');
            'N' : prompt(cstr(thisnode));
            'P' : prompt(cstr(status^.todayzlog.pubPost));
            'T' : prompt(cstr(status^.todayzlog.newusers));
            else prompt('|N'+c);
            end;
         end;
         'O' : begin
            case (c) of
            '1' : sprompt(changeablemci);
            'S' : sPrompt(osName);
            else prompt('|O'+c);
            end;
         end;
         'P' : begin
            case (c) of
            'A' : pauseScr;
            'N' : prompt(thisuser.ph);
            'S' : prompt(cstr(thisuser.msgPost));
            else prompt('|P'+c);
            end;
         end;
         'R' : begin
            case (c) of
            'B' : setc(random(255) mod 16);
            'C' : setc(random(255));
            'F' : setc(random(255) div 16);
            'N' : prompt(caps(thisUser.realName));
            else prompt('|R'+c);
            end;
         end;
         'S' : begin
            case (c) of
            'P' : prompt(spd);
            'N' : prompt(status^.sysopname);
            else prompt('|S'+c);
            end;
         end;
         'T' : begin
            case (c) of
            'C' : prompt(cstr(status^.todayZlog.calls));
            'L' : prompt(tlef);
            'T' : prompt(cstr(thisUser.tlToday));
            'P' : prompt(cstr(status^.todayZlog.pubPost));
            else prompt('|T'+c);
            end;
         end;
         'U' : begin
            case (c) of
            '0'..'9' : cl(value(c));
            'C' : prompt(thisUser.note);
            'D' : prompt(cstr(thisUser.uploads));
            'K' : prompt(cstr(thisuser.uk));
            'L' : prompt(cstr(thisUser.sl));
            'N' : prompt(caps(thisuser.name));
            'U' : prompt(cstr(small.number));
            else prompt('|U'+c);
            end;
         end;
         'V' : begin
            case (c) of
            'A' : prompt(acronym);
            'D' : prompt(verdate);
            'N' : prompt(ProgName);
            'V' : prompt(ver);
            else prompt('|V'+c);
            end;
         end;
         'X' : begin
            case(c) of
            'Y' : begin
               tmp := '';
               for i:=1 to 2 do begin
                  tmp := tmp+str[x];
                  inc(x);
               end;
               xpoint := value(tmp);
               tmp:='';
               for i:=1 to 2 do begin
                  tmp := tmp+str[x];
                  inc(x);
               end;
               ypoint := value(tmp);
               if xpoint = 0 then xpoint := wherex;
               if ypoint = 0 then ypoint := wherey;
               goxy(xpoint,ypoint);
            end;
            else prompt('|X'+c);
            end;
         end;
         else prompt('|'+c2+c);
         end;
      end
      else begin
         if (outcom) then pr1(str[x]);
         write(str[x]);
         inc(x);
      end;
 end;
end;

procedure oPrint(str : string);
var
   x: integer;
   tmp : string[2];
   c,
   c2 : char;
begin
   x:=1;
   randomize;
   while x <= length(str) do begin
      if str[x] = '|' then begin
         inc(x);
         c2 := upcase(str[x]);
         inc(x);
         c := upcase(str[x]);
         inc(x);
         case (c2) of
            '0' : begin
               case (c) of
                  '0'..'9' : setc(value(c));
                  else  prompt('|0'+c);
               end;
            end;
            '1' : begin
               case (c) of
                  '0'..'5' : setc(value('1'+c));
                  else prompt('|1'+c);
               end;
            end;
            'B' : begin
               case (c) of
                  '0'..'9': setc(textAttr+(16*(value(c))));
                  'A'..'F': setc(textAttr+(16*(ord(c) - 55)));
                  else prompt('|B'+c);
               end;
            end;
         'R' : begin
            case (c) of
            'B' : setc(random(255) mod 16);
            'C' : setc(random(255));
            'F' : setc(random(255) div 16);
            else prompt('|R'+c);
            end;
         end;
         'U' : begin
            case (c) of
            '0'..'9' : cl(value(c));
            else prompt('|U'+c);
            end;
         end;
         else prompt('|'+c2+c);
         end;
      end
      else begin
         prompt(str[x]);
         inc(x);
      end;
 end;
end;

procedure oPrintLn(str : string);
begin
  oPrint(str+^M^J);
end;

procedure oPrintMciLn(str : string);
begin
  oPrintMciLn(str+^M^J);
end;

end.
