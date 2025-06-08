{$A-,B+,D-,F+,I-,L-,N-,O-,Q+,R-,S-,V-}
Unit OvrLay;

Interface

Procedure OvrInitXMS;

Implementation

Uses
   Overlay,
   Dos;

Procedure OvrInitXMS; External;
{$L OVERXMS.OBJ }

Begin
End.

