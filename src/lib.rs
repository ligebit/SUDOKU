use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Timestamp};

use rand::rngs::StdRng;
use rand::SeedableRng;

use std::collections::{HashMap};
use std::convert::TryInto;

pub mod bitset;
pub mod board;
mod consts;
pub mod errors;
mod generator;
mod helper;
mod solver;
pub mod strategy;

pub use crate::board::Sudoku;
pub use crate::board::Symmetry;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct LastSlovedGame {
    sudoku: Sudoku,
    time_end: Timestamp,
    time_start: Timestamp,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Player {
    sudoku: Option<Sudoku>,
    start_time: Timestamp,

    generated_sudoku_count: u128,
    sloved_sudoku_count: u128,

    last_sloved_game: Option<LastSlovedGame>,

    best_time: Option<Timestamp>,
}

type SudokuTwoDimensionalArray = [[u8; 9]; 9];

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LastSlovedGameRequest {
    sudoku: SudokuTwoDimensionalArray,
    time_end: Timestamp,
    time_start: Timestamp,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerRequest {
    sudoku: Option<SudokuTwoDimensionalArray>,
    start_time: Timestamp,

    generated_sudoku_count: U128,
    sloved_sudoku_count: U128,

    last_sloved_game: Option<LastSlovedGameRequest>,

    best_time: Option<Timestamp>,
}

const PLAYER_SIZE: u128 = 403;
const LEADERBOARD_SIZE: usize = 10;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Leaderboard {
    pub top_by_count: HashMap<AccountId, u128>,
    pub top_by_time: HashMap<AccountId, Timestamp>,
}

impl Leaderboard {
    pub fn work_player(&mut self, player: &Player) {

        if self.top_by_count.len() < LEADERBOARD_SIZE {
            self.top_by_count.insert(env::predecessor_account_id(), player.sloved_sudoku_count);
        } else {
            let binding = self.top_by_count.clone();
            let (key, value) = binding.iter().min_by_key(|(_, value)| *value).unwrap();
            if value <= &player.sloved_sudoku_count {
                if key.eq(&env::predecessor_account_id()) {
                    self.top_by_count.insert(env::predecessor_account_id(), player.sloved_sudoku_count);
                } else {
                    self.top_by_count.remove(&key);
                    self.top_by_count.insert(env::predecessor_account_id(), player.sloved_sudoku_count);
                }
            }
        }

        if self.top_by_time.len() < LEADERBOARD_SIZE {
            self.top_by_time.insert(env::predecessor_account_id(), player.best_time.unwrap());
        } else {
            let binding = self.top_by_time.clone();
            let (key, value) = binding.iter().max_by_key(|(_, value)| *value).unwrap();
            if value >= &player.best_time.unwrap() {
                if key.eq(&env::predecessor_account_id()) {
                    self.top_by_time.insert(env::predecessor_account_id(), player.best_time.unwrap());
                } else {
                    self.top_by_time.remove(&key);
                    self.top_by_time.insert(env::predecessor_account_id(), player.best_time.unwrap());
                }
            }
        }
    }
}

impl Player {
    pub fn new(rnd: &mut StdRng) -> Player {
        Self {
            sudoku: Some(Sudoku::generate(rnd)),
            generated_sudoku_count: 1,
            sloved_sudoku_count: 0,
            start_time: env::block_timestamp_ms(),

            last_sloved_game: None,

            best_time: None,
        }
    }

    pub fn new_game(self, rnd: &mut StdRng) -> Player {
        Self {
            sudoku: Some(Sudoku::generate(rnd)),
            generated_sudoku_count: self.generated_sudoku_count + 1,
            sloved_sudoku_count: self.sloved_sudoku_count,
            start_time: env::block_timestamp_ms(),
            last_sloved_game: self.last_sloved_game,
            best_time: self.best_time,
        }
    }

    pub fn finish_game(self) -> Player {
        let time = env::block_timestamp_ms() - self.start_time;

        Self {
            sudoku: None,
            generated_sudoku_count: self.generated_sudoku_count,
            sloved_sudoku_count: self.sloved_sudoku_count + 1,

            start_time: env::block_timestamp_ms(),

            last_sloved_game: Some(LastSlovedGame {
                sudoku: self.sudoku.unwrap(),
                time_start: self.start_time,
                time_end: env::block_timestamp_ms(),
            }),

            best_time: if time < self.best_time.unwrap_or(u64::MAX) {
                Some(time)
            } else {
                self.best_time
            },
        }
    }

    pub fn get(&self) -> PlayerRequest {
        PlayerRequest {
            sudoku: match &self.sudoku {
                Some(sudoku) => Some(sudoku.to_two_dimensional_array()),
                None => None,
            },
            generated_sudoku_count: U128::from(self.generated_sudoku_count),
            sloved_sudoku_count: U128::from(self.sloved_sudoku_count),
            start_time: self.start_time,

            last_sloved_game: match &self.last_sloved_game {
                Some(last_game) => Some(LastSlovedGameRequest {
                    sudoku: last_game.sudoku.to_two_dimensional_array(),
                    time_end: last_game.time_end,
                    time_start: last_game.time_start,
                }),
                None => None,
            },
            best_time: self.best_time,
        }
    }

    pub fn sudoku_eq(&self, array: &SudokuTwoDimensionalArray) -> bool {
        let internal_sudoku = self.sudoku.unwrap().to_two_dimensional_array();

        for x in 0..9 {
            for y in 0..9 {
                if array[x][y] < 1 || array[x][y] > 9 {
                    return false;
                }

                if internal_sudoku[x][y] != 0 && array[x][y] != array[x][y] {
                    return false;
                }
            }
        }

        return true;
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub players: UnorderedMap<AccountId, Player>,
    pub leaderboard: Leaderboard,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            players: UnorderedMap::new(b"p".to_vec()),
            leaderboard: Leaderboard {
                top_by_count: HashMap::new(),
                top_by_time: HashMap::new(),
            },
        }
    }

    #[payable]
    pub fn start_game(&mut self) -> PlayerRequest {
        let seed: [u8; 32] = env::random_seed().try_into().unwrap();
        let mut rnd: StdRng = SeedableRng::from_seed(seed);

        match self.players.get(&env::predecessor_account_id()) {
            Some(player) => self
                .players
                .insert(&env::predecessor_account_id(), &player.new_game(&mut rnd))
                .unwrap()
                .get(),
            None => self.register_player(&mut rnd).get(),
        }
    }

    fn register_player(&mut self, rnd: &mut StdRng) -> Player {
        if env::attached_deposit() != (PLAYER_SIZE * env::STORAGE_PRICE_PER_BYTE) {
            panic!(
                "attach {} yoctonear",
                PLAYER_SIZE * env::STORAGE_PRICE_PER_BYTE
            );
        }

        let player = Player::new(rnd);

        self.players.insert(&env::predecessor_account_id(), &player);

        player
    }

    pub fn finish_game(&mut self, array: &SudokuTwoDimensionalArray) -> Option<PlayerRequest> {
        match self.players.get(&env::predecessor_account_id()) {
            Some(player) => {
                if Sudoku::from_two_dimensional_array(array).is_solved() && player.sudoku_eq(&array)
                {
                    let new_player = player.finish_game();

                    self.leaderboard.work_player(&new_player);

                    Some(
                        self.players
                            .insert(&env::predecessor_account_id(), &new_player)
                            .unwrap()
                            .get(),
                    )
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn check_sloved(&self, array: &SudokuTwoDimensionalArray) -> bool {
        Sudoku::from_two_dimensional_array(array).is_solved()
    }

    pub fn get_player(&self, account_id: AccountId) -> Option<PlayerRequest> {
        match self.players.get(&account_id) {
            Some(player) => Some(player.get()),
            None => None,
        }
    }

    pub fn delete_player(&mut self) {
        self.players.remove(&env::predecessor_account_id());
    }

    pub fn get_leaderboard(self) -> Leaderboard {
        self.leaderboard
    }

    // pub fn test_size(&mut self) {
    //     let seed: [u8; 32] = env::random_seed().try_into().unwrap();
    //     let mut rnd: StdRng = SeedableRng::from_seed(seed);

    //     let prev = env::storage_usage();

    //     self.players.insert(
    //         &env::predecessor_account_id(),
    //         &Player {
    //             best_time: Some(env::block_timestamp_ms()),
    //             generated_sudoku_count: 0,
    //             sloved_sudoku_count: 0,
    //             last_sloved_game: Some(LastSlovedGame {
    //                 sudoku: Sudoku::generate(&mut rnd),
    //                 time_end: env::block_timestamp_ms(),
    //                 time_start: env::block_timestamp_ms(),
    //             }),
    //             start_time: env::block_timestamp_ms(),
    //             sudoku: Some(Sudoku::generate(&mut rnd)),
    //         },
    //     );

    //     println!("{:?}", env::storage_usage() - prev)
    // }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn start_game(contract: &mut Contract, account: AccountId) {
        let mut context = get_context(account.clone());
        context.block_timestamp(0);
        context.attached_deposit(4030000000000000000000);
        testing_env!(context.build());

        contract.start_game();
    }

    fn play(contract: &mut Contract, account: AccountId, time: Timestamp) {
        let mut context = get_context(account.clone());
        start_game(contract, account.clone());

        let solution = contract.players.get(&account.clone()).unwrap().sudoku.unwrap().solution().unwrap();
        context.block_timestamp(time * 1_000_000);
        testing_env!(context.build());
        contract.finish_game(&solution.to_two_dimensional_array());
    }

    #[test]
    fn leaderboard() {
        let mut contract = Contract::new();

        play(&mut contract, accounts(0), 1000);
        start_game(&mut contract, accounts(0));
        play(&mut contract, accounts(0), 900);
        play(&mut contract, accounts(0), 900);
        start_game(&mut contract, accounts(0));
        play(&mut contract, accounts(0), 900);

        play(&mut contract, accounts(1), 950);
        play(&mut contract, accounts(1), 800);

        play(&mut contract, accounts(2), 100);

        play(&mut contract, accounts(3), 1000);
        play(&mut contract, accounts(3), 1000);
        play(&mut contract, accounts(3), 1000);
        play(&mut contract, accounts(3), 1000);
        play(&mut contract, accounts(3), 1000);

        let leaderboard = contract.get_leaderboard();

        println!("{:?}", leaderboard.top_by_count);
        println!("{:?}", leaderboard.top_by_time);
    }
}
