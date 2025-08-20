use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTqQ5hX7shYwB2qhJT2VJY5p8");

#[program]
pub mod payment_processor {
    use super::*;

    /// 任意のアカウントから資金を引き出せてしまう関数
    pub fn transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()> {
        let vault_info    = ctx.accounts.vault.to_account_info();
        let recipient_info = ctx.accounts.recipient.clone();
        // 本来ここで authority に #[account(signer)] を付ける必要があるが、省略している
        let _authority    = &ctx.accounts.authority;
        // 追加の未チェックアカウント（例として利用）
        let _fallback     = &ctx.accounts.fallback;

        // lamports を直接操作（署名チェックなしで実行可能）
        **vault_info.try_borrow_mut_lamports()?    -= amount;
        **recipient_info.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferContext<'info> {
    /// 資金を保管するアカウント
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// 出金先アカウント（AccountInfo を使用）
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    /// CHECK: 署名者チェックが省略されたままのアカウント
    pub authority: AccountInfo<'info>,

    /// CHECK: UncheckedAccount も併用
    pub fallback: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 本来はここで authority を検証する
    pub authority: Pubkey,
}
