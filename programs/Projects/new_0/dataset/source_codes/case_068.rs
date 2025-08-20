use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfTitleToken");

#[program]
pub mod rank_title_reward {
    use super::*;

    pub fn calculate_rank_title_reward(
        ctx: Context<UserContext>,
        activity_score: u64,
    ) -> Result<OutputBundle> {
        let capped_score = core::cmp::min(activity_score, 9999);
        let rank = (capped_score / 1000) + 1; // rank = 1〜10

        // 整数のみで称号生成
        let base = "Scholar";
        let title = format!("{} Lv{}", base, rank);

        // ボーナス：ランクが偶数なら+250, 奇数なら+100
        let is_even = (rank % 2 == 0) as u64;
        let bonus = 250 * is_even + 100 * (1 - is_even);
        let reward_tokens = rank * 100 + bonus;

        msg!("--- User Info ---");
        msg!("Rank         : {}", rank);
        msg!("Title        : {}", title);
        msg!("Token Reward : {}", reward_tokens);

        Ok(OutputBundle {
            rank,
            title,
            reward_tokens,
        })
    }
}

#[derive(Accounts)]
pub struct UserContext<'info> {
    pub user: Signer<'info>,
}

// Output structure: returned to frontend
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OutputBundle {
    pub rank: u64,
    pub title: String,
    pub reward_tokens: u64,
}
