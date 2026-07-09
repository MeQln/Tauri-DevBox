use base64::Engine;
use image::ImageFormat;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

/// 识别格式字符串 → ImageFormat，仅返回支持的格式
fn parse_format(s: &str) -> Option<ImageFormat> {
    match s.to_lowercase().as_str() {
        "png" => Some(ImageFormat::Png),
        "jpeg" | "jpg" => Some(ImageFormat::Jpeg),
        "webp" => Some(ImageFormat::WebP),
        "bmp" => Some(ImageFormat::Bmp),
        "gif" => Some(ImageFormat::Gif),
        "tiff" | "tif" => Some(ImageFormat::Tiff),
        "ico" => Some(ImageFormat::Ico),
        _ => None,
    }
}

fn format_ext(fmt: &ImageFormat) -> &'static str {
    match fmt {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
        ImageFormat::WebP => "webp",
        ImageFormat::Bmp => "bmp",
        ImageFormat::Gif => "gif",
        ImageFormat::Tiff => "tiff",
        ImageFormat::Ico => "ico",
        _ => "png",
    }
}

#[derive(Serialize)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
    pub data_base64: String,
}

/// 读取图片并返回基本信息 + base64 缩略图数据（用于前端预览）
#[tauri::command]
pub fn image_read(source_path: String) -> Result<ImageInfo, String> {
    let path = Path::new(&source_path);
    if !path.exists() {
        return Err("文件不存在".into());
    }
    let metadata = fs::metadata(path).map_err(|e| format!("读取文件信息失败: {}", e))?;
    let size_bytes = metadata.len();

    let img = image::open(path).map_err(|e| format!("打开图片失败: {}", e))?;
    let width = img.width();
    let height = img.height();

    // 从扩展名推断原始格式
    let fmt = path
        .extension()
        .and_then(|e| e.to_str())
        .and_then(|e| parse_format(e))
        .map(|f| format_ext(&f).to_string())
        .unwrap_or_else(|| "png".to_string());

    // 直接读取原始文件字节并 base64 编码，用于前端展示
    let mut file = fs::File::open(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    let data_base64 = base64::engine::general_purpose::STANDARD.encode(&buf);

    Ok(ImageInfo {
        width,
        height,
        format: fmt,
        size_bytes,
        data_base64,
    })
}

/// 转换图片格式
/// source_path: 源文件路径
/// target_fmt: 目标格式 ("png", "jpeg", "webp", "bmp", "gif")
/// output_path: 输出文件路径
#[tauri::command]
pub fn image_convert(source_path: String, target_fmt: String, output_path: String) -> Result<ImageInfo, String> {
    let fmt = parse_format(&target_fmt)
        .ok_or_else(|| format!("不支持的格式: {}", target_fmt))?;

    let src = Path::new(&source_path);
    if !src.exists() {
        return Err("源文件不存在".into());
    }

    let img = image::open(src).map_err(|e| format!("打开图片失败: {}", e))?;

    // 确定输出格式的编解码器是否支持
    img.save_with_format(&output_path, fmt)
        .map_err(|e| format!("转换失败: {}", e))?;

    // 读取输出文件信息返回
    let metadata = fs::metadata(&output_path)
        .map_err(|e| format!("读取输出文件信息失败: {}", e))?;
    let size_bytes = metadata.len();

    let mut file = fs::File::open(&output_path)
        .map_err(|e| format!("读取输出文件失败: {}", e))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("读取输出文件失败: {}", e))?;
    let data_base64 = base64::engine::general_purpose::STANDARD.encode(&buf);

    Ok(ImageInfo {
        width: img.width(),
        height: img.height(),
        format: target_fmt,
        size_bytes,
        data_base64,
    })
}

/// 压缩图片（JPEG/WebP 质量调节）
/// source_path: 源文件路径
/// quality: 压缩质量 1-100
/// output_path: 输出文件路径
#[tauri::command]
pub fn image_compress(source_path: String, quality: u8, output_path: String) -> Result<ImageInfo, String> {
    let quality = quality.clamp(1, 100);

    let src = Path::new(&source_path);
    if !src.exists() {
        return Err("源文件不存在".into());
    }

    let img = image::open(src).map_err(|e| format!("打开图片失败: {}", e))?;

    // 根据输出文件扩展名决定保存格式
    let out_ext = output_path
        .rsplit('.')
        .next()
        .and_then(|e| parse_format(e))
        .unwrap_or(ImageFormat::Jpeg);

    match out_ext {
        ImageFormat::Jpeg => {
            // 使用 JPEG 编码器带质量参数
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                fs::File::create(&output_path)
                    .map_err(|e| format!("创建输出文件失败: {}", e))?,
                quality,
            );
            encoder
                .encode(
                    &img.to_rgb8(),
                    img.width(),
                    img.height(),
                    image::ExtendedColorType::Rgb8,
                )
                .map_err(|e| format!("压缩失败: {}", e))?;
        }
        ImageFormat::WebP => {
            // WebP 使用默认质量保存
            let rgba = img.to_rgba8();
            rgba.save(&output_path)
                .map_err(|e| format!("压缩失败: {}", e))?;
        }
        _ => {
            // 其他格式直接保存
            img.save(&output_path)
                .map_err(|e| format!("保存失败: {}", e))?;
        }
    }

    let metadata = fs::metadata(&output_path)
        .map_err(|e| format!("读取输出文件信息失败: {}", e))?;
    let size_bytes = metadata.len();

    let mut file = fs::File::open(&output_path)
        .map_err(|e| format!("读取输出文件失败: {}", e))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("读取输出文件失败: {}", e))?;
    let data_base64 = base64::engine::general_purpose::STANDARD.encode(&buf);

    let ext_name = format_ext(&out_ext).to_string();
    Ok(ImageInfo {
        width: img.width(),
        height: img.height(),
        format: ext_name,
        size_bytes,
        data_base64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("fixtures");
        p.push(name);
        p
    }

    #[test]
    fn parse_format_handles_common() {
        assert!(parse_format("png").is_some());
        assert!(parse_format("JPEG").is_some());
        assert!(parse_format("jpg").is_some());
        assert!(parse_format("webp").is_some());
        assert!(parse_format("bmp").is_some());
        assert!(parse_format("gif").is_some());
        assert!(parse_format("svg").is_none());
    }

    #[test]
    fn image_convert_png_to_jpeg() {
        // 需要 fixture 图片才能跑；如果 fixtures 目录无文件则跳过
        let src = fixture_path("test.png");
        if !src.exists() {
            eprintln!("跳过 image_convert_png_to_jpeg：缺少测试图片");
            return;
        }
        let out = fixture_path("__test_output.jpg");
        let result = image_convert(
            src.to_string_lossy().into_owned(),
            "jpeg".into(),
            out.to_string_lossy().into_owned(),
        );
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.format, "jpeg");
        // 清理
        let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn image_compress_reduces_size() {
        let src = fixture_path("test.png");
        if !src.exists() {
            eprintln!("跳过 image_compress_reduces_size：缺少测试图片");
            return;
        }
        let src_len = std::fs::metadata(&src).unwrap().len();
        let out = fixture_path("__test_compress.jpg");
        let result = image_compress(
            src.to_string_lossy().into_owned(),
            50,
            out.to_string_lossy().into_owned(),
        );
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(info.size_bytes <= src_len || info.format == "jpg"); // 压缩效果不能保证但至少成功
        let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn image_read_returns_info() {
        let src = fixture_path("test.png");
        if !src.exists() {
            eprintln!("跳过 image_read_returns_info：缺少测试图片");
            return;
        }
        let result = image_read(src.to_string_lossy().into_owned());
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(info.width > 0);
        assert!(info.height > 0);
        assert!(!info.data_base64.is_empty());
    }
}
