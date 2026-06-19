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

#[tauri::command]
pub fn qr_decode(image: Vec<u8>) -> Result<String, String> {
    let img = image::load_from_memory(&image).map_err(|_| "图片解析失败".to_string())?;
    let luma = img.to_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    if grids.is_empty() {
        return Err("未识别到二维码".into());
    }
    let (_meta, content) = grids[0].decode().map_err(|_| "未识别到二维码".to_string())?;
    Ok(content)
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

    fn make_qr_png_bytes(text: &str) -> Vec<u8> {
        use image::{ImageBuffer, Luma};
        use std::io::Cursor;
        let code = QrCode::new(text.as_bytes()).unwrap();
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = code
            .render::<Luma<u8>>()
            .module_dimensions(8, 8)
            .build();
        let mut buf = Cursor::new(Vec::new());
        image::DynamicImage::ImageLuma8(img)
            .write_to(&mut buf, image::ImageFormat::Png)
            .unwrap();
        buf.into_inner()
    }

    #[test]
    fn decode_round_trip_ascii() {
        let png = make_qr_png_bytes("hello");
        assert_eq!(qr_decode(png).expect("应解码成功"), "hello");
    }

    #[test]
    fn decode_round_trip_chinese() {
        let png = make_qr_png_bytes("你好世界");
        assert_eq!(qr_decode(png).expect("应解码成功"), "你好世界");
    }

    #[test]
    fn decode_blank_image_returns_err() {
        use image::{ImageBuffer, Luma};
        use std::io::Cursor;
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_pixel(64, 64, Luma([255]));
        let mut buf = Cursor::new(Vec::new());
        image::DynamicImage::ImageLuma8(img)
            .write_to(&mut buf, image::ImageFormat::Png)
            .unwrap();
        assert!(qr_decode(buf.into_inner()).is_err());
    }

    #[test]
    fn decode_garbage_bytes_returns_err() {
        assert!(qr_decode(vec![0u8, 1, 2, 3]).is_err());
    }
}
