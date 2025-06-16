# Impulse 7.1

## Directory structure

- `/source` = The Pascal source files
- `/output` = where TPU/EXE files go from Borland Pascal
- `/include` = dependencies (currently just checkpat.tpu)
- `/imp71rel` = the Impulse 7.1 official release files from imp71.zip
- `build` = new build 
- `/newins` = [WIP] New installer - not a priority for now

## Getting started

# automated build

- `build.sh` = this runs on linux (dosbox req'd) and will build+compile
- `.gitlab-ci.yml` = this runs on gitlab (docker + dosbox) to build+compile

# compiling by hand:
step 1 is build:
 
bpc -$G+ -B -Uf:\ -Ee:\ imp.pas

this makes all of the TPU files (units) , 96 of them
then compile:
bpc imp.pas
this creates IMP.OVR and IMP.EXE

we're figuring it out!

## Source

we started from http://software.bbsdocumentary.com/IBM/DOS/IMPULSE/imp71src.zip

