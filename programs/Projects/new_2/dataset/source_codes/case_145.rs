use anchor_lang::prelude::*;

declare_id!("CfgVar0888888888888888888888888888888888");

#[program]
pub mod config_var8 {
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        // has_one で管理者チェック
        // 属性チェックで cfg.admin == signer
        cfg.params.clear();
        cfg.version += 1;

        // lock_file は unchecked
        let mut data = ctx.accounts.lock_file.data.borrow_mut();
        data[0] = 0xAA;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, ConfigData>,
    pub admin: Signer<'info>,
    #[account(mut)] pub lock_file: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct ConfigData {
    pub admin: Pubkey,
    pub params: Vec<u8>,
    pub version: u32,
}
