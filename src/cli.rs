use clap::Parser;

#[derive(Parser)]
pub struct CLIArgs {
    /// refresh time in milliseconds
    #[clap(short='i',long="interval",default_value_t=500)
    interval: u32, 
}
