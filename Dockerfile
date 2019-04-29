FROM rust:latest

# install node
RUN apt-get update
RUN apt-get -y install curl gnupg
RUN curl -sL https://deb.nodesource.com/setup_11.x  | bash -
RUN apt-get -y install nodejs

RUN npm install -g @angular/cli && npm install -g angular-cli-ghpages

RUN cargo install wasm-pack

WORKDIR /initial_build
COPY /rgb-solver /initial_build/rgb-solver/
COPY /wasm-typescript-definition /initial_build/wasm-typescript-definition/
RUN cd rgb-solver && cargo test --target x86_64-unknown-linux-gnu --lib --verbose
RUN cd rgb-solver && wasm-pack build --release
#&& cd pkg && npm link
#COPY /web_worker web_worker/
#RUN cd web_worker && npm link && npm link rgb-solver && npm run build
#COPY /grid-editor grid-editor/
#RUN cd grid-editor && npm ci && npm link web_worker && ng build --prod --base-href https://eric7237cire.github.io/rgb_delivery/
#
#RUN cd grid-editor && ngh --no-silent --dir dist/grid-editor
#COPY /grid-editor/package.json /grid-editor/package-lock.json ./gitroot/grid-editor/
#RUN cd /grid-editor/npm ci --prod

