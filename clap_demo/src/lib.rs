use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "hehe")]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let args = Args::parse();
        assert_eq!(args.name,"hehe");
        assert_eq!(args.count,1)
    }
}
