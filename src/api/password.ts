import { invoke } from '@tauri-apps/api/core'

export type PasswordOptions = {
  length: number
  upper: boolean
  lower: boolean
  digit: boolean
  symbol: boolean
  excludeAmbiguous: boolean
}

export const passwordApi = {
  generate: (opts: PasswordOptions, count: number) =>
    invoke<string[]>('generate_passwords', { opts, count }),
}
