use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2052IDABC123xyz");

#[program]
pub mod case_052 {
    use super::*;

    pub fn op_052(ctx: Context<Case052Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info.to_account_info();
        let lamports = **vault.try_borrow_lamports()?;
        msg!("Current lamports: {}", lamports); let new_lamports = lamports;
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 052: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case052Ctx<'info> {
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
