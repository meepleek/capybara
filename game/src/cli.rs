use crate::screens::Screen;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub screen: Option<Screen>,
}
