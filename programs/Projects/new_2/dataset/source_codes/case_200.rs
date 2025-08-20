use anchor_lang::prelude::*;

declare_id!("OwnChkE1000000000000000000000000000000002");

#[program]
pub mod register_vote {
    pub fn vote(
        ctx: Context<Register>,
        candidate: Pubkey,
    ) -> Result<()> {
        let ballot = &mut ctx.accounts.ballot;
        // 属性レベルで投票所管理者を検証
        *ballot.tally.entry(candidate).or_insert(0) += 1;
        ballot.total_votes = ballot.total_votes.saturating_add(1);

        // buffer_acc は unchecked で生データ書き込み
        let mut buf = ctx.accounts.buffer_acc.data.borrow_mut();
        buf.extend_from_slice(&candidate.to_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut, has_one = manager)]
    pub ballot: Account<'info, BallotBox>,
    pub manager: Signer<'info>,
    /// CHECK: 生バッファアカウント、所有者検証なし
    #[account(mut)]
    pub buffer_acc: AccountInfo<'info>,
}

#[account]
pub struct BallotBox {
    pub manager: Pubkey,
    pub tally: std::collections::BTreeMap<Pubkey, u64>,
    pub total_votes: u64,
}
