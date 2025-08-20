use anchor_lang::prelude::*;

declare_id!("OwnChkC2000000000000000000000000000000002");

#[program]
pub mod campaign_manager {
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        budget: u64,
    ) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        // 属性レベルで admin を検証
        c.name   = name.clone();
        c.budget = budget;
        c.active = true;

        // log_acc は unchecked
        let mut buf = ctx.accounts.log_acc.data.borrow_mut();
        buf.extend_from_slice(name.as_bytes());
        buf.extend_from_slice(&budget.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1, has_one = admin)]
    pub campaign: Account<'info, CampaignData>,
    pub admin: Signer<'info>,
    /// CHECK: ログ用アカウント、所有者検証なし
    #[account(mut)]
    pub log_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CampaignData {
    pub admin: Pubkey,
    pub name: String,
    pub budget: u64,
    pub active: bool,
}
