use clap::Parser;
use const_str::parse;
use puniyu_common::app as puniyu_common;
use puniyu_logger::{LoggerOptions, init as log_init};
use std::{
	net::{IpAddr, Ipv4Addr},
	path::Path,
};

static VERSION: puniyu_semver::Version = puniyu_semver::Version::new(
	parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
	parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
	parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
);

const NAME: &str = "Puniyu Server";

#[derive(Parser)]
#[command(
	name = "puniyu-server",
	about = "Puniyu 服务器",
	disable_help_flag = true,
	help_template = "{about}\n\n用法: {usage}\n\n选项:\n{options}"
)]
struct Args {
	/// 监听地址
	#[arg(short, long)]
	host: Option<IpAddr>,
	/// 监听端口
	#[arg(short, long)]
	port: Option<u16>,
	/// 显示帮助
	#[arg(long, action = clap::ArgAction::Help)]
	help: Option<bool>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let args = Args::parse();

	let info = puniyu_common::AppInfo::new(NAME, &VERSION, Path::new("."));
	puniyu_common::set_app_info(info);
	log_init(Some(LoggerOptions::default().with_prefix(NAME)));
	puniyu_server::run_server(
		args.host.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
		args.port.unwrap_or(33700),
	)
	.await?;
	Ok(())
}
