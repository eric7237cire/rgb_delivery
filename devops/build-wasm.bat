@echo OFF
SET DEVOPS_DIR=%~dp0
SET WEB_WORKER_BUILD_DIR=%DEVOPS_DIR%..\web_worker\
SET GRID_EDITOR_BUILD_DIR=%DEVOPS_DIR%..\grid-editor\
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\
SET CALL_NPM_LINK=NO

cd /D "%RGB_SOLVER_DIR%"

ECHO Building Web Assembly...

SET TRAVIS_BUILD_NUMBER=%date% %time%
rem wasm-pack build --dev
wasm-pack build --release
if %errorlevel% neq 0 exit /b %errorlevel%

CD pkg
REM we want to use the one built, not the one from npm
REM Normally only needed once, but we do it each time as npm installs can unlink it
IF /I "%CALL_NPM_LINK%" == "YES" CALL npm link

CD "%WEB_WORKER_BUILD_DIR%"

rem RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%dist"
rem RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%lib"

ECHO Building Web Worker...

rem link will erase other links
IF /I "%CALL_NPM_LINK%" == "YES" CALL npm link

rem make sure we are using the local one
IF /I "%CALL_NPM_LINK%" == "YES" CALL npm link rgb-solver

CALL npm run build
if %errorlevel% neq 0 exit /b %errorlevel%

CALL npm run build-lib
if %errorlevel% neq 0 exit /b %errorlevel%


CD "%GRID_EDITOR_BUILD_DIR%"
IF /I "%CALL_NPM_LINK%" == "YES" CALL npm link web_worker



