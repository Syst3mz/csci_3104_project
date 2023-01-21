#![feature(portable_simd)]

mod corpus;
mod almost_set;
mod bitset;

use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::time::Instant;
use rand::{Rng};
use clap::{Parser, Subcommand, Args, ValueEnum};
use indicatif::ProgressBar;
use crate::almost_set::AlmostSet;
use crate::corpus::{Corpus, KnownLength};


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
    length: u16,

    #[arg(required = true)]
    entries: u16,
}



fn main() {
    let config = Config::parse();
    println!("Initializing dataset");
    let then = Instant::now();
    let corpus = match config.command {
        Commands::Debug(command) => {
            match command.mode {
                DebugMode::Run(run_type) => {
                    let mut corpus: Corpus<AlmostSet<u16>> = Corpus::new();
                    match run_type.mode {
                        RunType::OneTo4=> {
                            corpus.add(AlmostSet::new(vec![1]));
                            corpus.add(AlmostSet::new(vec![1,2]));
                            corpus.add(AlmostSet::new(vec![1,2,3]));
                            corpus.add(AlmostSet::new(vec![1,2,4]));
                        }
                        RunType::Sample => {
                            corpus.add(AlmostSet::new(vec![1,2]));
                            corpus.add(AlmostSet::new(vec![1, 2, 3]));
                            corpus.add(AlmostSet::new(vec![1, 2, 3, 4]));
                            corpus.add(AlmostSet::new(vec![1, 2, 3, 4, 5]));
                            corpus.add(AlmostSet::new(vec![2]));
                            corpus.add(AlmostSet::new(vec![2, 3]));
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


fn write_to_output(data: Vec<(&AlmostSet<u16>, &AlmostSet<u16>)>, path: PathBuf) {
    let mut ret = String::new();
    for edge in data {
        ret.push_str(&format!("{}->{}\n", edge.0.to_string(), edge.1.to_string()))
    }
    fs::write(&path, ret).expect(&format!("Unable to write to {:?}", &path));
}

fn read_into_corpus(path: PathBuf) -> Corpus<AlmostSet<u16>> {
    let mut ret:Corpus<AlmostSet<u16>> = Corpus::new();
    let contents = fs::read_to_string(&path).expect(&format!("Unable to read from file file at {:?}", &path));
    for line in contents.lines() {
        let mut corp_line: Vec<u16> = Vec::new();
        for number in line.split(" ") {
            corp_line.push(number.parse::<u16>().expect("Unable to read numbers."));
        }

        ret.add(AlmostSet::new(corp_line));
    }

    ret
}

fn make_random_corpus(max_len: u16, num: u16) -> Corpus<AlmostSet<u16>> {
    let mut ret:Corpus<AlmostSet<u16>> = Corpus::new();
    let mut rng = rand::thread_rng();
    for _i in 0..num {
        let mut set:Vec<u16> = Vec::new();
        for x in 0_u16..rng.gen_range(1..max_len+1) {
            set.push(x);
        }

        ret.add(AlmostSet::new(set))
    }

    ret
}

fn get_minimum_edges<'a, T: Debug>(corpus: &'a Corpus<AlmostSet<T>>)
    -> Vec<(&'a AlmostSet<T>, &'a AlmostSet<T>)>
    where T: Eq, T:Hash, T:Clone, T:Ord
    {
        let mut ret: Vec<(&'a AlmostSet<T>, &'a AlmostSet<T>)> = Vec::new();
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

fn get_minimum_edges_for<'a, T: Debug>(corpus: &'a Corpus<AlmostSet<T>>, element: &'a AlmostSet<T>)
    -> Vec<(&'a AlmostSet<T>, &'a AlmostSet<T>)>
    where T: Eq, T:Hash, T:Clone, T:Ord
{
    let candidates = get_supersets(corpus, element);

    candidates.iter()
        .filter(|x| x.len() <= candidates[0].len())
        .map(|x| (element, *x))
        .collect()
}

fn get_supersets<'a, T: Debug>(corpus: &'a Corpus<AlmostSet<T>>, element: &'a AlmostSet<T>) -> Vec<&'a AlmostSet<T>>
    where T: Eq, T:Hash, T:Clone, T:Ord
{
    corpus.get_above(element.len()).filter(|x| element.is_subset(*x)).collect::<Vec<&'a AlmostSet<T>>>()
}

#[cfg(test)]
pub mod test {
    use crate::almost_set::AlmostSet;
    use crate::corpus::Corpus;
    use crate::get_supersets;

    fn build_corpus() -> Corpus<AlmostSet<u16>> {
        let mut corpus = Corpus::<AlmostSet<u16>>::new();
        corpus.add(AlmostSet::new(vec![1, 2, 3, 5]));
        corpus.add(AlmostSet::new(vec![1, 2, 3, 5, 11]));
        corpus.add(AlmostSet::new(vec![1, 2, 3, 5, 16, 17]));
        corpus
    }

    #[test]
    fn check_superset_1() {
        let corpus = build_corpus();
        assert_eq!(get_supersets(&corpus, &AlmostSet::<u16>::new(vec![1, 2])),
                    vec![
                        &AlmostSet::new(vec![1, 2, 3]),
                        &AlmostSet::new(vec![1, 2, 3, 4]),
                        &AlmostSet::new(vec![1, 2, 3, 4, 5]),
                    ])
    }

    #[test]
    fn check_superset_2() {
        let corpus = build_corpus();
        assert_eq!(get_supersets(&corpus, &AlmostSet::<u16>::new(vec![2])),
                   vec![
                       &AlmostSet::new(vec![1, 2]),
                       &AlmostSet::new(vec![1, 2, 3]),
                       &AlmostSet::new(vec![1, 2, 3, 4]),
                       &AlmostSet::new(vec![1, 2, 3, 4, 5]),
                       &AlmostSet::new(vec![2,4]),
                       &AlmostSet::new(vec![2, 3]),
                   ])
    }
}