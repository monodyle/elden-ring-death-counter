@echo off
SETLOCAL

SET SAVE_FILE=""

IF NOT EXIST ".\watchexec.exe" (
    ECHO watchexec.exe not found, attempting to download and extract...
    curl -L -o watchexec.zip https://github.com/watchexec/watchexec/releases/download/v2.1.1/watchexec-2.1.1-x86_64-pc-windows-msvc.zip
    tar -xf watchexec.zip
	xcopy .\watchexec-2.1.1-x86_64-pc-windows-msvc\watchexec.exe .\
	rmdir /s /q .\watchexec-2.1.1-x86_64-pc-windows-msvc
	del .\watchexec.zip
)

IF NOT EXIST ".\elden-ring-death-counter.exe" (
    ECHO elden-ring-death-counter.exe not found, attempting to download and extract...
    curl -L -o elden-ring-death-counter.zip https://github.com/monodyle/elden-ring-death-counter/releases/download/v0.1.0/elden-ring-death-counter-0.1.0-x86_64-pc-windows-msvc.zip
    tar -xf elden-ring-death-counter.zip
	xcopy .\elden-ring-death-counter-0.1.0-x86_64-pc-windows-msvc\elden-ring-death-counter.exe .\
	rmdir /s /q .\elden-ring-death-counter-0.1.0-x86_64-pc-windows-msvc
	del .\elden-ring-death-counter.zip
)

IF NOT EXIST ".\prompt.ps1" (
	curl -L -o prompt.ps1 https://gist.githubusercontent.com/monodyle/eecadfca32c4a2d87c9338f588f85291/raw/dbb752a8e9b737448600c16bcb1f4dad5d5b0dd9/gistfile1.txt
)

:: Save File
FOR /F "tokens=* usebackq" %%a IN (`powershell -executionpolicy bypass -file prompt.ps1`) DO IF NOT "%%a" == "Cancel" IF not "%%a" == "OK" SET SAVE_FILE=%%a

IF NOT %SAVE_FILE%=="" (
	ECHO loaded save file %SAVE_FILE%
	.\watchexec.exe -i %SAVE_FILE% .\elden-ring-death-counter.exe %SAVE_FILE% -o .\OBS
)

ENDLOCAL
