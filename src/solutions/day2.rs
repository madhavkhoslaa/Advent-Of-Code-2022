pub mod solution {
    #[derive(Clone)]
    enum RPS {
        ROCK = 1,
        PAPER = 2,
        SCISSORS = 3,
    }
    fn decide1(opp: RPS, yours: RPS) -> i32 {
        let outcome = match (&yours, &opp) {
            (RPS::ROCK, RPS::SCISSORS) => 6,
            (RPS::SCISSORS, RPS::PAPER) => 6,
            (RPS::PAPER, RPS::ROCK) => 6,
            (RPS::PAPER, RPS::PAPER) => 3,
            (RPS::SCISSORS, RPS::SCISSORS) => 3,
            (RPS::ROCK, RPS::ROCK) => 3,
            _ => 0,
        } + yours as i32;
        return outcome;
    }

    fn decide2(opp: RPS, yours: RPS) -> i32 {
        let outcome = match &yours {
            // ROCK = X = LOOSE
            // PAPER = Y = DRAW
            // SCISSOR = Z = WIN
            RPS::ROCK => {
                let loose = match &opp {
                    RPS::PAPER => RPS::ROCK as i32,
                    RPS::ROCK => RPS::SCISSORS as i32,
                    RPS::SCISSORS => RPS::PAPER as i32,
                };
                return loose;
            }
            RPS::PAPER => 3 + opp as i32,
            RPS::SCISSORS => {
                let win = match &opp {
                    RPS::ROCK => RPS::PAPER as i32,
                    RPS::SCISSORS => RPS::ROCK as i32,
                    RPS::PAPER => RPS::SCISSORS as i32,
                };
                return win + 6;
            }
        };
        return outcome;
    }

    fn parse(row: String) -> (RPS, RPS) {
        let values: Vec<&str> = row.split(" ").collect();
        let mut value1: RPS = RPS::ROCK;
        let mut value2: RPS = RPS::ROCK;
        if values[0] == "A" {
            value1 = RPS::ROCK;
        }
        if values[0] == "B" {
            value1 = RPS::PAPER;
        }
        if values[0] == "C" {
            value1 = RPS::SCISSORS;
        }
        if values[1] == "X" {
            value2 = RPS::ROCK;
        }
        if values[1] == "Y" {
            value2 = RPS::PAPER;
        }
        if values[1] == "Z" {
            value2 = RPS::SCISSORS;
        }
        return (value1, value2);
    }

    fn main<F: Fn(RPS, RPS) -> i32>(logix: F) -> i32 {
        let contents = crate::fs::read_to_string("./inputs/day2").expect("Cannot read file");
        return contents
            .split("\n")
            .into_iter()
            .map(|comp| {
                if comp != "" {
                    let (opponent, you) = parse(String::from(comp));
                    return logix(opponent, you);
                } else {
                    return 0;
                }
            })
            .sum::<i32>();
    }

    pub fn solve2() -> i32 {
        return main(decide2);
    }

    pub fn solve1() -> i32 {
        return main(decide1);
    }
}
