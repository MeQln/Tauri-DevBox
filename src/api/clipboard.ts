import { readText, writeText } from '@tauri-apps/plugin-clipboard-manager'

export const clipboardApi = {
  read: () => readText(),
  write: (text: string) => writeText(text),
}
