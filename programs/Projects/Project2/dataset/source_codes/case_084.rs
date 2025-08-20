use anchor_lang::prelude::*;

declare_id!("MetaUpdt1515151515151515151515151515151515");

#[program]
pub mod metadata_updater {
    use super::*;

    /// 初期化：メタデータを格納
    pub fn init_meta(ctx: Context<InitMeta>, uri: String) -> Result<()> {
        let m = &mut ctx.accounts.meta;
        m.author = ctx.accounts.author.key();
        m.uri = uri.clone();
        m.version = 1;
        emit!(MetaCreated { author: m.author, uri });
        Ok(())
    }

    /// URI 更新：作成者のみ呼出可
    pub fn update_meta(ctx: Context<ModifyMeta>, new_uri: String) -> Result<()> {
        let m = &mut ctx.accounts.meta;
        require_keys_eq!(m.author, ctx.accounts.author.key(), ErrorCode::NoAuth);
        m.uri = new_uri.clone();
        m.version = m.version.checked_add(1).unwrap();
        emit!(MetaUpdated { uri: new_uri, version: m.version });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMeta<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 200 + 8)]
    pub meta: Account<'info, Metadata>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyMeta<'info> {
    #[account(mut)]
    pub meta: Account<'info, Metadata>,
    pub author: Signer<'info>,
}

#[account]
pub struct Metadata {
    pub author: Pubkey,
    pub uri: String,
    pub version: u64,
}

#[event]
pub struct MetaCreated {
    pub author: Pubkey,
    pub uri: String,
}

#[event]
pub struct MetaUpdated {
    pub uri: String,
    pub version: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("認可がありません")]
    NoAuth,
}
