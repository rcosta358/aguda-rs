FROM rust:latest
WORKDIR /app
RUN git clone https://fc64371:glpat-xeARu-wf4uD2-1xXrZJh@git.alunos.di.fc.ul.pt/tcomp000/aguda-testing.git /app/aguda-testing
COPY . .
RUN cargo build
CMD ["cargo", "run"]