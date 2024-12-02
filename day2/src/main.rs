use clap::Parser;

fn get_lines() -> Vec<Vec<i64>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let split = line.split_whitespace();

            split.map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

enum Direction {
    Ascending,
    Descending,
}

fn check_line(line: &[i64]) -> bool {
    let mut dir = None;

    for window in line.windows(2) {
        let &[fst, snd] = window else {
            unreachable!();
        };

        let diff = (fst - snd).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        match dir {
            Some(Direction::Ascending) => {
                if fst > snd {
                    return false;
                }
            }
            Some(Direction::Descending) => {
                if fst < snd {
                    return false;
                }
            }
            None => {
                dir = Some(if fst > snd {
                    Direction::Descending
                } else {
                    Direction::Ascending
                });
            }
        }
    }

    return true;
}

fn part2() {
    let lines = get_lines();

    let res = lines
        .into_iter()
        .map(|line| (check_line(&line), line))
        .filter_map(|(pass, line)| {
            if pass {
                return Some(());
            } else {
                for i in 0..line.len() {
                    // This is inefficient, but there's so many edge cases to check, we'll just
                    // brute force it lol.
                    let mut line = line.clone();
                    line.remove(i);

                    if check_line(&line) {
                        return Some(());
                    }
                }

                return None;
            }
        })
        .count();

    println!("{}", res);
}

fn part1() {
    let lines = get_lines();

    let res = lines
        .into_iter()
        .map(|line| check_line(&line))
        .filter(|v| *v)
        .count();

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
