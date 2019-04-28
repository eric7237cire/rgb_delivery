rem @echo OFF
SET DEVOPS_DIR=%~dp0
SET WEB_WORKER_BUILD_DIR=%DEVOPS_DIR%..\web_worker\
SET GRID_EDITOR_BUILD_DIR=%DEVOPS_DIR%..\grid-editor\
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\

cd /D "%RGB_SOLVER_DIR%"
CD pkg
CALL npm link

CD "%WEB_WORKER_BUILD_DIR%"

rem make sure we are using the local one
CALL npm link
CALL npm link rgb-solver


CD "%GRID_EDITOR_BUILD_DIR%"
CALL npm link web_worker

CALL ng serve



