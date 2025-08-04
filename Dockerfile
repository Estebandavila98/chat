# Etapa 1: builder
FROM ubuntu:20.04 AS builder
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
      curl build-essential gcc libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: runtime mínimo
FROM ubuntu:20.04
WORKDIR /app
COPY --from=builder /app/target/release/server .
COPY --from=builder /app/target/release/client .

# Exponemos el puerto que usa el server
EXPOSE 9000

CMD ["./server"]