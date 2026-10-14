# Modificações do FisiLayout em relação ao DesignCraft

O **FisiLayout** é a versão do [DesignCraft](https://github.com/storytold/designcraft) (ArtCraft Team e
contribuidores, MIT OR Apache-2.0) disponibilizada pelo **FisiLab — LARFIC · UFRGS** em parceria com o
**CTA — Centro de Tecnologia Acadêmica**, como parte da Suíte FisiLab. Esta lista resume o que foi
alterado em relação à versão 0.8.0 do projeto original, e por quê. As modificações estão sob os
mesmos termos (MIT OR Apache-2.0).

## Nome e marca

- O nome visível do programa passou de "DesignCraft" para **FisiLayout**: título da janela, nome
  passado ao `eframe::run_native`, barra de título, tela inicial, menus ("About FisiLayout", "Quit
  FisiLayout"), janela Sobre, mensagens de erro e de status, ajuda e `--version` da linha de comando,
  servidor MCP, metadados de PDF/EPUB gerados e os catálogos de tradução (chaves e valores
  atualizados juntos em todos os idiomas).
- **Por quê:** o nome e as logos do ArtCraft são marcas registradas que **não** estão cobertas pela
  licença de código aberto (veja o NOTICE original). Por isso:
  - a pasta `docs/brand/` (logos e wordmark do ArtCraft) foi apagada;
  - a marca do ArtCraft desenhada em código na janela Sobre e na tela inicial foi removida e
    substituída pelo ícone do FisiLayout (`assets/fisilab/fisilayout-128.png`);
  - a logo do ArtCraft foi retirada do README original (agora `README.upstream.md`).
- O crédito em texto ao DesignCraft e à ArtCraft Team foi mantido (janela Sobre, tela inicial,
  ajuda da CLI, NOTICE, README) — ele é exigido e merecido.

## Identidade visual da Suíte FisiLab

- Novo crate `crates/fisilab-brand` (compartilhado pela suíte): paleta de cores, tela de abertura
  animada com a logo do CTA e janela "Sobre a Suíte FisiLab" (créditos, manifesto, logos da UFRGS e
  do CTA). A tela de abertura aparece só no aplicativo de desktop (`FISILAB_NO_SPLASH=1` a desliga).
- Novo item de menu **Ajuda › Sobre a Suíte FisiLab…** (comando `help.fisilab`), um botão "FisiLab"
  na barra do aplicativo e um botão "Sobre a Suíte FisiLab" na tela inicial e na janela Sobre.
- Novos temas de cores **"UFRGS Noite"** (escuro, padrão) e **"UFRGS Dia"** (claro), primeiros da
  lista *Janela › Interface Color Theme*, construídos a partir das cores da marca (vermelho da
  chama do LARFIC, marinho, creme, azul do CTA e da UFRGS). Só as cores mudam; o layout é o do tema
  padrão original. Os temas originais (Dark, Medium Dark, Medium Light, Light, High Contrast)
  continuam disponíveis.
- Ícone do aplicativo (janela, barra de tarefas, executável do Windows, instalador) trocado pelo
  ícone do FisiLayout em `assets/fisilab/`.

## Links

- Os links de comunidade do original (Discord do ArtCraft, getartcraft.com, página do app no site
  do ArtCraft, lista de outros apps do ArtCraft) foram substituídos por:
  - **Site da Suíte FisiLab** — https://github.com/mrcandrade/larfic/tree/main/suite-fisilab
  - **Manifesto pelas tecnologias livres** — https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md
  - **Projeto original (DesignCraft)** — https://github.com/storytold/designcraft
  - **Código-fonte do FisiLayout** — https://github.com/mrcandrade/fisilayout
  - **Reportar um problema** — https://github.com/mrcandrade/fisilayout/issues
- Valem no menu Ajuda, na janela Sobre, na tela inicial, em `fisilayout-cli links`, no comando
  `app.links` e no servidor MCP. Os comandos `help.discord`, `help.appPage`, `help.website` e
  `help.app` deixaram de existir; entraram `help.suite`, `help.manifesto`, `help.upstream` e
  `help.fisilab`.
- O programa não faz nenhum acesso automático à rede: não há verificação de atualizações,
  telemetria nem envio de relatórios de falha no original, e nada disso foi acrescentado. Os links
  só abrem no navegador quando a pessoa clica.

## Idioma

- Numa instalação nova, a interface começa em **português do Brasil** (a tradução já existia no
  original). Quem escolher outro idioma em *Edit › Interface Language* mantém a escolha nas próximas
  execuções. Foi acrescentada a tradução de "Sample Document" ("Documento de exemplo").

## Executáveis, identificador e pastas

- Executáveis: `fisilayout.exe` (interface) e `fisilayout-cli.exe` (linha de comando), definidos por
  `[[bin]]` nos crates `apps/designcraft` e `apps/designcraft-cli`. Os nomes dos pacotes Cargo e dos
  crates continuam os originais (`designcraft`, `designcraft-*`).
- Identificador do aplicativo: `br.ufrgs.fisilab.fisilayout` (antes `ai.storyteller.designcraft`).
- Pasta de configurações e de recuperação de documentos: `%APPDATA%\FisiLayout` no Windows,
  `~/Library/Application Support/FisiLayout` no macOS e `~/.config/fisilayout` /
  `~/.local/share/fisilayout/recovery` no Linux, para não misturar com uma instalação do DesignCraft.
- Recursos do executável do Windows (`apps/designcraft/build.rs`): ícone, ProductName "FisiLayout",
  FileDescription, CompanyName "FisiLab — LARFIC/UFRGS", LegalCopyright com o crédito ao original,
  OriginalFilename `fisilayout.exe`.
- Empacotamento do Windows (`packaging/windows/fisilayout.wxs`, `package.ps1`): nomes, ícone,
  links e novos GUIDs (UpgradeCode), para não atualizar nem remover uma instalação do DesignCraft.
  Os metadados de Linux (`nfpm.yaml`, `metainfo`) apontam para os links da suíte.
- `xtask` (`layers.rs`): o crate `fisilab-brand` foi registrado na camada da interface.

## Integração contínua

- A pasta `.github/` (workflows de release, FreeBSD, ARM64 e lint de empacotamento do original) foi
  apagada: ela rodaria contra a infraestrutura e os segredos do projeto original.

## Documentação

- Novo `README.md` em português; o README original virou `README.upstream.md`.
- `NOTICE` ganhou um cabeçalho do FisiLayout; o texto original está logo abaixo, sem alterações.
- `ASSETS.md`: removidas as linhas de `docs/brand/`; acrescentados o ícone e as logos da Suíte FisiLab.
- `docs/mcp.md`: descrição dos novos links.

## O que **não** mudou

- Todo o motor de diagramação, composição de texto, renderização, importação/exportação (IDML, PDF,
  EPUB, PNG) e as ferramentas são os do DesignCraft 0.8.0.
- O formato de arquivo e a extensão `.designcraft` continuam os mesmos (compatíveis com o original),
  assim como o identificador interno do formato, os URIs `designcraft://` do MCP, o nome do perfil
  de cor "DesignCraft Generic CMYK" e as variáveis de ambiente `DESIGNCRAFT_*`.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md` não foram alterados. Os créditos de
  contribuidores e de modelos da janela Sobre (abas Contributors/Models) são os do projeto original.
- Documentos históricos em `docs/`, `ROADMAP.md`, `AGENTS.md` e `CLAUDE.md` foram mantidos como estavam.
- O aplicativo web (`apps/designcraft-web`) só teve o título e algumas mensagens renomeados; ele não
  mostra a tela de abertura.
