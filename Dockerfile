FROM rust:latest

# install node
RUN apt-get update \
    && apt-get -y install curl gnupg git \
    && curl -sL https://deb.nodesource.com/setup_11.x  | bash - \
    && apt-get -y install nodejs cmake git \
    && npm install -g npm && npm install -g @angular/cli && npm install -g angular-cli-ghpages \
    && cargo install wasm-pack \
    && git clone --depth=1 https://github.com/WebAssembly/binaryen

WORKDIR /binaryen

RUN cmake -DCMAKE_BUILD_TYPE=Release . \
    && make -j$(nproc) \
    && make install

# Idea is to warm up the docker image by building the dependencies it will need
WORKDIR /initial_build
COPY /rgb-solver /initial_build/rgb-solver/
COPY /wasm-typescript-definition /initial_build/wasm-typescript-definition/
RUN cd rgb-solver && cargo test --target x86_64-unknown-linux-gnu --lib --verbose
RUN cd rgb-solver && wasm-pack build --release

RUN cd /initial_build/rgb-solver/pkg \
    && mv -v rgb_solver_bg.wasm rgb_solver_bg_non_opt.wasm \
    && wasm-opt -O4 rgb_solver_bg_non_opt.wasm -o rgb_solver_bg.wasm

# For the moment, not prefetching node modules

#&& cd pkg && npm link
#COPY /web_worker web_worker/
#RUN cd web_worker && npm link && npm link rgb-solver && npm run build
#COPY /grid-editor grid-editor/
#RUN cd grid-editor && npm ci && npm link web_worker && ng build --prod --base-href https://eric7237cire.github.io/rgb_delivery/
#
#RUN cd grid-editor && ngh --no-silent --dir dist/grid-editor
#COPY /grid-editor/package.json /grid-editor/package-lock.json ./gitroot/grid-editor/
#RUN cd /grid-editor/npm ci --prod

