use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    players: Vec<usize>,
    player_scores: Vec<usize>,
    player_turn: usize,
    boardsize: usize,
    die_state: usize,
}

impl Board {
    pub fn init(player1: usize, player2: usize) -> Self {
        Self {
            players: vec![player1 - 1, player2 - 1],
            player_scores: vec![0, 0],
            player_turn: 0,
            boardsize: 10,
            die_state: 0,
        }
    }

    pub fn turn(&mut self) -> bool {
        let roll = (3 * (self.die_state + 2)) % 100;
        self.die_state = (self.die_state + 3) % 100;
        let score = 1 + (self.players[self.player_turn] + roll) % self.boardsize;
        self.players[self.player_turn] = (self.players[self.player_turn] + roll) % self.boardsize;
        self.player_scores[self.player_turn] += score;
        if self.player_scores[self.player_turn] >= 1000 {
            return true;
        }

        self.player_turn = (self.player_turn + 1) % 2;
        false
    }

    pub fn get_losing_player_score(&self) -> usize {
        self.player_scores[(self.player_turn + 1) % 2]
    }
}

fn solve_part1() -> usize {
    //let mut board = Board::init(4, 8);
    let mut board = Board::init(8, 2);

    let mut dice_rolls = 3;
    while !board.turn() {
        dice_rolls += 3;
    }
    println!("turn[{}]: {:?}", dice_rolls, board);
    dice_rolls * board.get_losing_player_score()
}

type Player = (u8, u8);
type Players = (Player, Player);
type Wins = (usize, usize);

fn recursive_play(players: Players, cache: &mut HashMap<Players, Wins>) -> Wins {
    let ((player_0_board, player_0_score), (player_1_board, player_1_score)) = players;

    let winning_score = 21;

    if player_0_score >= winning_score {
        return (1, 0);
    }
    if player_1_score >= winning_score {
        return (0, 1);
    }

    if let Some(&wins) = cache.get(&players) {
        return wins;
    }

    let mut total_wins = (0, 0);

    for a in 0..3 {
        for b in 0..3 {
            for c in 0..3 {
                let roll = 3 + a + b + c;
                let next_board = (player_0_board + roll) % 10;
                let next_score = 1 + player_0_score + next_board;

                let wins = recursive_play(
                    ((player_1_board, player_1_score), (next_board, next_score)),
                    cache,
                );

                total_wins.0 += wins.1;
                total_wins.1 += wins.0;
            }
        }
    }
    cache.insert(players, total_wins);
    total_wins
}
fn solve_part2() -> usize {
    let mut cache = HashMap::<Players, Wins>::new();

    let score = recursive_play(((8 - 1, 0), (2 - 1, 0)), &mut cache);
    println!("scores: {:?}", score);

    if score.0 > score.1 {
        score.0
    } else {
        score.1
    }
}

fn main() {
    println!("Part1: {}", solve_part1());
    println!("Part2: {}", solve_part2());
}
