FROM r1c4rdco5t4/rust-llvm:17.0.6

ENV GITLAB_TOKEN=glpat-T6jycrFCyDsXt9vwyano
WORKDIR /app
RUN git clone https://fc64371:${GITLAB_TOKEN}@git.alunos.di.fc.ul.pt/tcomp000/aguda-testing.git /app/aguda-testing
COPY . .
RUN cargo build --release
CMD ["cargo", "run"]
