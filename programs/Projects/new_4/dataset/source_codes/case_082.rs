use anchor_lang::prelude::*;

declare_id!("Repertory11TokenVault11111111111111111111111");

#[program]
pub mod token_vault {
    use super::*;

    // バンを設定してバウチャーを生成
    pub fn init_vault(ctx: Context<InitVault>, bump: u8) -> Result<()> {
        let v = &mut ctx.accounts.vault; 
        v.authority = ctx.accounts.authority.key();
        v.bump = bump;
        v.total_deposits = 0;
        v.last_update = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // トークンを入金
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;          // ← initなし：既存参照
        v.total_deposits += amount;
        v.last_update = Clock::get()?.unix_timestamp;

        let dr = &mut ctx.accounts.deposit_record; // ← initなし（本来は init すべき）
        dr.user = ctx.accounts.user.key();
        dr.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1 + 8 + 8)]
    pub vault: Account<'info, VaultData>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub vault: Account<'info, VaultData>,
    pub deposit_record: Account<'info, DepositRecord>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultData {
    pub authority: Pubkey,
    pub bump: u8,
    pub total_deposits: u64,
    pub last_update: i64,
}

#[account]
pub struct DepositRecord {
    pub user: Pubkey,
    pub amount: u64,
}
