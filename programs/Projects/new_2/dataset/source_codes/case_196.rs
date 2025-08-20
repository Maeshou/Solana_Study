use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkD7000000000000000000000000000000008");

#[program]
pub mod crowdfunding {
    pub fn pledge(
        ctx: Context<Pledge>,
        amount: u64,
    ) -> Result<()> {
        let cf = &mut ctx.accounts.crowdfund;
        // 属性レベルで organizer を検証
        *cf.pledges.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        cf.total_pledged = cf.total_pledged.saturating_add(amount);

        // ledger は unchecked
        ctx.accounts.ledger.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Pledge<'info> {
    #[account(mut, has_one = organizer)]
    pub crowdfund: Account<'info, CrowdfundData>,
    pub organizer: Signer<'info>,
    pub user: Signer<'info>,
    /// CHECK: 台帳アカウント、所有者検証なし
    #[account(mut)]
    pub ledger: AccountInfo<'info>,
}

#[account]
pub struct CrowdfundData {
    pub organizer: Pubkey,
    pub total_pledged: u64,
    pub pledges: BTreeMap<Pubkey, u64>,
}
