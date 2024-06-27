@echo off

REM This is a comment

set VAR=1

dir "C:\Users"

if ("VAR" == "1") (
    echo VAR is 1
) else (
    echo VAR is not 1
)