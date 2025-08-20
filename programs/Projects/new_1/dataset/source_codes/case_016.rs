use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, mint_to, burn, Burn, MintTo, Token};

declare_id!("Fg6PaFpoGXkYsidMpWxZENHANCE9X3FT7Y8Z0HA1BC");

#[program]
pub mod nft_enhancer {
    use super::*;

    pub fn enhance(ctx: Context<Enhance>) -> Result<()> {
        // 1) 擬似乱数 (slot % 100)
        let slot = ctx.accounts.clock.slot;
        let rand = (slot % 100) as u8;

        // 2) 入力 NFT をすべてバーン
        let burns = [
            (&ctx.accounts.nft1_account, &ctx.accounts.nft1_mint),
            (&ctx.accounts.nft2_account, &ctx.accounts.nft2_mint),
            (&ctx.accounts.nft3_account, &ctx.accounts.nft3_mint),
            (&ctx.accounts.nft4_account, &ctx.accounts.nft4_mint),
            (&ctx.accounts.nft5_account, &ctx.accounts.nft5_mint),
        ];
        for (account, mint) in burns.iter() {
            let cpi_accounts = Burn {
                mint: mint.to_account_info(),
                from: account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            burn(cpi_ctx, 1)?;
        }

        // 3) 当選するミントを決定
        let chosen_mint: &Account<'_, Mint> = if rand < 80 {
            &ctx.accounts.mint_rank_up      // 80%
        } else if rand < 90 {
            &ctx.accounts.mint_double_up    // 次の10%
        } else if rand < 98 {
            &ctx.accounts.mint_rainbow      // 次の8%
        } else {
            &ctx.accounts.mint_special      // 残り2%
        };

        // 4) 新 NFT をミント (1 枚)
        let cpi_accounts = MintTo {
            mint:      chosen_mint.to_account_info(),
            to:        ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &[&[b"authority", &[*ctx.bumps.get("authority").unwrap()]]],
        );
        mint_to(cpi_ctx, 1)?;

        // 5) イベント通知
        emit!(EnhanceEvent {
            user:      *ctx.accounts.user.key,
            outcome:   rand,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enhance<'info> {
    /// 合成を実行するユーザー（署名必須）
    #[account(mut)]
    pub user:            Signer<'info>,

    /// バーン対象の５つの NFT アカウント
    #[account(mut, constraint = nft1_account.owner == *user.key)]
    pub nft1_account:    Account<'info, TokenAccount>,
    #[account(mut, constraint = nft2_account.owner == *user.key)]
    pub nft2_account:    Account<'info, TokenAccount>,
    #[account(mut, constraint = nft3_account.owner == *user.key)]
    pub nft3_account:    Account<'info, TokenAccount>,
    #[account(mut, constraint = nft4_account.owner == *user.key)]
    pub nft4_account:    Account<'info, TokenAccount>,
    #[account(mut, constraint = nft5_account.owner == *user.key)]
    pub nft5_account:    Account<'info, TokenAccount>,

    /// 各バーン用 Mint
    pub nft1_mint:       Account<'info, Mint>,
    pub nft2_mint:       Account<'info, Mint>,
    pub nft3_mint:       Account<'info, Mint>,
    pub nft4_mint:       Account<'info, Mint>,
    pub nft5_mint:       Account<'info, Mint>,

    /// 当選 NFT 用 Mint（各レアリティごとに用意）
    pub mint_rank_up:    Account<'info, Mint>,  // １ランク上
    pub mint_double_up:  Account<'info, Mint>,  // ２ランク上
    pub mint_rainbow:    Account<'info, Mint>,  // レインボースニーカー
    pub mint_special:    Account<'info, Mint>,  // その他特殊景品

    /// 新 NFT を受け取る先 (Associated Token Account)
    #[account(mut)]
    pub destination:     Account<'info, TokenAccount>,

    /// ミント権限を持つ PDA
    /// CHECK: プログラムシードから生成した権限アカウント
    #[account(seeds = [b"authority"], bump)]
    pub authority:       AccountInfo<'info>,

    pub token_program:   Program<'info, Token>,
    pub system_program:  Program<'info, System>,
    pub rent:            Sysvar<'info, Rent>,
    pub clock:           Sysvar<'info, Clock>,
}

#[event]
pub struct EnhanceEvent {
    pub user:    Pubkey,
    /// 擬似乱数結果 (0–99)
    pub outcome: u8,
}
