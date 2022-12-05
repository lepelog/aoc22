enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct InvalidValue(u8);

impl TryFrom<u8> for RPS {
    type Error = InvalidValue;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::Rock),
            b'B' => Ok(Self::Paper),
            b'C' => Ok(Self::Scissors),
            b'X' => Ok(Self::Rock),
            b'Y' => Ok(Self::Paper),
            b'Z' => Ok(Self::Scissors),
            v => Err(InvalidValue(v)),
        }
    }
}

impl RPS {
    pub fn score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play(&self, other: RPS) -> RPSResult {
        match (self, other) {
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => RPSResult::Win,
            (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors)
            | (Self::Rock, Self::Rock) => RPSResult::Draw,
            _ => RPSResult::Loss,
        }
    }

    pub fn for_expected_outcome(&self, result: &RPSResult) -> RPS {
        match (self, result) {
            (Self::Rock, RPSResult::Draw)
            | (Self::Paper, RPSResult::Loss)
            | (Self::Scissors, RPSResult::Win) => Self::Rock,
            (Self::Rock, RPSResult::Loss)
            | (Self::Paper, RPSResult::Win)
            | (Self::Scissors, RPSResult::Draw) => Self::Scissors,
            (Self::Rock, RPSResult::Win)
            | (Self::Paper, RPSResult::Draw)
            | (Self::Scissors, RPSResult::Loss) => Self::Paper,
        }
    }
}

enum RPSResult {
    Win,
    Draw,
    Loss,
}

impl RPSResult {
    pub fn score(&self) -> u8 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl TryFrom<u8> for RPSResult {
    type Error = InvalidValue;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Loss),
            b'Y' => Ok(Self::Draw),
            b'Z' => Ok(Self::Win),
            v => Err(InvalidValue(v)),
        }
    }
}

struct GuideEntry {
    own: RPS,
    other: RPS,
}

impl GuideEntry {
    pub fn from_str(s: &str) -> Self {
        let (other, own) = s.split_once(' ').unwrap();
        GuideEntry {
            own: own.bytes().next().unwrap().try_into().unwrap(),
            other: other.bytes().next().unwrap().try_into().unwrap(),
        }
    }
}

struct GuideEntry2 {
    other: RPS,
    outcome: RPSResult,
}

impl GuideEntry2 {
    pub fn from_str(s: &str) -> Self {
        let (other, outcome) = s.split_once(' ').unwrap();
        GuideEntry2 {
            outcome: outcome.bytes().next().unwrap().try_into().unwrap(),
            other: other.bytes().next().unwrap().try_into().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("../input/02.txt");
    let result: usize = input
        .lines()
        .map(GuideEntry::from_str)
        .map(|e| {
            let shape_score = e.own.score() as usize;
            let outcome_score = e.own.play(e.other).score() as usize;
            let total = shape_score + outcome_score;
            total
        })
        .sum();
    let result2: usize = input
        .lines()
        .map(GuideEntry2::from_str)
        .map(|e| {
            let own = e.other.for_expected_outcome(&e.outcome);
            let shape_score = own.score() as usize;
            let outcome_score = e.outcome.score() as usize;
            let total = shape_score + outcome_score;
            total
        })
        .sum();
    println!("{result}");
    println!("{result2}");
}
