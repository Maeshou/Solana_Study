use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA11mvTWf");

#[program]
pub mod key_mapped_storage_003 {
    use super::*;

    pub fn save_mapped_value(ctx: Context<Ctx003>, value: u64) -> Result<()> {
        let auth = ctx.accounts.authority.key();
        let store = &mut ctx.accounts.storage;

        let key1 = store.slot1;
        let key2 = store.slot2;
        let key3 = store.slot3;

        // 判定をboolとして事前に算出（ifやmatchは使わない）
        let a = (auth == key1) as u8;
        let b = (auth == key2) as u8;
        let c = (auth == key3) as u8;

        // 書き込むスロット決定（優先度順：slot1→slot2→slot3→fallback）
        let targets = [&mut store.data0, &mut store.data1, &mut store.data2];
        let idx = (a * 0) + (b * 1) + (c * 2); // あえて固定スロット番号を生成
        let dest = targets.get(idx as usize).unwrap_or(&mut store.data0);
        **dest = value;

        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Data0: {}", s.data0);
        msg!("Data1: {}", s.data1);
        msg!("Data2: {}", s.data2);
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
    pub slot1: Pubkey,
    pub slot2: Pubkey,
    pub slot3: Pubkey,
    pub data0: u64,
    pub data1: u64,
    pub data2: u64,
}
