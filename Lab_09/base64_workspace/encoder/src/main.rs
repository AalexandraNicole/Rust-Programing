use clap::{Parser, Arg};

#[derive(Parser)]
#[command(version, about = "base64 encoder")]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    // Print information about the crate
    println!("encoder, version {}, built for {}", env!("CARGO_PKG_VERSION"), env!("CARGO_CFG_TARGET_OS"));

    // Parse command-line arguments
    let args = Args::parse();

    if let (Some(input), Some(output)) = (args.input, args.output) {
        // Read bytes from the input file and write base64 to the output file
        // Use clap for this
        println!("Reading from {} and writing to {}", input, output);
        // Implement your file reading and base64 encoding logic here
    } else {
        // Read a line from stdin and print the base64-encoded string
        println!("Enter a line to encode:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let encoded = base64::encode(input.as_bytes());
        println!("Encoded: {}", encoded);
    }
}
