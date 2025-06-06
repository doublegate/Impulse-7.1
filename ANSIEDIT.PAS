{$R-,S-,I-,F+,V-,B-,N-,L+,O+ }
unit ansiedit;    (* Ansi Full Screen Editor *)

interface

uses
   Crt,      Dos,      common,   common1,  common2,  records,  common3,
   common4,  common5,  ansidrv,  sys;

const
   maxlines = 101;

type
   message = record
      text:array [1..maxlines] of String[79];
      numlines:integer;
      basename,
      msgtitle,
      from,
      touser,
      dateleft : string;
   end;

function ansireedit (var m:message; gettitle:boolean):boolean;

implementation

uses mail1;

function ansireedit (var m:message; gettitle:boolean):boolean;
var topline,curline,cx,cy,cols,scrnsize,lines,
    rightmargin,savedx,savedy,topscrn:integer;
    insertmode,msgdone,ansimode:boolean;
    badnum:boolean;
    hdrlen:integer;

   procedure sstr(s:string);
   var i:byte;
   begin
      for i:=1 to length(s) do begin
         cansi(s[i]);
         pr1(s[i]);
      end;
   end;

   function curx:integer;
   begin
      curx:=wherex
   end;

   function cury:integer;
   begin
      cury:=wherey-topscrn+1
   end;

   procedure moveto (x,y:integer);
   begin
      y:=y+topscrn-1;
      goxy(x,y);
   end;

   procedure cleareol;
   begin
      sstr(#27'[K')
   end;

   procedure savecsr;
   begin
      sstr(#27'[s')
   end;

   procedure restorecsr;
   begin
      sstr(#27'[u')
   end;

   procedure cmove (k:char; n,dx,dy:integer);
   var cnt:integer;
   begin
      if n<1 then exit;
      sstr(#27'[');
      if n<>1 then sstr(cstr(n));
      sstr(k)
   end;

   procedure cup (n:integer);
   begin
      cmove ('A',n,0,-1)
   end;

   procedure cdn (n:integer);
   begin
      cmove ('B',n,0,1)
   end;

   procedure clf (n:integer);
   begin
      cmove ('D',n,-1,0)
   end;

   procedure crg (n:integer);
   begin
      cmove ('C',n,1,0)
   end;

   procedure checkspaces;
   var q:^astr;
   begin
      q:=addr(m.text[curline]);
      while q^[length(q^)]=' ' do q^[0]:=pred(q^[0]);
   end;

   procedure checkcx;
   var n:integer;
   begin
      n:=length(m.text[curline])+1;
      if cx>n then cx:=n
   end;

   procedure computecy;
   begin
      cy:=curline-topline+1
   end;

   procedure updatecpos;
   begin
      computecy;
      moveto (cx,cy);
   end;

   procedure insertabove;
   var cnt:integer;
   begin
      if m.numlines=maxlines then exit;
      for cnt:=m.numlines downto curline do m.text[cnt+1]:=m.text[cnt];
      m.text[curline]:='';
      m.numlines:=m.numlines+1
   end;

   procedure deletethis;
   var cnt:integer;
   begin
      if m.numlines=1 then begin
         m.text[1]:='';
         exit;
      end;
      for cnt:=curline+1 to m.numlines do m.text[cnt-1]:=m.text[cnt];
      m.text[m.numlines]:='';
      dec(m.numlines);
      checkcx;
   end;

   procedure startscr;
   begin
      cls;
      if topline<1 then topline:=1;
      computecy;
      printf('FSEHEAD');
      hdrlen:=wherey;
      cl(1);
      moveto (1,1);
      updatecpos;
   end;

   procedure fullrefresh;
   var cnt,n:integer;
   begin
      cls;
      if topline<1 then topline:=1;
      computecy;
      printf('FSEHEAD');
      hdrlen:=wherey;
      cl(1);
      moveto (1,1);
      for cnt:=1 to lines do begin
         n:=cnt+topline-1;
         if n<=m.numlines then begin
            prompt (m.text[n]);
            if cnt<>lines then nl;
         end
      end;
      updatecpos;
   end;

   procedure repos (dorefresh:boolean);
   var cl,tl:integer;
   begin
      checkspaces;
      cl:=curline;
      tl:=topline;
      if curline<1 then curline:=1;
      if curline>m.numlines then curline:=m.numlines;
      if topline>curline then topline:=curline;
      if topline+lines<curline then topline:=curline-lines;
      if topline<1 then topline:=1;
      checkcx;
      computecy;
      if (cl=curline) and (tl=topline) and (not dorefresh) then updatecpos
      else fullrefresh;
   end;

   procedure partrefresh;  { Refreshes from CY }
   var cnt,n:integer;
   begin
      (**)
      {for n:=curline-1 to 101 do m.text[n]:='';}
      (**)
      if topline<1 then repos(true)
      else begin
         moveto (1,cy);
         for cnt:=cy to lines do begin
            n:=cnt+topline-1;
            if (n<=m.numlines) and (pos(#0,m.text[n])=0) then prompt(m.text[n]);
            cleareol;
            if cnt<>lines then nl;
         end;
         updatecpos;
      end;
  end;

  procedure pageup;
  begin
     checkspaces;
     if curline=1 then exit;
     curline:=curline-lines+hdrlen{4};
     topline:=topline-lines+hdrlen;
     repos(true)
  end;

  procedure pagedn;
  begin
     checkspaces;
     if curline=m.numlines then exit;
     curline:=curline+lines-hdrlen;
     topline:=topline+lines-hdrlen;
     repos (true)
  end;

  procedure scrolldown;
  begin
     topline:=curline-lines+2;
     repos (true)
  end;

   procedure scrollup;
   begin
      if topline<1 then begin
         inc(topline,1);
         moveto (1,lines);
         computecy;
         nl;
      end
      else begin
         topline:=curline-1;
         repos (true);
      end;
   end;

   procedure topofmsg;
   begin
     checkspaces;
     cx:=1;
     cy:=1;
     curline:=1;
     if topline=1 then updatecpos
     else begin
        topline:=1;
        {fullrefresh}
     end
   end;

   procedure updateToEol;
   var cnt:integer;
   begin
      savecsr;
      prompt(copy(m.text[curline],cx,255));
      cleareol;
      restorecsr
   end;

   procedure letterkey (k:char);
   var l:^astr;
      w:astr;
      n,ox:integer;
      q:char;
      inserted,refr:boolean;

      procedure scrollwwrap;
      begin
         if topline > 0 then begin
           scrollup;
           exit
         end;
         cy:=cy-1;
         moveto (length(m.text[curline-1])+1,cy);
         cleareol;
         nl;
         prompt(m.text[curline]);
         inc(topline,1);
         cx:=curx
      end;

   begin
      l:=addr(m.text[curline]);
      if length(l^)>=rightmargin then begin
         if (curline=maxlines) OR (cx <=length(l^)) then exit;
         l^:=l^+k;
         w:='';
         cx:=length(l^);
         repeat
            q:=l^[cx];
            if q<>' ' then insert (q,w,1);
            dec(cx);
         until (q=' ') or (cx<1);
         {write(w);}
         if cx<1 then begin
            cx:=length(l^)-1;
            w:=k
         end;
         l^[0]:=chr(cx);
         checkspaces;
         inc(curline);
         if curline>m.numlines then m.numlines:=curline;
         inserted:=m.text[curline]<>'';
         if inserted then insertabove;
         m.text[curline]:=w;
         inc(cy);
         ox:=cx;
         cx:=length(w)+1;
         refr:=cy>lines;
         if refr then scrollwwrap
         else begin
            if length(w)>0 then begin
               moveto (ox+1,cy-1);
               prompt(expand(' ',length(w)));
            end;
            if inserted and (m.numlines>curline) then begin
               partrefresh;
               m.text[curline+1]:='';
            end
            else begin
               {clrscr;
               gotoxy(1,1);
               pausescr;
               fullrefresh;}
               moveto (1,cy);
               prompt(m.text[curline]);
            end
         end;
         exit
      end;
      if insertmode then insert (k,l^,cx)
      else begin
         while length(l^)<cx do l^:=l^+' ';
         l^[cx]:=k
      end;
      if k=#27 then cansi(k) else prompt(k);
      inc(cx,1);
      if insertmode and (cx<=length(l^)) then updatetoeol;
   end;

   procedure back;
   begin
      if cx=1 then begin
         if curline=1 then exit;
         checkspaces;
         dec(curline);
         dec(cy);
         cx:=length(m.text[curline])+1;
         if cy<1 then scrolldown else updatecpos;
      end
      else begin
         dec(cx);
         clf (1)
      end
   end;

   procedure fowrd;
   begin
      if cx>length(m.text[curline]) then begin
         if curline=maxlines then exit;
         checkspaces;
         inc(curline);
         if curline>m.numlines then m.numlines:=curline;
         inc(cy);
         cx:=1;
         if cy>lines then scrollup else updatecpos
      end
      else begin
         inc(cx);
         crg (1)
      end
   end;

   procedure del;
   begin
      if length(m.text[curline])=0 then begin
         deletethis;
         partrefresh;
         exit;
      end;
      delete(m.text[curline],cx,1);
      if cx>length(m.text[curline]) then prompt(' '^H)
      else updatetoeol;
   end;

   procedure beginline;
   begin
      if cx=1 then exit;
      cx:=1;
      updatecpos
   end;

   procedure endline;
   var dx:integer;
   begin
      dx:=length(m.text[curline])+1;
      if cx=dx then exit;
      cx:=dx;
      updatecpos
   end;

   procedure upline;
   var chx:boolean;
      l:integer;
   begin
      checkspaces;
      if curline=1 then exit;
      dec(curline);
      l:=length(m.text[curline]);
      chx:=cx>l;
      if chx then cx:=l+1;
      dec(cy);
      if cy>0 then
         if chx then updatecpos
         else cup (1)
      else scrolldown;
   end;

   procedure downline;
   var chx:boolean;
      l:integer;
   begin
      checkspaces;
      if curline=maxlines then exit;
      inc(curline);
      if curline>m.numlines then m.numlines:=curline;
      l:=length(m.text[curline]);
      chx:=cx>l;
      if chx then cx:=l+1;
      inc(cy);
      if cy<=lines then
         if chx then updatecpos
         else cdn (1)
      else scrollup;
   end;

   procedure bkspace;
   begin
      if (m.text[curline] = '') then begin
         upline;
         endline;
         exit;
      end;
      if cx=1 then exit;
      cx:=cx-1;
      prompt(^H);
      del;
   end;


   procedure qnum(var i:integer);
   var s:string[5];
   begin
      badnum:=FALSE;
      input(s,5);
      if (s='') or (s='Q') then i:=0
      else i:=value(s);
      if (s='') then badnum:=TRUE;
   end;

   procedure doquote;
   type qline=array [1..101] of string[79];
   var f:text;
       tmp,t1:integer;
       blah,done:boolean;
       pp,x,rr1,rr2:integer;
       quotefrom:string[40];
       qlinef:array[1..2] of string[79];
       spos:byte;
       ql:^qline;

      function openquotefile : boolean;
      begin
         openquotefile:=FALSE;
         assign(f,'msgtmp');
         {$I-} reset(f); {$I+}
         if (ioresult=0) then openquotefile:=TRUE;
         done := false;
      end;

   begin
      if (openquotefile) then begin
         readln(f,quotefrom);
         QLineF[1]:=strings^.QuoteBegin;
         if pos('|QT',strings^.QuoteBegin)<>0 Then Begin
            spos:=pos('|QT',QLineF[1]);
            delete(QLineF[1],Spos,3);
            Insert(QuoteFrom,QLineF[1],Spos);
         End;
         QLineF[2]:=strings^.QuoteEnd;
         If pos('|QT',strings^.QuoteEnd)<>0 Then Begin
            spos:=pos('|QT',QLineF[2]);
            delete(QLineF[2],Spos,3);
            Insert(QuoteFrom,QLineF[2],Spos);
         End;
         cls;
         writehdr('Quoting');
         new(ql);
         for tmp:=1 to 101 do ql^[tmp]:='';
         if (not done) then begin
            done:=FALSE;
            blah:=false;
            x:=1;
            repeat
               {$I-} readln(f,ql^[x]); {$I+}
               if (ioresult=0) then blah:=false
               else blah:=true;
               sprint(mln(cstr(x),2)+': '+ql^[x]);
               inc(x);
               if (eof(f)) then blah:=true;
            until ((x=99) or (blah));
            if (not done) then begin
               while (blah) do begin
                  sprompt(strings^.Quote1);
                  qnum(rr1);
                  if (rr1<=0) then begin
                     dispose(ql);

                     exit;
                  end;
                  if (not badnum) and (rr1>=1) then begin
                     if (rr1<1) then rr1:=1;
                     done:=true;
                     blah:=false;
                  end;
                  sprompt(strings^.Quote2);
                  qnum(rr2);
                  if (rr2<=0) then begin
                     dispose(ql);
                     fullrefresh;
                     exit;
                  end;
                  if (not badnum) and (rr2<=x) then begin
                     if (rr2>x) then rr1:=x;
                     done:=true;
                     blah:=false;
                  end;
                  if rr1-rr2>status^.csmaxlines then begin
                     sprint('|07Too many lines in quote');
                     repeat
                        dec(rr2)
                     until (rr2-rr1<status^.csmaxlines);
                  end;
               end;
               if (rr2>x) then rr2:=x;
               if (rr2<rr1) then rr2:=rr1;
               if (rr1<1) then rr1:=1;
               if (rr2=rr1) then begin
                  print(ql^[rr1]);
                  m.text[curline]:=QLineF[1];
                  inc(curline);
                  m.text[curline]:=ql^[rr1];
                  inc(curline);
                  m.text[curline]:=QLineF[2];
                  inc(curline);
               end
               else begin
                  x:=rr1;
                  m.text[curline]:=QLineF[1];
                  inc(curline);
                  repeat
                     m.text[curline]:=ql^[x];
                     inc(curline);
                     inc(x);
                  until (x=rr2+1);
                  m.text[curline]:=QLineF[2];
                  inc(curline);
               end;
               m.text[curline]:=' ';
               inc(curline);
               done:=true;
            end;
         end;
         dispose(ql);
         m.numlines:=curline;
         fullrefresh;
      end; {if openquotefile}
   end;

   procedure help;
   begin
      cls;
      printf('fsehelp');
      pausescr;
      fullrefresh;
   end;

   procedure crlf;
   var k:char;
      i:byte;
   begin
      for i:=curline+1 to maxlines do m.text[i]:='';
      if (length(m.text[curline])=2) and (m.text[curline][1]='/') then begin
         k:=upcase(m.text[curline][2]);
         case k of
            '?':Begin
               deletethis;
               help;
            end;
            'S':begin
               m.numlines:=curline;
               for i:=curline+1 to maxlines do m.text[i]:='';
               deletethis;
               msgdone:=true;
               ansireedit:=true;
               exit;
            end;
            'Q':Begin
               deletethis;
               doquote;
            end;
            'A':begin
               m.numlines:=0;
               msgdone:=true;
               ansireedit := false;
               nl;
               exit;
            end;
         end;
      end;
      beginline;
      downline;
   end;

   function conword:boolean;
   var l:^astr;
   begin
      l:=addr(m.text[curline]);
      conword:=false;
      if (cx>length(l^)) or (cx=0) then exit;
      conword:=true;
      if cx=1 then exit;
      if (l^[cx-1]=' ') and (l^[cx]<>' ') then exit;
      conword:=false;
   end;

   procedure wordleft;
   begin
      repeat
         dec(cx);
         if cx<1 then begin
            if curline=1 then begin
               cx:=1;
               repos (false);
               exit;
            end;
            checkspaces;
            dec(curLine);
            dec(cy);
            cx:=length(m.text[curline])
         end;
      until conword;
      if cx=0 then cx:=1;
      if cy<1 then repos (true)
      else updatecpos
   end;

   procedure wordright;
   begin
      repeat
         inc(cx);
         if cx>length(m.text[curline]) then begin
            if curline=m.numlines then begin
               repos (false);
               exit;
            end;
            checkspaces;
            inc(curline);
            inc(cy);
            cx:=1
         end;
      until conword;
      if cy>lines then repos (true)
      else updatecpos
   end;

   procedure worddel;
   var l:^astr;
       b:byte;
       s,n:integer;
   begin
      l:=addr(m.text[curline]);
      b:=length(l^);
      if cx>b then exit;
      s:=cx;
      repeat
         inc(cx,1);
      until conword or (cx>b);
      n:=cx-s;
      delete (l^,s,n);
      cx:=s;
      updatetoeol;
   end;

   procedure deleteline;
   var cnt : byte;
   begin
      {deletethis;}
      if m.numlines = 1 then begin
         m.text[1] := '';
         exit;
      end;
      if (curline <= m.numlines) then begin
         for cnt := curline to m.numlines do m.text[cnt] := m.text[cnt+1];
         dec(m.numlines);
      end;
      partrefresh;
   end;

   procedure insertline;
   begin
      if m.numlines>=maxlines then exit;
      insertabove;
      checkcx;
      partrefresh
   end;

   procedure breakline;
   begin
      if (m.numlines>=maxlines) or (cy=lines) or (cx=1) or (cx>length(m.text[curline])) then exit;
      insertabove;
      m.text[curline]:=copy(m.text[curline+1],1,cx-1);
      delete (m.text[curline+1],1,cx-1);
      partrefresh
   end;

   procedure joinlines;
   var n:integer;
   begin
      if curline=m.numlines then exit;
      if length(m.text[curline])+length(m.text[curline+1])>rightmargin then exit;
      m.text[curline]:=m.text[curline]+m.text[curline+1];
      n:=cx;
      inc(curline);
      deletethis;
      dec(curline);
      cx:=n;
      partrefresh;
   end;

   procedure userescape;
   var k:char;
   begin
      repeat
         getkey(k);
         case k of
            'A':upline;
            'B':downline;
            'C':fowrd;
            'D':back
         end;
      until (k<>'[') or (hangup);
   end;

   procedure deleteeol;
   begin
      cleareol;
      m.text[curline][0]:=chr(cx-1)
   end;

   {procedure tab;
   var nx,n,cnt:integer;
   begin
      nx:=((cx+8) and 248)+1;
      dec(n,nx-cx);
      if (n+length(m.text[curline])>=cols) or (nx>=cols) then exit;
      for cnt:=1 to n do insert (' ',m.text[curline],cx);
      updatetoeol;
      inc(cx,n);
      updatecpos
   end;}

   procedure processkey;
   var k:char;
   begin
      getkey(k);
      case k of
         #27:userescape;
         ^G,'':del;
         {#09:tab;}
         ' '..#126,#128..#199,#209..#255:letterkey (k);
         ^S:back;
         ^D:fowrd;
         ^H:bkspace;
         ^M:crlf;
         ^I: insertMode := not insertMode; { toggle insert }
         ^E:upline;
         ^X:downline;
         ^U:help;
         ^R:pageup;
         ^C:pagedn;
         ^A:wordleft;
         ^F:wordright;
         ^T:worddel;
         ^Q:beginline;
         ^W:endline;
         ^L:fullrefresh;
         ^Y:deleteline;
         ^N:insertline;
         ^B:breakline;
         ^P:deleteeol;
         ^J:joinlines;
      end;
   end;

var cnt:integer;
    mp:boolean;
begin
   ansireedit:=false;
   write_msg:=true;
   fillchar(M.Text, SizeOf(M.Text), #0);
   if (not cwindowon) then scrnsize:=24
   else scrnsize := 22;
   cols:=80;
   ansimode:=true;
   mp:=true;
   {for cnt:=1 to maxlines do m.text[cnt]:='';}
   m.numlines:=1;
   with m do begin
      sysMciSet('BN',baseName);
      sysMciSet('DT',dateLeft);
      sysMciSet('FM',from);
      sysMciSet('TL',msgTitle);
      sysMciSet('TO',toUser);
   end;
   startscr;
   lines:=scrnsize-hdrlen;
   topscrn:=scrnsize-lines+1;
   insertmode:=false;
   rightmargin:=cols-2;
   msgdone:=false;
   cx:=1;
   curline:=1;
   topline:=2-lines;
   computecy;
   updatecpos;
   fullrefresh;
   {if m.numlines>0 then fullrefresh
   else begin
      m.numlines:=1;
      fullrefresh;
   end;}
   while (not msgdone) and (not hangup) do processkey;
   sysResetMciList;
end;

end.
