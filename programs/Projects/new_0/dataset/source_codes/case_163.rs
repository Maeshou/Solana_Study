use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUJ");

#[program]
pub mod donation_campaign {
    use super::*;

    /// キャンペーン作成：ID・説明・目標金額を受け取り、集計と状態を初期化
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        bump: u8,
        campaign_id: u64,
        description: String,
        goal: u64,
    ) -> Result<()> {
        *ctx.accounts.campaign = Campaign {
            owner:       ctx.accounts.organizer.key(),
            bump,
            campaign_id,
            description,
            goal,
            collected:   0,
            state:       String::from("active"),
        };
        Ok(())
    }

    /// 寄付実行：渡された金額を `collected` に加算し、状態は維持
    pub fn donate(
        ctx: Context<ModifyCampaign>,
        amount: u64,
    ) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.collected = c.collected.wrapping_add(amount);
        Ok(())
    }

    /// キャンペーン終了：`collected` と `goal` を比較し、状態を文字列で更新
    pub fn finalize_campaign(
        ctx: Context<ModifyCampaign>,
    ) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.state = if c.collected >= c.goal {
            String::from("successful")
        } else {
            String::from("failed")
        };
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, campaign_id: u64)]
pub struct CreateCampaign<'info> {
    /// PDA で生成する Campaign アカウント
    #[account(
        init,
        payer = organizer,
        // discriminator(8) + owner(32) + bump(1) + campaign_id(8)
        // + 4 + max 200 bytes for description
        // + 8 (goal) + 8 (collected)
        // + 4 + max 10 bytes for state string
        space = 8 + 32 + 1 + 8 + 4 + 200 + 8 + 8 + 4 + 10,
        seeds = [b"campaign", organizer.key().as_ref(), &campaign_id.to_le_bytes()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// キャンペーン主催者（署名必須）
    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCampaign<'info> {
    /// 既存の Campaign（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"campaign", campaign.owner.as_ref(), &campaign.campaign_id.to_le_bytes()],
        bump = campaign.bump,
        has_one = owner
    )]
    pub campaign: Account<'info, Campaign>,

    /// キャンペーン所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Campaign {
    pub owner:        Pubkey,
    pub bump:         u8,
    pub campaign_id:  u64,
    pub description:  String,
    pub goal:         u64,
    pub collected:    u64,
    pub state:        String,
}
