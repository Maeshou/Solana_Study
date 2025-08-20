use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2039IDABC123xyz");

#[program]
pub mod case_039 {
    use super::*;

    pub fn op_039(ctx: Context<Case039Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info.to_account_info();
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = lamports.checked_mul(2).unwrap();
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 039: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case039Ctx<'info> {
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
