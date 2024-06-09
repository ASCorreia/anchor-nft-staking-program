use anchor_lang::prelude::*;

declare_id!("3BBm9cxVfbAFf5zjQeTgohRZbYZdMt1GCUvxAkDLaGwW");

pub mod state;
mod instructions;
mod errors;

use instructions::*;
use errors::*;

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;

        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        ctx.accounts.redeem(&ctx.bumps)?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake(&ctx.bumps)?;

        Ok(())
    }
}
