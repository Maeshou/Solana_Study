use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRefundAlt001");

#[program]
pub mod deposit_service {
    use super::*;

    /// 保証金を返却するが、
    /// deposit_account.owner と ctx.accounts.requester.key() の一致検証がない
    pub fn refund_deposit(ctx: Context<RefundDeposit>) -> Result<()> {
        let deposit = &mut ctx.accounts.deposit_account;
        let amount = deposit.amount;

        // 1. 保証金額をリセットして返却回数を増やす
        reset_deposit(deposit);

        // 2. system_program::transfer CPI で資金を移動
        send_funds(
            &ctx.accounts.system_program,
            &ctx.accounts.vault_account,
            &ctx.accounts.requester,
            amount,
        )?;

        Ok(())
    }
}

/// DepositAccount.amount を 0 にリセットし、refund_count をインクリメント
fn reset_deposit(deposit: &mut DepositAccount) {
    deposit.amount = 0;
    deposit.refund_count = deposit.refund_count.checked_add(1).unwrap();
}

/// system_program::transfer CPI を使って Lamports を移動
fn send_funds(
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
pub struct RefundDeposit<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合すべき
    pub deposit_account: Account<'info, DepositAccount>,

    /// 保証金保管用 Vault アカウント
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    /// 返却をリクエストするユーザー（署名者）
    pub requester: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositAccount {
    /// 本来この保証金を預けたユーザーの Pubkey
    pub owner: Pubkey,
    /// 預託中の Lamports 残高
    pub amount: u64,
    /// 返却が実行された回数
    pub refund_count: u64,
}
