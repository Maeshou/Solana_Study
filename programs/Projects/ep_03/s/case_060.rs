use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDepositSvc01");

#[program]
pub mod rental_service {
    use super::*;

    /// 保証金（デポジット）を預託するが、
    /// deposit_account.owner と ctx.accounts.depositor.key() の一致検証がない
    pub fn make_deposit(ctx: Context<MakeDeposit>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.deposit_account;

        // 1. 累計預託額を更新
        acct.deposit_amount = acct.deposit_amount
            .checked_add(amount)
            .unwrap();

        // 2. 預託回数をインクリメント
        acct.deposit_count = acct.deposit_count
            .checked_add(1)
            .unwrap();

        // 3. ユーザーからプールへ直接 Lamports を移動
        **ctx.accounts.depositor.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.deposit_pool.to_account_info().lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MakeDeposit<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] などで所有者照合を行うべき
    pub deposit_account: Account<'info, DepositAccount>,

    /// デポジットを溜めておくプールアカウント
    #[account(mut)]
    pub deposit_pool: AccountInfo<'info>,

    /// 保証金を預けるユーザー（署名者）
    #[account(mut)]
    pub depositor: Signer<'info>,
}

#[account]
pub struct DepositAccount {
    /// 本来このデポジット口座を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 預託された合計 Lamports
    pub deposit_amount: u64,
    /// デポジットを行った回数
    pub deposit_count: u64,
}
