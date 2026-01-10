use anchor_lang::prelude::*;

declare_id!("ArDASFH22Daz7AS47QkACUxZomWTfGwbQhVvfpiqqYKu");

#[program]
pub mod spl_token_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
