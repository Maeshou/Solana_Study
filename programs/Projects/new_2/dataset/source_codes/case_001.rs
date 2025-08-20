use anchor_lang::prelude::*;

declare_id!("ProgMissingOwner002IDABCDEF1234567890");

#[program]
pub mod case_002_vulnerable {
    use super::*;

    /// Fully checked: both owner and signer enforced by Anchor
    pub fn op_full(ctx: Context<FullCheckCtx>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        // Anchor has_one = authority guarantees vault.owner == program_id
        // and #[account(signer)] on authority ensures signature check.
        **vault.to_account_info().try_borrow_mut_lamports()? =
            (**vault.to_account_info().try_borrow_lamports()?) + amount;
        msg!("op_full applied: {} lamports", **vault.to_account_info().lamports.borrow());
        Ok(())
    }

    /// Partially checked: signer is checked, but owner check is omitted
    pub fn op_partial(ctx: Context<PartialCheckCtx>, amount: u64) -> Result<()> {
        // We enforce that authority must sign, but we never verify vault_info.owner
        let vault_info = &mut ctx.accounts.vault_info;
        // signer check via Anchor
        // pub authority: Signer<'info>
        let lamports = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = lamports + amount;
        msg!("op_partial applied (no owner check): {} lamports", **vault_info.try_borrow_lamports()?);
        Ok(())
    }

    /// No checks at all: fully vulnerable
    pub fn op_none(ctx: Context<NoCheckCtx>, amount: u64) -> Result<()> {
        // Completely unchecked raw AccountInfo
        let vault_info = &mut ctx.accounts.vault_info;
        let lamports = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = lamports + amount;
        msg!("op_none applied (no checks): {} lamports", **vault_info.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FullCheckCtx<'info> {
    #[account(mut, has_one = authority)]
    pub vault_account: Account<'info, Vault>,
    #[account(mut, signer)]
    /// The current authority must sign
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PartialCheckCtx<'info> {
    /// CHECK: raw account info, no owner/program-id validation
    pub vault_info: AccountInfo<'info>,
    #[account(signer)]
    /// We do check that authority has signed, but skip owner check
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NoCheckCtx<'info> {
    /// CHECK: completely unchecked AccountInfo—no signer or owner validation
    pub vault_info: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}
