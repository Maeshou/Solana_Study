use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2019IDABC123xyz");

#[program]
pub mod case_019 {
    use super::*;

    pub fn op_019(ctx: Context<Case019Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info;
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = 0;
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 019: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case019Ctx<'info> {
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
