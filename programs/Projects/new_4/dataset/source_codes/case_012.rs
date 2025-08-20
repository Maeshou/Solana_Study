// 2. 金庫＋手数料口座
use anchor_lang::prelude::*;

declare_id!("Vau22222222222222222222222222222222");

#[program]
pub mod reinit_vault_v2 {
    use super::*;

    // 保有者と残高を設定
    pub fn setup_vault(
        ctx: Context<SetupVault>,
        owner: Pubkey,
        balance: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = owner;
        vault.balance = balance;
        vault.open = true;
        Ok(())
    }

    // 入金処理
    pub fn deposit(
        ctx: Context<ModifyVault>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let fees = &mut ctx.accounts.fees_account;
        vault.balance = vault.balance.checked_add(amount).unwrap();
        // 手数料口座も毎回上書き
        fees.balance = fees.balance.checked_add(amount / 100).unwrap();
        Ok(())
    }

    // 出金処理
    pub fn withdraw(
        ctx: Context<ModifyVault>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance = vault.balance.checked_sub(amount).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultData>,
    /// 手数料計算用、初期値なしで任意のアカウントを上書き
    #[account(mut)]
    pub fees_account: Account<'info, FeeData>,
}

#[account]
pub struct VaultData {
    pub owner: Pubkey,
    pub balance: u64,
    pub open: bool,
}

#[account]
pub struct FeeData {
    pub balance: u64,
}
