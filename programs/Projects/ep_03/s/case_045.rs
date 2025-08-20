use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGachaSvc01");

#[program]
pub mod gacha_service {
    use super::*;

    /// ガチャを引いてアイテムを取得するが、
    /// gacha_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn draw_once(ctx: Context<DrawOnce>) -> Result<()> {
        let gacha = &mut ctx.accounts.gacha_account;
        let cost = ctx.accounts.config.pull_cost;
        let drop_id = ctx.accounts.config.next_drop; // 固定設定のドロップID

        // 1. ガチャ回数をインクリメント
        gacha.pulls = gacha.pulls.checked_add(1).unwrap();

        // 2. 最後に引かれたドロップIDを記録
        gacha.last_drop = drop_id;

        // 3. ユーザーから運営プールへ Lamports を移動（所有者チェックなし）
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= cost;
        **ctx.accounts.pool.to_account_info().lamports.borrow_mut() += cost;

        // 4. （コメントアウト）CPI で NFT／アイテムを付与する処理例
        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.vault_item.to_account_info(),
        //     to: ctx.accounts.user_item_account.to_account_info(),
        //     authority: ctx.accounts.service_authority.to_account_info(),
        // };
        // token::transfer(
        //    CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        //    1,
        // )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DrawOnce<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者照合を行うべき
    pub gacha_account: Account<'info, GachaAccount>,

    /// ガチャコスト支払い元のユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// ガチャコスト受取用プールアカウント
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    /// ガチャ設定（コスト・次ドロップID）を保持するアカウント
    pub config: Account<'info, GachaConfig>,

    // 以下はアイテム付与時に使用（コメントアウト中）
    // #[account(mut)]
    // pub vault_item: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub user_item_account: Account<'info, TokenAccount>,
    // pub service_authority: Signer<'info>,
    // pub token_program: Program<'info, Token>,
}

#[account]
pub struct GachaAccount {
    /// 本来このガチャを回せるユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでに引いた回数
    pub pulls: u64,
    /// 最後に出たドロップの ID
    pub last_drop: u8,
}

#[account]
pub struct GachaConfig {
    /// ガチャ 1 回あたりのコスト（Lamports）
    pub pull_cost: u64,
    /// 次に出現するドロップの固定 ID
    pub next_drop: u8,
}
