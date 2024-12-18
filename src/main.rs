use clap::Parser;
use hash_finder::calculate;

// using clap crate to get command line arguments
#[derive(Parser, Debug)]
struct Args {
    // number of zeros in the end of hash
    #[arg(short = 'N')]
    zeros: usize,
    // number of hashes we want to calculate
    #[arg(short = 'F')]
    dst_count: usize,
}

fn main() {
    let args = Args::parse();
    // calculates hashes in multi-thread using rayon crate and
    // writes it to stdout immediately
    calculate(&args.zeros, &args.dst_count, None);
}
