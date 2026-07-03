import { invoke } from '@tauri-apps/api/core'

export const uuidApi = {
  generate: (version: number, count: number, uppercase: boolean, hyphen: boolean) =>
    invoke<string[]>('generate_uuids', { version, count, uppercase, hyphen }),
}
