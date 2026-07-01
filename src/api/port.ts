import { invoke } from '@tauri-apps/api/core'

export type PortEntry = {
  port: number
  pid: number
  process_name: string
  address: string
}

export const portApi = {
  list: () => invoke<PortEntry[]>('list_ports'),
  kill: (pid: number) => invoke<void>('kill_port', { pid }),
}
