use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEJp3hB6Qh2");

#[program]
pub mod vulnerable_data_match {
    use super::*;

    /// カウンターの値を上書きするが、対応する所有者アカウントとのマッチング検証を行っていない
    pub fn update_counter(ctx: Context<UpdateCounter>, new_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
        // ↓ 本来は authority.key と counter.owner が一致するか検証すべきだが、チェックがない
        counter.count = new_count;
        Ok(())
    }
}

/// チェック属性が欠けているため、任意の counter_account を渡すだけで書き換え可能
#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>,
    /// 本来は #[account(signer, has_one = owner)] のように書くべき
    pub owner: Signer<'info>,
}

#[account]
pub struct Counter {
    /// どのアカウントがこのカウンターを管理しているかを示すフィールド
    pub owner: Pubkey,
    pub count: u64,
}
