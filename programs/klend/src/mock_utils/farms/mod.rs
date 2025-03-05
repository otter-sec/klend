use anchor_lang::prelude::*;
pub mod accounts;


pub use accounts::*;

pub mod state {
    pub use super::accounts::*;
}

declare_id!("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr");

