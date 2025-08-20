use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkB8000000000000000000000000000000008");

#[program]
pub mod voting_system {
    pub fn cast_vote(ctx: Context<Cast>, option: u8) -> Result<()> {
        let v = &mut ctx.accounts.votes;
        // has_one で owner チェック済み
        *v.counts.entry(option).or_insert(0) += 1;
        v.total_votes = v.total_votes.saturating_add(1);

        // external_log は unchecked
        ctx.accounts.external_log.data.borrow_mut().extend_from_slice(&[option]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cast<'info> {
    #[account(mut, has_one = owner)]
    pub votes: Account<'info, VoteData>,
    pub owner: Signer<'info>,
    /// CHECK: 外部ログ、所有者検証なし
    #[account(mut)]
    pub external_log: AccountInfo<'info>,
}

#[account]
pub struct VoteData {
    pub owner: Pubkey,
    pub counts: BTreeMap<u8, u64>,
    pub total_votes: u64,
}
