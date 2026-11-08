use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("FZbATsaJLeuYmjqC6L4LbJU33fAMX4NLS4vdAdbazyqc");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
}
