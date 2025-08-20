use anchor_lang::prelude::*;

declare_id!("CfgZeroCopy45454545454545454545454545454545");

#[program]
pub mod config_zero45 {
    use super::*;

    /// 上限値を更新
    pub fn set_limit(ctx: Context<SetLimit>, limit: u32) -> Result<()> {
        let mut cfg = ctx.accounts.config.load_mut()?;
        cfg.limit = limit;
        Ok(())
    }

    /// 現在値を返す
    pub fn view_limit(ctx: Context<ViewLimit>) -> Result<LimitView> {
        let cfg = ctx.accounts.config.load()?;
        Ok(LimitView { limit: cfg.limit })
    }
}

#[derive(Accounts)]
pub struct SetLimit<'info> {
    pub config: AccountLoader<'info, ConfigData>,
}

#[derive(Accounts)]
pub struct ViewLimit<'info> {
    pub config: AccountLoader<'info, ConfigData>,
}

#[account(zero_copy)]
pub struct ConfigData {
    pub limit: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LimitView {
    pub limit: u32,
}
