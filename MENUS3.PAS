{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
unit menus3;

interface

uses
   Crt,      Dos,      file0,    records,  common,   strproc,  common1;

procedure dochangemenu(var done:boolean; var newmenucmd:astr;c2:char; mstr:astr);

implementation

procedure dochangemenu(var done:boolean; var newmenucmd:astr;c2:char; mstr:astr);
var s,
    s1 : astr;
    oldmnu : string;
begin
    oldmnu := curmenu;
    case c2 of
       '^': begin
          s1 := mstr;
          if (pos(';',s1)<>0) then s1:=copy(s1,1,pos(';',s1)-1);
          if (mstr<>'') then begin
             s:=mstr;
             if (pos(';',s)<>0) then s:=copy(s,pos(';',s)+1,length(s));
             if (copy(s,1,1)='C') then menustackptr:=0;
             if (pos(';',s)=0) or (length(s)=1) then s:=''
             else s:=copy(s,pos(';',s)+1,length(s));
          end;
          if (s1<>'') then begin
             last_menu := curmenu;
             curmenu := status^.menupath+s1+'.mnu';
             done:=TRUE;
             if (s<>'') then newmenucmd:=strAllCaps(s);
             newmenutoload:=TRUE;
          end;
       end;
       '/': begin
          s1:=mstr;
          if (pos(';',s1)<>0) then s1:=copy(s1,1,pos(';',s1)-1);
          if ((mstr<>'') and (menustackptr<>8)) then begin
             s:=mstr;
             if (pos(';',s)<>0) then s:=copy(s,pos(';',s)+1,length(s));
             if (copy(s,1,1)='C') then menustackptr:=0;
             if (pos(';',s)=0) or (length(s)=1) then s:=''
             else s:=copy(s,pos(';',s)+1,length(s));
             inc(menustackptr);
             menustack[menustackptr]:=stripname(curmenu);
          end;
          if (s1<>'') then begin
             last_menu:=curmenu;
             curmenu:=status^.menupath+s1+'.mnu';
             done:=TRUE;
             if (s<>'') then newmenucmd:=strAllCaps(s);
             newmenutoload:=TRUE;
          end;
       end;
       '\':begin
          s:=mstr;
          if (menustackptr<>0) then begin
             last_menu:=curmenu;
             curmenu:=status^.menupath+menustack[menustackptr];
             dec(menustackptr);
          end;
          if (copy(s,1,1)='C') then menustackptr:=0;
          done:=TRUE;
          if (pos(';',s)=0) then s:=''
          else newmenucmd:=strAllCaps(copy(s,pos(';',s)+1,length(s)));
          newmenutoload:=TRUE;
       end;
    end;
    if not exist(curmenu) then begin
       sysoplog('"'+curmenu+'" is MISSING.');
       print('"'+curmenu+'" is MISSING.  Please inform SysOp.');
       print('Dropping back to previous menu...');
       pauseScr;
       curmenu := oldmnu;
    end;
end;

end.
