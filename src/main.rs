use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::slice::Iter;
use std::time::Instant;
use rand::{Rng};
use clap::Parser;
use indicatif::ProgressBar;

#[derive(Parser, Debug)]
struct Config {
    #[arg(short, long)]
    debug: bool,
    #[arg(short = 'M', long)]
    debug_max_len: Option<usize>,
    #[arg(short = 'E', long)]
    debug_entries: Option<usize>
}

fn main() {
    let config = Config::parse();
    /*let mut corpus: Corpus<HashSet<usize>> = Corpus::new();
    corpus.add(HashSet::from([1,2]));
    corpus.add(HashSet::from([1, 2, 3]));
    corpus.add(HashSet::from([1, 2, 3, 4]));
    corpus.add(HashSet::from([1, 2, 3, 4, 5]));
    corpus.add(HashSet::from([2]));
    corpus.add(HashSet::from([2, 3]));*/
    /*corpus.add(HashSet::from([1]));
    corpus.add(HashSet::from([1,2]));
    corpus.add(HashSet::from([1,2,3]));
    corpus.add(HashSet::from([1,2,4]));*/
    if config.debug {
        if let Some(m) = config.debug_max_len {
            if let Some(n) = config.debug_entries {
                let then = Instant::now();
                let corpus = make_random_corpus(m, n);
                println!("Finished initializing dataset, took {}ms. crunching now!", (Instant::now() - then).as_millis());
                let then = Instant::now();
                get_minimum_edges(&corpus);
                println!("Finished crunching dataset. took {}ms", (Instant::now() - then).as_millis());
            }
        }
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

/*struct SupersetIterator<'a, T> {
    internal_iter: Iter<'a, Vec<HashSet<T>>>,
    element: &'a HashSet<T>
}

impl<'a, T> Iterator for SupersetIterator<'_, T> {
    type Item = &'a HashSet<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(candidate) = self.internal_iter.next() {
            for set in candidate {
                if self.element.is_subset(set) {
                    return Some(set);
                }
                return None;
            }
        }

        None
    }
}*/

#[derive(Debug)]
struct Corpus<T: Debug+KnownLength> {
    pub data: Vec<Vec<T>>
}

impl<T: Debug+KnownLength> Corpus<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    pub fn add(&mut self, item:T) {
        if item.len() > self.data.len() {
            for _c in self.data.len()..item.len() {
                self.data.push(vec![])
            }
        }

        self.data[item.len() - 1].push(item)
    }

    pub fn get_above(&self, n:usize) -> Iter<'_, Vec<T>> {
        self.data[n..].iter()
    }
}

pub trait KnownLength {
    fn len(&self) -> usize;
}

impl<T> KnownLength for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> KnownLength for HashSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}