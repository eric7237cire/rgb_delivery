SET BUILD_WASM_DIR=%~dp0
cd /D "%BUILD_WASM_DIR%"
CD "%BUILD_WASM_DIR%rgb-solver"
REM wasm-pack build --dev
wasm-pack build --release
CD "%BUILD_WASM_DIR%web_worker_build"
DEL "%BUILD_WASM_DIR%grid-editor\src\assets\*.wasm"
RMDIR /S /Q "%BUILD_WASM_DIR%web_worker_build\dist"
CALL .\node_modules\.bin\webpack
CALL "%BUILD_WASM_DIR%\copy-wasm-build-files.bat"
