# Guia de Deploy no Red Hat OpenShift

Este diretório contém todos os manifestos necessários para construir e executar o **Sistema de Inventário de Servidores DETRAN-BA** no Red Hat OpenShift, utilizando a estratégia **GitHub Docker BuildConfig** e **PostgreSQL** para o banco de produção.

---

## 📁 Estrutura dos Arquivos

1. `01-imagestream.yaml` - ImageStream para armazenar e rastrear a imagem da aplicação.
2. `02-buildconfig.yaml` - BuildConfig configurado com estratégia Docker apontando para o repositório GitHub.
3. `03-configmap.yaml` - Configurações não-sensíveis (`LDAP`, `LDAP_LOGIN`, `LDAP_GRUPOS`, `PORT`, `RUST_LOG`).
4. `04-secret.yaml` - Credenciais sensíveis (`LDAP_SENHA`, `DATABASE_URL` do PostgreSQL, `JWT_SECRET`).
5. `05-deployment.yaml` - Deployment da aplicação com probes de saúde (`/healthz`) e limites de recursos.
6. `06-service.yaml` - Service ClusterIP expondo a porta 8080.
7. `07-route.yaml` - Route com terminação TLS edge e redirecionamento HTTPS.
8. `08-postgres.yaml` - (Legado / Opcional) Manifesto para subir um PostgreSQL dedicado, caso não queira usar o banco compartilhado.

---

## 🗄️ Banco de Dados PostgreSQL (Existente no OpenShift)

A aplicação está configurada para conectar diretamente no banco PostgreSQL existente no namespace **`detran-apps-internos`**:
- **Namespace do Banco:** `detran-apps-internos`
- **Serviço:** `postgres` (endereço interno: `postgres.detran-apps-internos.svc.cluster.local:5432`)
- **Database:** `inventario`
- **Usuário:** `inventario`
- **Senha:** `pwdinventario`

As credenciais já estão configuradas no `04-secret.yaml` e as variáveis não-sensíveis no `03-configmap.yaml`.

---

## 🚀 Passo a Passo para Aplicação

### 1. Criar ou selecionar o projeto (namespace da aplicação)
```bash
oc project detran-inventario || oc new-project detran-inventario
```
*(Nota: Não é necessário rodar o `08-postgres.yaml`, pois o PostgreSQL já existe no namespace `detran-apps-internos`).*

### 3. Ajustar o Repositório no BuildConfig
Edite o arquivo `openshift/02-buildconfig.yaml` e informe a URL do seu repositório Git:
```yaml
git:
  uri: https://github.com/SEU_USUARIO_OU_ORG/InventarioServidores.git
  ref: main
```

### 4. Aplicar os Manifestos no OpenShift
```bash
oc apply -f openshift/01-imagestream.yaml
oc apply -f openshift/02-buildconfig.yaml
oc apply -f openshift/03-configmap.yaml
oc apply -f openshift/04-secret.yaml
oc apply -f openshift/05-deployment.yaml
oc apply -f openshift/06-service.yaml
oc apply -f openshift/07-route.yaml
```

### 5. Disparar a Construção (Build)
Para iniciar o build diretamente a partir do GitHub:
```bash
oc start-build inventario-servidores --follow
```

Ou, se preferir construir a partir do diretório local:
```bash
oc start-build inventario-servidores --from-dir=. --follow
```

### 6. Acompanhar a Inicialização e Obter a URL Pública
```bash
oc get pods -w
oc get route inventario-servidores -o jsonpath='{"https://"}{.spec.host}{"\n"}'
```
