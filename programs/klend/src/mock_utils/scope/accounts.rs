use anchor_lang::prelude::*;

// Updated to 10 instead of 512
pub const MAX_ENTRIES_U16: u16 = 10;
// Updated to 10 instead of 512
pub const MAX_ENTRIES: usize = 10;

#[account]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct OraclePrices {
    pub oracle_mappings: Pubkey,
    pub prices: [DatedPrice; MAX_ENTRIES],
}

impl Default for OraclePrices {
    fn default() -> Self {
        Self {
            oracle_mappings: Pubkey::default(),
            prices: [DatedPrice::default(); MAX_ENTRIES],
        }
    }
}


#[account]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct DatedPrice {
    pub price: Price,
    pub last_updated_slot: u64,
    pub unix_timestamp: u64,
    pub _reserved: [u64; 2],
    pub _reserved2: [u16; 3],
    // Current index of the dated price.
    pub index: u16,
}

impl Default for DatedPrice {
    fn default() -> Self {
        Self {
            price: Default::default(),
            last_updated_slot: Default::default(),
            unix_timestamp: Default::default(),
            _reserved: Default::default(),
            _reserved2: Default::default(),
            index: MAX_ENTRIES_U16,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, AnchorSerialize, AnchorDeserialize, Arbitrary, Clone, Copy)]
pub struct Price {
    // Pyth price, integer + exponent representation
    // decimal price would be
    // as integer: 6462236900000, exponent: 8
    // as float:   64622.36900000

    // value is the scaled integer
    // for example, 6462236900000 for btc
    pub value: u64,

    // exponent represents the number of decimals
    // for example, 8 for btc
    pub exp: u64,
}
