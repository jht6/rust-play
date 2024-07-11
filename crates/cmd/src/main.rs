use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "app", version = "1.0.0", next_display_order = None)]
pub struct App {
    #[command(subcommand)]
    pub cmd: Option<AppSubcommand>,
}

#[derive(Subcommand, Debug)]
pub enum AppSubcommand {
    #[command(visible_alias = "px")]
    PraiseXian,
}

fn main() {
    let app = App::parse();

    if let Some(ref cmd) = app.cmd {
        match cmd {
            AppSubcommand::PraiseXian => {
                println!("xian666");
            }
        }
    }
}
