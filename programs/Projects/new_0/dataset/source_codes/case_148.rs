use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

// ユーザーごとの機能フラグ管理プログラム

declare_id!("Feat111111111111111111111111111111111111");

#[program]
pub mod feature_flag_manager {
    /// 機能フラグ設定アカウントを作成
    pub fn init_flags(ctx: Context<InitFlags>) -> Result<()> {
        let flags = &mut ctx.accounts.flags;
        flags.owner = ctx.accounts.user.key();
        flags.flags.clear();
        Ok(())
    }

    /// 指定したフラグを有効化
    pub fn enable_flag(ctx: Context<ModifyFlags>, feature: String) -> Result<()> {
        let flags = &mut ctx.accounts.flags;
        require!(flags.owner == ctx.accounts.user.key(), ErrorCode::Forbidden);
        require!(feature.len() <= 32, ErrorCode::NameTooLong);

        // すでに有効なら何もしない
        for f in flags.flags.iter() {
            if f == &feature {
                return Ok(());
            }
        }
        flags.flags.push(feature);
        Ok(())
    }

    /// 指定したフラグを無効化
    pub fn disable_flag(ctx: Context<ModifyFlags>, feature: String) -> Result<()> {
        let flags = &mut ctx.accounts.flags;
        require!(flags.owner == ctx.accounts.user.key(), ErrorCode::Forbidden);

        // 無効化処理
        let mut idx: Option<usize> = None;
        for (i, f) in flags.flags.iter().enumerate() {
            if f == &feature {
                idx = Some(i);
                break;
            }
        }
        let i = idx.ok_or(ErrorCode::NotFound)?;
        flags.flags.remove(i);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFlags<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (10 * (4 + 32)))]
    pub flags:          Account<'info, FeatureFlags>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyFlags<'info> {
    #[account(mut)] pub flags: Account<'info, FeatureFlags>,
    pub user: Signer<'info>,
}

#[account]
pub struct FeatureFlags {
    /// 管理者としての所有者
    pub owner: Pubkey,
    /// 有効化された機能フラグのリスト
    pub flags: Vec<String>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("操作が許可されていません")] Forbidden,
    #[msg("フラグ名が長すぎます")] NameTooLong,
    #[msg("指定のフラグが見つかりません")] NotFound,
}
