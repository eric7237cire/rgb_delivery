SET DEVOPS_DIR=%~dp0
SET WEB_WORKER_BUILD_DIR=%DEVOPS_DIR%..\web_worker_build\
SET GRID_EDITOR_BUILD_DIR=%DEVOPS_DIR%..\grid-editor\

copy /Y "%WEB_WORKER_BUILD_DIR%dist\*"  "%GRID_EDITOR_BUILD_DIR%src\assets"
copy /Y "%WEB_WORKER_BUILD_DIR%worker\interface_types.ts" "%GRID_EDITOR_BUILD_DIR%src\app\typings\worker.ts
