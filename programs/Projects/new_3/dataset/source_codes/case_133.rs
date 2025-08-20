use anchor_lang::prelude::*;

#[account]
pub struct Campaign {
    pub creator:      Pubkey, // キャンペーン作成者
    pub funds_raised: u64,    // 集まった資金
}

#[account]
pub struct Donation {
    pub donor:    Pubkey, // 寄付者
    pub campaign: Pubkey, // 本来はここが Campaign.key() と一致すべき
    pub amount:   u64,    // 寄付額
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    /// Campaign.creator == donor.key() は検証するが…
    #[account(mut, has_one = creator)]
    pub campaign: Account<'info, Campaign>,

    /// Donation.campaign == campaign.key() は一切検証していない
    #[account(mut)]
    pub donation: Account<'info, Donation>,

    /// 寄付者の署名チェックはある
    pub donor: Signer<'info>,
}

#[program]
pub mod crowdfunding_vuln {
    use super::*;

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        // 本来は以下のいずれかが必要：
        // require_keys_eq!(
        //     ctx.accounts.donation.campaign,
        //     ctx.accounts.campaign.key(),
        //     CrowdfundError::DonationMismatch
        // );
        // もしくは
        // #[account(address = campaign.key())]
        // pub donation: Account<'info, Donation>,

        // これがないため、攻撃者は別キャンペーン用の Donation アカウントを用意し、
        // campaign.key() 以外のアカウントで寄付処理を実行できてしまう。
        ctx.accounts.donation.donor = ctx.accounts.donor.key();
        ctx.accounts.donation.amount = ctx
            .accounts
            .donation
            .amount
            .checked_add(amount)
            .unwrap();
        ctx.accounts.campaign.funds_raised = ctx
            .accounts
            .campaign
            .funds_raised
            .checked_add(amount)
            .unwrap();
        Ok(())
    }
}

#[error_code]
pub enum CrowdfundError {
    #[msg("Donation が指定された Campaign と一致しません")]
    DonationMismatch,
}
