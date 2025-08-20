use anchor_lang::prelude::*;

declare_id!("OwnChkE3000000000000000000000000000000004");

#[program]
pub mod dao_config {
    pub fn update_param(
        ctx: Context<UpdateParam>,
        key: String,
        val: u64,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        // 属性レベルで chairperson を検証
        cfg.params.insert(key.clone(), val);
        cfg.mod_count = cfg.mod_count.saturating_add(1);

        // log_acc は unchecked
        let mut buf = ctx.accounts.log_acc.data.borrow_mut();
        buf.extend_from_slice(key.as_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    #[account(mut, has_one = chairperson)]
    pub config: Account<'info, DaoConfig>,
    pub chairperson: Signer<'info>,
    /// CHECK: 設定変更ログ、所有者検証なし
    #[account(mut)]
    pub log_acc: AccountInfo<'info>,
}

#[account]
pub struct DaoConfig {
    pub chairperson: Pubkey,
    pub params: std::collections::HashMap<String, u64>,
    pub mod_count: u64,
}
