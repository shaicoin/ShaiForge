use std::io;

const KISMETS_PER_BTC: u64 = 100_000_000;
const DECAY_RATE: f64 = -0.00001;
const MIN_REWARD_HEIGHT: u64 = 420480;
const MIN_REWARD_AMOUNT: u64 = 10_000_000;
const BLOCK_TIME_MINUTES: u64 = 5;

fn get_coin_reward(block_height: u64) -> u64 {
    if block_height >= MIN_REWARD_HEIGHT {
        MIN_REWARD_AMOUNT
    } else {
        let reward = 50.0 * f64::exp(DECAY_RATE * block_height as f64);
        (reward * KISMETS_PER_BTC as f64) as u64
    }
}

fn display_block_info(user_block_height: u64, total_reward: u128) {
    let coin_reward = get_coin_reward(user_block_height) as f64 / KISMETS_PER_BTC as f64;
    let estimated_time_minutes = user_block_height * BLOCK_TIME_MINUTES;
    let estimated_years = estimated_time_minutes / (365 * 24 * 60);
    let estimated_days = (estimated_time_minutes % (365 * 24 * 60)) / (24 * 60);
    let estimated_hours = (estimated_time_minutes % (24 * 60)) / 60;
    let estimated_minutes = estimated_time_minutes % 60;

    println!("══════════════════════════════════════════════");
    println!("Block Height: {}", user_block_height);
    println!("Reward: {} SHA", coin_reward);
    println!("Total Supply: {} SHA", total_reward as f64 / KISMETS_PER_BTC as f64);
    println!(
        "Estimated Time: {} years, {} days, {} hours, {} minutes",
        estimated_years, estimated_days, estimated_hours, estimated_minutes
    );
    println!("══════════════════════════════════════════════");
}

fn main() {
    loop {
        println!("\n\nEnter a block height to explore (or 'q' to quit):");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim().to_lowercase();

        if user_input == "q" {
            break;
        }

        match user_input.parse::<u64>() {
            Ok(user_block_height) => {
                let total_reward: u128 = (0..=user_block_height)
                    .map(|i| get_coin_reward(i) as u128)
                    .sum();

                display_block_info(user_block_height, total_reward);
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid block height or 'q' to quit.");
            }
        }
    }
}
