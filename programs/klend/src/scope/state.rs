use anchor_lang::prelude::Pubkey;


pub const PROGRAM_ID:Pubkey = pubkey!("HFn8GnPADiny6XqUoWE8uRPPxb29ikn4yTuPa9MF2fWJ");

declare_id!(PROGRAM_ID);

// Note: Need to be directly integer value to not confuse the IDL generator
pub const MAX_ENTRIES_U16: u16 = 512;
// Note: Need to be directly integer value to not confuse the IDL generator
pub const MAX_ENTRIES: usize = 512;


#[derive(Debug, Default)]
pub struct Price {
    pub value: u64,
    pub exp: u64,
}

#[derive(Debug, Default)]
pub struct OraclePrices {
    pub prices: [Price; MAX_ENTRIES],
    pub timestamp: u64,
}