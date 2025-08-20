// 5. 設定値＋更新履歴
use anchor_lang::prelude::*;

declare_id!("Set55555555555555555555555555555555");

#[program]
pub mod reinit_settings_v2 {
    use super::*;

    // 閾値と上限値を設定
    pub fn setup_parameters(
        ctx: Context<SetupParameters>,
        threshold: u16,
        limit: u16,
    ) -> Result<()> {
        let params = &mut ctx.accounts.parameters;
        params.threshold = threshold;
        params.limit = limit;
        // バージョン情報が毎回初期化される
        params.version = 1;
        Ok(())
    }

    // 閾値のみ更新
    pub fn update_threshold(
        ctx: Context<SetupParameters>,
        new_threshold: u16,
    ) -> Result<()> {
        let params = &mut ctx.accounts.parameters;
        params.threshold = new_threshold;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupParameters<'info> {
    #[account(mut)]
    pub parameters: Account<'info, ParametersData>,
    /// CHECK: 変更履歴用、初期化なし
    #[account(mut)]
    pub history: AccountInfo<'info>,
}

#[account]
pub struct ParametersData {
    pub threshold: u16,
    pub limit: u16,
    pub version: u8,
}
