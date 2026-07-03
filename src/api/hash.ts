import { invoke } from '@tauri-apps/api/core'

export type HashResult = {
  size: number
  md5: string
  sha1: string
  sha256: string
  sha384: string
  sha512: string
}

export const hashApi = {
  text: (text: string) => invoke<HashResult>('hash_text', { text }),
  bytes: (bytes: number[]) => invoke<HashResult>('hash_bytes', { bytes }),
  file: (path: string) => invoke<HashResult>('hash_file', { path }),
}
