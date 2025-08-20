use anchor_lang::prelude::*;
declare_id!("CrowdSafe1111111111111111111111111111111");

/// キャンペーン情報
#[account]
pub struct Campaign {
    pub organizer: Pubkey,  // キャンペーン主催者
    pub goal:      u64,     // 目標金額
    pub raised:    u64,     // これまでの寄付合計
}

/// 寄付記録
#[account]
pub struct Donation {
    pub donor:    Pubkey,   // 寄付者
    pub campaign: Pubkey,   // Campaign.key()
    pub amount:   u64,      // 寄付額
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 8 + 8)]
    pub campaign:      Account<'info, Campaign>,
    #[account(mut)]
    pub organizer:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    /// organizer が正しいことをチェック
    #[account(mut, has_one = organizer)]
    pub campaign:      Account<'info, Campaign>,

    /// Donation.campaign == campaign.key()、Donation.donor == donor.key() をチェック
    #[account(
        init,
        payer = donor,
        space = 8 + 32 + 32 + 8,
        has_one = campaign,
        has_one = donor
    )]
    pub donation:      Account<'info, Donation>,

    #[account(mut)]
    pub organizer:     Signer<'info>,
    #[account(mut)]
    pub donor:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeDonation<'info> {
    /// Donation.campaign と campaign.key()、Donation.donor と donor.key() をチェック
    #[account(mut, has_one = campaign, has_one = donor)]
    pub donation:      Account<'info, Donation>,

    #[account(mut)]
    pub campaign:      Account<'info, Campaign>,
    #[account(mut)]
    pub donor:         Signer<'info>,
}

#[program]
pub mod crowdfunding_safe {
    use super::*;

    /// 新しいキャンペーンを作成
    pub fn create_campaign(ctx: Context<CreateCampaign>, goal: u64) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.organizer = ctx.accounts.organizer.key();
        c.goal      = goal;
        c.raised    = 0;
        Ok(())
    }

    /// 寄付を行う
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let c  = &mut ctx.accounts.campaign;
        let d  = &mut ctx.accounts.donation;

        // 明示的にフィールドをセット
        d.donor    = ctx.accounts.donor.key();
        d.campaign = ctx.accounts.campaign.key();
        d.amount   = amount;

        // 二重チェック（optional）
        require_keys_eq!(d.campaign, c.key(), CrowdError::CampaignMismatch);
        require_keys_eq!(d.donor, ctx.accounts.donor.key(), CrowdError::DonorMismatch);

        // 寄付額を累積
        c.raised = c
            .raised
            .checked_add(amount)
            .ok_or(CrowdError::Overflow)?;

        Ok(())
    }

    /// 寄付を確定（例：払い戻しや締め切り処理など）
    pub fn finalize_donation(ctx: Context<FinalizeDonation>) -> Result<()> {
        let c  = &mut ctx.accounts.campaign;
        let d  = &ctx.accounts.donation;

        // 再チェック
        require_keys_eq!(d.campaign, c.key(), CrowdError::CampaignMismatch);
        require_keys_eq!(d.donor, ctx.accounts.donor.key(), CrowdError::DonorMismatch);

        // 特に他の処理はなし
        Ok(())
    }
}

#[error_code]
pub enum CrowdError {
    #[msg("Donation.campaign が Campaign に一致しません")]
    CampaignMismatch,
    #[msg("Donation.donor が Donor に一致しません")]
    DonorMismatch,
    #[msg("金額の累積でオーバーフローが発生しました")]
    Overflow,
}
