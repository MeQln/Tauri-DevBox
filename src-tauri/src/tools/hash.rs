// 哈希 / 校验：对文本或文件字节计算 MD5 / SHA1 / SHA256 / SHA384 / SHA512。
// 三种入口：hash_text（文本）、hash_bytes（前端拖拽读到的字节）、
// hash_file（文件路径，64KB 分块流式读取，避免大文件占内存）。
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

#[derive(serde::Serialize)]
pub struct HashResult {
    pub size: u64,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha384: String,
    pub sha512: String,
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
}

fn compute_hash(data: &[u8]) -> HashResult {
    HashResult {
        size: data.len() as u64,
        md5: hex(Md5::digest(data)),
        sha1: hex(Sha1::digest(data)),
        sha256: hex(Sha256::digest(data)),
        sha384: hex(Sha384::digest(data)),
        sha512: hex(Sha512::digest(data)),
    }
}

#[tauri::command]
pub fn hash_text(text: String) -> HashResult {
    compute_hash(text.as_bytes())
}

#[tauri::command]
pub fn hash_bytes(bytes: Vec<u8>) -> HashResult {
    compute_hash(&bytes)
}

#[tauri::command]
pub fn hash_file(path: String) -> Result<HashResult, String> {
    use std::io::Read;
    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha384 = Sha384::new();
    let mut sha512 = Sha512::new();
    let mut file = std::fs::File::open(&path).map_err(|e| format!("打开文件失败: {e}"))?;
    let mut buf = [0u8; 65536];
    let mut size: u64 = 0;
    loop {
        let n = file.read(&mut buf).map_err(|e| format!("读取文件失败: {e}"))?;
        if n == 0 {
            break;
        }
        size += n as u64;
        md5.update(&buf[..n]);
        sha1.update(&buf[..n]);
        sha256.update(&buf[..n]);
        sha384.update(&buf[..n]);
        sha512.update(&buf[..n]);
    }
    Ok(HashResult {
        size,
        md5: hex(md5.finalize()),
        sha1: hex(sha1.finalize()),
        sha256: hex(sha256.finalize()),
        sha384: hex(sha384.finalize()),
        sha512: hex(sha512.finalize()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_md5_of_abc() {
        // md5("abc") = 900150983cd24fb0d6963f7d28e17f72
        assert_eq!(hash_text("abc".to_string()).md5, "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn known_sha1_of_abc() {
        assert_eq!(hash_text("abc".to_string()).sha1, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn known_sha256_of_abc() {
        assert_eq!(
            hash_text("abc".to_string()).sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn bytes_match_text() {
        // 同一内容的字节哈希应与文本哈希一致
        let a = hash_text("hello".to_string());
        let b = hash_bytes(b"hello".to_vec());
        assert_eq!(a.md5, b.md5);
        assert_eq!(a.sha512, b.sha512);
    }

    #[test]
    fn file_hash_matches_text() {
        // 写临时文件 "abc"，文件流式哈希应等于文本哈希
        let path = std::env::temp_dir().join("devbox_hash_test.txt");
        std::fs::write(&path, "abc").unwrap();
        let r = hash_file(path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(r.md5, "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(r.sha256, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(r.size, 3);
        std::fs::remove_file(&path).ok();
    }
}
