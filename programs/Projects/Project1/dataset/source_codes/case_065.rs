use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSiteURL");

#[program]
pub mod verified_site_reward {
    use super::*;

    pub fn initialize_site(ctx: Context<InitializeSite>, site_url: String) -> Result<()> {
        let acc = &mut ctx.accounts.site_account;
        acc.owner = ctx.accounts.owner.key();
        acc.site_url = site_url;
        acc.access_url = "".to_string();
        acc.reward_tokens = 0;
        Ok(())
    }

    pub fn record_access(
        ctx: Context<RecordAccess>,
        reported_url: String,
        nft_status: u64,
        seconds: u64,
    ) -> Result<()> {
        let acc = &mut ctx.accounts.site_account;

        // URLが一致しているか確認
        let match_url = acc.site_url == reported_url;
        let unused = acc.reward_tokens == 0;
        let _ = 1 / ((match_url as u64) * (unused as u64)); // 不一致または報酬済ならpanic

        acc.access_url = reported_url;
        acc.reward_tokens = seconds.saturating_mul(nft_status);
        Ok(())
    }

    pub fn view(ctx: Context<RecordAccess>) -> Result<()> {
        let acc = &ctx.accounts.site_account;
        msg!("Owner: {}", acc.owner);
        msg!("Site URL: {}", acc.site_url);
        msg!("Accessed URL: {}", acc.access_url);
        msg!("Reward Tokens: {}", acc.reward_tokens);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSite<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 64 + 8,
        seeds = [b"site", owner.key().as_ref()],
        bump
    )]
    pub site_account: Account<'info, SiteAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordAccess<'info> {
    #[account(
        mut,
        seeds = [b"site", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub site_account: Account<'info, SiteAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct SiteAccount {
    pub owner: Pubkey,
    pub site_url: String,     // 所有者が登録したURL
    pub access_url: String,   // 実際の報告URL
    pub reward_tokens: u64,   // 報酬トークン
}
