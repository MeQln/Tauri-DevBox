import { invoke } from '@tauri-apps/api/core'

export type ImageInfo = {
  width: number
  height: number
  format: string
  size_bytes: number
  data_base64: string
}

export const imageApi = {
  read: (sourcePath: string) =>
    invoke<ImageInfo>('image_read', { sourcePath }),

  convert: (sourcePath: string, targetFmt: string, outputPath: string) =>
    invoke<ImageInfo>('image_convert', { sourcePath, targetFmt, outputPath }),

  compress: (sourcePath: string, quality: number, outputPath: string) =>
    invoke<ImageInfo>('image_compress', { sourcePath, quality, outputPath }),
}