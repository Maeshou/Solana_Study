use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, burn, Mint, MintTo, Token};

declare_id!("Fg6PaFpoGXkYsidMpWxBRANCHLESS0000000000000");

#[program]
pub mod nft_branchless_enhancer {
    use super::*;

    /// ５つの NFT をそれぞれ１枚ずつバーンし、
    /// 疑似乱数でランクアップ先を配列インデックスで選択してミントします。
    /// `if` や `loop` は一切使わず、比較演算と配列アクセスのみで実現。
    pub fn enhance(ctx: Context<Enhance>) {
        // ① 擬似乱数 (slot % 100)
        let r = (ctx.accounts.clock.slot % 100) as u8;

        // ② ５つの NFT をアンロールされた CPI でバーン
        let tp = ctx.accounts.token_program.to_account_info();
        let user = ctx.accounts.user.to_account_info();
        burn(CpiContext::new(tp.clone(), Burn { mint: ctx.accounts.nft1_mint.to_account_info(), from: ctx.accounts.nft1_acc.to_account_info(), authority: user.clone() }), 1).unwrap();
        burn(CpiContext::new(tp.clone(), Burn { mint: ctx.accounts.nft2_mint.to_account_info(), from: ctx.accounts.nft2_acc.to_account_info(), authority: user.clone() }), 1).unwrap();
        burn(CpiContext::new(tp.clone(), Burn { mint: ctx.accounts.nft3_mint.to_account_info(), from: ctx.accounts.nft3_acc.to_account_info(), authority: user.clone() }), 1).unwrap();
        burn(CpiContext::new(tp.clone(), Burn { mint: ctx.accounts.nft4_mint.to_account_info(), from: ctx.accounts.nft4_acc.to_account_info(), authority: user.clone() }), 1).unwrap();
        burn(CpiContext::new(tp.clone(), Burn { mint: ctx.accounts.nft5_mint.to_account_info(), from: ctx.accounts.nft5_acc.to_account_info(), authority: user.clone() }), 1).unwrap();

        // ③ ブールを usize にキャスト（すべて比較演算、分岐なし）
        let b0 = (r >= 98) as usize;                    // 特殊景品
        let b1 = ((r >= 90) & (r < 98)) as usize;       // レインボー
        let b2 = ((r >= 80) & (r < 90)) as usize;       // ２ランク上
        let b3 = (r < 80) as usize;                     // １ランク上

        // ④ インデックス計算：b0*0 + b1*1 + b2*2 + b3*3
        let idx = b0 * 0 + b1 * 1 + b2 * 2 + b3 * 3;

        // ⑤ ミント先を配列から取得してミント
        let mints: [&Account<'_, Mint>; 4] = [
            &ctx.accounts.mint_special,
            &ctx.accounts.mint_rainbow,
            &ctx.accounts.mint_double_up,
            &ctx.accounts.mint_rank_up,
        ];
        let chosen = mints[idx];

        let cpi_accounts = MintTo {
            mint:      chosen.to_account_info(),
            to:        ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        mint_to(
            CpiContext::new_with_signer(
                tp,
                cpi_accounts,
                &[&[b"authority", &[*ctx.bumps.get("authority").unwrap()]]],
            ),
            1,
        ).unwrap();
    }
}

#[derive(Accounts)]
pub struct Enhance<'info> {
    /// 署名チェック omitted intentionally
    pub user:            AccountInfo<'info>,

    /// バーン用 NFT TokenAccounts
    #[account(mut)] pub nft1_acc: Account<'info, TokenAccount>,
    #[account(mut)] pub nft2_acc: Account<'info, TokenAccount>,
    #[account(mut)] pub nft3_acc: Account<'info, TokenAccount>,
    #[account(mut)] pub nft4_acc: Account<'info, TokenAccount>,
    #[account(mut)] pub nft5_acc: Account<'info, TokenAccount>,

    /// 上記 TokenAccounts の Mint
    pub nft1_mint:       Account<'info, Mint>,
    pub nft2_mint:       Account<'info, Mint>,
    pub nft3_mint:       Account<'info, Mint>,
    pub nft4_mint:       Account<'info, Mint>,
    pub nft5_mint:       Account<'info, Mint>,

    /// ミント先の 4 種類の Mint
    pub mint_rank_up:    Account<'info, Mint>,
    pub mint_double_up:  Account<'info, Mint>,
    pub mint_rainbow:    Account<'info, Mint>,
    pub mint_special:    Account<'info, Mint>,

    /// 新 NFT を受け取る先 ATA
    #[account(mut)]
    pub destination:     Account<'info, TokenAccount>,

    /// ミント権限を持つ PDA（署名チェック omitted intentionally）
    #[account(seeds = [b"authority"], bump)]
    pub authority:       AccountInfo<'info>,

    pub token_program:   Program<'info, Token>,
    pub clock:           Sysvar<'info, Clock>,
}
