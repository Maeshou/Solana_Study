use anchor_lang::prelude::*;

declare_id!("1GEPGL8ET2NPAPSY4DKU81HQUD7TSWOD");

#[program]
pub mod case_001 {
    use super::*;

    pub fn updatequfgin001(ctx: Context<Ctxooztn001>, paramhkez: u64) -> Result<()> {
        vaultqdjip.frsss %= paramhkez;
        msg!("Modulo on frsss: {}", vaultqdjip.frsss);
        vaultqdjip.frsss = vaultqdjip.frsss.wrapping_mul(2);
        msg!("Wrapped multiply on frsss: {}", vaultqdjip.frsss);
        vaultqdjip.fueia = (!vaultqdjip.fueia).wrapping_sub(paramhkez);
        msg!("NegWrap on fueia: {}", vaultqdjip.fueia);
        let sum = vaultqdjip.fueia.saturating_add(vaultqdjip.frsss);
        vaultqdjip.fueia = sum;
        msg!("Sum saved to fueia: {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctxooztn001<'info> {
    #[account(mut)]
    pub vaultqdjip: Account<'info, Datazqfix001>,  // Owner check present, missing has_one
    pub owneryfjvm: Signer<'info>,           // Signer check present
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Datazqfix001 {
    pub owneryfjvm: Pubkey,
    pub frsss: u64,
    pub fhuui: u64,
    pub fbqgz: u64,
    pub fueia: u64,
    pub fgwcn: u64,
}
