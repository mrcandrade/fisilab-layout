# Janela do DMG (macOS)

Dois arquivos prontos, copiados por `package.sh` para dentro da imagem; nada aqui é gerado na hora do build.

| Arquivo | O que é |
|---|---|
| `background.tiff` | Fundo da janela do Finder (HiDPI, 1x e 2x). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes de visualização do Finder (tamanho da janela, ícones, posições, fundo). Vai para `.DS_Store`. |

Ambos vêm de `suite-fisilab/scripts/gerar_mac.py`, no repositório [mrcandrade/larfic](https://github.com/mrcandrade/larfic).
Para mudar o visual, regenere lá e copie os dois arquivos para cá.

Nomes que o layout espera (não mude sem regenerar):

- nome do volume: `Layout` (sem versão);
- aplicativo: `Layout.app`;
- atalho para a pasta de programas: `Aplicativos`.
