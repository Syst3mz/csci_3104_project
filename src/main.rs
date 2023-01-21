#![feature(portable_simd)]

mod corpus;
mod bitset;
mod string_wrapped;

use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use rand::{Rng};
use clap::{Parser, Subcommand, Args, ValueEnum};
use indicatif::ProgressBar;
use crate::bitset::{Bitset, BitsetBuilder};
use crate::corpus::{Corpus, KnownLength};
use crate::string_wrapped::StringWrapped;


#[derive(Parser, Debug)]
struct Config {
    #[command(subcommand)]
    command: Commands,
    #[arg(short)]
    out_file: PathBuf
}

#[derive(Debug, Subcommand)]
enum Commands {
    Debug(DebugCommand),
    On {
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



fn main() {
    let config = Config::parse();
    println!("Initializing dataset");
    let then = Instant::now();
    let corpus = match config.command {
        Commands::Debug(command) => {
            match command.mode {
                DebugMode::Run(run_type) => {
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
                    make_random_corpus(args.length, args.entries)
                }
            }
        }
        Commands::On { in_file } => {
            read_into_corpus(in_file)
        }
    };
    println!("Finished initializing dataset, took {}ms. Crunching dataset.", (Instant::now() - then).as_millis());
    let then = Instant::now();
    let data= get_minimum_edges(&corpus);
    println!("Finished crunching dataset, took {}ms. Writing data.", ( Instant::now() - then).as_millis());
    let then = Instant::now();
    write_to_output(data, config.out_file);
    println!("Finished! (took {}ms)", (Instant::now() - then).as_millis())
}


fn write_to_output(data: Vec<(&StringWrapped<Bitset>, &StringWrapped<Bitset>)>, path: PathBuf) {
    let mut ret = String::new();
    for edge in data {
        ret.push_str(&format!("{}->{}\n", edge.0.to_string().replace(" ", ", "), edge.1.to_string().replace(" ", ", ")))
    }
    fs::write(&path, ret).expect(&format!("Unable to write to {:?}", &path));
}

fn read_into_corpus(path: PathBuf) -> Corpus<StringWrapped<Bitset>> {
    let mut ret:Corpus<StringWrapped<Bitset>> = Corpus::new();
    let mut builder = BitsetBuilder::<u32>::new();
    let contents = fs::read_to_string(&path).expect(&format!("Unable to read from file file at {:?}", &path));
    for line in contents.lines() {
        let mut corp_line: Vec<u32> = Vec::new();
        for number in line.split(" ") {
            corp_line.push(number.parse::<u32>().expect("Unable to read numbers."));
        }


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

fn get_minimum_edges<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>)
    -> Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)>
    {
        let mut ret:Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)> = Vec::new();
        let bar = ProgressBar::new(
            corpus.data.iter()
                .map(|x| x.len() as u64)
                .sum::<u64>());
        for datum in &corpus.data {
            for set in datum {
                let x = get_minimum_edges_for(corpus, set);
                if !x.is_empty() {
                    for x in x {
                        ret.push(x);
                    }
                }
                bar.inc(1);
            }
        }
        bar.finish();
        ret
    }

fn get_minimum_edges_for<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>, element: &'a StringWrapped<Bitset>)
    -> Vec<(&'a StringWrapped<Bitset>, &'a StringWrapped<Bitset>)>
{
    let candidates = get_supersets(corpus, element);

    candidates.iter()
        .filter(|x| x.len() <= candidates[0].len())
        .map(|x| (element, *x))
        .collect()
}

fn get_supersets<'a>(corpus: &'a Corpus<StringWrapped<Bitset>>, element: &'a StringWrapped<Bitset>) -> Vec<&'a StringWrapped<Bitset>>
{
    corpus.get_above(element.len()).filter(|x| element.is_subset(*x)).collect::<Vec<&'a StringWrapped<Bitset>>>()
}


