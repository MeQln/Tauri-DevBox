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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    /// 构造一个 TCP socket 字面量，便于测试打桩。
    /// remote_addr/remote_port 对 build_entries 无意义（非 LISTEN 直接被过滤，
    /// LISTEN 不读 remote 字段），统一给占位值。
    /// Linux/Android 上 SocketInfo 多两个 cfg 字段 inode/uid，在此一并填占位 0，
    /// 让所有测试字面量只经此 helper 构造，cfg 字段只写一次。
    fn tcp_state(
        addr: &str,
        port: u16,
        pids: Vec<u32>,
        state: netstat2::TcpState,
    ) -> netstat2::SocketInfo {
        netstat2::SocketInfo {
            protocol_socket_info: netstat2::ProtocolSocketInfo::Tcp(netstat2::TcpSocketInfo {
                local_addr: addr.parse::<IpAddr>().unwrap(),
                local_port: port,
                remote_addr: "0.0.0.0".parse::<IpAddr>().unwrap(),
                remote_port: 0,
                state,
            }),
            associated_pids: pids,
            #[cfg(any(target_os = "linux", target_os = "android"))]
            inode: 0,
            #[cfg(any(target_os = "linux", target_os = "android"))]
            uid: 0,
        }
    }

    /// 构造一个 TCP LISTEN socket（tcp_state 的常用包装）。
    fn tcp_listen(addr: &str, port: u16, pids: Vec<u32>) -> netstat2::SocketInfo {
        tcp_state(addr, port, pids, netstat2::TcpState::Listen)
    }

    /// 非 LISTEN 状态的 socket 必须被过滤掉。
    #[test]
    fn build_entries_filters_non_listen() {
        let sockets = vec![
            tcp_listen("0.0.0.0", 8080, vec![100]),
            tcp_state(
                "1.2.3.4",
                9090,
                vec![200],
                netstat2::TcpState::Established,
            ),
        ];
        let sys = sysinfo::System::new();
        let result = build_entries(sockets, &sys);
        assert_eq!(result.len(), 1, "Established 应被过滤，只留 LISTEN");
        assert_eq!(result[0].port, 8080);
        assert_eq!(result[0].pid, 100);
    }

    /// 进程表查不到 pid 时，process_name 回退为 "unknown"。
    #[test]
    fn build_entries_unknown_process_name() {
        let sockets = vec![tcp_listen("0.0.0.0", 443, vec![999_999])];
        let sys = sysinfo::System::new(); // 空进程表，任何 pid 都查不到
        let result = build_entries(sockets, &sys);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].pid, 999_999);
        assert_eq!(result[0].process_name, "unknown");
    }

    /// 同端口不同地址（不同 PID）两行都保留，不去重。
    #[test]
    fn build_entries_keeps_same_port_multi_address() {
        let sockets = vec![
            tcp_listen("0.0.0.0", 8080, vec![100]),
            tcp_listen("127.0.0.1", 8080, vec![200]),
        ];
        let sys = sysinfo::System::new();
        let result = build_entries(sockets, &sys);
        assert_eq!(result.len(), 2, "同端口不同地址应保留两行");
        assert_eq!(result[0].port, 8080);
        assert_eq!(result[1].port, 8080);
        assert_ne!(result[0].address, result[1].address);
        assert_ne!(result[0].pid, result[1].pid);
    }
}
