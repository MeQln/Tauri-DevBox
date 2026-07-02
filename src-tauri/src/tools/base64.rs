use base64::{engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}, Engine};

// 编码：UTF-8 字节 → base64。url_safe 时用 URL_SAFE_NO_PAD（+→- / →_ 、去 = 填充）。
#[tauri::command]
pub fn base64_encode(text: String, url_safe: bool) -> String {
    if url_safe {
        URL_SAFE_NO_PAD.encode(text.as_bytes())
    } else {
        STANDARD.encode(text.as_bytes())
    }
}

// 解码：base64 → 字节 → UTF-8 字符串。解码失败或非合法 UTF-8 返回原文
// （与 url::decode_one 一致，遵循项目「错误处理反原则」：不 panic / 不 Result::Err）。
// 先剥离所有空白（RFC 4648 §3.1 允许折行换行，引擎本身不容忍非字母表字符）。
#[tauri::command]
pub fn base64_decode(text: String, url_safe: bool) -> String {
    let stripped: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let decoded = if url_safe {
        URL_SAFE_NO_PAD.decode(stripped.as_bytes())
    } else {
        STANDARD.decode(stripped.as_bytes())
    };
    match decoded {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or(text),
        Err(_) => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_roundtrip_with_unicode() {
        let s = "中文 abc!".to_string();
        let encoded = base64_encode(s.clone(), false);
        assert_eq!(base64_decode(encoded, false), s);
    }

    #[test]
    fn url_safe_charset_excludes_special() {
        // 选中一段会产出 + / 的输入，验证 URL Safe 输出不含 + / =
        let s = "???>>>@@@###".to_string();
        let std = base64_encode(s.clone(), false);
        assert!(std.contains('+') || std.contains('/') || std.contains('='));
        let safe = base64_encode(s, true);
        assert!(!safe.contains('+'));
        assert!(!safe.contains('/'));
        assert!(!safe.contains('='));
        // URL Safe 仍可 roundtrip
        assert_eq!(base64_decode(safe, true), "???>>>@@@###");
    }

    #[test]
    fn decode_invalid_returns_original() {
        // 含非法 base64 字符：解码失败返回原文
        let r = base64_decode("!!!not-base64!!!".to_string(), false);
        assert_eq!(r, "!!!not-base64!!!");
    }

    #[test]
    fn decode_ignores_wrapped_whitespace() {
        // RFC 4648 §3.1 允许折行换行：含 \n / 空格 的 base64 仍应正常解码
        let s = "中文 abc!".to_string();
        let encoded = base64_encode(s.clone(), false);
        // 每 4 字符插一个换行/空格，模拟终端 76 列折行
        let wrapped: String = encoded
            .as_bytes()
            .chunks(4)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(base64_decode(wrapped, false), s);
    }
}
