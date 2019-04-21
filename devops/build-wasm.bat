
SET DEVOPS_DIR=%~dp0
SET WEB_WORKER_BUILD_DIR=%DEVOPS_DIR%..\web_worker_build\
SET GRID_EDITOR_BUILD_DIR=%DEVOPS_DIR%..\grid-editor\
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\


cd /D "%RGB_SOLVER_DIR%"
REM wasm-pack build --dev
wasm-pack build --release

CD "%WEB_WORKER_BUILD_DIR%"
DEL "%GRID_EDITOR_BUILD_DIR%src\assets\*.wasm"
RMDIR /S /Q "%WEB_WORKER_BUILD_DIR%dist"
CALL .\node_modules\.bin\webpack
CALL "%DEVOPS_DIR%copy-wasm-build-files.bat"
