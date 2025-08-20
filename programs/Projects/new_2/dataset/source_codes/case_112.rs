use anchor_lang::prelude::*;

declare_id!("GameCfgV333333333333333333333333333333333");

#[program]
pub mod game_cfg_vuln {
    pub fn update_param(ctx: Context<UpdateParam>, param: u64) -> Result<()> {
        // cfg.admin の検証なし
        let cfg = &mut ctx.accounts.cfg;
        cfg.param = param;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    #[account(mut)]
    pub cfg: Account<'info, GameConfig>,
}

#[account]
pub struct GameConfig {
    pub admin: Pubkey,
    pub param: u64,
}
