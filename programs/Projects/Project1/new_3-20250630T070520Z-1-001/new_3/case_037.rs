use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLandDev001");

#[program]
pub mod land_development_service {
    use super::*;

    /// 土地の建築レベルを上げ、生産量を増加させるが、
    /// plot_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn upgrade_plot(ctx: Context<UpgradePlot>) -> Result<()> {
        let plot = &mut ctx.accounts.plot_account;
        let cfg = &ctx.accounts.config;

        // 1. 現在レベルを取得してインクリメント
        let new_level = plot.level.checked_add(1).unwrap();
        plot.level = new_level;

        // 2. レベルに応じた生産能力を上乗せ
        plot.production_rate = plot
            .production_rate
            .checked_add(cfg.base_production)
            .unwrap();

        // 3. 建設コストを累積
        plot.construction_costs = plot
            .construction_costs
            .checked_add(cfg.cost_per_level)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpgradePlot<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを行うべき
    pub plot_account: Account<'info, PlotAccount>,

    /// 土地所有者としての署名者
    pub user: Signer<'info>,

    /// 建設パラメータを保持する設定アカウント
    pub config: Account<'info, LandConfig>,
}

#[account]
pub struct PlotAccount {
    /// この土地を所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 建築レベル
    pub level: u8,
    /// 毎秒あたりの資源生産量
    pub production_rate: u64,
    /// 累積建築コスト（Lamports）
    pub construction_costs: u64,
}

#[account]
pub struct LandConfig {
    /// レベルアップごとに追加される生産量
    pub base_production: u64,
    /// レベルアップ1回あたりの建設コスト
    pub cost_per_level: u64,
}
