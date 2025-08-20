use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf774mvTWf");

#[program]
pub mod create_config_774 {
    use super::*;

    pub fn create_config(ctx: Context<CreateConfig774>, value: u64) -> Result<()> {
        let cfg_bump = *ctx.bumps.get("config").unwrap();
        let log_bump = *ctx.bumps.get("log").unwrap();
        let cfg = &mut ctx.accounts.config;
        cfg.bump = cfg_bump;
        cfg.owner = ctx.accounts.user.key();
        cfg.value = value;
        let lg = &mut ctx.accounts.log;
        lg.bump = log_bump;
        lg.last_value = value;
        msg!(
            "Case 774: cfg_bump={} log_bump={} value={}",
            cfg_bump,
            log_bump,
            value
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateConfig774<'info> {
    #[account(init, seeds = [b"config"], bump, payer = user, space = 8 + 1 + 32 + 8)]
    pub config: Account<'info, Config774>,
    #[account(init, seeds = [b"log"], bump, payer = user, space = 8 + 1 + 8)]
    pub log: Account<'info, Log774>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config774 {
    pub bump: u8,
    pub owner: Pubkey,
    pub value: u64,
}

#[account]
pub struct Log774 {
    pub bump: u8,
    pub last_value: u64,
}
