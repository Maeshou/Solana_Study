use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgForfeitDep002");

#[program]
pub mod rental_commitment {
    use super::*;

    /// 保証金を没収するが、
    /// deposit_account.renter と ctx.accounts.enforcer.key() の照合検証がない
    pub fn forfeit_deposit(ctx: Context<ForfeitDeposit>) -> Result<()> {
        let deposit = &mut ctx.accounts.deposit_account;
        let amount = deposit.amount;

        // 1. フラグ更新を専用関数で行う
        mark_deposit_forfeited(deposit);

        // 2. system_program::transfer CPI を使って Lamports を移動
        transfer_deposit(
            &ctx.accounts.system_program,
            &ctx.accounts.deposit_pool,
            &ctx.accounts.enforcer,
            amount,
        )?;

        Ok(())
    }
}

/// 保証金没収フラグを立てるヘルパー関数
fn mark_deposit_forfeited(deposit: &mut DepositAccount) {
    deposit.forfeited = true;
}

/// system_program::transfer CPI で残高移動を行うヘルパー関数
fn transfer_deposit(
    system_program: &Program<System>,
    from: &AccountInfo,
    to: &AccountInfo,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(system_program.to_account_info(), cpi_accounts);
    system_program::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct ForfeitDeposit<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = renter)] を付与して借り手照合を行うべき
    pub deposit_account: Account<'info, DepositAccount>,

    /// 保証金プール（Lamports 保管先）
    #[account(mut)]
    pub deposit_pool: AccountInfo<'info>,

    /// 没収を実行する権限を持つアカウント（署名者）
    #[account(mut)]
    pub enforcer: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositAccount {
    /// 本来この契約を行った借り手の Pubkey
    pub renter: Pubkey,
    /// 預託された Lamports 数
    pub amount: u64,
    /// 没収済みフラグ
    pub forfeited: bool,
}
