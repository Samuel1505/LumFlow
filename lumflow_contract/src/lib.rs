#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct LumFlowContract;

#[contractimpl]
impl LumFlowContract {
    /// Simple greeting — useful as a smoke-test for bindings.
    pub fn hello(_env: Env, name: String) -> String {
        // Note: in real Soroban code prefer returning `soroban_sdk::Symbol` or bytes
        // This is a small example for developers to see how functions look.
        ["Hello, ", &name, "!"].concat()
    }

    /// Store a numeric value under a string key.
    pub fn set_u64(env: Env, key: Symbol, value: u64) {
        env.storage().set(&key, &value);
    }

    /// Read a numeric value previously stored with `set_u64`.
    /// Returns 0 if the key is not present.
    pub fn get_u64(env: Env, key: Symbol) -> u64 {
        env.storage().get(&key).unwrap_or(0u64)
    }

    /// Calculate claimable amount for a stream given a per-second rate and timestamps.
    /// - `rate_per_second`: tokens (in smallest units) earned per second.
    /// - `start` / `end`: Unix timestamps defining the stream window.
    /// - `now`: current Unix timestamp to evaluate claimable portion.
    pub fn claimable_amount(_env: Env, rate_per_second: u128, start: u64, end: u64, now: u64) -> u128 {
        if now <= start {
            return 0u128;
        }
        let elapsed: u64 = if now >= end { end.saturating_sub(start) } else { now.saturating_sub(start) };
        rate_per_second.saturating_mul(elapsed as u128)
    }
}

mod examples {
    //! Example usage snippets for devs — not compiled as tests here.
    //!
    //! - Call `hello("World")` to verify bindings.
    //! - Use `set_u64` / `get_u64` for simple on-chain storage tests.
}
