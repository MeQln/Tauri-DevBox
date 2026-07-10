/// 将子进程输出字节解码为字符串。
///
/// - UTF-8 有效 → 直接返回（macOS / Linux / 现代 Windows 快速路径）
/// - UTF-8 无效且为 Windows → 尝试 GBK 解码（中文 Windows 常见场景）
/// - 其他情况 → `from_utf8_lossy` 兜底
pub fn decode_output(bytes: &[u8]) -> String {
    // UTF-8 快速路径
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // Windows 非 UTF-8 locale（如中文 GBK）：用 encoding_rs 解码
    // encoding_rs::decode() 返回 (Cow<str>, &Encoding, bool)，
    // 第二/三元组分别指"使用了哪个编码"和"是否有解码错误"，在此不需要。
    #[cfg(target_os = "windows")]
    {
        let (cow, _, _) = encoding_rs::GBK.decode(bytes);
        return cow.into_owned();
    }

    // 非 Windows 平台兜底（macOS/Linux 上非 UTF-8 几乎不会出现，但保底）
    #[cfg(not(target_os = "windows"))]
    String::from_utf8_lossy(bytes).to_string()
}