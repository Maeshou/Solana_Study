use anchor_lang::prelude::*;

declare_id!("ConfigLd2020202020202020202020202020202020");

#[program]
pub mod config_loader {
    use super::*;

    /// 設定の反映：ロード後に更新
    pub fn update_config(ctx: Context<UpdateCfg>, new_limit: u32) -> Result<()> {
        let mut cfg = ctx.accounts.loader.load_mut()?;
        cfg.limit = new_limit;
        emit!(ConfigUpdated { new_limit });
        Ok(())
    }

    /// 設定の参照：読み取り専用
    pub fn view_config(ctx: Context<ViewCfg>) -> Result<CfgView> {
        let cfg = ctx.accounts.loader.load()?;
        Ok(CfgView { limit: cfg.limit })
    }
}

#[derive(Accounts)]
pub struct UpdateCfg<'info> {
    pub loader: AccountLoader<'info, ConfigData>,
}

#[derive(Accounts)]
pub struct ViewCfg<'info> {
    pub loader: AccountLoader<'info, ConfigData>,
}

#[account(zero_copy)]
pub struct ConfigData {
    pub limit: u32,
}

#[event]
pub struct ConfigUpdated {
    pub new_limit: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CfgView {
    pub limit: u32,
}
