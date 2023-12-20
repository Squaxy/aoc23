@ECHO off
IF "%~1" == "" GOTO NOINPUT

cargo new --vcs=none Task_%1
mkdir .\Task_%1\res
cat "" > .\Task_%1\res\input.txt
move .\Task_%1\src\main.rs .\Task_%1\src\part-1.rs
EXIT

:NOINPUT
echo Error: No input number
EXIT