use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqVxVote123");

#[program]
pub mod insecure_governance {
    use super::*;

    /// 提案に投票を追加するインストラクション
    pub fn submit_vote(ctx: Context<SubmitVote>, vote_option: u8) -> Result<()> {
        // ★ オーナーチェックをしていないため、任意の外部アカウントをproposal_accountとして渡されると
        //   dataを不正に上書きされる可能性がある
        let mut proposal = Proposal::try_from_slice(&ctx.accounts.proposal_account.data.borrow())?;
        proposal.votes.push(vote_option);
        proposal.serialize(&mut &mut ctx.accounts.proposal_account.data.borrow_mut()[..])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    /// CHECK: オーナーチェック（owner == program_id）を行っていない
    #[account(mut)]
    pub proposal_account: AccountInfo<'info>,

    /// 投票者が署名していることのみを検証
    pub voter: Signer<'info>,
}

#[account]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Proposal {
    pub title: String,
    pub votes: Vec<u8>,
}
