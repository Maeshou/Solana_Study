use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct UrlShortener(pub u8, pub Vec<(u8, String)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV2");

#[program]
pub mod url_shortener {
    use super::*;

    /// 初期化：内部 Vec はデフォルトで空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("shortener").unwrap();
        ctx.accounts.shortener.0 = bump;
        Ok(())
    }

    /// URL 登録：同じ ID がなければ追加、あれば上書き
    pub fn register(ctx: Context<Modify>, id: u8, url: String) -> Result<()> {
        let list = &mut ctx.accounts.shortener.1;
        let mut found = false;

        for entry in list.iter_mut() {
            if entry.0 == id {
                entry.1 = url.clone();
                found = true;
            }
        }
        if !found {
            list.push((id, url));
        }
        Ok(())
    }

    /// URL 取得：ID が一致した最初の URL をログ出力
    pub fn lookup(ctx: Context<Modify>, id: u8) -> Result<()> {
        let list = &ctx.accounts.shortener.1;
        for entry in list.iter() {
            if entry.0 == id {
                msg!("URL for {}: {}", id, entry.1);
            }
        }
        Ok(())
    }

    /// URL 削除：ID が一致するものを一括除去
    pub fn remove(ctx: Context<Modify>, id: u8) -> Result<()> {
        let list = &mut ctx.accounts.shortener.1;
        list.retain(|&(eid, _)| eid != id);
        Ok(())
    }

    /// 登録数報告：現在のエントリ数をログ出力
    pub fn count(ctx: Context<Modify>) -> Result<()> {
        let length = ctx.accounts.shortener.1.len() as u64;
        msg!("Total entries: {}", length);
        Ok(())
    }
}

// ── Context 定義は末尾に配置 ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"url_shortener", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<(u8,String)> (max10件: 4 + 10*(1+4+200))
        space = 8 + 1 + 4 + 10 * (1 + 4 + 200)
    )]
    pub shortener: Account<'info, UrlShortener>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"url_shortener", authority.key().as_ref()],
        bump = shortener.0,
    )]
    pub shortener: Account<'info, UrlShortener>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
