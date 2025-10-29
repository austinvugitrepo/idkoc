use clap::Parser;

#[derive(Parser, Debug)] // derive works with structs, enums, etc.
#[command(version, long_version = "v0.1.1\nLicense:CC0 1.0 Universal\ncreated by: Austin Vu", about = "idkoc, the best CLI image formatter", long_about = "idkoc (I dont know or care) is a modern CLI image formatter\nwritten in Rust designed to be an all in one image tool")]

struct CliArgs {

    /// webp image format
    #[arg(short, long)]
    webp: String,

    /// png image format
    #[arg(short, long)]
    png: String,

}

fn main() {
    let _args = CliArgs::parse();

}
