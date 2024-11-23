use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    pub taker: Option<Pubkey>,         // The student who accepts the bet
    pub open_until: u64,              // Bet remains open for this period
    pub resolve_date: u64,            // Resolution date for the bet
    pub average_grade_prediction: u64, // Predicted average grade, scaled by 10^2 (e.g., 3.5 is stored as 350)
    pub resolver: Pubkey,             // Trusted resolver (e.g., teacher or admin account)
    pub winner: Option<Pubkey>,       // Winner of the bet
    pub bet_seed: u64,                // Unique identifier for the bet
    pub pool_bump: u8,                // Metadata for security/account derivation
    pub bump: u8,                     // Metadata for security/account derivation
}

impl Space for Bet {
    const INIT_SPACE: usize = 8 + 33 + 8 + 8 + 8 + 32 + 33 + 8 + 1 + 1;
}
