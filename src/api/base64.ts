import { invoke } from '@tauri-apps/api/core'

export const base64Api = {
  encode: (text: string, urlSafe: boolean) =>
    invoke<string>('base64_encode', { text, urlSafe }),
  decode: (text: string, urlSafe: boolean) =>
    invoke<string>('base64_decode', { text, urlSafe }),
}
