FROM r1c4rdco5t4/rust-llvm:17.0.6

WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["cargo", "run"]
