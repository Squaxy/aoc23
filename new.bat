@ECHO off
IF "%~1" == "" GOTO NOINPUT

cargo new --vcs=none Task_%1
mkdir .\Task_%1\res
cat "" > .\Task_%1\res\input.txt
move .\Task_%1\src\main.rs .\Task_%1\src\part-1.rs

echo.>> .\Task_%1\Cargo.toml
echo [[bin]] >> .\Task_%1\Cargo.toml
echo name = "part-1" >> .\Task_%1\Cargo.toml
echo path = "src/part-1.rs" >> .\Task_%1\Cargo.toml
echo bench = true >> .\Task_%1\Cargo.toml

EXIT

:NOINPUT
echo Error: No input number
EXIT