import { useEffect, useState } from 'react'
import { Download, FolderOpen, Link, LoaderCircle, Music2, Settings2 } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import logoUrl from './assets/logo.jpg'

type DownloadState = 'idle' | 'downloading' | 'done' | 'error'

export function App() {
  const [url, setUrl] = useState('')
  const [folder, setFolder] = useState('')
  const [state, setState] = useState<DownloadState>('idle')
  const [message, setMessage] = useState('Cole um link do YouTube para começar.')

  useEffect(() => {
    invoke<string>('get_download_folder').then(setFolder).catch(() => {})
  }, [])

  async function chooseFolder() {
    const selected = await open({ directory: true, multiple: false, title: 'Escolha a pasta dos downloads' })
    if (typeof selected === 'string') {
      setFolder(selected)
      await invoke('set_download_folder', { path: selected })
      setMessage('Pasta de download atualizada.')
    }
  }

  async function download() {
    if (!url.trim()) return setMessage('Informe um link do YouTube.')
    if (!folder) return setMessage('Escolha uma pasta para salvar os arquivos.')
    setState('downloading'); setMessage('Baixando e convertendo para MP3...')
    try {
      await invoke<string>('download_mp3', { url: url.trim(), outputDir: folder })
      setState('done'); setMessage('Tudo pronto! O MP3 foi salvo na pasta escolhida.'); setUrl('')
    } catch (error) {
      setState('error'); setMessage(String(error))
    }
  }

  return <main className="shell">
    <header><div className="brand"><img className="logo-image" src={logoUrl} alt="YToMP3"/><span>YToMP3</span></div><button className="icon-button" onClick={chooseFolder} title="Configurar pasta"><Settings2 size={19}/></button></header>
    <section className="hero"><div className="eyebrow">YOUTUBE → MP3</div><h1>Seu áudio,<br/><em>do seu jeito.</em></h1><p>Baixe o áudio de qualquer vídeo do YouTube em MP3, sem complicação.</p></section>
    <section className="card">
      <label className="field-label"><Link size={16}/> LINK DO VÍDEO</label>
      <input autoFocus value={url} onChange={e => setUrl(e.target.value)} onKeyDown={e => e.key === 'Enter' && download()} placeholder="https://www.youtube.com/watch?v=..." disabled={state === 'downloading'} />
      <button className="download" onClick={download} disabled={state === 'downloading'}>{state === 'downloading' ? <LoaderCircle className="spin" size={18}/> : <Download size={18}/>} {state === 'downloading' ? 'CONVERTENDO...' : 'BAIXAR MP3'}</button>
      <div className="folder-row"><div><label className="field-label"><FolderOpen size={16}/> SALVAR EM</label><div className="folder-path">{folder || 'Nenhuma pasta selecionada'}</div></div><button className="secondary" onClick={chooseFolder}>ALTERAR</button></div>
    </section>
    <div className={`status ${state}`}><span className="dot"/>{message}</div>
    <footer>YToMP3 <span>•</span> simples, rápido e local</footer>
  </main>
}
