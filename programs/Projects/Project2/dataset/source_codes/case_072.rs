use anchor_lang::prelude::*;

declare_id!("FlipLedger22222222222222222222222222222222");

#[program]
pub mod ledger_flag {
    use super::*;

    pub fn flip(ctx: Context<Flip>) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        // フラグ反転
        rec.flag = !rec.flag;
        rec.last_actor = ctx.accounts.authority.key();
        rec.flip_count = rec
            .flip_count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        emit!(FlagToggled {
            actor: ctx.accounts.authority.key(),
            new_flag: rec.flag,
            total: rec.flip_count
        });
        Ok(())
    }

    pub fn reset_flag(ctx: Context<Flip>) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.flag = false;
        rec.last_actor = ctx.accounts.authority.key();
        rec.flip_count = 0;
        emit!(FlagReset {
            actor: ctx.accounts.authority.key()
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(mut, has_one = authority)]
    pub rec: Account<'info, FlipData>,
    pub authority: Signer<'info>,
}

#[account]
pub struct FlipData {
    pub authority: Pubkey,
    pub flag: bool,
    pub last_actor: Pubkey,
    pub flip_count: u64,
}

#[event]
pub struct FlagToggled {
    pub actor: Pubkey,
    pub new_flag: bool,
    pub total: u64,
}

#[event]
pub struct FlagReset {
    pub actor: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter overflowed")]
    Overflow,
}
