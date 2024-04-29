#![warn(unused_extern_crates)]
extern crate soroban_env_host;
extern crate soroban_env_stellar;

use soroban_env_host::{log, println, Symbol};
use soroban_env_stellar::xdr::{DecodedScDataValue, ScVal};

pub const VERSION: Symbol = Symbol::from_str("version");
pub const ROUND_UP_PERCENTAGE: Symbol = Symbol::from_str("round_up_percentage");
pub const DAILY_DEPOSIT: Symbol = Symbol::from_str("daily_deposit");
pub const LUMPSUM_DEPOSIT: Symbol = Symbol::from_str("lumpsum_deposit");
pub const USER_BALANCES: Symbol = Symbol::from_str("user_balances");
pub const INVESTMENT_TOOLS: Symbol = Symbol::from_str("investment_tools");

pub fn initialize(data: &mut soroban_env_stellar::stellar_data::ScData) {
    data.data.insert(VERSION, ScVal::Obj(None, vec![
        (Symbol::from_str("version"), ScVal::U32(1)),
    ]));

    data.data.insert(ROUND_UP_PERCENTAGE, ScVal::U8(1)); // 1% by default
    data.data.insert(DAILY_DEPOSIT, ScVal::U64(100));
    data.data.insert(USER_BALANCES, ScVal::Obj(None, vec![]));
    data.data.insert(INVESTMENT_TOOLS, ScVal::Obj(None, vec![
        (Symbol::from_str("stocks"), ScVal::Bool(true)),
        (Symbol::from_str("mutual_funds"), ScVal::Bool(true)),
        (Symbol::from_str("crypto"), ScVal::Bool(true)),
    ]));
}

pub fn round_up(data: &mut soroban_env_stellar::stellar_data::ScData, amount: u64) {
    let round_up_percentage = match data.data.get(&ROUND_UP_PERCENTAGE) {
        Some(ScVal::U8(percentage)) => *percentage,
        _ => return,
    };

    let round_up_amount = (amount as f64 * round_up_percentage as f64 / 100.0).round() as u64;

    if round_up_amount > 0 {
        update_user_balance(data, round_up_amount);
    }
}

pub fn daily_deposit(data: &mut soroban_env_stellar::stellar_data::ScData) {
    let daily_deposit_amount = match data.data.get(&DAILY_DEPOSIT) {
        Some(ScVal::U64(amount)) => *amount,
        _ => return,
    };

    update_user_balance(data, daily_deposit_amount);
}

pub fn lumpsum_deposit(data: &mut soroban_env_stellar::stellar_data::ScData, amount: u64) {
    update_user_balance(data, amount);
}

pub fn invest(data: &mut soroban_env_stellar::stellar_data::ScData, investment_type: &str, amount: u64) {
    let user_balance = get_user_balance(data);
    if user_balance < amount {
        return; // Insufficient balance
    }

    let investment_tools = match data.data.get(&INVESTMENT_TOOLS) {
        Some(ScVal::Obj(_, tools)) => tools,
        _ => return,
    };

    let tool_enabled = investment_tools.iter().any(|(key, _)| {
        key.to_string() == investment_type
    });

    if !tool_enabled {
        return; // Investment tool not enabled
    }

    // Perform investment logic here
    update_user_balance(data, amount * -1);
}

fn update_user_balance(data: &mut soroban_env_stellar::stellar_data::ScData, amount: i64) {
    let user_balances = match data.data.get_mut(&USER_BALANCES) {
        Some(ScVal::Obj(_, balances)) => balances,
        _ => return,
    };

    let user_id = soroban_env_host::current_id();
    let user_balance = user_balances.iter_mut().find_map(|(key, value)| {
        if *key == Symbol::from_public_key(&user_id) {
            Some(value)
        } else {
            None
        }
    });

    match user_balance {
        Some(ScVal::I64(balance)) => {
            *balance += amount;
        }
        None => {
            user_balances.push((Symbol::from_public_key(&user_id), ScVal::I64(amount as i64)));
        }
        _ => return,
    }
}

fn get_user_balance(data: &soroban_env_stellar::stellar_data::ScData) -> u64 {
    let user_balances = match data.data.get(&USER_BALANCES) {
        Some(ScVal::Obj(_, balances)) => balances,
        _ => return 0,
    };

    let user_id = soroban_env_host::current_id();
    let user_balance = user_balances.iter().find_map(|(key, value)| {
        if *key == Symbol::from_public_key(&user_id) {
            match value {
                ScVal::I64(balance) => Some(*balance as u64),
                _ => None,
            }
        } else {
            None
        }
    });

    user_balance.unwrap_or(0)
}