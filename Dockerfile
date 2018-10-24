FROM rustlang/rust:nightly

RUN cargo install cargo-watch

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install

CMD ["rust101"]