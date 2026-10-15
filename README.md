<p align="center">
  <img src="assets/fisilab/fisilab-layout-256.png" alt="Ícone do Layout" width="128">
</p>

<h1 align="center">Layout · Suíte FisiLab</h1>

<p align="center">
  <b>Diagramação e editoração</b><br>
  Alternativa livre ao <b>Adobe InDesign</b>
</p>

<p align="center">
  Disponibilizado pelo <b>FisiLab — LARFIC · UFRGS</b> em parceria com o <b>CTA — Centro de
  Tecnologia Acadêmica</b>, para democratizar o acesso à tecnologia entre os alunos da UFRGS.
</p>

<p align="center">
  Parte da <a href="https://github.com/mrcandrade/larfic/tree/main/suite-fisilab">Suíte FisiLab</a> ·
  <a href="https://github.com/mrcandrade/fisilab-layout/issues">Reportar um problema</a>
</p>

## O que faz

- **Diagramação no estilo do InDesign:** páginas espelhadas e páginas-mestre, quadros de texto e
  de imagem, textos encadeados entre quadros e colunas, painel de Controle, estilos de parágrafo e
  de caractere, amostras de cor e contorno de texto (text wrap), com menus, ferramentas e atalhos
  familiares.
- **Tipografia de qualidade:** compositor de parágrafo Knuth–Plass, hifenização por dicionário,
  justificação por palavras, letras e escala de glifos, alinhamento óptico de margem, grade de linha
  de base, tabulações, fios e sombreamento. As quebras de linha são idênticas na tela e no PDF.
- **Rápido:** renderização multithread (vello_cpu), documentos com cópia sob demanda e desfazer
  instantâneo, composição em cache.
- **Formatos abertos:** formato nativo documentado (`.designcraft`), importação e exportação de IDML,
  e exportação para PNG, PDF (PDF/X-4, PDF/A-2b) e EPUB. Sem assinatura, sem servidor de licenças,
  sem telemetria.
- **Automatizável:** todo item de menu, ferramenta, painel e diálogo pode ser acionado por um canal
  de controle JSON e por um **servidor MCP**; há também uma linha de comando (`fisilab-layout-cli`) para
  renderizar, exportar e rodar scripts de comandos sem abrir a janela.
- **Revista de exemplo:** *Ajuda › Documento de exemplo* (ou `fisilab-layout --sample`) abre uma revista inteira montada pelo
  próprio programa, para explorar os recursos.
- **Interface em português do Brasil** na primeira abertura, além de inglês, alemão, francês, espanhol,
  japonês, chinês, árabe, italiano e ucraniano; paleta de cores original, com os temas **Noite** (padrão) e **Dia**.

## Instalação

### Windows

Baixe o instalador `FisiLab-Layout-Setup-<versão>.exe` na [página de downloads da Suíte FisiLab](https://github.com/mrcandrade/larfic/releases/latest) e execute-o.

### Linux

Baixe o `FisiLab-Layout-<versão>-x86_64.AppImage` na [página de downloads da Suíte FisiLab](https://github.com/mrcandrade/larfic/releases/latest), marque-o como executável (`chmod +x` ou Propriedades › Permitir executar) e abra-o; ou, no Ubuntu/Debian, instale o `fisilab-layout_<versão>_amd64.deb` com `sudo apt install ./fisilab-layout_<versão>_amd64.deb`. Requer Ubuntu 22.04 ou mais recente (ou equivalente).

### Compilar a partir do código

É preciso o [Rust](https://rustup.rs) 1.95 ou mais recente.

```sh
git clone https://github.com/mrcandrade/fisilab-layout
cd fisilab-layout
cargo run --release -p designcraft                          # o aplicativo (fisilab-layout)
cargo run --release -p designcraft -- --sample              # abre a revista de exemplo
cargo run --release -p designcraft-cli -- --help            # a linha de comando (fisilab-layout-cli)
cargo build --release -p designcraft -p designcraft-cli     # gera target/release/fisilab-layout e fisilab-layout-cli
```

Os nomes dos pacotes Cargo continuam os do projeto original (`designcraft`, `designcraft-cli`);
os executáveis gerados se chamam `fisilab-layout` e `fisilab-layout-cli`.

## Créditos

O Layout é uma versão modificada do **[DesignCraft](https://github.com/storytold/designcraft)**,
criado pela **ArtCraft Team e contribuidores**, distribuído sob a licença MIT ou Apache 2.0, à
escolha de quem o usa ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)). Todo o
trabalho de diagramação, tipografia, formatos de arquivo e ferramentas é deles; o FisiLab alterou o
nome, a identidade visual, o tema de cores, a tela de abertura, os links e o instalador.

Veja [NOTICE](NOTICE), [ATTRIBUTION.md](ATTRIBUTION.md), [MODIFICACOES.md](MODIFICACOES.md) e o
README original em [README.upstream.md](README.upstream.md).

Os nomes e marcas do projeto original (incluindo ArtCraft e DesignCraft) pertencem aos seus
titulares; esta versão não é afiliada nem endossada por eles.

## Manifesto

Conhecimento produzido em uma universidade pública deve circular livremente, e as ferramentas que
usamos para ensinar, pesquisar e criar também devem ser livres. Leia o
[Manifesto pelas tecnologias livres](https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md).

## Privacidade

O Layout não coleta dados de uso nem envia o conteúdo dos seus arquivos para lugar nenhum: não há
telemetria, estatísticas nem verificação automática de atualizações. Veja o que ele guarda no seu
computador e quando usa a internet no [aviso de privacidade da Suíte FisiLab](https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/PRIVACIDADE.md).

---

<p align="center">
  <img src="assets/fisilab/larfic-logo.png" alt="LARFIC — FisiLab" height="48">
  &nbsp;&nbsp;&nbsp;
  <img src="assets/fisilab/cta-logo.webp" alt="CTA — Centro de Tecnologia Acadêmica" height="48">
</p>

<p align="center"><sub>FisiLab — LARFIC · UFRGS, em parceria com o CTA — Centro de Tecnologia Acadêmica.</sub></p>
