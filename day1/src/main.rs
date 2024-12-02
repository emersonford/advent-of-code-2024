use std::collections::HashMap;

use clap::Parser;

fn get_lists() -> (Vec<i64>, Vec<i64>) {
    let (fst_list, snd_list): (Vec<_>, Vec<_>) = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let mut split = line.split_whitespace();

            let (fst, snd) = (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            );

            (fst, snd)
        })
        .unzip();

    (fst_list, snd_list)
}

fn part2() {
    let (fst_list, snd_list): (Vec<_>, Vec<_>) = get_lists();

    let simularity_score: HashMap<i64, usize> =
        snd_list.into_iter().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e).or_insert(0) += 1;
            acc
        });

    let res = fst_list
        .into_iter()
        .map(|i| i * (simularity_score.get(&i).copied().unwrap_or(0) as i64))
        .sum::<i64>();

    println!("{}", res);
}

fn part1() {
    let (mut fst_list, mut snd_list): (Vec<_>, Vec<_>) = get_lists();

    fst_list.sort();
    snd_list.sort();

    let res = fst_list
        .into_iter()
        .zip(snd_list)
        .map(|(fst, snd)| (fst - snd).abs())
        .sum::<i64>();

    println!("{}", res);
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Cli::parse();

    if args.part2 {
        part2();
    } else {
        part1();
    }
}
