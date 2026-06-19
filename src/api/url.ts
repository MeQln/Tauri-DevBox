import { invoke } from '@tauri-apps/api/core'

export const urlApi = {
  encode: (text: string, multiline: boolean) =>
    invoke<string>('url_encode', { text, multiline }),
  decode: (text: string, multiline: boolean) =>
    invoke<string>('url_decode', { text, multiline }),
}
