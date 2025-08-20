use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("TrEaSuRyTyPeC0sp1ay11111111111111111111");

#[program]
pub mod treasury_gate {
    use super::*;
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }
        let data = ctx.accounts.cfg.data.borrow();
        let treasury = GuildTreasury::try_from_slice(&data)?;
        require_keys_eq!(treasury.manager, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);
        msg!("Withdrawing {} lamports", amount);
        Ok(())
    }
    pub fn write_wallet(ctx: Context<WriteWallet>, key: Pubkey) -> Result<()> {
        let wallet = PlayerWallet { user: key };
        let bytes = wallet.try_to_vec()?;
        ctx.accounts.cfg.data.borrow_mut()[..32].copy_from_slice(&bytes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct WriteWallet<'info> {
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GuildTreasury { pub manager: Pubkey }
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PlayerWallet  { pub user: Pubkey }
