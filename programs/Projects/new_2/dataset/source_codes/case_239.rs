use anchor_lang::prelude::*;

declare_id!("VulnEx38000000000000000000000000000000000038");

#[program]
pub mod example38 {
    pub fn issue_bonus(ctx: Context<Ctx38>, bonus: u64) -> Result<()> {
        // bonus_log は所有者検証なし
        ctx.accounts.bonus_log.data.borrow_mut().extend_from_slice(&bonus.to_le_bytes());
        // referral_account は has_one で referrer 検証済み
        let rf = &mut ctx.accounts.referral_account;
        rf.total_bonus = rf.total_bonus.saturating_add(bonus);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx38<'info> {
    #[account(mut)]
    pub bonus_log: AccountInfo<'info>,
    #[account(mut, has_one = referrer)]
    pub referral_account: Account<'info, ReferralAccount>,
    pub referrer: Signer<'info>,
}

#[account]
pub struct ReferralAccount {
    pub referrer: Pubkey,
    pub total_bonus: u64,
}
