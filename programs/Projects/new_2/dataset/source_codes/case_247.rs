use anchor_lang::prelude::*;

declare_id!("VulnEx54000000000000000000000000000000000054");

#[program]
pub mod voting {
    pub fn cast(ctx: Context<Ctx4>, option: u8) -> Result<()> {
        // tally_log: OWNER CHECK SKIPPED
        ctx.accounts.tally_log.data.borrow_mut().push(option);

        // vote_acc: has_one = chair
        let v = &mut ctx.accounts.vote_acc;
        *v.counts.entry(option).or_insert(0) += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx4<'info> {
    #[account(mut)]
    pub tally_log: AccountInfo<'info>,

    #[account(mut, has_one = chair)]
    pub vote_acc: Account<'info, VoteAcc>,
    pub chair: Signer<'info>,
}

#[account]
pub struct VoteAcc {
    pub chair: Pubkey,
    pub counts: std::collections::BTreeMap<u8,u64>,
}
