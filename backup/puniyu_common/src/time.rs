use puniyu_system_info::ProcessInfo;

/// 获取应用运行时间
///
/// # 返回
///
/// * `u64` - 应用运行时间，时间戳，单位秒
pub fn uptime() -> u64 {
	ProcessInfo::default().run_time
}
