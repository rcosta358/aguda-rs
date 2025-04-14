FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo install rustlr
RUN cd src && rustlr rustlr.grammar
RUN cargo build --release
CMD ["cargo", "run"]