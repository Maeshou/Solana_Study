use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct FeatureFlagManager(pub u8, pub Vec<(Vec<u8>, bool)>); // (bump, Vec<(flag_name, enabled)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVJ");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of flags reached")]
    MaxFlagsReached,
    #[msg("Flag not found")]
    FlagNotFound,
}

#[program]
pub mod feature_flag_manager {
    use super::*;

    const MAX_FLAGS: usize = 16;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("flags").unwrap();
        ctx.accounts.flags.0 = b;
        Ok(())
    }

    /// フラグ追加：件数制限チェック＋初期 false で追加
    pub fn add_flag(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.flags.1;
        if list.len() >= MAX_FLAGS {
            return err!(ErrorCode::MaxFlagsReached);
        }
        list.push((name, false));
        Ok(())
    }

    /// フラグ有効化：該当フラグを検索し、enabled を true に
    pub fn enable_flag(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.flags.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == name {
                entry.1 = true;
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::FlagNotFound);
        }
        Ok(())
    }

    /// フラグ無効化：該当フラグを検索し、enabled を false に
    pub fn disable_flag(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.flags.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == name {
                entry.1 = false;
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::FlagNotFound);
        }
        Ok(())
    }

    /// 削除されたフラグ（常に enabled = false）を一括除去
    pub fn purge_disabled(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.flags.1;
        list.retain(|&(_, enabled)| {
            if enabled == false {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 有効フラグ数をログ出力
    pub fn count_enabled(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.flags.1;
        let mut cnt = 0u64;
        for &(_, enabled) in list.iter() {
            if enabled == true {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Enabled flags: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"flags", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max16*(4+32+1)
        // flag_name: 4-byte length + up to 32-byte UTF-8 name
        space = 8 + 1 + 4 + 16 * (4 + 32 + 1)
    )]
    pub flags:     Account<'info, FeatureFlagManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"flags", authority.key().as_ref()],
        bump = flags.0,
    )]
    pub flags:     Account<'info, FeatureFlagManager>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
