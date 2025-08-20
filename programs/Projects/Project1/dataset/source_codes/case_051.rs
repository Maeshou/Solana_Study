use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfF01mvTWf");

#[program]
pub mod nft_rank_fusion_003 {
    use super::*;

    pub fn fuse_nfts(ctx: Context<FuseCtx>, nft_rank_1: u8, nft_rank_2: u8, random_seed: u64) -> Result<()> {
        let outcome = &mut ctx.accounts.result;

        let base_rank = (nft_rank_1 + nft_rank_2) / 2;

        // 擬似確率判定：random_seed % 100 < 30 → 成功（30%想定）
        let random_value = random_seed % 100;
        let success_flag = (100 - random_value) / 71; // 0:fail, 1:success（30%相当）

        // 新ランクは base_rank + success_flag
        outcome.result_rank = base_rank + success_flag as u8;
        outcome.base_rank = base_rank;
        outcome.success = success_flag as u8;

        Ok(())
    }

    pub fn show(ctx: Context<FuseCtx>) -> Result<()> {
        let r = &ctx.accounts.result;
        msg!("Base Rank: {}", r.base_rank);
        msg!("Result Rank: {}", r.result_rank);
        msg!("Success: {}", r.success);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FuseCtx<'info> {
    #[account(mut, has_one = user)]
    pub result: Account<'info, FusionOutcome>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct FusionOutcome {
    pub user: Pubkey,
    pub base_rank: u8,
    pub result_rank: u8,
    pub success: u8, // 1=成功, 0=失敗
}
