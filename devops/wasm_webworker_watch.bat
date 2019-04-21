@echo OFF
SET DEVOPS_DIR=%~dp0
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\

cd /D "%RGB_SOLVER_DIR%"
rem --debug
cargo watch   -i .gitignore -i "pkg/*" -s "%DEVOPS_DIR%build-wasm.bat" -w ../wasm-typescript-definition -w . -w "../web_worker/worker"