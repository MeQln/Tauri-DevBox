use qrcode::QrCode;
use qrcode::render::svg;

#[tauri::command]
pub fn qr_encode(text: String) -> Result<String, String> {
    if text.is_empty() {
        return Err("文本为空".into());
    }
    let code = QrCode::new(text.as_bytes())
        .map_err(|_| "文本过长，无法生成".to_string())?;
    Ok(code.render::<svg::Color>().build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ascii_returns_svg() {
        let r = qr_encode("hello".into()).expect("应成功");
        assert!(r.contains("<svg"), "返回值应是 SVG markup");
    }

    #[test]
    fn encode_chinese_returns_svg() {
        let r = qr_encode("你好世界".into()).expect("应成功");
        assert!(r.contains("<svg"));
    }

    #[test]
    fn encode_empty_returns_err() {
        assert!(qr_encode("".into()).is_err());
    }
}
