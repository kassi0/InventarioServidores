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
8. `08-postgres.yaml` - (Opcional) Banco PostgreSQL interno no cluster com PVC persistente de 10Gi.

---

## 🚀 Passo a Passo para Aplicação

### 1. Criar ou selecionar o projeto (namespace)
```bash
oc project detran-inventario || oc new-project detran-inventario
```

### 2. (Opcional) Subir o banco PostgreSQL no OpenShift
Se for utilizar o PostgreSQL dentro do OpenShift:
```bash
oc apply -f openshift/08-postgres.yaml
```

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
