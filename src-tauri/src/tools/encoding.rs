/// 将子进程输出字节解码为字符串。
///
/// - UTF-8 有效 → 直接返回（macOS / Linux / 现代 Windows 快速路径）
/// - UTF-8 无效且为 Windows → 按系统 OEM 代码页解码（自动适配简体中文 GBK、
///   日文 Shift-JIS、韩文 EUC-KR、繁体中文 Big5、俄文 IBM866 等）
/// - 其他情况 → `from_utf8_lossy` 兜底
pub fn decode_output(bytes: &[u8]) -> String {
    // UTF-8 快速路径
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // Windows：按系统 OEM 代码页解码（console 程序使用 OEM 代码页而非 ANSI）
    #[cfg(target_os = "windows")]
    {
        return decode_windows_oem(bytes);
    }

    // 非 Windows 平台兜底
    #[cfg(not(target_os = "windows"))]
    String::from_utf8_lossy(bytes).into_owned()
}

/// 按 Windows OEM 代码页解码。
///
/// 控制台程序（ping.exe、taskkill.exe 等）输出使用系统 OEM 代码页，
/// 不同 locale 各不相同，通过映射表自动选择正确的 encoding_rs 编码。
///
/// 代码页 → encoding 对照（覆盖中日韩俄四大 CJK/Cyrillic locale）：
///
/// | 代码页 | 地区     | encoding_rs 编码 |
/// |--------|----------|------------------|
/// | 932    | 日文     | SHIFT_JIS        |
/// | 936    | 简体中文 | GBK              |
/// | 949    | 韩文     | EUC_KR           |
/// | 950    | 繁体中文 | BIG5             |
/// | 866    | 俄文     | IBM866           |
/// | 其他   | —        | from_utf8_lossy  |
#[cfg(target_os = "windows")]
fn decode_windows_oem(bytes: &[u8]) -> String {
    let cp = oem_code_page();
    let encoding = match cp {
        932 => encoding_rs::SHIFT_JIS,
        936 => encoding_rs::GBK,
        949 => encoding_rs::EUC_KR,
        950 => encoding_rs::BIG5,
        866 => encoding_rs::IBM866,
        _ => return String::from_utf8_lossy(bytes).into_owned(),
    };
    let (cow, _, _) = encoding.decode(bytes);
    cow.into_owned()
}

/// 调用 Windows API `GetOEMCP()` 获取系统 OEM 代码页。
/// 直接通过 extern FFI 声明，无需额外引入任何 Windows crate。
#[cfg(target_os = "windows")]
fn oem_code_page() -> u32 {
    extern "system" {
        /// kernel32.dll 始终可用，返回当前系统 OEM 代码页编号
        ///（如 936=GBK, 932=Shift-JIS, 949=EUC-KR, 65001=UTF-8）。
        fn GetOEMCP() -> u32;
    }
    unsafe { GetOEMCP() }
}