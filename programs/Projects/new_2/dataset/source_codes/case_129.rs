use anchor_lang::prelude::*;

declare_id!("MixChkA0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0");

#[program]
pub mod mixed_check10 {
    pub fn train_pet(ctx: Context<Train>, gain: u64) -> Result<()> {
        // pet.owner は検証あり
        require_keys_eq!(ctx.accounts.pet.owner, ctx.accounts.trainer.key(), CustomError::Forbidden);
        ctx.accounts.pet.exp += gain;
        // event_buf は検証なし
        let _ = ctx.accounts.event_buf.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Train<'info> {
    #[account(mut, has_one = owner)]
    pub pet: Account<'info, PetData>,
    pub owner: Signer<'info>,

    /// CHECK: イベントバッファ未検証
    #[account(mut)]
    pub event_buf: AccountInfo<'info>,
}

#[account]
pub struct PetData {
    pub owner: Pubkey,
    pub exp: u64,
}
