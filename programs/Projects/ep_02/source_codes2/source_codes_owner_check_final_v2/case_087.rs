use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2087IDABC123xyz");

#[program]
pub mod case_087 {
    use super::*;

    pub fn op_087(ctx: Context<Case087Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info;
        let lamports = **vault.try_borrow_lamports()?;
        let fee = amount / 10;
    let new_lamports = lamports.checked_sub(amount + fee).unwrap();
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 087: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case087Ctx<'info> {
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
