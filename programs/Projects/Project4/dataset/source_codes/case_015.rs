use anchor_lang::prelude::*;

declare_id!("Var3Pay3333333333333333333333333333333333");

#[program]
pub mod varied_payment {
    use super::*;

    pub fn init_account(ctx: Context<InitAccount>, balance: u64) -> Result<()> {
        let a = &mut ctx.accounts.payment_account;
        a.balance = balance;
        Ok(())
    }

    pub fn process(ctx: Context<Process>, code: u8) -> Result<()> {
        let _a = &ctx.accounts.payment_account;
        
        // match だけなので ||/& は使わない
        let status = match code {
            0 => "failed".to_string(),
            1 => "partial".to_string(),
            _ => "full".to_string(),
        };

        let log = &mut ctx.accounts.event_log;
        log.status = status;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAccount<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub payment_account: Account<'info, PaymentAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Process<'info> {
    pub payment_account: Account<'info, PaymentAccount>,
    #[account(mut, init, payer = user, space = 8 + 64)]
    pub event_log: Account<'info, EventLog>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaymentAccount {
    pub balance: u64,
}

#[account]
pub struct EventLog {
    pub status: String,
}
