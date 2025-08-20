use anchor_lang::prelude::*;

declare_id!("GovVar0222222222222222222222222222222222");

#[program]
pub mod governance_var2 {
    pub fn propose(ctx: Context<Propose>, title: String) -> Result<()> {
        let p = &mut ctx.accounts.proposal;
        // 属性レベルで creator == signer をチェック
        // #[account(mut, has_one = creator)] が自動的に検証
        p.title = title;
        p.open = true;
        p.votes = 0;

        // event_cap は unchecked
        msg!("Capturing event to raw account");
        let _ = ctx.accounts.event_cap.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 128 + 1 + 8, has_one = creator)]
    pub proposal: Account<'info, ProposalData>,
    pub creator: Signer<'info>,
    #[account(mut)] pub event_cap: AccountInfo<'info>,  // unchecked
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProposalData {
    pub creator: Pubkey,
    pub title: String,
    pub open: bool,
    pub votes: u64,
}
