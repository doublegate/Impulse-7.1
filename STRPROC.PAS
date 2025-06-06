{$A+,B+,F+,I+,L+,O+,R-,S-,V-}
{ Commonly used string procedures }
unit strProc;
interface
function strAllCaps(s : string) : string;
function strStringToWord(s:string):word;
function strInsertComma(s : string) : string;
implementation

function strAllCaps(s : string) : string;
var cnt:byte;
begin
  for cnt:=1 to length(s) do s[cnt]:=upcase(s[cnt]);
  strAllCaps:=s;
end;

function strStringToWord(s:string):word;
var i    ,
    code : integer;
begin
   val(s,i,code);
   if code<>0 then i:=0;
   strStringToWord := i;
end;

function strInsertComma(s : string) : string;
begin
   if length(s) > 3 then insert(',',s,length(s)-2);
   if length(s) > 6 then insert(',',s,length(s)-6);
   if length(s) > 9 then insert(',',s,length(s)-10);
   strInsertComma := s;
end;

end.