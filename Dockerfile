FROM rust:1.70.0
WORKDIR /starn
COPY . /starn
RUN cd server && cargo build --release && mkdir bin log && mv target/release/server bin/ && rm -rf src target Cargo.lock Cargo.toml README.md
CMD cd server && bin/server --release 
