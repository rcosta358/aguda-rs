FROM rust:latest

# install llvm
ENV LLVM_VERSION=17.0.6
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      wget xz-utils ca-certificates && \
    wget -O /tmp/llvm-${LLVM_VERSION}.tar.xz \
      https://github.com/llvm/llvm-project/releases/download/llvmorg-${LLVM_VERSION}/clang+llvm-${LLVM_VERSION}-x86_64-linux-gnu-ubuntu-22.04.tar.xz && \
    tar -xJf /tmp/llvm-${LLVM_VERSION}.tar.xz -C /usr/local --strip-components=1 && \
    rm /tmp/llvm-${LLVM_VERSION}.tar.xz

WORKDIR /app
RUN git clone https://fc64371:glpat-xeARu-wf4uD2-1xXrZJh@git.alunos.di.fc.ul.pt/tcomp000/aguda-testing.git /app/aguda-testing
COPY . .
RUN cargo build
CMD ["cargo", "run"]
