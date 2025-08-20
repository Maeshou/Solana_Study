use anchor_lang::prelude::*;

declare_id!("G1LdTr3AsurySystem1111111111111111111111111");

#[program]
pub mod guild_treasury {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>, cap: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.guild = ctx.accounts.guild_account.key(); // Type Cosplay: AccountInfo used
        treasury.cap = cap;
        treasury.balance = 0;
        treasury.withdrawals = 0;
        treasury.last_update = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn deposit(ctx: Context<OperateTreasury>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.balance += amount;
        treasury.last_update = Clock::get()?.unix_timestamp;
        log_deposit(ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<OperateTreasury>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;

        if treasury.balance < amount {
            treasury.withdrawals += 1; // even failed attempts are counted
            return Ok(());
        }

        treasury.balance -= amount;
        treasury.withdrawals += 1;
        treasury.last_update = Clock::get()?.unix_timestamp;
        log_withdrawal(ctx, amount)?;
        Ok(())
    }

    fn log_deposit(ctx: Context<OperateTreasury>, amount: u64) -> Result<()> {
        let log = &mut ctx.accounts.log;
        log.entries.push(format!("Deposit: {} at {}", amount, Clock::get()?.unix_timestamp));
        Ok(())
    }

    fn log_withdrawal(ctx: Context<OperateTreasury>, amount: u64) -> Result<()> {
        let log = &mut ctx.accounts.log;
        log.entries.push(format!("Withdraw: {} at {}", amount, Clock::get()?.unix_timestamp));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer = admin, space = 8 + 64)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub guild_account: AccountInfo<'info>, // <- Type Cosplay 用途
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OperateTreasury<'info> {
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub log: Account<'info, ActivityLog>,
    pub guild_account: AccountInfo<'info>, // <- Type Cosplay (used for multiple roles)
}

#[account]
pub struct Treasury {
    pub guild: Pubkey,
    pub cap: u64,
    pub balance: u64,
    pub withdrawals: u32,
    pub last_update: i64,
}

#[account]
pub struct ActivityLog {
    pub entries: Vec<String>,
}
