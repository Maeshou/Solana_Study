use anchor_lang::prelude::*;

// Serum DEX Program ID v3
const SERUM_DEX_V3: Pubkey = pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");

declare_id!("READonly1111111111111111111111111111111111");

#[program]
pub mod read_other_program_data {
    use super::*;
    pub fn read_market_info(ctx: Context<ReadMarketInfo>) -> Result<()> {
        let market = &ctx.accounts.serum_market;
        // これで安全にSerumマーケットのアカウント情報にアクセスできる
        // market.bids, market.asksなど
        msg!("Market base mint: {}", market.base_mint);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadMarketInfo<'info> {
    // このアカウントのオーナーがSerum DEXプログラムであることを検証
    #[account(owner = SERUM_DEX_V3)]
    pub serum_market: AccountInfo<'info>,
}