use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2094IDABC123xyz");

#[program]
pub mod case_094 {
    use super::*;

    pub fn op_094(ctx: Context<Case094Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info;
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = lamports - 1;
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 094: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case094Ctx<'info> {
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
