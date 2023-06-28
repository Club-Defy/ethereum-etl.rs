use clap::{Parser};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
   #[arg(short = 's')]
    pub start_block: Option<u64>,

    #[arg(short='e')]
    pub end_block: Option<u64>,

    #[arg(short='l')]
    pub lag : Option<u64>,

    #[arg(short='b')]
    pub batch_size: Option<u64>

}


















