use serde::Serialize;

/// 一条监听端口占用记录。只保留有用字段，去掉恒为 TCP / LISTEN 的冗余列。
#[derive(Serialize, Clone)]
pub struct PortEntry {
    /// 监听端口，排序键
    pub port: u16,
    pub pid: u32,
    /// 进程名；sysinfo 查不到时回退 "unknown"
    pub process_name: String,
    /// 绑定地址 "127.0.0.1" / "0.0.0.0" / "::1"
    pub address: String,
}

/// 把 netstat2 原始 socket 列表过滤 + 提取 + 查进程名，组装成 PortEntry 列表。
///
/// 抽成纯函数便于单测打桩：入参是 netstat2 产出的 socket 列表与 sysinfo 进程表，
/// 不触碰任何外部状态，相同输入产出相同输出。
fn build_entries(
    sockets: Vec<netstat2::SocketInfo>,
    processes: &sysinfo::System,
) -> Vec<PortEntry> {
    let mut out = Vec::new();
    for sock in sockets {
        // 只取 TCP；UDP 直接跳过
        let tcp = match sock.protocol_socket_info {
            netstat2::ProtocolSocketInfo::Tcp(tcp) => tcp,
            netstat2::ProtocolSocketInfo::Udp(_) => continue,
        };
        // 仅保留 LISTEN 状态的监听 socket
        if tcp.state != netstat2::TcpState::Listen {
            continue;
        }
        if tcp.local_port == 0 {
            continue;
        }
        // associated_pids 在 SocketInfo 顶层（非 TcpSocketInfo）；
        // 取第一个非 0 的 PID 作为占用进程
        let pid = match sock.associated_pids.iter().copied().find(|p| *p != 0) {
            Some(p) => p,
            None => continue,
        };
        let process_name = processes
            .process(sysinfo::Pid::from_u32(pid))
            .map(|p| p.name().to_string_lossy().into_owned())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| "unknown".to_string());
        out.push(PortEntry {
            port: tcp.local_port,
            pid,
            process_name,
            address: tcp.local_addr.to_string(),
        });
    }
    out
}

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortEntry>, String> {
    let af_flags = netstat2::AddressFamilyFlags::IPV4 | netstat2::AddressFamilyFlags::IPV6;
    let proto_flags = netstat2::ProtocolFlags::TCP;
    let sockets = netstat2::get_sockets_info(af_flags, proto_flags).map_err(|e| e.to_string())?;
    let sys = sysinfo::System::new_all();
    Ok(build_entries(sockets, &sys))
}

#[tauri::command]
pub fn kill_port(pid: u32) -> Result<(), String> {
    let sys = sysinfo::System::new_all();
    let sys_pid = sysinfo::Pid::from_u32(pid);
    match sys.process(sys_pid) {
        Some(proc) => {
            // Unix: kill_with(Signal::Term) 发 SIGTERM（优雅结束）
            // Windows: kill_with(Term) 返回 None（Windows 无 SIGTERM 语义），
            //          fallback 到 kill()（TerminateProcess，强制）—— 平台固有限制，不报错
            let ok = proc
                .kill_with(sysinfo::Signal::Term)
                .unwrap_or_else(|| proc.kill());
            if ok {
                Ok(())
            } else {
                Err("结束失败".to_string())
            }
        }
        None => Err(format!("进程 {} 不存在", pid)),
    }
}
