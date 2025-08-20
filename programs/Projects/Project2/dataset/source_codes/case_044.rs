use anchor_lang::prelude::*;

declare_id!("DataWrite44444444444444444444444444444444");

#[program]
pub mod data_writer {
    use super::*;

    pub fn save(ctx: Context<Save>, val: u64) -> Result<()> {
        let (key, _bump) = Pubkey::find_program_address(
            &[b"store", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        require_keys_eq!(ctx.accounts.storage.key, key);
        let mut buf = ctx.accounts.storage.data.borrow_mut();
        buf[..8].copy_from_slice(&val.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Save<'info> {
    #[account(mut)]
    pub storage: AccountInfo<'info>,
    pub user: Signer<'info>,
}
