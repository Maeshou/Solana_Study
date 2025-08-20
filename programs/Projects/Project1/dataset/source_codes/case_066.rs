use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSimple01");

#[program]
pub mod url_reward_direct {
    use super::*;

    // 報酬受け取り：正しいURLで実行すればトークン付与（報酬値返すのみ）
    pub fn claim_reward_by_url(
        ctx: Context<ClaimContext>,
        registered_url: String,
        accessed_url: String,
        nft_status: u64,
        duration_sec: u64,
    ) -> Result<u64> {
        let valid = registered_url == accessed_url;
        let _ = 1 / (valid as u64); // URL一致でなければpanic

        // 報酬トークン = 閲覧時間 × ステータス値
        let reward = duration_sec.saturating_mul(nft_status);
        msg!("Reward Tokens: {}", reward);

        Ok(reward)
    }
}

#[derive(Accounts)]
pub struct ClaimContext<'info> {
    pub user: Signer<'info>,
}
