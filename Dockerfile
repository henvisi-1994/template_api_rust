FROM rust:1.83-alpine AS builder

# Dependencias de compilación
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig

# Permite enlazado dinámico con OpenSSL (evita errores de linking estático)
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

# Cache de dependencias
COPY Cargo.toml Cargo.lock ./

# Crea src con main.rs dummy
RUN mkdir -p src && \
    echo 'fn main() { println!("Cache dummy"); }' > src/main.rs

# Build dummy (cachea dependencias)
RUN cargo build --release

# Elimina solo el main.rs dummy
RUN rm -f src/main.rs

# Copia el código real
COPY src/ ./src/

# Fuerza recompilación del binario real
RUN rm -rf target/release

# Build final
RUN cargo build --release

# Limpieza del cache de Cargo
RUN rm -rf /usr/local/cargo/registry /usr/local/cargo/git

# Etapa final: imagen mínima
FROM alpine:latest

# Dependencias runtime (OpenSSL + libgcc para unwinding)
RUN apk add --no-cache \
    ca-certificates \
    openssl \
    libgcc

WORKDIR /app

# Copia el binario real
COPY --from=builder /app/target/release/template_api_rust /usr/local/bin/template_api_rust

CMD ["/usr/local/bin/template_api_rust"]