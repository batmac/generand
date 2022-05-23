use generand::{consts::*, generate::*};

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short, default_value="12")]
    size: usize,
    #[clap(long, short, group="dict")]
    full: bool,
    #[clap(long, short, group="dict")]
    cats: bool,
    #[clap(long, short, group="dict")]
    hex: bool,
    #[clap(long, short, group="dict")]
    numeric: bool,
    #[clap(group="dict")]
    dictionary: Option<String>, 
}

fn main() {
    let cli = Cli::parse();

    let s= if let Some(x)=cli.dictionary {
        x
    } else if cli.full {
        SEVENBITS.to_string()
    } else if cli.cats {
        CATS.to_string()
    } else if cli.hex {
        LOW_HEX.to_string()
    } else if cli.numeric {
        NUMERIC.to_string()
    } else {
        ALPHANUMERIC.to_string()
    };

    let generated = generate(&s, cli.size);
    println!("{}", &generated);
}
