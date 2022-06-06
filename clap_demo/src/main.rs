use clap::Parser;
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "hehe")]
    name: String,

    /// Number of times to greet
    #[clap(short, long)]
    count: Option<u8>,
}
fn main() {
    let args = Args::parse();
    assert_eq!(args.name, "hehe");
    assert_eq!(args.count, None);
}
