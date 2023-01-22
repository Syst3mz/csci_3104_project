#![feature(portable_simd)]

// bring my files into scope
mod corpus;
mod bitset;
mod string_wrapped;

// everything prefixed by std is from the rust standard library
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

// rand is a super common library for making random data.
// It is only used for generating data for debug
use rand::{Rng};

// Command Line Argument Parser (clap) is only used to make the command line interface
// and is not relevant to the solution of the actual problem
use clap::{Parser, Subcommand, Args, ValueEnum};

// indicatif is a library to make the progress bars, and is also not relevant
// to solving the computational problem in head
use indicatif::ProgressBar;

// Because rust is well built, despite having brought my files into scope I can't use them
// unless I specify using that I am using them.
use crate::bitset::{Bitset, BitsetBuilder};
use crate::corpus::{Corpus, KnownLength};
use crate::string_wrapped::StringWrapped;

/// Everything bellow here until the main function is about the command line, and not relevant
/// to the solution to the computational problem.
/// Config is the configuration that the program should run in. it is handed to me by CLAP's magic
#[derive(Parser, Debug)]
struct Config {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, help="Path to the output file from the program")]
    out_file: PathBuf
}

// Commands is cut and paste from the CLAP cookbook, and I only mostly understand what it is doing.
// Commands is a set of subcommands, which the user can pick from.
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about="Run the program in debug mode")]
    Debug(DebugCommand),
    #[command(about="Run the program on a file")]
    On {
        #[arg(help="Path to the input file")]
        in_file: PathBuf,
    }
}

#[derive(Debug, Args)]
#[command()]
struct  DebugCommand {
    #[command(subcommand)]
    mode: DebugMode
}

#[derive(Debug, Subcommand)]
enum DebugMode {
    Generate(GenerateArgs),
    Run(RunArgs)
}
#[derive(Debug, Args)]
struct RunArgs {
    #[arg(value_enum)]
    mode: RunType
}

#[derive(Debug, ValueEnum, Clone)]
enum RunType {
    OneTo4,
    Sample
}

#[derive(Debug, Args, Clone)]
struct  GenerateArgs {
    #[arg(required = true)]
    length: u32,

    #[arg(required = true)]
    entries: u32,
}


/// The entry point for the program.
fn main() {
    // Use CLAP to get the config of the program, and write out a message to the user
    let config = Config::parse();
    println!("Initializing dataset");

    // then is used to measure timing information about how long the program has been running for
    let then = Instant::now();

    // set up the corpus I am working on, by matching which command I am using.
    let corpus = match config.command {
        // this arm is just for debugging and can be ignored for the solution
        Commands::Debug(command) => {
            match command.mode {
                // Run a specific pre-programmed run of data.
                DebugMode::Run(run_type) => {
                    // initialize the builder and the corpus so it can be reused
                    let mut corpus: Corpus<StringWrapped<Bitset>> = Corpus::new();
                    let mut builder = BitsetBuilder::<u32>::new();


                    match run_type.mode {
                        RunType::OneTo4=> {
                            corpus.add(StringWrapped {payload: String::from("1"), internal: builder.add(vec![1])});
                            corpus.add(StringWrapped {payload: String::from("1, 2"), internal: builder.add(vec![1, 2])});
                            corpus.add(StringWrapped {payload: String::from("1, 2, 3"), internal: builder.add(vec![1, 2, 3])});
                            corpus.add(StringWrapped {payload: String::from("1, 2, 4"), internal: builder.add(vec![1, 2, 4])});
                        }
                        RunType::Sample => {
                            corpus.add(StringWrapped {payload: String::from("1, 2"), internal: builder.add(vec![1, 2])});
                            corpus.add(StringWrapped {payload: String::from("1, 2, 3"), internal: builder.add(vec![1,2 ,3])});
                            corpus.add(StringWrapped {payload: String::from("1, 2, 3, 4"), internal: builder.add(vec![1, 2, 3, 4])});
                            corpus.add(StringWrapped {payload: String::from("1, 2, 3, 4, 5"), internal: builder.add(vec![1, 2, 3, 4, 5])});
                            corpus.add(StringWrapped {payload: String::from("2"), internal: builder.add(vec![2])});
                            corpus.add(StringWrapped {payload: String::from("2, 3"), internal: builder.add(vec![2, 3])});
                        }
                    }
                    corpus
                }
                DebugMode::Generate(args) => {
                    // Generate a new random corpus with the information provided
                    make_random_corpus(args.length, args.entries)
                }
            }
        }
        Commands::On { in_file } => {
            // read the corpus from the file specified by the command line.
            read_into_corpus(in_file)
        }
    };
    println!("Finished initializing dataset, took {}ms. Crunching dataset.", (Instant::now() - then).as_millis());

    // reset then to time the the minimum subset finding
    let then = Instant::now();

    // compute the minimum subsets
    let data= get_minimum_edges(&corpus);
    println!("Finished crunching dataset, took {}ms. Writing data.", ( Instant::now() - then).as_millis());

    // reset then for timing the output writing
    let then = Instant::now();
    write_to_output(data, config.out_file);
    println!("Finished! (took {}ms)", (Instant::now() - then).as_millis())
}


// Unsurprisingly, write the found minimum subsets to a file
fn write_to_output(data: Vec<(&StringWrapped<Bitset>, &StringWrapped<Bitset>)>, path: PathBuf) {
    let mut ret = String::new();
    for edge in data {

        // replacing the " " with "," takes a not insignificant amount of time, so if this can be
        // omitted that would be great.
        ret.push_str(&format!("{}->{}\n", edge.0.to_string().replace(" ", ", "), edge.1.to_string().replace(" ", ", ")))
    }
    fs::write(&path, ret).expect(&format!("Unable to write to {:?}", &path));
}

// Unsurprisingly, reads the data from the input file
fn read_into_corpus(path: PathBuf) -> Corpus<StringWrapped<Bitset>> {
    let mut ret:Corpus<StringWrapped<Bitset>> = Corpus::new();
    let mut builder = BitsetBuilder::<u32>::new();

    let contents = fs::read_to_string(&path).expect(&format!("Unable to read from file file at {:?}", &path));

    for line in contents.lines() {
        // this represents one item in the corpus.
        let mut corp_line: Vec<u32> = Vec::new();
        for number in line.split(" ") {
            corp_line.push(number.parse::<u32>().expect("Unable to read numbers."));
        }

        // wrap it up nicely
        ret.add(StringWrapped {
                    payload: line.to_string(),
                    internal: builder.add(corp_line),
                },
        );
    }

    ret
}


fn make_random_corpus(max_len: u32, num: u32) -> Corpus<StringWrapped<Bitset>> {
    let mut ret:Corpus<StringWrapped<Bitset>> = Corpus::new();
    let mut builder = BitsetBuilder::<u32>::new();
    let mut rng = rand::thread_rng();
    for _i in 0..num {
        let mut set:Vec<u32> = Vec::new();
        let mut str = String::new();
        for x in 0..rng.gen_range(1..max_len+1) {
            set.push(x);
            str.push_str(&format!("{}, ", x))
        }
        str = str[0..str.len() - 2].to_string();


        ret.add(StringWrapped {
                    payload: str,
                    internal: builder.add(set),
                },
        )
    }

    ret
}

/// returns a vector of all minium edges across the entire corpus
fn get_minimum_edges<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>)
    -> Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)>
    {
        let mut ret:Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)> = Vec::new();
        // set up the progress bar
        let bar = ProgressBar::new(
            corpus.data.iter()
                .map(|x| x.len() as u64)
                .sum::<u64>());
        // for everything in the corpus
        for datum in &corpus.data {
            for set in datum {
                // get the edges
                let x = get_minimum_edges_for(corpus, set);
                if !x.is_empty() {
                    // push all elements of x
                    for x in x {
                        ret.push(x);
                    }
                }
                // let the progress bar know it can increment
                bar.inc(1);
            }
        }
        // let the progress bar know I'm done
        bar.finish();
        ret
    }

/// returns the vector of all minimum edges for a element.
fn get_minimum_edges_for<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>, element: &'a StringWrapped<Bitset>)
    -> Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)>
{
    let candidates = get_supersets(corpus, element);

    // turn candidates into an iterator for rust to make them fast
    candidates.iter()
        // filter only for candidates whose length <= to the minium length present in candidates
        .filter(|x| x.len() <= candidates[0].len())
        // turn them into an edge
        .map(|x| (element, *x))
        // turn that back into an array for later processing
        .collect()
}

/// returns the supersets of a given element in the corpus
fn get_supersets<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>, element: &'a StringWrapped<Bitset>) -> Vec<&'a StringWrapped<Bitset>>
{
    // get all elements in the corpus whose length is longer than element's length
    corpus.get_above(element.len())
        // filter this result to only be those elements of the corpus which are a superset of element
        .filter(|x| element.is_subset(*x))
        // turn it into a vector for later processing
        .collect::<Vec<&'a StringWrapped<Bitset>>>()
}


