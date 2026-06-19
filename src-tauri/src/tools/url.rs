use percent_encoding::{utf8_percent_encode, percent_decode_str, AsciiSet, CONTROLS};

// 与 JS encodeURIComponent 完全一致：不编码 A-Z a-z 0-9 - _ . ! ~ * ' ( )
const COMPONENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ').add(b'"').add(b'#').add(b'$').add(b'%').add(b'&')
    .add(b'+').add(b',').add(b'/').add(b':').add(b';').add(b'<')
    .add(b'=').add(b'>').add(b'?').add(b'@').add(b'[').add(b'\\')
    .add(b']').add(b'^').add(b'`').add(b'{').add(b'|').add(b'}');

fn encode_one(s: &str) -> String {
    utf8_percent_encode(s, COMPONENT_ENCODE_SET).to_string()
}

fn decode_one(s: &str) -> String {
    match percent_decode_str(s).decode_utf8() {
        Ok(cow) => cow.into_owned(),
        Err(_) => s.to_string(),
    }
}

#[tauri::command]
pub fn url_encode(text: String, multiline: bool) -> String {
    if multiline {
        text.split('\n').map(encode_one).collect::<Vec<_>>().join("\n")
    } else {
        encode_one(&text)
    }
}

#[tauri::command]
pub fn url_decode(text: String, multiline: bool) -> String {
    if multiline {
        text.split('\n').map(decode_one).collect::<Vec<_>>().join("\n")
    } else {
        decode_one(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_space_as_pct20() {
        assert_eq!(url_encode("hello world".to_string(), false), "hello%20world");
    }

    #[test]
    fn does_not_encode_unreserved() {
        // 与 JS encodeURIComponent 一致：- _ . ! ~ * ' ( ) 不编码
        assert_eq!(url_encode("a-_.!~*'()".to_string(), false), "a-_.!~*'()");
    }

    #[test]
    fn multiline_each_line_independently() {
        let input = "hello world\nfoo bar".to_string();
        assert_eq!(url_encode(input, true), "hello%20world\nfoo%20bar");
    }

    #[test]
    fn decode_invalid_returns_original() {
        // %zz 非法百分号序列：percent_decode_str 保留为字面量
        let r = url_decode("%zz".to_string(), false);
        assert_eq!(r, "%zz");
    }

    #[test]
    fn decode_roundtrip() {
        let s = "中文 abc!".to_string();
        let encoded = url_encode(s.clone(), false);
        assert_eq!(url_decode(encoded, false), s);
    }
}
