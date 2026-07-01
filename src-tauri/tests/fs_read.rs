// 运行时验证：通过 mock Tauri app 注册 fs 插件，验证 read_to_string 在运行时
// 能正确读取磁盘文件内容。
// 读取文件按钮（readTextFile → invoke plugin:fs|read_text_file → Fs::read_to_string
// → std::fs::read_to_string）走的就是同一条代码路径。
use std::io::Write;
use tauri_plugin_fs::{init, FsExt};

#[test]
fn fs_read_to_string_roundtrip() {
    let dir = std::env::temp_dir();
    let path = dir.join("devbox-fs-runtime-check.txt");
    let marker = "DevBox-fs-runtime-check-Ψ\n第二行";
    {
        let mut f = std::fs::File::create(&path).expect("create temp file");
        f.write_all(marker.as_bytes()).expect("write temp file");
    }

    let app = tauri::test::mock_builder()
        .plugin(init())
        .build(tauri::generate_context!())
        .expect("mock app builds");

    let got = app.fs().read_to_string(&path).expect("read_to_string should succeed");
    assert_eq!(got, marker, "fs read_to_string must preserve file content");

    let _ = std::fs::remove_file(&path);
}
