# ==========================================
# Multi-stage Dockerfile para OpenShift
# Sistema de Inventário de Servidores - DETRAN-BA
# ==========================================

# Estágio 1: Builder
FROM rust:1-bookworm AS builder

WORKDIR /app

# Cache de dependências criando um projeto dummy
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copia o código-fonte e dados de seed
COPY src ./src
COPY seed_servidores.json ./seed_servidores.json

# Compila o binário final otimizado
RUN touch src/main.rs && cargo build --release

# ==========================================
# Estágio 2: Imagem final UBI 9 Minimal (Padrão Red Hat / OpenShift)
FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

LABEL maintainer="DETRAN-BA" \
      io.k8s.description="Sistema de Inventário de Servidores DETRAN-BA" \
      io.k8s.display-name="Inventário de Servidores" \
      io.openshift.expose-services="8080:http" \
      io.openshift.tags="rust,detran,inventario,openshift"

WORKDIR /app

# Instala certificados CA e bibliotecas de execução
RUN microdnf update -y && \
    microdnf install -y ca-certificates libgcc tzdata && \
    microdnf clean all

# Copia o binário compilado
COPY --from=builder /app/target/release/inventario-servidores /app/inventario-servidores

# Configura permissões para usuário arbitrário do OpenShift (UID 1001 / GID 0)
RUN chown -R 1001:0 /app && \
    chmod -R g=u /app && \
    chmod +x /app/inventario-servidores

USER 1001

ENV PORT=8080 \
    RUST_LOG=info \
    DEV_MODE=false

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8080/healthz || exit 1

ENTRYPOINT ["/app/inventario-servidores"]
