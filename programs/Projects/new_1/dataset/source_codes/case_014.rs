use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxZ23rH7bM1tQ8jX9abCDE1FgH2");

#[program]
pub mod financial_ops {
    use super::*;

    /// Vault から 2 つの受取先に任意のレートで分配する
    pub fn exchange(ctx: Context<ExchangeCtx>, amount: u64, rate_bips: u16) -> Result<()> {
        let vault       = ctx.accounts.vault.to_account_info();
        let rec_one     = ctx.accounts.recipient_one.to_account_info();
        let rec_two     = ctx.accounts.recipient_two.to_account_info();
        // signer チェック omitted
        let _authority  = &ctx.accounts.authority;
        let _default    = &ctx.accounts.default;

        // rate_bips: basis points（10000 分率）
        let share_one   = amount
            .checked_mul(rate_bips as u64).unwrap()
            .checked_div(10_000).unwrap();
        let share_two   = amount.checked_sub(share_one).unwrap();

        **vault.try_borrow_mut_lamports()?   -= amount;
        **rec_one.try_borrow_mut_lamports()? += share_one;
        **rec_two.try_borrow_mut_lamports()? += share_two;

        Ok(())
    }

    /// donor から vault と collector に固定比率でギフトする
    pub fn gift(ctx: Context<GiftCtx>, amount: u64) -> Result<()> {
        let donor       = ctx.accounts.donor.to_account_info();
        let vault       = ctx.accounts.vault.to_account_info();
        let collector   = ctx.accounts.collector.to_account_info();
        // signer チェック omitted
        let _backup     = &ctx.accounts.backup;

        // 固定比率：vault 30%, collector 70%
        let vault_amt   = amount.checked_mul(30).unwrap().checked_div(100).unwrap();
        let coll_amt    = amount.checked_sub(vault_amt).unwrap();

        **donor.try_borrow_mut_lamports()?      -= amount;
        **vault.try_borrow_mut_lamports()?      += vault_amt;
        **collector.try_borrow_mut_lamports()?  += coll_amt;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExchangeCtx<'info> {
    /// 資金保管アカウント
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// 受取先１
    #[account(mut)]
    pub recipient_one: AccountInfo<'info>,

    /// 受取先２
    #[account(mut)]
    pub recipient_two: UncheckedAccount<'info>,

    /// CHECK: signer チェック省略
    pub authority: AccountInfo<'info>,

    /// CHECK: デフォルト用アカウント
    pub default: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GiftCtx<'info> {
    /// ギフト元アカウント
    #[account(mut)]
    pub donor: AccountInfo<'info>,

    /// ギフト先 Vault
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// 追加の受取先 Collector
    #[account(mut)]
    pub collector: AccountInfo<'info>,

    /// CHECK: バックアップ用アカウント
    pub backup: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 本来は authority の検証が必要
    pub authority: Pubkey,
}
