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

rem RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%dist"
rem RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%lib"

ECHO Building Web Worker...

rem make sure we are using the local one
CALL npm link rgb-solver

CALL npm run build
CALL npm run build-lib

if %errorlevel% neq 0 exit /b %errorlevel%


