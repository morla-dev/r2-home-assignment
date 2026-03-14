use std::sync::RwLock;
use chrono::Local;
use rand::Rng;

pub const DAILY_WIN_THRESHOLD: u32 = 30;
const BASE_WIN_CHANCE: f64 = 0.7;
const REDUCED_WIN_CHANCE: f64 = 0.4;


struct DailyWins {
    count: u32,
    date: String,
}

pub struct LuckStore {
    daily_wins: RwLock<DailyWins>,
}

impl LuckStore {
    pub fn new() -> Self {
        LuckStore {
            daily_wins: RwLock::new(DailyWins {
                count: 0,
                date: today(),
            }),
        }
    }

    pub fn try_luck(&self) -> bool {
        let mut guard = self.daily_wins.write().unwrap();
        let current_date = today();

        if guard.date != current_date {
            guard.count = 0;
            guard.date = current_date;
        }

        let chance = if guard.count >= DAILY_WIN_THRESHOLD {
            REDUCED_WIN_CHANCE
        } else {
            BASE_WIN_CHANCE
        };

        let won = rand::thread_rng().gen_bool(chance);
        if won {
            guard.count += 1;
        }
        won
    }
}

fn today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
