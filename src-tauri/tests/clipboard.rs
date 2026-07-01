// 运行时验证：通过 mock Tauri app 调用 clipboard-manager 插件的 read_text / write_text，
// 证明剪贴板读写在这台机器的运行时确实生效。
// 粘贴按钮（clipboardApi.read → invoke read_text → Clipboard::read_text → arboard）
// 与复制按钮走的就是同一条代码路径。
use tauri_plugin_clipboard_manager::{init, ClipboardExt};

#[test]
fn clipboard_write_then_read_roundtrip() {
    let app = tauri::test::mock_builder()
        .plugin(init())
        .build(tauri::generate_context!())
        .expect("mock app builds");

    let cb = app.clipboard();
    let marker = "DevBox-clipboard-runtime-check-Ψ";
    cb.write_text(marker).expect("write_text should succeed");

    let got = cb.read_text().expect("read_text should succeed");
    assert_eq!(got, marker, "clipboard roundtrip must preserve text");
}
