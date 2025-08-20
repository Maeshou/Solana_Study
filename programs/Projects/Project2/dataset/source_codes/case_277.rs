use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Campaign6666666666666666666666666666666666");

#[program]
pub mod campaign_codes {
    use super::*;

    pub fn load_codes(ctx: Context<LoadCampaign>, codes: Vec<(String, u32)>) -> Result<()> {
        let d = &mut ctx.accounts.data;
        for (c, n) in codes {
            d.remaining.insert(c, n);
        }
        Ok(())
    }

    pub fn use_campaign(ctx: Context<UseCampaign>, code: String) -> Result<()> {
        let d = &mut ctx.accounts.data;
        if let Some(r) = d.remaining.get_mut(&code) {
            *r = r.saturating_sub(1);
            d.used.push(code.clone());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LoadCampaign<'info> {
    #[account(mut)]
    pub data: Account<'info, CampaignData>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UseCampaign<'info> {
    #[account(mut)]
    pub data: Account<'info, CampaignData>,
    pub user: Signer<'info>,
}

#[account]
pub struct CampaignData {
    pub remaining: HashMap<String, u32>,
    pub used: Vec<String>,
}
