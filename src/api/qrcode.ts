import { invoke } from '@tauri-apps/api/core'

export const qrApi = {
  encode: (text: string) => invoke<string>('qr_encode', { text }),
  decode: (image: number[]) => invoke<string>('qr_decode', { image }),
}
