SET DEVOPS_DIR=%~dp0
SET RGB_SOLVER_DIR=%~dp0..\rgb-solver\
cd /D "%RGB_SOLVER_DIR%"

REM With output
REM cargo test --target x86_64-pc-windows-msvc --lib -- --nocapture

cargo test --target x86_64-pc-windows-msvc --lib