use anchor_lang::prelude::*;

declare_id!("VulnEx74000000000000000000000000000000000074");

#[program]
pub mod example74 {
    pub fn xor_entire_buffer(ctx: Context<Ctx74>, key: u8) -> Result<()> {
        // buf_acc: OWNER CHECK SKIPPED
        let mut buf = ctx.accounts.buf_acc.data.borrow_mut();
        for byte in buf.iter_mut() {
            *byte ^= key;
        }

        // cipher_state: has_one = encrypter
        let st = &mut ctx.accounts.cipher_state;
        st.last_key = key;
        st.run_count = st.run_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx74<'info> {
    #[account(mut)]
    pub buf_acc: AccountInfo<'info>,  // unchecked
    #[account(mut, has_one = encrypter)]
    pub cipher_state: Account<'info, CipherState>,
    pub encrypter: Signer<'info>,
}

#[account]
pub struct CipherState {
    pub encrypter: Pubkey,
    pub last_key: u8,
    pub run_count: u64,
}
