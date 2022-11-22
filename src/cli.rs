use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dummy api",
    about = "模擬API"
)]
pub struct CommandLineArgs {
    /// 設定監聽通訊埠。
    #[structopt(short, long)]
    pub listening_port: Option<u16>,
}