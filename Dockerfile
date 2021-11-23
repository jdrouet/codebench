FROM rust:bullseye AS builder

ENV USER=alice

WORKDIR /code
COPY Cargo.lock Cargo.toml /code/
COPY codebench-cli /code/codebench-cli
COPY codebench-criterion /code/codebench-criterion
COPY codebench-env /code/codebench-env
COPY codebench-reader /code/codebench-reader
COPY codebench-server /code/codebench-server

RUN cargo fetch

FROM builder AS cli

RUN cargo build -p codebench-cli --release

FROM builder AS server

RUN cargo build -p codebench-server --release

