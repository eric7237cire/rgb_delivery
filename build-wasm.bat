SET BUILD_WASM_DIR=%~dp0
cd /D "BUILD_WASM_DIR"
CD "%BUILD_WASM_DIR%rgb-solver"
wasm-pack build
CD "%BUILD_WASM_DIR%web_worker_build"
CALL .\node_modules\.bin\webpack
CALL "%BUILD_WASM_DIR%\copy-wasm-build-files.bat"
