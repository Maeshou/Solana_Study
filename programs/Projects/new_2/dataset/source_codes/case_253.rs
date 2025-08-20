use anchor_lang::prelude::*;

declare_id!("VulnEx60000000000000000000000000000000000060");

#[program]
pub mod meta_update {
    pub fn update(ctx: Context<Ctx10>, key: String, value: String) -> Result<()> {
        // sig_history: OWNER CHECK SKIPPED
        ctx.accounts.sig_history.data.borrow_mut()
            .extend_from_slice(&ctx.accounts.updater.key().to_bytes());

        // meta_acc: has_one = updater
        let m = &mut ctx.accounts.meta_acc;
        m.map.insert(key, value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx10<'info> {
    /// CHECK: 署名履歴、所有者検証なし
    #[account(mut)]
    pub sig_history: AccountInfo<'info>,

    #[account(mut, has_one = updater)]
    pub meta_acc: Account<'info, MetaAcc>,
    pub updater: Signer<'info>,
}

#[account]
pub struct MetaAcc {
    pub updater: Pubkey,
    pub map: std::collections::HashMap<String,String>,
}
