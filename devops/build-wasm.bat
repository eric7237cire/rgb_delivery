@echo OFF
SET DEVOPS_DIR=%~dp0
SET WEB_WORKER_BUILD_DIR=%DEVOPS_DIR%..\web_worker\
SET GRID_EDITOR_BUILD_DIR=%DEVOPS_DIR%..\grid-editor\
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\


cd /D "%RGB_SOLVER_DIR%"


ECHO Building Web Assembly...

REM wasm-pack build --dev
wasm-pack build --release

if %errorlevel% neq 0 exit /b %errorlevel%

CD "%WEB_WORKER_BUILD_DIR%"
DEL "%GRID_EDITOR_BUILD_DIR%src\assets\*.wasm"
RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%dist"

ECHO Building Web Worker...

rem make sure we are using the local one
CALL npm link rgb-solver

CALL .\node_modules\.bin\webpack

if %errorlevel% neq 0 exit /b %errorlevel%

copy /Y "%WEB_WORKER_BUILD_DIR%dist\*"  "%GRID_EDITOR_BUILD_DIR%src\assets"
copy /Y "%WEB_WORKER_BUILD_DIR%worker\interface_types.ts" "%GRID_EDITOR_BUILD_DIR%src\app\typings\worker.ts

