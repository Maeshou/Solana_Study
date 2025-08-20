use anchor_lang::prelude::*;

declare_id!("CrowdVuln222222222222222222222222222222222");

/// 資金調達キャンペーン情報
#[account]
pub struct Campaign {
    pub owner:                 Pubkey, // キャンペーン作成者
    pub total_contributions:   u64,    // 累積寄付総額
    pub backers_count:         u64,    // 支援者数
}

/// 寄付記録
#[account]
pub struct Donation {
    pub donor:        Pubkey, // 寄付者
    pub campaign:     Pubkey, // 本来は Campaign.key() と一致すべき
    pub contribution: u64,    // 個別累積寄付額
}

#[derive(Accounts)]
pub struct InitializeCampaign<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub campaign:       Account<'info, Campaign>,
    #[account(mut)]
    pub owner:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakeDonation<'info> {
    /// Campaign.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub campaign:       Account<'info, Campaign>,

    /// Donation.campaign == campaign.key() の検証が **ない**
    #[account(init_if_needed, payer = donor, space = 8 + 32 + 32 + 8)]
    pub donation:       Account<'info, Donation>,

    #[account(mut)]
    pub donor:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod crowdfunding_vuln2 {
    use super::*;

    pub fn initialize_campaign(ctx: Context<InitializeCampaign>, _goal: u64) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.owner               = ctx.accounts.owner.key();
        c.total_contributions = 0;
        c.backers_count       = 0;
        Ok(())
    }

    pub fn make_donation(ctx: Context<MakeDonation>, amount: u64) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        let d = &mut ctx.accounts.donation;

        // ── 脆弱性ポイント ──
        // d.campaign と c.key() の整合性チェックがないため、
        // 攻撃者が偽の Donation アカウントを用意して渡すと通ってしまう。

        d.donor        = ctx.accounts.donor.key();
        d.campaign     = c.key();

        // ■ 累積寄付額は saturating_add で更新
        d.contribution = d.contribution.saturating_add(amount);
        // ■ 総寄付額は checked_add＋unwrap_or で加算
        c.total_contributions = c
            .total_contributions
            .checked_add(amount)
            .unwrap_or(c.total_contributions);
        // ■ 支援者数は saturating_add でインクリメント
        c.backers_count = c.backers_count.saturating_add(1);

        Ok(())
    }
}

#[error_code]
pub enum CrowdfundError {
    #[msg("Donation が指定の Campaign と一致しません")]
    CampaignMismatch,
}
