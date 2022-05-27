use generand::{consts::*, generate::*};

use terminal_size::{terminal_size, Width};

use std::cmp;

use clap::Parser;
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short, default_value = "12")]
    size: usize,
    #[clap(long, short, group = "dict")]
    full: bool,
    #[clap(long, short, group = "dict")]
    cats: bool,
    #[clap(long, short, group = "dict")]
    hex: bool,
    #[clap(long, short, group = "dict")]
    digits: bool,
    #[clap(long, short, default_value = "1")]
    number: usize,
    #[clap(group = "dict")]
    dictionary: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let s = if let Some(x) = cli.dictionary {
        x
    } else if cli.full {
        SEVENBITS.to_string()
    } else if cli.cats {
        CATS.to_string()
    } else if cli.hex {
        LOW_HEX.to_string()
    } else if cli.digits {
        NUMERIC.to_string()
    } else {
        ALPHANUMERIC.to_string()
    };

    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        1
    };

    let max_per_line = cmp::max(1, term_width / (cli.size + 1));
    // eprintln!("max per line {max_per_line}");

    let mut to_display = cli.number;
    while to_display > 0 {
        //eprintln!("to display: {to_display}");
        for _ in 0..max_per_line {
            let generated = generate(&s, cli.size);
            print!("{} ", &generated);
            to_display -= 1;
            if to_display == 0 {
                break;
            }
        }
        println!()
    }
}
