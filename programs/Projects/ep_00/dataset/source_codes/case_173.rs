use anchor_lang::prelude::*;

declare_id!("KmEZxay93mpjHKJq1TaC17JdPg8atsQkP0OTjYWjdQi9");

#[derive(Accounts)]
pub struct Case173<'info> {
    #[account(mut, has_one = owner34)] pub acct52: Account<'info, DataAccount>,
    pub owner34: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_173_program {
    use super::*;

    pub fn case_173(ctx: Context<Case173>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct52.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct52.data = tripled;
        Ok(())
    }
}
