FROM rust:latest

# install node
RUN apt-get update
RUN apt-get -y install curl gnupg
RUN curl -sL https://deb.nodesource.com/setup_11.x  | bash -
RUN apt-get -y install nodejs

RUN npm install -g @angular/cli && npm install -g angular-cli-ghpages

RUN cargo install wasm-pack

WORKDIR /initial_build
COPY . .
RUN cd rgb-solver && wasm-pack build --release && cd pkg && npm link
RUN cd web_worker && npm link && npm link rgb-solver && npm run build
RUN cd ../grid-editor && ng build --prod --base-href https://eric7237cire.github.io/rgb_delivery/

#COPY /grid-editor/package.json /grid-editor/package-lock.json ./gitroot/grid-editor/
#RUN cd /grid-editor/npm ci --prod

