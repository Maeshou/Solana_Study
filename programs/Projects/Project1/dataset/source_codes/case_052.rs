use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfF02mvTWf");

#[program]
pub mod nft_enhance_lottery_003 {
    use super::*;

    pub fn enhance_five(ctx: Context<EnhanceCtx>, base_rank: u8, random_seed: u64) -> Result<()> {
        let result = &mut ctx.accounts.result;

        result.user = ctx.accounts.user.key();
        result.base_rank = base_rank;

        let r = random_seed % 1000;

        // 確率（整数演算で模倣）：
        // 000–799: 1ランクアップ (80%)
        // 800–949: 2ランクアップ (15%)
        // 950–989: レインボースニーカー (4%)
        // 990–999: 特殊景品 (1%)

        let is_rank1 = (r < 800) as u8;
        let is_rank2 = ((r >= 800) as u8) * ((r < 950) as u8);
        let is_rainbow = ((r >= 950) as u8) * ((r < 990) as u8);
        let is_special = ((r >= 990) as u8);

        result.result_type = is_rank1 * 0 + is_rank2 * 1 + is_rainbow * 2 + is_special * 3;
        result.result_rank = base_rank + is_rank1 + is_rank2 * 2;

        Ok(())
    }

    pub fn show(ctx: Context<EnhanceCtx>) -> Result<()> {
        let r = &ctx.accounts.result;
        msg!("User: {}", r.user);
        msg!("Base Rank: {}", r.base_rank);
        msg!("Result Type: {}", r.result_type); // 0:Normal, 1:Double, 2:Rainbow, 3:Special
        msg!("Result Rank: {}", r.result_rank);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnhanceCtx<'info> {
    #[account(mut, has_one = user)]
    pub result: Account<'info, EnhanceResult>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct EnhanceResult {
    pub user: Pubkey,
    pub base_rank: u8,
    pub result_type: u8, // 0=+1, 1=+2, 2=rainbow, 3=special
    pub result_rank: u8,
}
