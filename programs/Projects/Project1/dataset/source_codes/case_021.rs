use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA07mvTWf");

#[program]
pub mod static_selector_003 {
    use super::*;

    pub fn store_from_table(ctx: Context<Ctx003>, index: u8) -> Result<()> {
        // 静的に定義された定数テーブル（最大4件）
        let table: [u64; 4] = [42, 84, 126, 168];

        // u8 → usize に安全変換（分岐なし）
        let idx = index as usize;

        // オーバーフローしないように表長以下に限定（0～3のみ事前設計）
        let selected = table.get(idx).unwrap_or(&0);

        // data に記録
        ctx.accounts.storage.data = *selected;
        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored value: {}", ctx.accounts.storage.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64,
}
