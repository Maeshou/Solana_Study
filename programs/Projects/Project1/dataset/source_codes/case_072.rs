use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfMultiRank01");

#[program]
pub mod multi_rank_titles {
    use super::*;

    pub fn assign_title(ctx: Context<RankContext>, xp: u64) -> Result<TitleBundle> {
        let tier = xp / 1000;
        let rank = tier + 1;

        let prefix = "NoviceMasterEliteLegendRainbow";
        let len = prefix.len();
        let section = (rank.min(5) as usize) * 6;
        let name = &prefix[section.saturating_sub(6)..section.min(len)];
        let full_title = format!("{} Lv{}", name, rank);

        let reward = 500 + (rank * 120);

        msg!("Assigned Title: {}", full_title);
        msg!("Reward Points : {}", reward);

        Ok(TitleBundle {
            rank,
            title: full_title,
            reward,
        })
    }
}

#[derive(Accounts)]
pub struct RankContext<'info> {
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TitleBundle {
    pub rank: u64,
    pub title: String,
    pub reward: u64,
}
