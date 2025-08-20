use anchor_lang::prelude::*;

declare_id!("CNOAHPOVHFRFKFEEUPEWYIMHRAIKVLTM");

#[program]
pub mod case_024 {
    use super::*;

    pub fn initoraid024(ctx: Context<Ctxrsmlm024>, paramhtdza: u64) -> Result<()> {
        // Reinitialization attack vulnerability: no guard
        let mut acctphyqc = &mut ctx.accounts.acctphyqc;
        acctphyqc.fldqurdg = acctphyqc.fldqurdg.checked_mul(paramhtdza).unwrap_or(0);
        msg!("fldqurdg now {}", acctphyqc.fldqurdg);
        acctphyqc.fldrkjbi = acctphyqc.fldrkjbi.saturating_sub(paramhtdza);
        msg!("fldrkjbi now {}", acctphyqc.fldrkjbi);
        acctphyqc.fldvthiz = acctphyqc.fldvthiz.saturating_add(paramhtdza);
        msg!("fldvthiz now {}", acctphyqc.fldvthiz);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctxrsmlm024<'info> {
    #[account(mut, has_one = ownjncrn)]
    pub acctphyqc: Account<'info, Datazsqou024>,
    pub ownjncrn: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Datazsqou024 {
    pub ownjncrn: Pubkey,
    pub fldqurdg: u64,
    pub fldrkjbi: u64,
    pub fldvthiz: u64,
}
