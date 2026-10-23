use axum::{
    response::{Html, IntoResponse},
};

pub async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}

pub async fn health_handler() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "inventario-servidores",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR" class="h-full bg-slate-950 text-slate-100">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Inventário de Servidores | DETRAN-BA</title>
    <!-- Tailwind CSS CDN -->
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            darkMode: 'class',
            theme: {
                extend: {
                    colors: {
                        detran: {
                            50: '#f0f7ff',
                            100: '#e0effe',
                            200: '#bae0fd',
                            500: '#0284c7',
                            600: '#0369a1',
                            700: '#075985',
                            800: '#0c4a6e',
                            900: '#082f49',
                        }
                    }
                }
            }
        }
    </script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
    <!-- Lucide Icons -->
    <script src="https://unpkg.com/lucide@latest"></script>
    <style>
        body { font-family: 'Inter', sans-serif; }
        h1, h2, h3, .font-heading { font-family: 'Outfit', sans-serif; }
        .font-mono { font-family: 'JetBrains Mono', monospace; }
        /* Custom scrollbars */
        ::-webkit-scrollbar { width: 6px; height: 6px; }
        ::-webkit-scrollbar-track { background: #0f172a; }
        ::-webkit-scrollbar-thumb { background: #334155; border-radius: 4px; }
        ::-webkit-scrollbar-thumb:hover { background: #475569; }
        .glass-panel {
            background: rgba(15, 23, 42, 0.75);
            backdrop-filter: blur(16px);
            border: 1px solid rgba(51, 65, 85, 0.5);
        }
    </style>
</head>
<body class="h-full flex flex-col bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">

    <!-- Top Navigation Bar -->
    <header class="glass-panel sticky top-0 z-40 border-b border-slate-800/80 px-6 py-3.5 flex items-center justify-between shadow-2xl">
        <div class="flex items-center space-x-4">
            <div class="h-10 w-10 rounded-xl bg-gradient-to-tr from-sky-600 via-blue-500 to-indigo-600 flex items-center justify-center shadow-lg shadow-sky-500/20 ring-1 ring-white/20">
                <i data-lucide="server" class="w-5 h-5 text-white"></i>
            </div>
            <div>
                <div class="flex items-center space-x-2">
                    <span class="font-heading font-bold text-lg tracking-tight text-white">DETRAN-BA</span>
                    <span class="text-xs px-2 py-0.5 rounded-full font-medium bg-sky-500/10 text-sky-400 border border-sky-500/30">Infraestrutura</span>
                    <span id="db-badge" class="text-xs px-2 py-0.5 rounded-full font-mono bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">Conectado</span>
                </div>
                <p class="text-xs text-slate-400">Inventário Geral de Servidores e Aplicações</p>
            </div>
        </div>

        <!-- User Controls / Auth -->
        <div class="flex items-center space-x-3">
            <button onclick="openAuditModal()" title="Visualizar Histórico e Registro de Auditoria" class="flex items-center space-x-1.5 bg-slate-900/90 hover:bg-slate-800 text-slate-300 hover:text-white text-xs px-3 py-1.5 rounded-full border border-slate-700/80 transition-colors shadow-sm">
                <i data-lucide="shield-alert" class="w-3.5 h-3.5 text-amber-400"></i>
                <span>Logs de Auditoria</span>
            </button>

            <div id="user-info-bar" class="hidden flex items-center space-x-3 bg-slate-900/80 border border-slate-800 rounded-full py-1.5 px-3.5 shadow-inner">
                <div class="w-7 h-7 rounded-full bg-gradient-to-tr from-cyan-500 to-blue-600 flex items-center justify-center text-xs font-bold text-white shadow">
                    <span id="user-avatar">U</span>
                </div>
                <div class="text-left">
                    <div id="user-name" class="text-xs font-semibold text-slate-200">Usuário</div>
                    <div id="user-groups" class="text-[10px] text-sky-400 font-mono">LDAP</div>
                </div>
                <button onclick="logout()" title="Sair do sistema" class="ml-2 text-slate-400 hover:text-red-400 transition-colors p-1 rounded-full hover:bg-red-500/10">
                    <i data-lucide="log-out" class="w-4 h-4"></i>
                </button>
            </div>

            <button id="btn-login-header" onclick="openLoginModal()" class="flex items-center space-x-2 bg-sky-600 hover:bg-sky-500 text-white text-xs font-semibold px-4 py-2 rounded-lg shadow-md shadow-sky-600/20 transition-all">
                <i data-lucide="lock" class="w-3.5 h-3.5"></i>
                <span>Acessar via LDAP</span>
            </button>
        </div>
    </header>

    <!-- Main Container -->
    <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">

        <!-- Metrics Dashboard -->
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4">
            <!-- Card 1: Total Servidores -->
            <div class="glass-panel p-4 rounded-2xl relative overflow-hidden group hover:border-sky-500/40 transition-all shadow-lg">
                <div class="absolute -right-3 -bottom-3 w-16 h-16 bg-sky-500/10 rounded-full blur-xl group-hover:bg-sky-500/20 transition-all"></div>
                <div class="flex items-center justify-between text-slate-400 mb-2">
                    <span class="text-xs font-medium uppercase tracking-wider">Total Servidores</span>
                    <i data-lucide="database" class="w-4 h-4 text-sky-400"></i>
                </div>
                <div id="stat-total" class="text-2xl font-bold font-heading text-white">--</div>
                <div class="text-[11px] text-slate-400 mt-1 flex items-center">
                    <span class="text-sky-400 font-medium">100%</span>&nbsp;ativos no parque
                </div>
            </div>

            <!-- Card 2: Power On / Off -->
            <div class="glass-panel p-4 rounded-2xl relative overflow-hidden group hover:border-emerald-500/40 transition-all shadow-lg">
                <div class="absolute -right-3 -bottom-3 w-16 h-16 bg-emerald-500/10 rounded-full blur-xl group-hover:bg-emerald-500/20 transition-all"></div>
                <div class="flex items-center justify-between text-slate-400 mb-2">
                    <span class="text-xs font-medium uppercase tracking-wider">Status Ativo</span>
                    <div class="flex items-center space-x-1">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                    </div>
                </div>
                <div class="flex items-baseline space-x-2">
                    <span id="stat-power-on" class="text-2xl font-bold font-heading text-emerald-400">--</span>
                    <span class="text-xs text-slate-500">/ <span id="stat-power-off">--</span> off</span>
                </div>
                <div class="text-[11px] text-slate-400 mt-1">Servidores ligados</div>
            </div>

            <!-- Card 3: vCPUs -->
            <div class="glass-panel p-4 rounded-2xl relative overflow-hidden group hover:border-indigo-500/40 transition-all shadow-lg">
                <div class="absolute -right-3 -bottom-3 w-16 h-16 bg-indigo-500/10 rounded-full blur-xl group-hover:bg-indigo-500/20 transition-all"></div>
                <div class="flex items-center justify-between text-slate-400 mb-2">
                    <span class="text-xs font-medium uppercase tracking-wider">Total vCPUs</span>
                    <i data-lucide="cpu" class="w-4 h-4 text-indigo-400"></i>
                </div>
                <div id="stat-cpus" class="text-2xl font-bold font-heading text-white">--</div>
                <div class="text-[11px] text-slate-400 mt-1">Capacidade de processamento</div>
            </div>

            <!-- Card 4: Memória RAM -->
            <div class="glass-panel p-4 rounded-2xl relative overflow-hidden group hover:border-purple-500/40 transition-all shadow-lg">
                <div class="absolute -right-3 -bottom-3 w-16 h-16 bg-purple-500/10 rounded-full blur-xl group-hover:bg-purple-500/20 transition-all"></div>
                <div class="flex items-center justify-between text-slate-400 mb-2">
                    <span class="text-xs font-medium uppercase tracking-wider">Memória Total</span>
                    <i data-lucide="zap" class="w-4 h-4 text-purple-400"></i>
                </div>
                <div class="flex items-baseline space-x-1">
                    <span id="stat-ram" class="text-2xl font-bold font-heading text-white">--</span>
                    <span class="text-xs text-slate-400 font-mono">GB</span>
                </div>
                <div class="text-[11px] text-slate-400 mt-1">RAM provisionada</div>
            </div>

            <!-- Card 5: Armazenamento -->
            <div class="glass-panel p-4 rounded-2xl relative overflow-hidden group hover:border-amber-500/40 transition-all shadow-lg col-span-2 sm:col-span-1">
                <div class="absolute -right-3 -bottom-3 w-16 h-16 bg-amber-500/10 rounded-full blur-xl group-hover:bg-amber-500/20 transition-all"></div>
                <div class="flex items-center justify-between text-slate-400 mb-2">
                    <span class="text-xs font-medium uppercase tracking-wider">Disco Total</span>
                    <i data-lucide="hard-drive" class="w-4 h-4 text-amber-400"></i>
                </div>
                <div class="flex items-baseline space-x-1">
                    <span id="stat-disk" class="text-2xl font-bold font-heading text-white">--</span>
                    <span class="text-xs text-slate-400 font-mono">GB</span>
                </div>
                <div class="text-[11px] text-slate-400 mt-1">Armazenamento somado</div>
            </div>
        </div>

        <!-- Filter and Search Header -->
        <div class="glass-panel p-4 rounded-2xl space-y-3 shadow-lg">
            <!-- Ambiente Tabs & Action Buttons -->
            <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3 pb-2 border-b border-slate-800">
                <div class="flex items-center space-x-1 overflow-x-auto pb-1 sm:pb-0" id="ambiente-tabs">
                    <button onclick="setAmbiente('')" data-amb="" class="amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold bg-sky-600 text-white shadow-sm transition-all whitespace-nowrap">
                        Todos (<span id="count-todos">0</span>)
                    </button>
                    <button onclick="setAmbiente('Produção')" data-amb="Produção" class="amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-800 transition-all whitespace-nowrap">
                        Produção (<span id="count-prod">0</span>)
                    </button>
                    <button onclick="setAmbiente('Homologação')" data-amb="Homologação" class="amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-800 transition-all whitespace-nowrap">
                        Homologação (<span id="count-hml">0</span>)
                    </button>
                    <button onclick="setAmbiente('Desenvolvimento')" data-amb="Desenvolvimento" class="amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-800 transition-all whitespace-nowrap">
                        Desenvolvimento (<span id="count-dev">0</span>)
                    </button>
                </div>

                <div class="flex items-center space-x-2">
                    <button onclick="openCreateModal()" class="flex items-center space-x-1.5 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold px-3.5 py-2 rounded-lg shadow-md shadow-emerald-600/20 transition-all">
                        <i data-lucide="plus" class="w-4 h-4"></i>
                        <span>Novo Servidor</span>
                    </button>

                    <!-- Column Customizer Button -->
                    <button type="button" onclick="openColumnsModal()" id="btn-toggle-columns" class="flex items-center space-x-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold px-3 py-2 rounded-lg border border-slate-700 transition-all shadow-sm">
                        <i data-lucide="sliders-horizontal" class="w-4 h-4 text-sky-400"></i>
                        <span>Colunas</span>
                        <span id="badge-columns-count" class="bg-sky-500/20 text-sky-300 text-[10px] px-1.5 py-0.5 rounded-full font-mono">5 / 5</span>
                    </button>

                    <a href="/api/export/csv" class="flex items-center space-x-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold px-3.5 py-2 rounded-lg border border-slate-700 transition-all">
                        <i data-lucide="download" class="w-4 h-4"></i>
                        <span>Exportar CSV</span>
                    </a>

                    <button onclick="loadServidores()" title="Atualizar dados" class="p-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg border border-slate-700 transition-all">
                        <i data-lucide="refresh-cw" class="w-4 h-4"></i>
                    </button>
                </div>
            </div>

            <!-- Search and Sub-filters -->
            <div class="grid grid-cols-1 sm:grid-cols-12 gap-3">
                <!-- Search Input -->
                <div class="sm:col-span-6 relative">
                    <i data-lucide="search" class="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2"></i>
                    <input type="search" id="input-search" name="search_servidores" autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false" oninput="debounceSearch()" placeholder="Buscar por Nome, Hostname, IP, Serviço ou Sistema Operacional..." class="w-full bg-slate-900/90 border border-slate-700/80 rounded-xl pl-10 pr-9 py-2 text-xs text-white placeholder-slate-400 focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500 transition-all">
                    <button type="button" id="btn-clear-search" onclick="clearSearch()" title="Limpar busca" class="hidden absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white p-1 rounded-md hover:bg-slate-800 transition-all">
                        <i data-lucide="x" class="w-3.5 h-3.5"></i>
                    </button>
                </div>

                <!-- Equipe Dropdown -->
                <div class="sm:col-span-3">
                    <select id="select-equipe" onchange="onFilterChange()" class="w-full bg-slate-900/90 border border-slate-700/80 rounded-xl px-3 py-2 text-xs text-slate-200 focus:outline-none focus:border-sky-500 transition-all">
                        <option value="">Todas as Equipes</option>
                        <option value="Windows">Windows</option>
                        <option value="Linux">Linux</option>
                        <option value="Banco de Dados">Banco de Dados</option>
                        <option value="Mainframe">Mainframe</option>
                        <option value="Segurança da Informação">Segurança da Informação</option>
                        <option value="Prodeb">Prodeb</option>
                    </select>
                </div>

                <!-- Status Dropdown -->
                <div class="sm:col-span-3">
                    <select id="select-state" onchange="onFilterChange()" class="w-full bg-slate-900/90 border border-slate-700/80 rounded-xl px-3 py-2 text-xs text-slate-200 focus:outline-none focus:border-sky-500 transition-all">
                        <option value="">Todos os Status</option>
                        <option value="POWER ON">POWER ON (Ligado)</option>
                        <option value="POWER OFF">POWER OFF (Desligado)</option>
                    </select>
                </div>
            </div>
        </div>

        <!-- Inventory Table Container -->
        <div class="glass-panel rounded-2xl overflow-hidden border border-slate-800 shadow-2xl">
            <div class="overflow-x-auto">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="bg-slate-900/90 border-b border-slate-800 text-[11px] uppercase tracking-wider text-slate-400 font-semibold select-none">
                        <tr id="table-header-row">
                            <!-- Renderizado dinamicamente via renderTableHeader() -->
                        </tr>
                    </thead>
                    <tbody id="table-body" class="divide-y divide-slate-800/60 font-normal">
                        <!-- Filled dynamically -->
                        <tr>
                            <td id="table-loading-td" colspan="5" class="py-12 text-center text-slate-500">
                                <div class="flex flex-col items-center justify-center space-y-2">
                                    <i data-lucide="loader-2" class="w-6 h-6 animate-spin text-sky-500"></i>
                                    <span>Carregando servidores do inventário...</span>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <!-- Pagination Bar -->
            <div class="bg-slate-900/60 px-4 py-3 border-t border-slate-800 flex items-center justify-between text-xs text-slate-400">
                <div id="pagination-info">Exibindo 0 de 0 servidores</div>
                <div class="flex items-center space-x-1" id="pagination-controls">
                    <!-- Page buttons filled dynamically -->
                </div>
            </div>
        </div>
    </main>

    <!-- Modal: Detalhes do Servidor & Credenciais Administrativas -->
    <div id="modal-details" class="fixed inset-0 z-50 hidden bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
        <div class="glass-panel bg-slate-900 border border-slate-700/80 rounded-2xl max-w-3xl w-full p-6 shadow-2xl relative space-y-4 max-h-[92vh] overflow-y-auto">
            <button onclick="closeModal('modal-details')" class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg">
                <i data-lucide="x" class="w-5 h-5"></i>
            </button>
            
            <!-- Header do Servidor -->
            <div class="flex items-center justify-between border-b border-slate-800 pb-3 pr-8">
                <div class="flex items-center space-x-3">
                    <div class="w-11 h-11 rounded-xl bg-gradient-to-tr from-sky-600/20 to-blue-500/20 text-sky-400 flex items-center justify-center border border-sky-500/30 shadow-inner">
                        <i data-lucide="server" class="w-6 h-6"></i>
                    </div>
                    <div>
                        <div class="flex items-center space-x-2">
                            <h3 id="det-nome" class="font-heading font-bold text-xl text-white">Nome do Servidor</h3>
                            <span id="det-item-badge" class="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400 border border-slate-700">Item #--</span>
                        </div>
                        <p id="det-hostname" class="text-xs text-sky-400 font-mono mt-0.5">hostname.detrandom.tran</p>
                    </div>
                </div>
                <div class="flex items-center space-x-2">
                    <span id="det-status-badge" class="px-2.5 py-1 rounded-full text-xs font-semibold">--</span>
                    <span id="det-ambiente-badge" class="px-2.5 py-1 rounded-full text-xs font-semibold">--</span>
                </div>
            </div>

            <!-- SEÇÃO 1: CAMPOS OCULTOS DA LISTA (NÃO VISÍVEIS NAS 5 COLUNAS ATUAIS) -->
            <div class="bg-gradient-to-br from-slate-950/90 to-slate-900/90 p-4 rounded-2xl border border-sky-500/30 space-y-2.5">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-2">
                        <div class="w-6 h-6 rounded-lg bg-sky-500/20 text-sky-400 flex items-center justify-center">
                            <i data-lucide="eye-off" class="w-3.5 h-3.5"></i>
                        </div>
                        <div>
                            <span class="text-xs font-semibold text-white">Campos Ocultos no Painel Principal</span>
                            <p class="text-[10px] text-slate-400">Dados deste servidor que não estão presentes nas suas 5 colunas configuradas no momento.</p>
                        </div>
                    </div>
                    <span class="text-[10px] px-2 py-0.5 rounded-full font-mono bg-sky-500/20 text-sky-300 border border-sky-500/30">Visão Complementar</span>
                </div>
                <div id="popup-hidden-fields" class="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs">
                    <!-- Preenchido dinamicamente via renderHiddenFieldsInPopup(s) -->
                </div>
            </div>

            <!-- SEÇÃO 2: CREDENCIAIS ADMINISTRATIVAS DO SERVIDOR -->
            <div class="bg-gradient-to-br from-amber-500/10 via-slate-950/80 to-slate-900/80 p-4 rounded-2xl border border-amber-500/30 space-y-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-2">
                        <div class="w-6 h-6 rounded-lg bg-amber-500/20 text-amber-400 flex items-center justify-center">
                            <i data-lucide="key-round" class="w-3.5 h-3.5"></i>
                        </div>
                        <div>
                            <span class="text-xs font-semibold text-amber-300">Credenciais Administrativas do Servidor</span>
                            <p class="text-[10px] text-slate-400">Login e senha de administração do host (exclusivo para autorizados).</p>
                        </div>
                    </div>
                    <span id="popup-cred-status" class="text-[10px] px-2 py-0.5 rounded-full font-mono bg-amber-500/20 text-amber-300 border border-amber-500/30">Seguro & Auditado</span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Login / Usuário Administrativo</label>
                        <div class="relative flex items-center">
                            <input type="text" id="popup-admin-user" placeholder="Ex: root, Administrator, sas-admin" class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-3 pr-9 py-2 text-white font-mono text-xs focus:outline-none focus:border-amber-500">
                            <button type="button" onclick="copyInputVal('popup-admin-user', 'Login')" title="Copiar Login" class="absolute right-2 text-slate-400 hover:text-white p-1">
                                <i data-lucide="copy" class="w-3.5 h-3.5"></i>
                            </button>
                        </div>
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Senha Administrativa</label>
                        <div class="relative flex items-center">
                            <input type="password" id="popup-admin-password" placeholder="••••••••••••" class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-3 pr-16 py-2 text-white font-mono text-xs focus:outline-none focus:border-amber-500">
                            <div class="absolute right-1.5 flex items-center space-x-0.5">
                                <button type="button" onclick="togglePasswordVisibility('popup-admin-password', this)" title="Mostrar / Ocultar" class="text-slate-400 hover:text-white p-1">
                                    <i data-lucide="eye" class="w-3.5 h-3.5"></i>
                                </button>
                                <button type="button" onclick="copyInputVal('popup-admin-password', 'Senha')" title="Copiar Senha" class="text-slate-400 hover:text-white p-1">
                                    <i data-lucide="copy" class="w-3.5 h-3.5"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="flex items-center justify-between pt-1">
                    <span id="popup-cred-feedback" class="text-[11px] text-emerald-400 font-medium hidden flex items-center space-x-1">
                        <i data-lucide="check-circle" class="w-3.5 h-3.5"></i>
                        <span>Credenciais salvas e registradas em auditoria!</span>
                    </span>
                    <div class="ml-auto">
                        <button type="button" onclick="saveAdminCredentialsFromPopup()" id="btn-save-credentials" class="px-3.5 py-1.5 bg-amber-600 hover:bg-amber-500 text-slate-950 font-bold text-xs rounded-lg shadow-sm shadow-amber-600/20 transition-all flex items-center space-x-1.5">
                            <i data-lucide="save" class="w-3.5 h-3.5"></i>
                            <span>Salvar Credenciais</span>
                        </button>
                    </div>
                </div>
            </div>

            <!-- SEÇÃO 3: VISÃO COMPLETA DE RECURSOS E HARDWARE -->
            <div class="bg-slate-950/60 p-4 rounded-2xl border border-slate-800 space-y-2.5">
                <span class="text-slate-400 text-[10px] uppercase font-semibold block">Configuração Completa do Equipamento:</span>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5 text-xs">
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Endereço IP</span>
                        <span id="det-ip" class="font-mono text-sky-400 font-semibold mt-0.5 inline-block cursor-pointer hover:underline" onclick="copyToClipboard(this.innerText)">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">vCPUs</span>
                        <span id="det-cpus" class="font-semibold text-white mt-0.5 inline-block">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Memória RAM</span>
                        <span id="det-memoria" class="font-semibold text-white mt-0.5 inline-block">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Disco</span>
                        <span id="det-disco" class="font-semibold text-white mt-0.5 inline-block">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800 col-span-2">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Sistema Operacional</span>
                        <span id="det-so" class="text-slate-300 mt-0.5 inline-block truncate">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800 col-span-2">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Serviço / Aplicação</span>
                        <span id="det-servico" class="text-slate-300 mt-0.5 inline-block truncate">--</span>
                    </div>
                    <div class="bg-slate-900/80 p-2.5 rounded-xl border border-slate-800 col-span-2 sm:col-span-4">
                        <span class="text-slate-500 block text-[10px] uppercase font-semibold">Observações</span>
                        <p id="det-obs" class="text-slate-400 mt-0.5">--</p>
                    </div>
                </div>
            </div>

            <!-- Rodapé de Ações do Modal -->
            <div class="flex items-center justify-between pt-2 border-t border-slate-800">
                <button type="button" onclick="editServerFromPopup()" class="px-3.5 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-lg flex items-center space-x-1.5 transition-colors">
                    <i data-lucide="edit-3" class="w-3.5 h-3.5 text-amber-400"></i>
                    <span>Editar Cadastro Completo</span>
                </button>
                <button onclick="closeModal('modal-details')" class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-lg">
                    Fechar
                </button>
            </div>
        </div>
    </div>

    <!-- Modal: Cadastrar / Editar Servidor -->
    <div id="modal-form" class="fixed inset-0 z-50 hidden bg-slate-950/80 backdrop-blur-sm flex items-center justify-center p-4">
        <div class="glass-panel bg-slate-900 border border-slate-700/80 rounded-2xl max-w-2xl w-full p-6 shadow-2xl relative space-y-4 max-h-[90vh] overflow-y-auto">
            <button onclick="closeModal('modal-form')" class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg">
                <i data-lucide="x" class="w-5 h-5"></i>
            </button>
            <div class="border-b border-slate-800 pb-3">
                <h3 id="form-modal-title" class="font-heading font-bold text-lg text-white">Novo Servidor</h3>
                <p class="text-xs text-slate-400">Preencha os campos para salvar no inventário</p>
            </div>

            <form id="server-form" onsubmit="handleFormSubmit(event)" class="space-y-4 text-xs">
                <input type="hidden" id="form-server-id">

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Nome do Servidor *</label>
                        <input type="text" id="form-nome" required placeholder="Ex: DETRAN-APP-01" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Hostname / FQDN *</label>
                        <input type="text" id="form-hostname" required placeholder="Ex: servdetran01.detrandom.tran" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Ambiente *</label>
                        <select id="form-ambiente" required class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                            <option value="Produção">Produção</option>
                            <option value="Homologação">Homologação</option>
                            <option value="Desenvolvimento">Desenvolvimento</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Status Operacional *</label>
                        <select id="form-state" required class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                            <option value="POWER ON">POWER ON (Ligado)</option>
                            <option value="POWER OFF">POWER OFF (Desligado)</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Equipe Responsável *</label>
                        <select id="form-equipe" required class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                            <option value="Windows">Windows</option>
                            <option value="Linux">Linux</option>
                            <option value="Banco de Dados">Banco de Dados</option>
                            <option value="Mainframe">Mainframe</option>
                            <option value="Segurança da Informação">Segurança da Informação</option>
                            <option value="Prodeb">Prodeb</option>
                        </select>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Endereço IP *</label>
                        <input type="text" id="form-ip" required placeholder="Ex: 10.21.246.50" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Serviço / Aplicação *</label>
                        <input type="text" id="form-servico" required placeholder="Ex: PRD - Portal DETRAN" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                </div>

                <div class="grid grid-cols-3 gap-4">
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">vCPUs</label>
                        <input type="text" id="form-cpus" placeholder="Ex: 8" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Memória RAM</label>
                        <input type="text" id="form-memoria" placeholder="Ex: 32 GB" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                    <div>
                        <label class="block text-slate-300 font-medium mb-1">Disco (GB)</label>
                        <input type="text" id="form-disco" placeholder="Ex: 250" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                </div>

                <div>
                    <label class="block text-slate-300 font-medium mb-1">Sistema Operacional</label>
                    <input type="text" id="form-so" placeholder="Ex: Red Hat Enterprise Linux 9 / Windows Server 2022" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                </div>

                <div>
                    <label class="block text-slate-300 font-medium mb-1">Observações</label>
                    <textarea id="form-obs" rows="2" placeholder="Informações complementares sobre backup, NAT, portas..." class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500"></textarea>
                </div>

                <!-- Credenciais Administrativas no Cadastro/Edição -->
                <div class="border-t border-slate-800 pt-3 space-y-2">
                    <div class="flex items-center space-x-2">
                        <i data-lucide="shield-check" class="w-4 h-4 text-amber-400"></i>
                        <span class="font-semibold text-white text-xs">Credenciais de Acesso Administrativo (Opcional)</span>
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-slate-300 font-medium mb-1">Login / Usuário Administrativo</label>
                            <input type="text" id="form-admin-user" placeholder="Ex: root, Administrator, sas-admin" class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-sky-500">
                        </div>
                        <div>
                            <label class="block text-slate-300 font-medium mb-1">Senha Administrativa</label>
                            <div class="relative">
                                <input type="password" id="form-admin-password" placeholder="••••••••••••" class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-3 pr-9 py-2 text-white focus:outline-none focus:border-sky-500">
                                <button type="button" onclick="togglePasswordVisibility('form-admin-password', this)" class="absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white">
                                    <i data-lucide="eye" class="w-4 h-4"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="flex justify-end space-x-2 pt-3 border-t border-slate-800">
                    <button type="button" onclick="closeModal('modal-form')" class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-lg">
                        Cancelar
                    </button>
                    <button type="submit" id="btn-save-server" class="px-4 py-2 bg-sky-600 hover:bg-sky-500 text-white text-xs font-semibold rounded-lg shadow-md shadow-sky-600/20">
                        Salvar Servidor
                    </button>
                </div>
            </form>
        </div>
    </div>

    <!-- Modal: Login LDAP DETRAN -->
    <div id="modal-login" class="fixed inset-0 z-50 hidden bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
        <div class="glass-panel bg-slate-900 border border-slate-700/80 rounded-2xl max-w-md w-full p-6 shadow-2xl relative space-y-5">
            <button onclick="closeModal('modal-login')" class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg">
                <i data-lucide="x" class="w-5 h-5"></i>
            </button>
            <div class="text-center space-y-2">
                <div class="inline-flex h-12 w-12 rounded-2xl bg-sky-500/10 text-sky-400 border border-sky-500/20 items-center justify-center shadow-inner">
                    <i data-lucide="shield-check" class="w-6 h-6"></i>
                </div>
                <h3 class="font-heading font-bold text-xl text-white">Autenticação LDAP DETRAN</h3>
                <p class="text-xs text-slate-400">Domínio <span class="font-mono text-sky-400">DETRANDOM.TRAN</span></p>
            </div>

            <!-- LDAP Grupos info badge -->
            <div class="bg-slate-950/60 p-3 rounded-xl border border-slate-800 text-[11px] space-y-1">
                <span class="text-slate-400 block font-semibold">Grupos com Acesso Permitido:</span>
                <div class="flex flex-wrap gap-1 mt-1">
                    <span class="px-2 py-0.5 rounded bg-sky-500/10 text-sky-400 font-mono text-[10px] border border-sky-500/20">OSRH-DES-PROD</span>
                    <span class="px-2 py-0.5 rounded bg-sky-500/10 text-sky-400 font-mono text-[10px] border border-sky-500/20">OSRH-INFRA</span>
                    <span class="px-2 py-0.5 rounded bg-sky-500/10 text-sky-400 font-mono text-[10px] border border-sky-500/20">OSRH-DES-COD</span>
                </div>
            </div>

            <div id="login-error" class="hidden p-3 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-xs"></div>

            <form onsubmit="handleLoginSubmit(event)" class="space-y-4 text-xs">
                <div>
                    <label class="block text-slate-300 font-medium mb-1">Usuário de Rede (sAMAccountName)</label>
                    <div class="relative">
                        <i data-lucide="user" class="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2"></i>
                        <input type="text" id="login-username" required placeholder="Ex: sas-admin ou seu login de rede" class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-9 pr-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                </div>

                <div>
                    <label class="block text-slate-300 font-medium mb-1">Senha de Rede</label>
                    <div class="relative">
                        <i data-lucide="key" class="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2"></i>
                        <input type="password" id="login-password" required placeholder="••••••••••••" class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-9 pr-3 py-2 text-white focus:outline-none focus:border-sky-500">
                    </div>
                </div>

                <button type="submit" id="btn-login-submit" class="w-full py-2.5 bg-gradient-to-r from-sky-600 to-blue-600 hover:from-sky-500 hover:to-blue-500 text-white font-semibold rounded-lg shadow-lg shadow-sky-600/25 transition-all flex items-center justify-center space-x-2">
                    <i data-lucide="log-in" class="w-4 h-4"></i>
                    <span>Entrar no Sistema</span>
                </button>
            </form>
        </div>
    </div>

    <!-- Modal: Configurar as 5 Colunas -->
    <div id="modal-columns" class="fixed inset-0 z-50 hidden bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
        <div class="glass-panel bg-slate-900 border border-slate-700/80 rounded-2xl max-w-xl w-full p-6 shadow-2xl relative space-y-4 max-h-[92vh] overflow-y-auto">
            <button onclick="applyAndCloseColumnsModal()" class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg">
                <i data-lucide="x" class="w-5 h-5"></i>
            </button>
            <div class="flex items-center justify-between border-b border-slate-800 pb-3 pr-8">
                <div class="flex items-center space-x-3">
                    <div class="w-10 h-10 rounded-xl bg-sky-500/20 text-sky-400 flex items-center justify-center border border-sky-500/30">
                        <i data-lucide="columns-3" class="w-5 h-5"></i>
                    </div>
                    <div>
                        <h3 class="font-heading font-bold text-lg text-white">Configurar as 5 Colunas</h3>
                        <p class="text-xs text-slate-400">A tabela exibe rigorosamente 5 colunas por usuário.</p>
                    </div>
                </div>
                <span id="modal-columns-counter" class="bg-sky-500/20 text-sky-300 border border-sky-500/30 text-xs font-mono font-semibold px-2.5 py-1 rounded-full whitespace-nowrap">
                    5 / 5 Selecionadas
                </span>
            </div>

            <!-- Modelos rápidos (Presets) -->
            <div>
                <span class="text-slate-400 text-[10px] uppercase font-semibold block mb-1.5">Modelos Rápidos (5 Colunas):</span>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-1.5 text-xs">
                    <button type="button" onclick="applyColumnPreset('padrao')" class="text-left p-2 rounded-xl bg-slate-950/80 hover:bg-sky-500/10 hover:border-sky-500/40 border border-slate-800 text-slate-200 transition-all">
                        <div class="font-semibold text-sky-400 text-[11px]">⭐ Padrão DETRAN</div>
                        <div class="text-[10px] text-slate-400 truncate mt-0.5">Status até Ambiente</div>
                    </button>
                    <button type="button" onclick="applyColumnPreset('equipe')" class="text-left p-2 rounded-xl bg-slate-950/80 hover:bg-sky-500/10 hover:border-sky-500/40 border border-slate-800 text-slate-200 transition-all">
                        <div class="font-semibold text-emerald-400 text-[11px]">👥 Suporte / Equipe</div>
                        <div class="text-[10px] text-slate-400 truncate mt-0.5">Status, IP, Equipe, Ações</div>
                    </button>
                    <button type="button" onclick="applyColumnPreset('servico')" class="text-left p-2 rounded-xl bg-slate-950/80 hover:bg-sky-500/10 hover:border-sky-500/40 border border-slate-800 text-slate-200 transition-all">
                        <div class="font-semibold text-amber-400 text-[11px]">⚙️ Sistemas & SO</div>
                        <div class="text-[10px] text-slate-400 truncate mt-0.5">Nome, IP, Serviço, SO</div>
                    </button>
                    <button type="button" onclick="applyColumnPreset('recursos')" class="text-left p-2 rounded-xl bg-slate-950/80 hover:bg-sky-500/10 hover:border-sky-500/40 border border-slate-800 text-slate-200 transition-all">
                        <div class="font-semibold text-purple-400 text-[11px]">⚡ Hardware</div>
                        <div class="text-[10px] text-slate-400 truncate mt-0.5">Nome, IP, Amb, Recursos</div>
                    </button>
                </div>
            </div>

            <!-- Seleção de Colunas disponíveis -->
            <div class="space-y-2">
                <div class="flex items-center justify-between">
                    <span class="text-slate-400 text-[10px] uppercase font-semibold">Escolha os 5 campos desejados:</span>
                    <span id="columns-warning-text" class="text-[11px] text-amber-400 hidden font-medium"></span>
                </div>
                <div id="modal-columns-grid" class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                    <!-- Cards dinâmicos gerados em renderColumnsModalContent() -->
                </div>
            </div>

            <!-- Ordenação das 5 posições -->
            <div class="space-y-2 pt-2 border-t border-slate-800">
                <span class="text-slate-400 text-[10px] uppercase font-semibold block">Ordem das Colunas na Tabela (Posição 1ª à 5ª):</span>
                <div id="modal-columns-slots" class="space-y-1.5">
                    <!-- Dropdowns das 5 posições -->
                </div>
            </div>

            <div class="flex items-center justify-between pt-3 border-t border-slate-800">
                <button type="button" onclick="resetDefaultColumns()" class="text-slate-400 hover:text-slate-200 text-xs flex items-center space-x-1 py-1.5 px-3 rounded-lg hover:bg-slate-800 transition-colors">
                    <i data-lucide="rotate-ccw" class="w-3.5 h-3.5"></i>
                    <span>Restaurar Padrão</span>
                </button>
                <button type="button" onclick="applyAndCloseColumnsModal()" class="px-5 py-2 bg-sky-600 hover:bg-sky-500 text-white text-xs font-semibold rounded-lg shadow-md shadow-sky-600/20 transition-all flex items-center space-x-1.5">
                    <i data-lucide="check" class="w-3.5 h-3.5"></i>
                    <span>Aplicar e Fechar</span>
                </button>
            </div>
        </div>
    </div>

    <!-- Modal: Logs de Auditoria -->
    <div id="modal-audit-logs" class="fixed inset-0 z-50 hidden bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
        <div class="glass-panel bg-slate-900 border border-slate-700/80 rounded-2xl max-w-4xl w-full p-6 shadow-2xl relative space-y-4 max-h-[90vh] overflow-y-auto">
            <button onclick="closeModal('modal-audit-logs')" class="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg">
                <i data-lucide="x" class="w-5 h-5"></i>
            </button>
            <div class="flex items-center justify-between border-b border-slate-800 pb-3 pr-8">
                <div class="flex items-center space-x-3">
                    <div class="w-10 h-10 rounded-xl bg-amber-500/20 text-amber-400 flex items-center justify-center border border-amber-500/30">
                        <i data-lucide="shield-alert" class="w-5 h-5"></i>
                    </div>
                    <div>
                        <h3 class="font-heading font-bold text-lg text-white">Registro e Logs de Auditoria</h3>
                        <p class="text-xs text-slate-400">Histórico de ações (criação, edição, exclusão e credenciais) com usuário e timestamp.</p>
                    </div>
                </div>
                <button type="button" onclick="loadAuditLogs()" class="p-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-xs flex items-center space-x-1">
                    <i data-lucide="refresh-cw" class="w-3.5 h-3.5"></i>
                    <span>Atualizar</span>
                </button>
            </div>

            <div class="overflow-x-auto rounded-xl border border-slate-800">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="bg-slate-950/80 text-[10px] uppercase font-semibold text-slate-400 border-b border-slate-800">
                        <tr>
                            <th class="py-2.5 px-3">Data / Hora</th>
                            <th class="py-2.5 px-3">Usuário</th>
                            <th class="py-2.5 px-3">Ação</th>
                            <th class="py-2.5 px-3">Servidor</th>
                            <th class="py-2.5 px-3">Detalhes</th>
                        </tr>
                    </thead>
                    <tbody id="audit-table-body" class="divide-y divide-slate-800/60 font-mono text-[11px]">
                        <tr>
                            <td colspan="5" class="py-8 text-center text-slate-500">Carregando logs...</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div class="flex justify-end pt-2 border-t border-slate-800">
                <button type="button" onclick="closeModal('modal-audit-logs')" class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-lg">
                    Fechar
                </button>
            </div>
        </div>
    </div>

    <!-- Frontend Script Logic -->
    <script>
        let currentQuery = {
            q: '',
            ambiente: '',
            equipe: '',
            state: '',
            page: 1,
            per_page: 20,
            sort_by: 'id',
            sort_dir: 'asc'
        };

        let currentUser = null;
        let searchTimeout = null;
        let lastLoadedData = null;
        let currentPopupServer = null;

        // Dicionário completo de colunas disponíveis
        const COLUMN_DEFS = {
            status: {
                id: 'status',
                label: 'Status',
                renderTh: () => '<th class="py-3 px-4 w-24 text-center font-semibold">Status</th>',
                renderTd: (s) => {
                    const isOnline = (s.state || '').toUpperCase().includes('ON');
                    const stateBadge = isOnline
                        ? `<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"><span class="w-1.5 h-1.5 rounded-full bg-emerald-400 mr-1.5 animate-pulse"></span>ON</span>`
                        : `<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-medium bg-red-500/10 text-red-400 border border-red-500/20"><span class="w-1.5 h-1.5 rounded-full bg-red-400 mr-1.5"></span>OFF</span>`;
                    return `<td class="py-3 px-4 text-center">${stateBadge}</td>`;
                }
            },
            nome: {
                id: 'nome',
                label: 'Nome do Servidor',
                renderTh: () => `<th class="py-3 px-4 cursor-pointer hover:text-white" onclick="changeSort('nome')"><div class="flex items-center space-x-1"><span>Nome do Servidor</span><i data-lucide="arrow-up-down" class="w-3 h-3"></i></div></th>`,
                renderTd: (s) => `
                    <td class="py-3 px-4">
                        <button type="button" onclick="viewDetails(${s.id})" title="Clique para ver campos ocultos e credenciais" class="text-left group flex flex-col focus:outline-none">
                            <span class="font-semibold text-white group-hover:text-sky-400 group-hover:underline transition-colors flex items-center space-x-1">
                                <span>${escapeHtml(s.nome)}</span>
                                <i data-lucide="external-link" class="w-3 h-3 text-slate-500 group-hover:text-sky-400 transition-colors"></i>
                            </span>
                            <span class="text-[10px] text-slate-500">Item #${s.item || s.id}</span>
                        </button>
                    </td>`
            },
            hostname: {
                id: 'hostname',
                label: 'Hostname',
                renderTh: () => `<th class="py-3 px-4 cursor-pointer hover:text-white" onclick="changeSort('hostname')"><div class="flex items-center space-x-1"><span>Hostname</span><i data-lucide="arrow-up-down" class="w-3 h-3"></i></div></th>`,
                renderTd: (s) => `<td class="py-3 px-4 font-mono text-[11px] text-slate-300">${escapeHtml(s.hostname)}</td>`
            },
            ip: {
                id: 'ip',
                label: 'Endereço IP',
                renderTh: () => `<th class="py-3 px-4 font-semibold">Endereço IP</th>`,
                renderTd: (s) => `<td class="py-3 px-4 font-mono text-[11px] text-sky-400"><span class="cursor-pointer hover:underline" onclick="copyToClipboard('${escapeHtml(s.ip)}')" title="Clique para copiar">${escapeHtml(s.ip)}</span></td>`
            },
            ambiente: {
                id: 'ambiente',
                label: 'Ambiente',
                renderTh: () => `<th class="py-3 px-4 cursor-pointer hover:text-white" onclick="changeSort('ambiente')"><div class="flex items-center space-x-1"><span>Ambiente</span><i data-lucide="arrow-up-down" class="w-3 h-3"></i></div></th>`,
                renderTd: (s) => {
                    let ambColor = 'bg-slate-800 text-slate-300 border-slate-700';
                    if (s.ambiente === 'Produção') ambColor = 'bg-blue-500/10 text-blue-400 border-blue-500/20';
                    else if (s.ambiente === 'Homologação') ambColor = 'bg-purple-500/10 text-purple-400 border-purple-500/20';
                    else if (s.ambiente === 'Desenvolvimento') ambColor = 'bg-amber-500/10 text-amber-400 border-amber-500/20';
                    return `<td class="py-3 px-4"><span class="px-2 py-0.5 rounded text-[10px] font-medium border ${ambColor}">${escapeHtml(s.ambiente)}</span></td>`;
                }
            },
            equipe: {
                id: 'equipe',
                label: 'Equipe Responsável',
                renderTh: () => `<th class="py-3 px-4 cursor-pointer hover:text-white" onclick="changeSort('equipe_resp')"><div class="flex items-center space-x-1"><span>Equipe</span><i data-lucide="arrow-up-down" class="w-3 h-3"></i></div></th>`,
                renderTd: (s) => `<td class="py-3 px-4 font-medium text-slate-200">${escapeHtml(s.equipe_resp || '--')}</td>`
            },
            servico: {
                id: 'servico',
                label: 'Serviço / Aplicação',
                renderTh: () => `<th class="py-3 px-4 font-semibold">Serviço / Aplicação</th>`,
                renderTd: (s) => `<td class="py-3 px-4 max-w-xs truncate text-slate-300" title="${escapeHtml(s.servico || '')}">${escapeHtml(s.servico || '--')}</td>`
            },
            recursos: {
                id: 'recursos',
                label: 'Recursos (Hardware)',
                renderTh: () => `<th class="py-3 px-4 font-semibold">Recursos</th>`,
                renderTd: (s) => `<td class="py-3 px-4 font-mono text-[11px] text-slate-400">${s.cpus ? `${escapeHtml(s.cpus)}c / ` : ''}${s.memoria_gb ? `${escapeHtml(s.memoria_gb)} / ` : ''}${s.disco_gb ? `${escapeHtml(s.disco_gb)}GB` : ''}</td>`
            },
            so: {
                id: 'so',
                label: 'Sistema Operacional',
                renderTh: () => `<th class="py-3 px-4 font-semibold">Sistema Operacional</th>`,
                renderTd: (s) => `<td class="py-3 px-4 text-slate-400 max-w-[150px] truncate" title="${escapeHtml(s.sistema_operacional || '')}">${escapeHtml(s.sistema_operacional || '--')}</td>`
            },
            acoes: {
                id: 'acoes',
                label: 'Ações (CRUD)',
                renderTh: () => `<th class="py-3 px-4 text-right font-semibold">Ações</th>`,
                renderTd: (s) => `
                    <td class="py-3 px-4 text-right">
                        <div class="flex items-center justify-end space-x-1">
                            <button onclick="viewDetails(${s.id})" title="Visualizar Detalhes" class="p-1.5 text-slate-400 hover:text-sky-400 hover:bg-sky-500/10 rounded-lg transition-colors">
                                <i data-lucide="eye" class="w-4 h-4"></i>
                            </button>
                            <button onclick="editServer(${s.id})" title="Editar" class="p-1.5 text-slate-400 hover:text-amber-400 hover:bg-amber-500/10 rounded-lg transition-colors">
                                <i data-lucide="edit-3" class="w-4 h-4"></i>
                            </button>
                            <button onclick="deleteServer(${s.id}, '${escapeHtml(s.nome)}')" title="Excluir" class="p-1.5 text-slate-400 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors">
                                <i data-lucide="trash-2" class="w-4 h-4"></i>
                            </button>
                        </div>
                    </td>`
            }
        };

        // Lista de todas as 10 colunas disponíveis no sistema
        const AVAILABLE_COLUMNS = [
            { id: 'status', label: 'Status Operacional', desc: 'Indicador POWER ON / OFF' },
            { id: 'nome', label: 'Nome do Servidor', desc: 'Identificação e número do item' },
            { id: 'hostname', label: 'Hostname / FQDN', desc: 'Nome de rede do host' },
            { id: 'ip', label: 'Endereço IP', desc: 'IPv4 principal (com cópia rápida)' },
            { id: 'ambiente', label: 'Ambiente', desc: 'Produção, Homologação, Dev' },
            { id: 'equipe', label: 'Equipe Responsável', desc: 'Windows, Linux, Banco de Dados, etc.' },
            { id: 'servico', label: 'Serviço / Aplicação', desc: 'Finalidade do servidor' },
            { id: 'recursos', label: 'Recursos (Hardware)', desc: 'vCPUs, RAM e Disco GB' },
            { id: 'so', label: 'Sistema Operacional', desc: 'Versão do SO instalado' },
            { id: 'acoes', label: 'Ações (CRUD)', desc: 'Visualizar, Editar e Excluir' }
        ];

        // A listagem exibe RIGOROSAMENTE 5 colunas configuráveis por usuário
        // Padrão DETRAN: Status, Nome, Hostname, IP, Ambiente
        let visibleColumns = ['status', 'nome', 'hostname', 'ip', 'ambiente'];

        // Init on page load
        document.addEventListener('DOMContentLoaded', async () => {
            initVisibleColumns();
            lucide.createIcons();
            const searchInput = document.getElementById('input-search');
            if (searchInput) searchInput.value = '';
            currentQuery.q = '';
            updateClearSearchButton();
            await checkAuth();
            await loadStats();
            await loadServidores();
        });

        function initVisibleColumns() {
            try {
                // Limpeza preventiva de chaves legadas
                localStorage.removeItem('detran_columns');
                localStorage.removeItem('detran_visible_columns');

                const saved = localStorage.getItem('detran_5_columns');
                if (saved) {
                    const parsed = JSON.parse(saved);
                    const validIds = AVAILABLE_COLUMNS.map(c => c.id);
                    if (Array.isArray(parsed) && parsed.length >= 1 && parsed.length <= 5 && parsed.every(id => validIds.includes(id))) {
                        visibleColumns = parsed;
                    } else {
                        visibleColumns = ['status', 'nome', 'hostname', 'ip', 'ambiente'];
                        localStorage.setItem('detran_5_columns', JSON.stringify(visibleColumns));
                    }
                }
            } catch (e) {
                console.error('Erro ao ler configuração de colunas:', e);
                visibleColumns = ['status', 'nome', 'hostname', 'ip', 'ambiente'];
            }

            saveVisibleColumns();
            renderTableHeader();
            updateToolbarColumnBadge();
        }

        function saveVisibleColumns() {
            try {
                if (visibleColumns.length > 5) {
                    visibleColumns = visibleColumns.slice(0, 5);
                }
                localStorage.setItem('detran_5_columns', JSON.stringify(visibleColumns));
            } catch (e) {
                console.error(e);
            }
            updateToolbarColumnBadge();
        }

        function updateToolbarColumnBadge() {
            const badge = document.getElementById('badge-columns-count');
            if (badge) {
                badge.innerText = `${visibleColumns.length} / 5`;
            }
        }

        function openColumnsModal() {
            renderColumnsModalContent();
            openModal('modal-columns');
        }

        function toggleColumnSelection(colId) {
            const warningEl = document.getElementById('columns-warning-text');
            if (warningEl) warningEl.classList.add('hidden');

            const idx = visibleColumns.indexOf(colId);

            if (idx !== -1) {
                // Já está selecionada - desmarcar (mínimo 1)
                if (visibleColumns.length <= 1) {
                    alert('A tabela precisa de pelo menos 1 coluna selecionada.');
                    return;
                }
                visibleColumns.splice(idx, 1);
            } else {
                // Não está selecionada - tentar adicionar
                if (visibleColumns.length >= 5) {
                    if (warningEl) {
                        warningEl.innerHTML = '<span class="inline-flex items-center text-amber-400 font-semibold"><i data-lucide="alert-triangle" class="w-3.5 h-3.5 mr-1"></i> Limite de 5 atingido! Desmarque uma coluna acima para poder adicionar esta.</span>';
                        warningEl.classList.remove('hidden');
                        lucide.createIcons();
                    }
                    return;
                }
                visibleColumns.push(colId);
            }

            saveVisibleColumns();
            renderTableHeader();
            if (lastLoadedData) renderTable(lastLoadedData);
            renderColumnsModalContent();
        }

        function removeColumnAt(index) {
            if (visibleColumns.length <= 1) {
                alert('A tabela precisa de pelo menos 1 coluna selecionada.');
                return;
            }
            if (index >= 0 && index < visibleColumns.length) {
                visibleColumns.splice(index, 1);
                saveVisibleColumns();
                renderTableHeader();
                if (lastLoadedData) renderTable(lastLoadedData);
                renderColumnsModalContent();
            }
        }

        function addColumnToSlot(slotIdx, colId) {
            if (!colId) return;
            if (visibleColumns.includes(colId)) return;
            if (visibleColumns.length < 5) {
                visibleColumns.push(colId);
                saveVisibleColumns();
                renderTableHeader();
                if (lastLoadedData) renderTable(lastLoadedData);
                renderColumnsModalContent();
            }
        }

        function applyAndCloseColumnsModal() {
            // Se o usuário fechou o modal com menos de 5 colunas, preenche as faltantes até 5 para manter o painel com 5 colunas
            if (visibleColumns.length < 5) {
                const defaults = ['status', 'nome', 'hostname', 'ip', 'ambiente', 'equipe', 'servico', 'recursos', 'so', 'acoes'];
                for (const d of defaults) {
                    if (!visibleColumns.includes(d)) {
                        visibleColumns.push(d);
                        if (visibleColumns.length === 5) break;
                    }
                }
                saveVisibleColumns();
                renderTableHeader();
                if (lastLoadedData) renderTable(lastLoadedData);
            }
            closeModal('modal-columns');
        }

        function updateColumnSlot(slotIdx, newColId) {
            // Se a nova coluna já estiver selecionada em outro slot, faz swap inteligente
            const existingIdx = visibleColumns.indexOf(newColId);
            if (existingIdx !== -1 && existingIdx !== slotIdx) {
                const temp = visibleColumns[slotIdx];
                visibleColumns[slotIdx] = newColId;
                visibleColumns[existingIdx] = temp;
            } else {
                visibleColumns[slotIdx] = newColId;
            }

            saveVisibleColumns();
            renderTableHeader();
            if (lastLoadedData) renderTable(lastLoadedData);
            renderColumnsModalContent();
        }

        function applyColumnPreset(preset) {
            if (preset === 'padrao') {
                visibleColumns = ['status', 'nome', 'hostname', 'ip', 'ambiente'];
            } else if (preset === 'equipe') {
                visibleColumns = ['status', 'nome', 'ip', 'equipe', 'acoes'];
            } else if (preset === 'servico') {
                visibleColumns = ['nome', 'hostname', 'ip', 'servico', 'so'];
            } else if (preset === 'recursos') {
                visibleColumns = ['nome', 'hostname', 'ip', 'ambiente', 'recursos'];
            }
            saveVisibleColumns();
            renderTableHeader();
            if (lastLoadedData) renderTable(lastLoadedData);
            renderColumnsModalContent();
        }

        function resetDefaultColumns() {
            visibleColumns = ['status', 'nome', 'hostname', 'ip', 'ambiente'];
            saveVisibleColumns();
            renderTableHeader();
            if (lastLoadedData) renderTable(lastLoadedData);
            renderColumnsModalContent();
        }

        function renderColumnsModalContent() {
            updateToolbarColumnBadge();

            const modalCounter = document.getElementById('modal-columns-counter');
            if (modalCounter) {
                modalCounter.innerText = `${visibleColumns.length} / 5 Selecionadas`;
                if (visibleColumns.length === 5) {
                    modalCounter.className = 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-xs font-mono font-semibold px-2.5 py-1 rounded-full whitespace-nowrap';
                } else {
                    modalCounter.className = 'bg-amber-500/20 text-amber-400 border border-amber-500/30 text-xs font-mono font-semibold px-2.5 py-1 rounded-full whitespace-nowrap';
                }
            }

            // Cards de seleção das 10 colunas
            const grid = document.getElementById('modal-columns-grid');
            if (grid) {
                let html = '';
                AVAILABLE_COLUMNS.forEach(col => {
                    const isSelected = visibleColumns.includes(col.id);
                    const pos = visibleColumns.indexOf(col.id);

                    const borderClass = isSelected
                        ? 'border-sky-500/80 bg-sky-500/10 hover:bg-sky-500/20 hover:border-red-500/60 shadow-sm shadow-sky-500/10'
                        : 'border-slate-800 bg-slate-950/60 hover:border-slate-600 hover:bg-slate-900/80 opacity-90';

                    const rightBadge = isSelected
                        ? `<div class="flex items-center space-x-1.5">
                             <span class="bg-sky-500 text-white font-mono text-[10px] font-bold px-1.5 py-0.5 rounded">#${pos + 1}</span>
                             <span class="text-[10px] text-red-400 font-medium hidden group-hover:inline-block">✕ Desmarcar</span>
                           </div>`
                        : (visibleColumns.length >= 5
                            ? `<span class="text-[10px] text-slate-500 font-mono">Cheio (5/5)</span>`
                            : `<span class="text-[10px] text-sky-400 font-mono font-medium group-hover:text-sky-300">+ Selecionar</span>`);

                    html += `
                        <div onclick="toggleColumnSelection('${col.id}')" title="${isSelected ? 'Clique para desmarcar' : (visibleColumns.length >= 5 ? 'Limite de 5 atingido - desmarque uma primeiro' : 'Clique para selecionar')}" class="group cursor-pointer p-2.5 rounded-xl border ${borderClass} flex items-center justify-between transition-all select-none">
                            <div class="flex items-center space-x-2.5 min-w-0">
                                <div class="w-4 h-4 rounded border flex items-center justify-center transition-colors ${isSelected ? 'bg-sky-600 border-sky-500 text-white group-hover:bg-red-500/20 group-hover:border-red-400' : 'border-slate-600 bg-slate-900 group-hover:border-sky-500'}">
                                    ${isSelected ? '<i data-lucide="check" class="w-3 h-3 group-hover:hidden"></i><i data-lucide="x" class="w-3 h-3 text-red-400 hidden group-hover:block"></i>' : ''}
                                </div>
                                <div class="truncate">
                                    <span class="font-semibold text-white text-xs block truncate group-hover:text-sky-300 transition-colors">${col.label}</span>
                                    <span class="text-[10px] text-slate-400 block truncate">${col.desc}</span>
                                </div>
                            </div>
                            <div class="flex-shrink-0 ml-2">
                                ${rightBadge}
                            </div>
                        </div>
                    `;
                });
                grid.innerHTML = html;
            }

            // Slots de ordenação (1 a 5)
            const slotsContainer = document.getElementById('modal-columns-slots');
            if (slotsContainer) {
                let html = '';
                for (let i = 0; i < 5; i++) {
                    const curCol = visibleColumns[i];
                    if (curCol) {
                        html += `
                            <div class="flex items-center justify-between space-x-2 bg-slate-950/60 p-2 rounded-xl border border-slate-800">
                                <span class="text-xs font-mono font-semibold text-sky-400 w-24">Posição ${i + 1}:</span>
                                <select onchange="updateColumnSlot(${i}, this.value)" class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-2.5 py-1.5 text-xs text-white focus:outline-none focus:border-sky-500">
                        `;
                        AVAILABLE_COLUMNS.forEach(c => {
                            const sel = c.id === curCol ? 'selected' : '';
                            html += `<option value="${c.id}" ${sel}>${c.label}</option>`;
                        });
                        html += `
                                </select>
                                <button type="button" onclick="removeColumnAt(${i})" title="Desmarcar esta coluna" class="p-1 text-slate-500 hover:text-red-400 hover:bg-red-500/10 rounded transition-colors">
                                    <i data-lucide="x" class="w-3.5 h-3.5"></i>
                                </button>
                            </div>
                        `;
                    } else {
                        html += `
                            <div class="flex items-center justify-between space-x-2 bg-slate-950/30 p-2 rounded-xl border border-dashed border-slate-800">
                                <span class="text-xs font-mono font-semibold text-slate-500 w-24">Posição ${i + 1}:</span>
                                <select onchange="addColumnToSlot(${i}, this.value)" class="flex-1 bg-slate-900/60 border border-slate-700/60 border-dashed rounded-lg px-2.5 py-1.5 text-xs text-slate-400 focus:outline-none focus:border-sky-500">
                                    <option value="" selected disabled>+ Selecione para a Posição ${i + 1}...</option>
                        `;
                        AVAILABLE_COLUMNS.forEach(c => {
                            if (!visibleColumns.includes(c.id)) {
                                html += `<option value="${c.id}" class="text-white bg-slate-900">${c.label}</option>`;
                            }
                        });
                        html += `
                                </select>
                            </div>
                        `;
                    }
                }
                slotsContainer.innerHTML = html;
            }

            lucide.createIcons();
        }

        function renderTableHeader() {
            const tr = document.getElementById('table-header-row');
            if (!tr) return;

            let html = '';
            // Renderiza RIGOROSAMENTE as 5 colunas configuradas pelo usuário
            const colsToRender = visibleColumns.slice(0, 5);
            colsToRender.forEach(colId => {
                const def = COLUMN_DEFS[colId] || COLUMN_DEFS['nome'];
                html += def.renderTh();
            });

            tr.innerHTML = html;
            lucide.createIcons();
        }

        async function checkAuth() {
            try {
                const res = await fetch('/api/auth/me');
                if (res.ok) {
                    currentUser = await res.json();
                    renderUserBar();
                } else {
                    currentUser = null;
                    renderUserBar();
                }
            } catch (e) {
                console.error('Erro ao verificar autenticação:', e);
            }
        }

        function renderUserBar() {
            const bar = document.getElementById('user-info-bar');
            const loginBtn = document.getElementById('btn-login-header');
            if (currentUser) {
                bar.classList.remove('hidden');
                loginBtn.classList.add('hidden');
                document.getElementById('user-name').innerText = currentUser.name || currentUser.username;
                document.getElementById('user-avatar').innerText = (currentUser.name || currentUser.username).charAt(0).toUpperCase();
                document.getElementById('user-groups').innerText = (currentUser.grupos || []).join(' | ') || 'LDAP';
            } else {
                bar.classList.add('hidden');
                loginBtn.classList.remove('hidden');
            }
        }

        async function loadStats() {
            try {
                const res = await fetch('/api/stats');
                if (!res.ok) return;
                const stats = await res.json();

                document.getElementById('stat-total').innerText = stats.total_servidores;
                document.getElementById('stat-power-on').innerText = stats.total_power_on;
                document.getElementById('stat-power-off').innerText = stats.total_power_off;
                document.getElementById('stat-cpus').innerText = stats.total_cpus;
                document.getElementById('stat-ram').innerText = stats.total_memoria_gb.toLocaleString('pt-BR');
                document.getElementById('stat-disk').innerText = stats.total_disco_gb.toLocaleString('pt-BR');

                // Counts by environment
                document.getElementById('count-todos').innerText = stats.total_servidores;
                let prod = 0, hml = 0, dev = 0;
                (stats.por_ambiente || []).forEach(a => {
                    if (a.name === 'Produção') prod = a.count;
                    if (a.name === 'Homologação') hml = a.count;
                    if (a.name === 'Desenvolvimento') dev = a.count;
                });
                document.getElementById('count-prod').innerText = prod;
                document.getElementById('count-hml').innerText = hml;
                document.getElementById('count-dev').innerText = dev;
            } catch (e) {
                console.error('Erro ao carregar estatísticas:', e);
            }
        }

        async function loadServidores() {
            const tbody = document.getElementById('table-body');
            tbody.innerHTML = `
                <tr>
                    <td colspan="5" class="py-12 text-center text-slate-500">
                        <div class="flex flex-col items-center justify-center space-y-2">
                            <i data-lucide="loader-2" class="w-6 h-6 animate-spin text-sky-500"></i>
                            <span>Carregando servidores...</span>
                        </div>
                    </td>
                </tr>
            `;
            lucide.createIcons();

            const params = new URLSearchParams();
            if (currentQuery.q) params.set('q', currentQuery.q);
            if (currentQuery.ambiente) params.set('ambiente', currentQuery.ambiente);
            if (currentQuery.equipe) params.set('equipe', currentQuery.equipe);
            if (currentQuery.state) params.set('state', currentQuery.state);
            params.set('page', currentQuery.page);
            params.set('per_page', currentQuery.per_page);
            params.set('sort_by', currentQuery.sort_by);
            params.set('sort_dir', currentQuery.sort_dir);

            try {
                const res = await fetch(`/api/servidores?${params.toString()}`);
                if (!res.ok) {
                    if (res.status === 401) {
                        openLoginModal();
                        tbody.innerHTML = `
                            <tr>
                                <td colspan="5" class="py-12 text-center text-amber-400">
                                    <div class="flex flex-col items-center justify-center space-y-2">
                                        <i data-lucide="lock" class="w-6 h-6"></i>
                                        <span>Acesso restrito. Faça login via LDAP para consultar o inventário.</span>
                                    </div>
                                </td>
                            </tr>
                        `;
                        lucide.createIcons();
                        return;
                    }
                    throw new Error('Falha na resposta do servidor');
                }

                const data = await res.json();
                lastLoadedData = data;
                renderTable(data);
                renderPagination(data);
            } catch (e) {
                console.error('Erro ao buscar servidores:', e);
                tbody.innerHTML = `
                    <tr>
                        <td colspan="5" class="py-12 text-center text-red-400">
                            Erro ao carregar servidores. Verifique sua conexão e autenticação.
                        </td>
                    </tr>
                `;
            }
        }

        function renderTable(data) {
            lastLoadedData = data;
            const tbody = document.getElementById('table-body');

            if (!data.items || data.items.length === 0) {
                const hasSearch = currentQuery.q && currentQuery.q.trim().length > 0;
                const hasAmb = currentQuery.ambiente && currentQuery.ambiente.trim().length > 0;
                const hasEquipe = currentQuery.equipe && currentQuery.equipe.trim().length > 0;
                const hasState = currentQuery.state && currentQuery.state.trim().length > 0;

                tbody.innerHTML = `
                    <tr>
                        <td colspan="5" class="py-12 text-center text-slate-400">
                            <div class="flex flex-col items-center justify-center space-y-3">
                                <i data-lucide="search-x" class="w-8 h-8 text-slate-500"></i>
                                <span class="font-semibold text-slate-300 text-sm">Nenhum servidor encontrado</span>
                                ${hasSearch ? `<span class="text-xs text-amber-400 font-mono bg-amber-500/10 border border-amber-500/30 px-3 py-1 rounded-full">Filtro de busca ativo: "${escapeHtml(currentQuery.q)}"</span>` : ''}
                                ${(hasSearch || hasAmb || hasEquipe || hasState) ? `
                                    <button onclick="clearAllFilters()" class="mt-2 px-3.5 py-1.5 bg-sky-600 hover:bg-sky-500 text-white text-xs font-semibold rounded-lg shadow-sm transition-all flex items-center space-x-1.5">
                                        <i data-lucide="rotate-ccw" class="w-3.5 h-3.5"></i>
                                        <span>Limpar Busca e Filtros</span>
                                    </button>
                                ` : ''}
                            </div>
                        </td>
                    </tr>
                `;
                lucide.createIcons();
                return;
            }

            let html = '';
            data.items.forEach(s => {
                html += `<tr class="hover:bg-slate-800/40 transition-colors">`;
                visibleColumns.forEach(colId => {
                    const def = COLUMN_DEFS[colId];
                    if (def) {
                        html += def.renderTd(s);
                    }
                });
                html += `</tr>`;
            });

            tbody.innerHTML = html;
            lucide.createIcons();
        }

        function clearSearch() {
            const input = document.getElementById('input-search');
            if (input) input.value = '';
            currentQuery.q = '';
            currentQuery.page = 1;
            updateClearSearchButton();
            loadServidores();
        }

        function clearAllFilters() {
            const input = document.getElementById('input-search');
            if (input) input.value = '';
            currentQuery.q = '';
            currentQuery.ambiente = '';
            currentQuery.equipe = '';
            currentQuery.state = '';
            currentQuery.page = 1;
            const eq = document.getElementById('select-equipe');
            if (eq) eq.value = '';
            const st = document.getElementById('select-state');
            if (st) st.value = '';
            document.querySelectorAll('.amb-tab').forEach(b => {
                if (b.getAttribute('data-amb') === '') {
                    b.className = 'amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold bg-sky-600 text-white shadow-sm transition-all whitespace-nowrap';
                } else {
                    b.className = 'amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-800 transition-all whitespace-nowrap';
                }
            });
            updateClearSearchButton();
            loadServidores();
        }

        function updateClearSearchButton() {
            const input = document.getElementById('input-search');
            const btn = document.getElementById('btn-clear-search');
            if (btn && input) {
                if (input.value && input.value.trim().length > 0) {
                    btn.classList.remove('hidden');
                    lucide.createIcons();
                } else {
                    btn.classList.add('hidden');
                }
            }
        }

        function renderPagination(data) {
            const info = document.getElementById('pagination-info');
            const start = (data.page - 1) * data.per_page + 1;
            const end = Math.min(data.page * data.per_page, data.total);
            info.innerText = `Exibindo ${data.total > 0 ? start : 0} a ${end} de ${data.total} servidores`;

            const ctrl = document.getElementById('pagination-controls');
            let btns = '';

            if (data.page > 1) {
                btns += `<button onclick="goToPage(${data.page - 1})" class="px-2.5 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-300 transition-colors">&laquo; Anterior</button>`;
            }

            for (let p = Math.max(1, data.page - 2); p <= Math.min(data.total_pages, data.page + 2); p++) {
                const active = p === data.page ? 'bg-sky-600 text-white font-bold' : 'bg-slate-800 hover:bg-slate-700 text-slate-300';
                btns += `<button onclick="goToPage(${p})" class="px-2.5 py-1 rounded ${active} transition-colors">${p}</button>`;
            }

            if (data.page < data.total_pages) {
                btns += `<button onclick="goToPage(${data.page + 1})" class="px-2.5 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-300 transition-colors">Próxima &raquo;</button>`;
            }

            ctrl.innerHTML = btns;
        }

        function goToPage(p) {
            currentQuery.page = p;
            loadServidores();
        }

        function setAmbiente(amb) {
            currentQuery.ambiente = amb;
            currentQuery.page = 1;
            document.querySelectorAll('.amb-tab').forEach(b => {
                if (b.getAttribute('data-amb') === amb) {
                    b.className = 'amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold bg-sky-600 text-white shadow-sm transition-all whitespace-nowrap';
                } else {
                    b.className = 'amb-tab px-3 py-1.5 rounded-lg text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-800 transition-all whitespace-nowrap';
                }
            });
            loadServidores();
        }

        function onFilterChange() {
            currentQuery.equipe = document.getElementById('select-equipe').value;
            currentQuery.state = document.getElementById('select-state').value;
            currentQuery.page = 1;
            loadServidores();
        }

        function debounceSearch() {
            updateClearSearchButton();
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => {
                currentQuery.q = document.getElementById('input-search').value.trim();
                currentQuery.page = 1;
                loadServidores();
            }, 300);
        }

        function changeSort(col) {
            if (currentQuery.sort_by === col) {
                currentQuery.sort_dir = currentQuery.sort_dir === 'asc' ? 'desc' : 'asc';
            } else {
                currentQuery.sort_by = col;
                currentQuery.sort_dir = 'asc';
            }
            loadServidores();
        }

        // Modals management
        function openModal(id) {
            document.getElementById(id).classList.remove('hidden');
            lucide.createIcons();
        }
        function closeModal(id) {
            document.getElementById(id).classList.add('hidden');
        }

        function openLoginModal() {
            document.getElementById('login-error').classList.add('hidden');
            openModal('modal-login');
        }

        async function handleLoginSubmit(e) {
            e.preventDefault();
            const btn = document.getElementById('btn-login-submit');
            const errDiv = document.getElementById('login-error');
            errDiv.classList.add('hidden');

            const username = document.getElementById('login-username').value.trim();
            const password = document.getElementById('login-password').value;

            btn.disabled = true;
            btn.innerHTML = `<i data-lucide="loader-2" class="w-4 h-4 animate-spin mr-1"></i> Autenticando no LDAP...`;
            lucide.createIcons();

            try {
                const res = await fetch('/api/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, password })
                });

                const data = await res.json();
                if (res.ok) {
                    currentUser = data;
                    renderUserBar();
                    closeModal('modal-login');
                    await loadStats();
                    await loadServidores();
                } else {
                    errDiv.innerText = data.error || 'Credenciais LDAP inválidas ou grupo não autorizado.';
                    errDiv.classList.remove('hidden');
                }
            } catch (err) {
                errDiv.innerText = 'Erro ao conectar ao serviço de autenticação.';
                errDiv.classList.remove('hidden');
            } finally {
                btn.disabled = false;
                btn.innerHTML = `<i data-lucide="log-in" class="w-4 h-4 mr-1"></i> Entrar no Sistema`;
                lucide.createIcons();
            }
        }

        async function logout() {
            try {
                await fetch('/api/auth/logout', { method: 'POST' });
                currentUser = null;
                renderUserBar();
                await loadServidores();
            } catch (e) {
                console.error(e);
            }
        }

        async function viewDetails(id) {
            try {
                const res = await fetch(`/api/servidores/${id}`);
                if (!res.ok) {
                    if (res.status === 401) {
                        openLoginModal();
                    }
                    return;
                }
                const s = await res.json();
                currentPopupServer = s;

                document.getElementById('det-nome').innerText = s.nome;
                document.getElementById('det-hostname').innerText = s.hostname;
                document.getElementById('det-item-badge').innerText = `Item #${s.item || s.id}`;

                const isOnline = (s.state || '').toUpperCase().includes('ON');
                const stateBadge = document.getElementById('det-status-badge');
                if (isOnline) {
                    stateBadge.className = 'px-2.5 py-1 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20';
                    stateBadge.innerText = '● POWER ON';
                } else {
                    stateBadge.className = 'px-2.5 py-1 rounded-full text-xs font-semibold bg-red-500/10 text-red-400 border border-red-500/20';
                    stateBadge.innerText = '○ POWER OFF';
                }

                const ambBadge = document.getElementById('det-ambiente-badge');
                let ambClass = 'bg-slate-800 text-slate-300 border-slate-700';
                if (s.ambiente === 'Produção') ambClass = 'bg-blue-500/10 text-blue-400 border-blue-500/20';
                else if (s.ambiente === 'Homologação') ambClass = 'bg-purple-500/10 text-purple-400 border-purple-500/20';
                else if (s.ambiente === 'Desenvolvimento') ambClass = 'bg-amber-500/10 text-amber-400 border-amber-500/20';
                ambBadge.className = `px-2.5 py-1 rounded-full text-xs font-semibold border ${ambClass}`;
                ambBadge.innerText = s.ambiente;

                // Credenciais Administrativas
                document.getElementById('popup-admin-user').value = s.admin_user || '';
                document.getElementById('popup-admin-password').value = s.admin_password || '';
                const feedback = document.getElementById('popup-cred-feedback');
                if (feedback) feedback.classList.add('hidden');

                // Hardware e observações
                document.getElementById('det-ip').innerText = s.ip;
                document.getElementById('det-cpus').innerText = s.cpus ? `${s.cpus} vCPUs` : '--';
                document.getElementById('det-memoria').innerText = s.memoria_gb ? `${s.memoria_gb}` : '--';
                document.getElementById('det-disco').innerText = s.disco_gb ? `${s.disco_gb} GB` : '--';
                document.getElementById('det-so').innerText = s.sistema_operacional || 'Não especificado';
                document.getElementById('det-servico').innerText = s.servico || '--';
                document.getElementById('det-obs').innerText = s.observacoes || 'Nenhuma observação informada.';

                // Renderiza campos que NÃO estão nas 5 colunas visíveis
                renderHiddenFieldsInPopup(s);

                openModal('modal-details');
                lucide.createIcons();
            } catch (e) {
                console.error('Erro ao buscar detalhes do servidor:', e);
            }
        }

        function renderHiddenFieldsInPopup(s) {
            const container = document.getElementById('popup-hidden-fields');
            if (!container) return;

            // Todos os campos comparados contra as 5 colunas ativas do usuário
            const potentialCols = [
                { id: 'status', label: 'Status Operacional', val: s.state, icon: 'activity' },
                { id: 'hostname', label: 'Hostname / FQDN', val: s.hostname, icon: 'network', mono: true },
                { id: 'ip', label: 'Endereço IP', val: s.ip, icon: 'globe', mono: true, copy: true },
                { id: 'ambiente', label: 'Ambiente', val: s.ambiente, icon: 'layers' },
                { id: 'equipe', label: 'Equipe Responsável', val: s.equipe_resp || '--', icon: 'users' },
                { id: 'servico', label: 'Serviço / Aplicação', val: s.servico || '--', icon: 'cpu' },
                { id: 'recursos', label: 'Recursos de Hardware', val: `${s.cpus ? `${s.cpus}c / ` : ''}${s.memoria_gb ? `${s.memoria_gb} / ` : ''}${s.disco_gb ? `${s.disco_gb}GB` : ''}` || '--', icon: 'hard-drive' },
                { id: 'so', label: 'Sistema Operacional', val: s.sistema_operacional || '--', icon: 'terminal' },
                { id: 'observacoes', label: 'Observações Complementares', val: s.observacoes || 'Nenhuma', icon: 'file-text' }
            ];

            // Filtra os campos que NÃO estão entre as 5 colunas atualmente visíveis
            const hidden = potentialCols.filter(col => !visibleColumns.includes(col.id));

            if (hidden.length === 0) {
                container.innerHTML = `
                    <div class="col-span-2 text-center py-2 text-slate-400 text-xs">
                        Todos os campos principais já estão presentes nas 5 colunas da listagem principal.
                    </div>
                `;
                return;
            }

            let html = '';
            hidden.forEach(col => {
                const copyBtn = col.copy
                    ? `<button type="button" onclick="copyToClipboard('${escapeHtml(col.val)}')" title="Copiar" class="text-slate-500 hover:text-sky-400 p-0.5"><i data-lucide="copy" class="w-3 h-3"></i></button>`
                    : '';

                html += `
                    <div class="bg-slate-900/90 p-2.5 rounded-xl border border-slate-800 flex items-start space-x-2.5">
                        <div class="w-7 h-7 rounded-lg bg-sky-500/10 text-sky-400 flex items-center justify-center flex-shrink-0 mt-0.5">
                            <i data-lucide="${col.icon}" class="w-3.5 h-3.5"></i>
                        </div>
                        <div class="min-w-0 flex-1">
                            <div class="flex items-center justify-between">
                                <span class="text-[10px] font-semibold uppercase text-slate-400 block">${col.label}</span>
                                ${copyBtn}
                            </div>
                            <span class="text-xs text-white font-medium block truncate ${col.mono ? 'font-mono text-sky-300' : ''}" title="${escapeHtml(col.val)}">
                                ${escapeHtml(col.val)}
                            </span>
                        </div>
                    </div>
                `;
            });

            container.innerHTML = html;
        }

        async function saveAdminCredentialsFromPopup() {
            if (!currentPopupServer) return;
            const btn = document.getElementById('btn-save-credentials');
            const userVal = document.getElementById('popup-admin-user').value.trim();
            const passVal = document.getElementById('popup-admin-password').value;

            btn.disabled = true;
            btn.innerHTML = `<i data-lucide="loader-2" class="w-3.5 h-3.5 animate-spin"></i><span>Salvando...</span>`;
            lucide.createIcons();

            try {
                const res = await fetch(`/api/servidores/${currentPopupServer.id}/credentials`, {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ admin_user: userVal || null, admin_password: passVal || null })
                });

                if (res.ok) {
                    const updated = await res.json();
                    currentPopupServer = updated;
                    const feedback = document.getElementById('popup-cred-feedback');
                    if (feedback) {
                        feedback.classList.remove('hidden');
                        setTimeout(() => feedback.classList.add('hidden'), 4000);
                    }
                    await loadServidores();
                } else {
                    const err = await res.json();
                    alert(err.error || 'Erro ao salvar credenciais');
                }
            } catch (e) {
                alert('Erro na comunicação com o servidor');
            } finally {
                btn.disabled = false;
                btn.innerHTML = `<i data-lucide="save" class="w-3.5 h-3.5"></i><span>Salvar Credenciais</span>`;
                lucide.createIcons();
            }
        }

        function editServerFromPopup() {
            if (currentPopupServer) {
                closeModal('modal-details');
                editServer(currentPopupServer.id);
            }
        }

        function openCreateModal() {
            if (!currentUser) {
                openLoginModal();
                return;
            }
            document.getElementById('form-modal-title').innerText = 'Novo Servidor';
            document.getElementById('form-server-id').value = '';
            document.getElementById('form-admin-user').value = '';
            document.getElementById('form-admin-password').value = '';
            document.getElementById('server-form').reset();
            openModal('modal-form');
        }

        async function editServer(id) {
            if (!currentUser) {
                openLoginModal();
                return;
            }
            try {
                const res = await fetch(`/api/servidores/${id}`);
                if (!res.ok) return;
                const s = await res.json();

                document.getElementById('form-modal-title').innerText = 'Editar Servidor';
                document.getElementById('form-server-id').value = s.id;
                document.getElementById('form-nome').value = s.nome;
                document.getElementById('form-hostname').value = s.hostname;
                document.getElementById('form-ambiente').value = s.ambiente;
                document.getElementById('form-state').value = s.state;
                document.getElementById('form-equipe').value = s.equipe_resp;
                document.getElementById('form-ip').value = s.ip;
                document.getElementById('form-servico').value = s.servico;
                document.getElementById('form-cpus').value = s.cpus || '';
                document.getElementById('form-memoria').value = s.memoria_gb || '';
                document.getElementById('form-disco').value = s.disco_gb || '';
                document.getElementById('form-so').value = s.sistema_operacional || '';
                document.getElementById('form-obs').value = s.observacoes || '';
                document.getElementById('form-admin-user').value = s.admin_user || '';
                document.getElementById('form-admin-password').value = s.admin_password || '';

                openModal('modal-form');
            } catch (e) {
                console.error('Erro ao carregar servidor para edição:', e);
            }
        }

        async function handleFormSubmit(e) {
            e.preventDefault();
            const id = document.getElementById('form-server-id').value;
            const payload = {
                nome: document.getElementById('form-nome').value.trim(),
                hostname: document.getElementById('form-hostname').value.trim(),
                ambiente: document.getElementById('form-ambiente').value,
                state: document.getElementById('form-state').value,
                equipe_resp: document.getElementById('form-equipe').value,
                ip: document.getElementById('form-ip').value.trim(),
                servico: document.getElementById('form-servico').value.trim(),
                cpus: document.getElementById('form-cpus').value.trim() || null,
                memoria_gb: document.getElementById('form-memoria').value.trim() || null,
                disco_gb: document.getElementById('form-disco').value.trim() || null,
                sistema_operacional: document.getElementById('form-so').value.trim() || null,
                observacoes: document.getElementById('form-obs').value.trim() || null,
                admin_user: document.getElementById('form-admin-user').value.trim() || null,
                admin_password: document.getElementById('form-admin-password').value || null,
            };

            const url = id ? `/api/servidores/${id}` : '/api/servidores';
            const method = id ? 'PUT' : 'POST';

            try {
                const res = await fetch(url, {
                    method: method,
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });

                if (res.ok) {
                    closeModal('modal-form');
                    await loadStats();
                    await loadServidores();
                } else {
                    const data = await res.json();
                    alert(data.error || 'Erro ao salvar servidor');
                }
            } catch (err) {
                alert('Erro na requisição');
            }
        }

        async function deleteServer(id, nome) {
            if (!currentUser) {
                openLoginModal();
                return;
            }
            if (!confirm(`Deseja realmente excluir o servidor "${nome}" do inventário? Todas as ações são auditadas.`)) {
                return;
            }
            try {
                const res = await fetch(`/api/servidores/${id}`, { method: 'DELETE' });
                if (res.ok) {
                    await loadStats();
                    await loadServidores();
                } else {
                    const data = await res.json();
                    alert(data.error || 'Erro ao excluir servidor');
                }
            } catch (err) {
                alert('Erro ao excluir');
            }
        }

        function togglePasswordVisibility(inputId, btnEl) {
            const input = document.getElementById(inputId);
            if (!input) return;
            if (input.type === 'password') {
                input.type = 'text';
                btnEl.innerHTML = `<i data-lucide="eye-off" class="w-3.5 h-3.5 text-sky-400"></i>`;
            } else {
                input.type = 'password';
                btnEl.innerHTML = `<i data-lucide="eye" class="w-3.5 h-3.5 text-slate-400"></i>`;
            }
            lucide.createIcons();
        }

        function copyInputVal(inputId, fieldName) {
            const input = document.getElementById(inputId);
            if (!input || !input.value) {
                alert(`Nenhum valor informado para ${fieldName}.`);
                return;
            }
            navigator.clipboard.writeText(input.value);
            alert(`${fieldName} copiado para a área de transferência!`);
        }

        async function openAuditModal() {
            openModal('modal-audit-logs');
            await loadAuditLogs();
        }

        async function loadAuditLogs() {
            const tbody = document.getElementById('audit-table-body');
            if (!tbody) return;
            tbody.innerHTML = `<tr><td colspan="5" class="py-6 text-center text-slate-500"><i data-lucide="loader-2" class="w-5 h-5 animate-spin inline-block text-amber-500 mr-2"></i>Carregando logs de auditoria...</td></tr>`;
            lucide.createIcons();

            try {
                const res = await fetch('/api/audit-logs');
                if (!res.ok) {
                    tbody.innerHTML = `<tr><td colspan="5" class="py-6 text-center text-red-400">Não foi possível carregar os logs de auditoria.</td></tr>`;
                    return;
                }
                const logs = await res.json();
                if (!logs || logs.length === 0) {
                    tbody.innerHTML = `<tr><td colspan="5" class="py-6 text-center text-slate-500">Nenhum evento registrado ainda.</td></tr>`;
                    return;
                }

                let html = '';
                logs.forEach(l => {
                    let badgeClass = 'bg-slate-800 text-slate-300';
                    if (l.action.includes('CRIAR')) badgeClass = 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30';
                    else if (l.action.includes('EDITAR')) badgeClass = 'bg-sky-500/20 text-sky-400 border border-sky-500/30';
                    else if (l.action.includes('EXCLUIR')) badgeClass = 'bg-red-500/20 text-red-400 border border-red-500/30';
                    else if (l.action.includes('CREDENCIAIS')) badgeClass = 'bg-amber-500/20 text-amber-400 border border-amber-500/30';
                    else if (l.action.includes('LOGIN')) badgeClass = 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30';

                    html += `
                        <tr class="hover:bg-slate-800/30">
                            <td class="py-2.5 px-3 text-slate-400 whitespace-nowrap">${escapeHtml(l.created_at)}</td>
                            <td class="py-2.5 px-3 font-semibold text-white">${escapeHtml(l.username)}</td>
                            <td class="py-2.5 px-3"><span class="px-2 py-0.5 rounded text-[10px] font-bold ${badgeClass}">${escapeHtml(l.action)}</span></td>
                            <td class="py-2.5 px-3 text-slate-200">${escapeHtml(l.servidor_nome || '--')}</td>
                            <td class="py-2.5 px-3 text-slate-400 truncate max-w-xs" title="${escapeHtml(l.detalhes || '')}">${escapeHtml(l.detalhes || '--')}</td>
                        </tr>
                    `;
                });
                tbody.innerHTML = html;
                lucide.createIcons();
            } catch (e) {
                tbody.innerHTML = `<tr><td colspan="5" class="py-6 text-center text-red-400">Erro na requisição.</td></tr>`;
            }
        }

        function copyToClipboard(text) {
            navigator.clipboard.writeText(text);
            alert(`Copiado para a área de transferência: ${text}`);
        }

        function escapeHtml(text) {
            if (!text) return '';
            const map = { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#039;' };
            return text.toString().replace(/[&<>"']/g, m => map[m]);
        }
    </script>
</body>
</html>
"#;
