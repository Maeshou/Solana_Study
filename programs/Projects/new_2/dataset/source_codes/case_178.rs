use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkCAA0000000000000000000000000000000A");

#[program]
pub mod governance {
    pub fn cast_vote(
        ctx: Context<CastVote>,
        option: u8,
    ) -> Result<()> {
        let g = &mut ctx.accounts.poll;
        // 属性レベルで chairperson を検証
        *g.votes.entry(option).or_insert(0) += 1;

        // vote_log は unchecked
        ctx.accounts.vote_log.data.borrow_mut().push(option);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut, has_one = chairperson)]
    pub poll: Account<'info, PollData>,
    pub chairperson: Signer<'info>,
    /// CHECK: 投票ログ、所有者検証なし
    #[account(mut)]
    pub vote_log: AccountInfo<'info>,
}

#[account]
pub struct PollData {
    pub chairperson: Pubkey,
    pub votes: BTreeMap<u8, u64>,
}
