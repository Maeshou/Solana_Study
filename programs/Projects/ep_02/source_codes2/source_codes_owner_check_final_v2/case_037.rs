use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2037IDABC123xyz");

#[program]
pub mod case_037 {
    use super::*;

    pub fn op_037(ctx: Context<Case037Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info;
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = 0;
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 037: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case037Ctx<'info> {
    #[account(mut)]
    pub vault_info: AccountInfo<'info>,  // Missing program ownership check
    #[account(mut, has_one = authority)]
    pub vault_data: Account<'info, VaultData>,  // Account matching fixed here
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultData {
    pub authority: Pubkey,
}
