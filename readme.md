A simple tool to make a list of sets into a digraph from a set to its minimum superset. 

It takes in data in the form of(1):
```
1
1 2
1 2 3
1 2 4
```

and turns it into:
```
1->1,2
1, 2->1, 2, 3
1, 2, 3->1, 2, 3, 4
```
The command line tool has a fully fleshed out help command (debug commands do not have help texts) so should be easy to test.

(1) The syllabus says the data should look like:
```
1
1, 2
1, 2, 3
1, 2, 4
```
but the provided example looks like the above input. I decided to match the input files, not the syllabus.

# Algorithm Information
Try as I might, I was unable to reduce the worst-case time complexity of the function below O(n^2). I suspect that the minimum
worst case time complexity for this problem is N^2 though I have not done any research into proving that.
However, I have applied two optimizations to reduce the average time complexity significantly.

- Using `Corpus` I sort the sets by length to avoid checking smaller sets, and check the smallest sets first which 
are more likely to be the minimum subset
- Using bitsets to make finding out if a subset in constant time.

I have not done analysis to find the average time complexity for my algorithm.

# Compilation Instructions
1. Clone the git repo
2. run `cargo build --release`

Compilation is done on `rustc 1.65.0` on the `nightly-x86_64-pc-windows-msvc` build.