use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2002IDABC123xyz");

#[program]
pub mod case_002 {
    use super::*;

    pub fn op_002(ctx: Context<Case002Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info.to_account_info();
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = lamports.checked_mul(amount).unwrap();
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 002: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case002Ctx<'info> {
    #[account(mut)]
    pub vault_info: UncheckedAccount<'info>,  // Missing program ownership check
    #[account(mut, has_one = authority)]
    pub vault_data: Account<'info, VaultData>,  // Account matching fixed here
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultData {
    pub authority: Pubkey,
}
