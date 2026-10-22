# Sistema de Inventário de Servidores — DETRAN-BA

Aplicação Web de alta performance desenvolvida em **Rust** para gerenciamento e consulta do inventário de servidores e infraestrutura do **DETRAN-BA**, baseada nos dados consolidados da planilha `InventarioServidoress.xlsx`.

---

## 🌟 Principais Recursos

- **Backend em Rust com Axum 0.8 & Tokio**: Alta performance, baixo consumo de memória (~15MB RAM em execução) e máxima segurança.
- **Banco de Dados Híbrido**:
  - **Desenvolvimento**: SQLite3 embutido (`sqlite://servidores.db`).
  - **Produção**: PostgreSQL via pool assíncrono (`sqlx`).
  - **Auto-migração & Auto-seed**: Na primeira inicialização, o banco cria as tabelas automaticamente e popula todos os 155 servidores reais da planilha `InventarioServidoress.xlsx`.
- **Autenticação LDAP / Active Directory**:
  - Conexão direta com o servidor de domínio DETRAN (`10.21.246.3`).
  - Bind com conta de serviço e validação de credenciais de rede.
  - Verificação estrita de grupos autorizados:
    - `OSRH-DES-PROD`
    - `OSRH-INFRA`
    - `OSRH-DES-COD`
  - Fallback inteligente para ambiente de desenvolvimento local (`DEV_MODE`).
  - Sessão segura via cookie `HttpOnly` com token JWT assinado.
- **Interface Moderna com Tailwind CSS**:
  - Tema escuro com identidade visual do DETRAN-BA e design *glassmorphism*.
  - Dashboard de métricas em tempo real: Total de servidores, status Power On / Power Off, vCPUs somadas, total de memória RAM e total de disco.
  - Abas rápidas por ambiente: **Todos**, **Produção**, **Homologação** e **Desenvolvimento**.
  - **Exibição Estrita de 5 Colunas Configuráveis**:
    - A listagem principal exibe rigorosamente **5 colunas** por usuário, mantendo layout limpo e sem rolagem horizontal desnecessária.
    - O usuário pode personalizar livremente quais são os 5 campos exibidos e sua ordem de exibição através do modal **Personalizar Colunas** (com modelos rápidos em 1 clique: Padrão DETRAN, Suporte/Equipe, Sistemas & SO, Hardware/Recursos), com persistência automática no `localStorage`.
  - Filtros avançados por Equipe Responsável e Status Operacional.
  - Busca em tempo real por Nome, Hostname, IP, Serviço ou Sistema Operacional.
  - Modais interativos para visualização completa de detalhes e cadastro/edição (CRUD).
  - Exportação completa do inventário em **CSV (compatível com Excel com acentuação UTF-8)**.
- **Pronto para Red Hat OpenShift**:
  - Dockerfile multi-stage com imagem base Red Hat UBI 9 Minimal (`registry.access.redhat.com/ubi9/ubi-minimal`).
  - Suporte completo a UID arbitrário e boas práticas de segurança do OpenShift.
  - BuildConfig com estratégia GitHub Docker.
  - Manifestos completos: ImageStream, BuildConfig, ConfigMap, Secret, Deployment, Service, Route com TLS Edge e PostgreSQL.

---

## ⚙️ Variáveis de Ambiente (`.env`)

```env
# Banco de dados (Se postgres:// usa PostgreSQL, caso contrário usa SQLite)
DATABASE_URL=sqlite://servidores.db

# Configurações do Active Directory / LDAP DETRAN
LDAP=10.21.246.3
LDAP_PORT=389
LDAP_LOGIN=sas-admin@detrandom.tran
LDAP_SENHA=Detr@nBA2025!
LDAP_GRUPOS=OSRH-DES-PROD | OSRH-INFRA | OSRH-DES-COD

# Porta e Sessão
PORT=8080
JWT_SECRET=sua-chave-secreta-detran-2026
DEV_MODE=true
```

---

## 🚀 Como Executar Localmente (Ambiente Dev com SQLite)

### 1. Pré-requisitos
- Rust instalado (versão 1.80+): [rustup.rs](https://rustup.rs/)

### 2. Executar a Aplicação
```bash
cargo run
```

A aplicação iniciará na porta `8080`:
```
http://localhost:8080
```

Na primeira inicialização:
1. O banco `servidores.db` será criado automaticamente.
2. A migração das tabelas e índices será executada.
3. Os 155 servidores do DETRAN serão importados.

---

## 🚢 Deploy no Red Hat OpenShift

Todos os manifestos estão prontos no diretório `openshift/`:

```bash
# 1. Aplicar todos os manifestos
oc apply -f openshift/01-imagestream.yaml
oc apply -f openshift/02-buildconfig.yaml
oc apply -f openshift/03-configmap.yaml
oc apply -f openshift/04-secret.yaml
oc apply -f openshift/05-deployment.yaml
oc apply -f openshift/06-service.yaml
oc apply -f openshift/07-route.yaml

# 2. Iniciar a compilação no OpenShift a partir do GitHub
oc start-build inventario-servidores --follow

# 3. Obter a URL pública gerada pela Route
oc get route inventario-servidores
```

Consulte [openshift/README.md](openshift/README.md) para detalhes adicionais.

---

## 📡 Endpoints da API REST

| Método | Endpoint | Descrição |
|---|---|---|
| `GET` | `/healthz` | Health check para probes do OpenShift/Kubernetes |
| `POST` | `/api/auth/login` | Login com usuário e senha do LDAP |
| `POST` | `/api/auth/logout` | Encerramento da sessão |
| `GET` | `/api/auth/me` | Dados do usuário autenticado e seus grupos |
| `GET` | `/api/servidores` | Listagem com busca (`q`), filtros, ordenação e paginação |
| `GET` | `/api/servidores/:id` | Detalhes de um servidor |
| `POST` | `/api/servidores` | Cadastrar novo servidor |
| `PUT` | `/api/servidores/:id` | Atualizar dados de um servidor |
| `DELETE` | `/api/servidores/:id` | Excluir servidor do inventário |
| `GET` | `/api/stats` | Estatísticas consolidadas e métricas da infraestrutura |
| `GET` | `/api/export/csv` | Download do inventário completo em formato CSV |
