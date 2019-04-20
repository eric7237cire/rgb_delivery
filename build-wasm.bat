E:
cd E:\git\rgb_delivery\rgb-solver
wasm-pack build
cd E:\git\rgb_delivery\web_worker_build
CALL .\node_modules\.bin\webpack
copy /Y E:\git\rgb_delivery\web_worker_build\dist\*  E:\git\rgb_delivery\grid-editor\src\assets
