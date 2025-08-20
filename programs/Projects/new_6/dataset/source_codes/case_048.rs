use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("VaUlTc0nF1gc0sp1ay1111111111111111111");

#[program]
pub mod vault_gate {
    use super::*;
    pub fn withdraw(ctx: Context<WithdrawVault>, amount: u64) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let vault = VaultConfig::try_from_slice(&ctx.accounts.cfg.data.borrow())?;
        require_keys_eq!(vault.authority, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Withdrawing {} from vault", amount);
        Ok(())
    }
    pub fn write_depositor(ctx: Context<WriteDepositor>, key: Pubkey) -> Result<()> {
        let depositor = DepositorRecord { depositor: key };
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&depositor.try_to_vec()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteDepositor<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultConfig { pub authority: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct DepositorRecord { pub depositor: Pubkey }
