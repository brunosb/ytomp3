# YToMP3

Aplicativo desktop minimalista para baixar o áudio de vídeos do YouTube e converter para MP3.

## Recursos

- Escolha da pasta de destino.
- Download e conversão local usando `yt-dlp` + FFmpeg.
- Atualização automática do `yt-dlp` ao abrir o aplicativo.
- Instalador Windows NSIS gerado automaticamente a cada push na `main`.

## Desenvolvimento

```bash
npm install
npm run build:ui
npm run tauri:dev
```

O instalador é publicado como artefato do GitHub Actions. A versão vem do `package.json` e do `src-tauri/tauri.conf.json`.
