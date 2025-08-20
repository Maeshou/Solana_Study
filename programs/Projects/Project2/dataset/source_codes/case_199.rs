use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ApiKeyManager(pub u8, pub Vec<(Vec<u8>, u64)>); // (bump, Vec<(api_key, usage_count)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVK");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of API keys reached")]
    MaxKeysReached,
    #[msg("API key not found")]
    KeyNotFound,
}

#[program]
pub mod api_key_manager {
    use super::*;

    const MAX_KEYS: usize = 25;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("manager").unwrap();
        ctx.accounts.manager.0 = b;
        Ok(())
    }

    /// API キー追加：件数制限チェック＋初期利用 0 で追加
    pub fn add_key(ctx: Context<Modify>, api_key: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        if list.len() >= MAX_KEYS {
            return err!(ErrorCode::MaxKeysReached);
        }
        list.push((api_key, 0));
        Ok(())
    }

    /// 利用記録：該当キーを探索し、カウントを加算
    pub fn record_usage(ctx: Context<Modify>, api_key: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == api_key {
                entry.1 = entry.1.wrapping_add(1);
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::KeyNotFound);
        }
        Ok(())
    }

    /// API キー削除：該当キーを一括除去
    pub fn remove_key(ctx: Context<Modify>, api_key: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        list.retain(|(k, _)| {
            if *k == api_key {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 利用回数が 0 のキーを一括削除
    pub fn purge_unused(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        list.retain(|&(_, count)| {
            if count > 0 {
                true
            } else {
                false
            }
        });
        Ok(())
    }

    /// 総利用回数をログ出力
    pub fn total_usage(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.manager.1;
        let mut sum = 0u64;
        for &(_, count) in list.iter() {
            sum = sum.wrapping_add(count);
        }
        msg!("Total API usage: {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max25*(4+32+8)
        // api_key: 4-byte length + up to 32-byte key
        space = 8 + 1 + 4 + 25 * (4 + 32 + 8)
    )]
    pub manager:    Account<'info, ApiKeyManager>,
    #[account(mut)]
    pub authority:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"manager", authority.key().as_ref()],
        bump = manager.0
    )]
    pub manager:    Account<'info, ApiKeyManager>,
    #[account(signer)]
    pub authority:  AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
