@ECHO off
IF "%~1" == "" GOTO NOINPUT

cargo new --vcs=none Task_%1
EXIT

:NOINPUT
echo Error: No input number
EXIT