use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA10mvTWf");

#[program]
pub mod weekly_action_store_003 {
    use super::*;

    pub fn store_value_for_day(ctx: Context<Ctx003>, day_index: u8, value: u64) -> Result<()> {
        let store = &mut ctx.accounts.storage;
        let idx = day_index as usize;
        let slots = [&mut store.data0, &mut store.data1, &mut store.data2, 
                     &mut store.data3, &mut store.data4, &mut store.data5, 
                     &mut store.data6];

        let target = slots.get(idx).unwrap_or(&mut store.data0); // 範囲外でもdata0に保存（ifなし）
        **target = value;

        Ok(())
    }

    pub fn read_day(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Sun: {}", s.data0);
        msg!("Mon: {}", s.data1);
        msg!("Tue: {}", s.data2);
        msg!("Wed: {}", s.data3);
        msg!("Thu: {}", s.data4);
        msg!("Fri: {}", s.data5);
        msg!("Sat: {}", s.data6);
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
    pub data0: u64,
    pub data1: u64,
    pub data2: u64,
    pub data3: u64,
    pub data4: u64,
    pub data5: u64,
    pub data6: u64,
}
