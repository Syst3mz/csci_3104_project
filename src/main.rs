mod corpus;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::path::PathBuf;
use std::slice::Iter;
use std::time::Instant;
use rand::{Rng};
use clap::{Parser, Subcommand, Args, ValueEnum};
use indicatif::ProgressBar;
use crate::corpus::{Corpus, KnownLength};

#[derive(Parser, Debug)]
struct Config {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Debug(DebugCommand),

    On {
        in_file: PathBuf,
        out_file: PathBuf
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
    length: usize,

    #[arg(required = true)]
    entries: usize,
}



fn main() {
    let config = Config::parse();
    match config.command {
        Commands::Debug(command) => {
            println!("Initializing dataset");
            let then = Instant::now();
            let corpus = match command.mode {
                DebugMode::Run(run_type) => {
                    let mut corpus: Corpus<HashSet<usize>> = Corpus::new();
                    match run_type.mode {
                        RunType::OneTo4=> {
                            corpus.add(HashSet::from([1]));
                            corpus.add(HashSet::from([1,2]));
                            corpus.add(HashSet::from([1,2,3]));
                            corpus.add(HashSet::from([1,2,4]));
                        }
                        RunType::Sample => {
                            corpus.add(HashSet::from([1,2]));
                            corpus.add(HashSet::from([1, 2, 3]));
                            corpus.add(HashSet::from([1, 2, 3, 4]));
                            corpus.add(HashSet::from([1, 2, 3, 4, 5]));
                            corpus.add(HashSet::from([2]));
                            corpus.add(HashSet::from([2, 3]));
                        }
                    }
                    corpus
                }
                DebugMode::Generate(args) => {
                    make_random_corpus(args.length, args.entries)
                }

            };
            println!("Finished initializing dataset, took {}ms.", (Instant::now() - then).as_millis());
            let then = Instant::now();
            let data= get_minimum_edges(&corpus);
            let now = Instant::now();

            for datum in data {
                println!("{:?}", datum)
            }

            println!("Finished crunching dataset. took {}ms", (now - then).as_millis());
        }
        Commands::On { .. } => {}
    }

}

fn make_random_corpus(max_len: usize, num: usize) -> Corpus<HashSet<usize>> {
    let mut ret:Corpus<HashSet<usize>> = Corpus::new();
    let mut rng = rand::thread_rng();
    for _i in 0..num {
        let mut set:Vec<usize> = Vec::new();
        for x in 0_usize..rng.gen_range(1..max_len+1) {
            set.push(x);
        }

        ret.add(HashSet::from_iter(set))
    }

    ret
}

fn get_minimum_edges<'a, T: Debug>(corpus: &'a Corpus<HashSet<T>>)
    -> Vec<Vec<(&'a HashSet<T>, &'a HashSet<T>)>>
    where T: Eq, T:Hash, T:Clone
    {

        let mut ret:Vec<Vec<(&'a HashSet<T>, &'a HashSet<T>)>> = Vec::new();
        let bar = ProgressBar::new(
            corpus.data.iter()
                .map(|x| x.len() as u64)
                .sum::<u64>());
        for datum in &corpus.data {
            for set in datum {
                let x = get_minimum_edges_for(corpus, set);
                if !x.is_empty() {
                    ret.push(x);
                }
                bar.inc(1);
            }
        }
        bar.finish();
        ret
    }

fn get_minimum_edges_for<'a, T: Debug>(corpus: &'a Corpus<HashSet<T>>, element: &'a HashSet<T>)
    -> Vec<(&'a HashSet<T>, &'a HashSet<T>)>
    where T: Eq, T:Hash, T:Clone
{
    let candidates = get_supersets(corpus, element);

    candidates.iter()
        .filter(|x| x.len() <= candidates[0].len())
        .map(|x| (element, *x))
        .collect()
}

fn get_supersets<'a, T: Debug>(corpus: &'a Corpus<HashSet<T>>, element: &'a HashSet<T>) -> Vec<&'a HashSet<T>>
where T: Eq, T:Hash, T:Clone
{
    let mut ret:Vec<&HashSet<T>> = Vec::new();
    let candidate_sets = corpus.get_above(element.len());
    for candidate in candidate_sets {
        for set in candidate {
            if element.is_subset(set) {
                ret.push(set)
            };
        }
    }

    ret
}

struct Tag<T> {
    number:usize,
    item:T
}

fn get_superset_table<'a, T: Debug>(corpus: &'a Corpus<HashSet<T>>) -> Vec<Vec<&'a HashSet<T>>>
    where T: Eq, T:Hash, T:Clone
{
    let mut ret:Vec<Vec<&HashSet<T>>> = Vec::new();
    for row_idx in 0..corpus.data {

    }

    ret
}
