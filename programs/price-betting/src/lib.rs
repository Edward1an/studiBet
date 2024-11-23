use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

pub mod error;

declare_id!("5RoVruk757C3LWt6ZVXajctxrqTDdGJEEmH1sh5qDTPL");

#[program]
pub mod price_betting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fees: u16) -> Result<()> {
        ctx.accounts.initialize(seed, fees, &ctx.bumps)
    }

    pub fn create_bet(
        ctx: Context<Create>, 
        bet_seed: u64, 
        open_until: u64, 
        resolve_date: u64, 
        average_grade_prediction: u64, // Prediction in 10^2 format (e.g., 3.5 -> 350)
        resolver: Pubkey,
        amount: u64
    ) -> Result<()> {
        require!(
            average_grade_prediction >= 100 && average_grade_prediction <= 400,
            CustomError::InvalidPrediction
        ); // Validate the prediction range
        ctx.accounts.create_bet(
            bet_seed,
            open_until,
            resolve_date,
            average_grade_prediction,
            resolver,
            &ctx.bumps,
        )?;
        ctx.accounts.deposit_wager(amount)
    }
    

    pub fn accept_bet(ctx: Context<Accept>, bet_seed: u64) -> Result<()> {
        let _ = bet_seed; //seed only used for account derivation
        ctx.accounts.validate()?;
        ctx.accounts.deposit_wager()?;
        ctx.accounts.set_bet_taker()?;
        ctx.accounts.pay_protocol_fee()
    }

    pub fn cancel_bet(ctx: Context<Cancel>, bet_seed: u64) -> Result<()> {
        ctx.accounts.validate()?;
        ctx.accounts.withdraw_wager(bet_seed)
    }

    pub fn resolve(ctx: Context<Resolve>, actual_average_grade: u64) -> Result<()> {
        require!(
            actual_average_grade >= 100 && actual_average_grade <= 400,
            CustomError::InvalidGrade
        ); // Validate the actual grade range
        
        let bet = &mut ctx.accounts.bet;
        let creator_prediction_diff = (bet.average_grade_prediction as i64 - actual_average_grade as i64).abs();
        let taker_prediction_diff = if let Some(taker_prediction) = bet.taker_average_grade {
            (taker_prediction as i64 - actual_average_grade as i64).abs()
        } else {
            i64::MAX // If no taker, creator wins
        };
        
        bet.winner = if creator_prediction_diff <= taker_prediction_diff {
            Some(ctx.accounts.creator.key())
        } else {
            bet.taker
        };
        Ok(())
    }
    

    pub fn resolve_bet_local_test_dummy(ctx: Context<Resolve>, bet_seed: u64) -> Result<()> {
        let _ = bet_seed; //seed only used for account derivation
        // ctx.accounts.validate()?; //TODO: activate validations once actual resolver is implemented
        ctx.accounts.resolve_bet_dummy_impl() //TODO: replace resolve with an actual switchboard implementation
    }

    //this only works if the 
    pub fn resolve_bet_wihtout_update(ctx: Context<Resolve>, bet_seed: u64) -> Result<()> {
        let _ = bet_seed; //seed only used for account derivation
        ctx.accounts.validate()?; //TODO: activate validations once actual resolver is implemented
        ctx.accounts.resolve_bet_switchboard_impl() //TODO: replace resolve with an actual switchboard implementation
    }

    pub fn claim_bet(ctx: Context<Claim>, bet_seed: u64) -> Result<()> {
        let _ = bet_seed; //seed only used for account derivation
        ctx.accounts.validate()?;
        ctx.accounts.claim_winnings()
    }

    pub fn withdraw_from_treasury(ctx: Context<Withdraw>, seed: u64) -> Result<()> {
        let _ = seed; //seed only used for account derivation
        ctx.accounts.validate()?;
        ctx.accounts.withdraw_from_treasury()
    }
}

