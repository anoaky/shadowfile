FROM rustlang/rust:nightly-trixie

WORKDIR /usr/src/shadowfile
COPY . .

RUN cargo build
RUN cargo test

CMD ["bash"]