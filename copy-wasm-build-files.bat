SET BUILD_WASM_DIR=%~dp0
copy /Y "%BUILD_WASM_DIR%web_worker_build\dist\*"  "%BUILD_WASM_DIR%grid-editor\src\assets"
copy /Y "%BUILD_WASM_DIR%web_worker_build\worker\interface_types.ts" "%BUILD_WASM_DIR%grid-editor\src\app\typings\worker.ts
