extern crate rand;
extern crate regex;
extern crate docopt;
extern crate rustc_serialize;

mod stats;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::VecDeque;

use regex::Regex;
use docopt::Docopt;

use stats::MarkovStats;


const USAGE: &'static str = r"
Markov Chain Text Generator

Usage:
  markov_rust [options] <input>
  markov_rust [options] <input> <output>
  markov_rust (-h | --help)

Options:
  -o <n>, --order=<n>   Generate text with markov order n.
  -l <n>, --length=<n>  Generate n characters of text.
  -h --help             Show this screen.
  --version             Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input: String,
    arg_output: Option<String>,
    flag_order: Option<usize>,
    flag_length: Option<usize>,
}


fn main() {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let order = {
        if let Some(order) = args.flag_order {
            order
        } else {
            8
        }
    };

    let output_length = {
        if let Some(length) = args.flag_length {
            length
        } else {
            250
        }
    };

    // Read input text to a string and collapse whitespace appropriately
    let text = {
        let mut text = String::new();
        let _ = File::open(&args.arg_input).unwrap().read_to_string(&mut text);
        text = Regex::new(r"(?P<a>[^\n])\n(?P<b>[^\n])").unwrap().replace_all(&text, "$a $b");
        text = Regex::new(r" +").unwrap().replace_all(&text, " ");
        text
    };

    // Build statistics
    println!("Building statistics... ");
    let stats = MarkovStats::from_str(&text);
    println!("done.\nMax order {}.", stats.max_order());
    if let None = args.arg_output {
        println!("----------------");
    }

    // Generate hilarious text
    let mut gen_text = String::new();
    let mut window = VecDeque::new();
    window.push_back(0);
    for _ in 0..output_length {
        if let Some(c) = stats.markov_char(&gen_text[window[0]..]) {
            gen_text.push(c);
        } else {
            gen_text.push(stats.random_char());
            while window.len() > 1 {
                window.pop_front();
            }
        }

        window.push_back(gen_text.len());
        if window.len() > order {
            window.pop_front();
        }
    }

    if let Some(output_path) = args.arg_output {
        let mut f = File::create(&output_path).unwrap();
        let _ = write!(&mut f, "{}", gen_text);
    } else {
        println!("{}\n----------------", gen_text);
    }
}
