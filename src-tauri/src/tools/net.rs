use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// ping 单行输出事件载荷。
#[derive(Serialize, Clone)]
pub struct PingLine {
    pub host: String,
    pub line: String,
}

/// TCP 端口连通性结果。
#[derive(Serialize, Clone)]
pub struct PortCheckResult {
    pub host: String,
    pub port: u16,
    pub open: bool,
    pub latency_ms: u64,
    pub message: String,
}

/// 按目标平台拼装 ping 参数。
/// -W 在 macOS 是毫秒、Linux 是秒、Windows 用 -w 毫秒，三者单位不同，必须分支。
fn ping_args(host: &str) -> Vec<&str> {
    if cfg!(target_os = "windows") {
        vec!["-n", "4", "-w", "2000", host]
    } else if cfg!(target_os = "macos") {
        vec!["-c", "4", "-W", "2000", host]
    } else {
        vec!["-c", "4", "-W", "2", host]
    }
}

/// ICMP ping 主机。逐行读取系统 ping 的 stdout，通过 `ping:line` 事件实时推送；
/// 命令返回 alive（ping 退出码是否成功）。
#[tauri::command]
pub fn ping_host(app: AppHandle, host: String) -> Result<bool, String> {
    let args = ping_args(&host);
    let mut cmd = Command::new("ping");
    cmd.args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    // Windows: 不弹出控制台窗口
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("执行 ping 失败: {}", e))?;

    if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout);
        let mut buf = Vec::new();
        loop {
            match reader.read_until(b'\n', &mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    // 去除尾部 \r\n / \n
                    while matches!(buf.last(), Some(b'\n' | b'\r')) {
                        buf.pop();
                    }
                    if buf.is_empty() {
                        continue;
                    }
                    // 使用 lossy 转换支持非 UTF-8 编码（如中文 Windows 的 GBK）
                    let line = String::from_utf8_lossy(&buf).to_string();
                    buf.clear();
                    let _ = app.emit(
                        "ping:line",
                        PingLine {
                            host: host.clone(),
                            line,
                        },
                    );
                }
                Err(e) => {
                    eprintln!("读取 ping 输出错误: {}", e);
                    break;
                }
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    Ok(status.success())
}

/// TCP 端口连通性测试。3 秒超时内能否完成三次握手，返回结果。
#[tauri::command]
pub fn check_port(host: String, port: u16) -> Result<PortCheckResult, String> {
    use std::net::ToSocketAddrs;
    let addr = format!("{}:{}", host, port);
    let target = match addr.to_socket_addrs().ok().and_then(|mut it| it.next()) {
        Some(a) => a,
        None => {
            return Ok(PortCheckResult {
                host,
                port,
                open: false,
                latency_ms: 0,
                message: "无法解析地址".to_string(),
            });
        }
    };
    let start = Instant::now();
    let res = TcpStream::connect_timeout(&target, Duration::from_secs(3));
    let latency_ms = start.elapsed().as_millis() as u64;
    Ok(match res {
        Ok(_) => PortCheckResult {
            host,
            port,
            open: true,
            latency_ms,
            message: "连接成功".to_string(),
        },
        Err(e) => PortCheckResult {
            host,
            port,
            open: false,
            latency_ms,
            message: e.to_string(),
        },
    })
}
