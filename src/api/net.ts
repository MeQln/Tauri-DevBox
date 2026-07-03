import { invoke } from '@tauri-apps/api/core'

export type PortCheckResult = {
  host: string
  port: number
  open: boolean
  latency_ms: number
  message: string
}

export const netApi = {
  ping: (host: string) => invoke<boolean>('ping_host', { host }),
  checkPort: (host: string, port: number) =>
    invoke<PortCheckResult>('check_port', { host, port }),
}
