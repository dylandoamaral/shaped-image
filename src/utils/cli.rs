use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Dylan D. <do.amaral.dylan@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub input_path: String,

    #[clap(short, long)]
    pub output_path: String,

    #[clap(short, long, default_value = "100")]
    pub specimens: i32,

    #[clap(short, long, default_value = "100")]
    pub until: i32,

    #[clap(short, long, default_value = "100")]
    pub convergence: i32,
}
