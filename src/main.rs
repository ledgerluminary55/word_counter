use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let bytes =  content.bytes().count();

    let lines = content.lines().count();

    let chars = content.chars().count();

    let _word = content.as_str();

    // {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path)
    // };
    println!("bytes: {}, lines: {}, chars:{} path: {:?}", bytes, lines, chars, args.path)
}