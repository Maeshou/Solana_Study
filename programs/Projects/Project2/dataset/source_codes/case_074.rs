use anchor_lang::prelude::*;

declare_id!("DataWrite44444444444444444444444444444444");

#[program]
pub mod data_writer {
    use super::*;

    pub fn save(ctx: Context<Save>, val: u64) -> Result<()> {
        let (expected, _bump) = Pubkey::find_program_address(
            &[b"store", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        require_keys_eq!(ctx.accounts.storage.key, expected);

        // 書き込み
        let mut buf = ctx.accounts.storage.data.borrow_mut();
        buf[..8].copy_from_slice(&val.to_le_bytes());
        msg!("Written value: {}", val);

        // 読み出し確認
        let read_val = u64::from_le_bytes(buf[..8].try_into().unwrap());
        emit!(DataStored { value: read_val });
        Ok(())
    }

    pub fn clear(ctx: Context<Save>) -> Result<()> {
        let mut buf = ctx.accounts.storage.data.borrow_mut();
        for b in buf.iter_mut() {
            *b = 0;
        }
        emit!(DataCleared {});
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Save<'info> {
    #[account(mut)]
    pub storage: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[event]
pub struct DataStored {
    pub value: u64,
}

#[event]
pub struct DataCleared {}
