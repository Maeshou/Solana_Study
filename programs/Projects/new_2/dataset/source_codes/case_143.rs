use anchor_lang::prelude::*;

declare_id!("RepVar0444444444444444444444444444444444");

#[program]
pub mod reputation_var4 {
    pub fn adjust(ctx: Context<Adjust>, delta: i64) -> Result<()> {
        let rp = &mut ctx.accounts.rep;
        // require_keys_eq! でモデレーター検証
        require_keys_eq!(rp.moderator, ctx.accounts.modr.key(), CustomError::NoAuth);
        rp.score = (rp.score as i64 + delta).max(0) as u64;

        // audit_mem は unchecked
        ctx.accounts.audit_mem.data.borrow_mut()[0] = delta as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Adjust<'info> {
    #[account(mut)]
    pub rep: Account<'info, RepData>,
    pub modr: Signer<'info>,
    #[account(mut)] pub audit_mem: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct RepData {
    pub moderator: Pubkey,
    pub score: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Not authorized")]
    NoAuth,
}
