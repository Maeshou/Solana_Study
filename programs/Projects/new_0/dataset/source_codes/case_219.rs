use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpY7x8w9v0u1t2s3r4q5t6s7u8v9w");

#[program]
pub mod config_manager {
    use super::*;

    /// 初期設定アカウントの生成
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        bump: u8,
        initial_threshold: u64,
    ) -> ProgramResult {
        let config = &mut ctx.accounts.config;
        config.owner = *ctx.accounts.admin.key;
        config.bump = bump;
        config.threshold = initial_threshold;
        config.enabled = true;
        Ok(())
    }

    /// 閾値の更新（最大1000まで）
    pub fn set_threshold(
        ctx: Context<ModifyConfig>,
        new_threshold: u64,
    ) -> ProgramResult {
        require!(new_threshold <= 1000, ErrorCode::ThresholdTooHigh);
        let config = &mut ctx.accounts.config;
        config.threshold = new_threshold;
        Ok(())
    }

    /// 有効/無効フラグの切り替え
    pub fn toggle_enabled(
        ctx: Context<ModifyConfig>
    ) -> ProgramResult {
        let config = &mut ctx.accounts.config;
        config.enabled = !config.enabled;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        seeds = [b"config", admin.key().as_ref()],
        bump = bump,
        payer = admin,
        space = 8 + 32 + 1 + 8 + 1,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyConfig<'info> {
    #[account(
        mut,
        seeds = [b"config", config.owner.as_ref()],
        bump = config.bump,
        has_one = owner,
    )]
    pub config: Account<'info, Config>,
    /// 設定変更権限を持つ管理者
    pub owner: Signer<'info>,
}

#[account]
pub struct Config {
    pub owner: Pubkey,
    pub bump: u8,
    pub threshold: u64,
    pub enabled: bool,
}

#[error]
pub enum ErrorCode {
    #[msg("Threshold must be <= 1000.")]
    ThresholdTooHigh,
}
