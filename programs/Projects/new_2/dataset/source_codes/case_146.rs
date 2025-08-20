use anchor_lang::prelude::*;

declare_id!("MktVarAA0000000000000000000000000000000AA");

#[program]
pub mod marketplace_var10 {
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        let mp = &mut ctx.accounts.market;
        // 属性チェック (has_one) で lister 検証
        mp.price = price;
        mp.active = true;

        // meta_acc は unchecked でバイト操作
        let mut md = ctx.accounts.meta_acc.data.borrow_mut();
        md[0..8].copy_from_slice(&price.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut, has_one = lister)]
    pub market: Account<'info, MarketData>,
    pub lister: Signer<'info>,
    #[account(mut)] pub meta_acc: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct MarketData {
    pub lister: Pubkey,
    pub price: u64,
    pub active: bool,
}
