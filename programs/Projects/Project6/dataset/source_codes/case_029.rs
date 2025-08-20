use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, token};

declare_id!("TyPeCoSpLaYPrEv1111111111111111111111111111");

#[program]
pub mod nft_crafting_guarded {
    use super::*;

    // プレイヤー作成（PDA・ディスクリミネータ確立）
    pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.authority = ctx.accounts.authority.key();
        player.level = 1;
        player.exp = 0;
        player.last_crafted = 0;
        player.bump = *ctx.bumps.get("player").ok_or(error!(GameErr::MissingBump))?;
        Ok(())
    }

    // インベントリ作成（プレイヤーと1:1でPDAを固定。型・関係性を明示）
    pub fn init_inventory(ctx: Context<InitInventory>, slots: u16) -> Result<()> {
        let inv = &mut ctx.accounts.inventory;
        inv.owner = ctx.accounts.player.key();
        inv.slots = slots;
        inv.seed_tag = *b"inv_v1__";
        inv.bump = *ctx.bumps.get("inventory").ok_or(error!(GameErr::MissingBump))?;
        Ok(())
    }

    // レシピ登録（PDAで固定）
    pub fn init_recipe(ctx: Context<InitRecipe>, cost: u64, min_level: u32) -> Result<()> {
        let recipe = &mut ctx.accounts.recipe;
        recipe.mint = ctx.accounts.item_mint.key();
        recipe.cost = cost;
        recipe.min_level = min_level;
        recipe.bump = *ctx.bumps.get("recipe").ok_or(error!(GameErr::MissingBump))?;
        Ok(())
    }

    // クラフト実行：
    //  - すべて型付きアカウント（Account<T> / TokenAccount / Mint）で受ける
    //  - has_one / seeds / constraint で取り違えを遮断
    //  - TokenAccountはmint/owner一致を厳格化
    pub fn craft_item(ctx: Context<CraftItem>, spend: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let inv = &mut ctx.accounts.inventory;
        let recipe = &ctx.accounts.recipe;

        // 前提チェック（型はAccount<T>で自動検証済み。ここでは論理条件）
        let now = Clock::get()?.unix_timestamp;
        if player.level < recipe.min_level {
            // 低レベルなら訓練的な経験値付与処理に切り替える（早期中断ではなく状態変化を伴う分岐に）
            player.exp = player.exp.saturating_add((recipe.min_level as u64).saturating_mul(3));
        }
        if spend > recipe.cost {
            // 余剰支払いはボーナス経験値に（単純なreturnにせず別処理）
            let surplus = spend.saturating_sub(recipe.cost);
            player.exp = player.exp.saturating_add(surplus.rotate_left(1));
        }

        // 連続クラフト抑制（時間ベース）
        let delta = now.saturating_sub(player.last_crafted);
        if delta < 5 {
            // クールダウン中は軽微なペナルティや在庫チェック風の処理を入れる
            if inv.slots > 0 {
                inv.slots = inv.slots.saturating_sub(1);
            }
        }
        player.last_crafted = now;

        // SPLトークンからコスト徴収（ミント一致・オーナ一致はconstraintで担保済み）
        if spend > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_token.to_account_info(),
                to: ctx.accounts.vault_token.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, spend)?;
        }

        // 経験値→レベルアップ（分岐やループを短くしない方針で複数手順）
        let mut step = 0u8;
        while step < 3 {
            let threshold = 50u64.saturating_mul((step as u64).saturating_add(1));
            if player.exp >= threshold {
                player.level = player.level.saturating_add(1);
                player.exp = player.exp.saturating_sub(threshold / 2);
            }
            step = step.saturating_add(1);
        }

        // インベントリも軽く更新（例: seed_tagのローリング）
        let mut rolled = [0u8; 8];
        let mut i = 0usize;
        while i < 8 {
            let v = inv.seed_tag[i];
            rolled[i] = v.rotate_left((i as u32) & 3);
            i += 1;
        }
        inv.seed_tag = rolled;

        Ok(())
    }

    // --- 参考：やむを得ず生のAccountInfoを受ける場合の「型確認→変換」パターン ---
    //  外部とのブリッジ等でAccountInfoが来るケースを想定。
    //  1) ディスクリミネータを手動で検査 → 2) Account<Inventory>に変換してから処理
    pub fn bind_untyped_inventory(ctx: Context<BindUntypedInventory>) -> Result<()> {
        let ai = &ctx.accounts.untyped_inventory;
        assert_account_type::<Inventory>(ai).map_err(|_| error!(GameErr::AccountTypeMismatch))?;

        // 検査のうえで型付きへ変換（以降は通常のAccount<T>として扱う）
        let mut inv: Account<Inventory> = Account::<Inventory>::try_from(ai)?;
        if inv.slots < 10 {
            inv.slots = inv.slots.saturating_add(2);
        }

        Ok(())
    }
}

// -------------------- ヘルパ --------------------

fn assert_account_type<T: anchor_lang::Discriminator>(acc: &AccountInfo) -> Result<()> {
    // データ先頭8バイトのディスクリミネータを比較
    let data = acc.try_borrow_data()?;
    if data.len() < 8 {
        return Err(error!(GameErr::AccountDataTooSmall));
    }
    let expected = T::discriminator();
    let mut same = true;
    let mut i = 0usize;
    while i < 8 {
        if data[i] != expected[i] { same = false; }
        i += 1;
    }
    if !same {
        return Err(error!(GameErr::AccountTypeMismatch));
    }
    Ok(())
}

// -------------------- エラー --------------------

#[error_code]
pub enum GameErr {
    #[msg("bump not found in bumps map")]
    MissingBump,
    #[msg("account data is too small")]
    AccountDataTooSmall,
    #[msg("account type mismatch (discriminator mismatch)")]
    AccountTypeMismatch,
}

// -------------------- アカウント --------------------

#[account]
pub struct PlayerProfile {
    pub authority: Pubkey,
    pub level: u32,
    pub exp: u64,
    pub last_crafted: i64,
    pub bump: u8,
}

#[account]
pub struct Inventory {
    pub owner: Pubkey,     // PlayerProfileのPDA（player.key()）を指す
    pub slots: u16,
    pub seed_tag: [u8; 8],
    pub bump: u8,
}

#[account]
pub struct Recipe {
    pub mint: Pubkey,
    pub cost: u64,
    pub min_level: u32,
    pub bump: u8,
}

// -------------------- コンテキスト --------------------

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"player", authority.key().as_ref()],
        bump,
        space = 8 + 32 + 4 + 8 + 8 + 1
    )]
    pub player: Account<'info, PlayerProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    // プレイヤーは型付き + 所有者をSignerと関連付け
    #[account(mut, has_one = authority)]
    pub player: Account<'info, PlayerProfile>,

    #[account(
        init,
        payer = authority,
        seeds = [b"inventory", player.key().as_ref()],
        bump,
        space = 8 + 32 + 2 + 8 + 1
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitRecipe<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"recipe", item_mint.key().as_ref()],
        bump,
        space = 8 + 32 + 8 + 4 + 1
    )]
    pub recipe: Account<'info, Recipe>,

    pub item_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    // 型付きで受けることで「別型の同形アカウント」を排除
    #[account(mut, has_one = authority)]
    pub player: Account<'info, PlayerProfile>,

    #[account(
        mut,
        seeds = [b"inventory", player.key().as_ref()],
        bump = inventory.bump,
        constraint = inventory.owner == player.key()
    )]
    pub inventory: Account<'info, Inventory>,

    #[account(
        seeds = [b"recipe", item_mint.key().as_ref()],
        bump = recipe.bump
    )]
    pub recipe: Account<'info, Recipe>,

    pub item_mint: Account<'info, Mint>,

    // SPL Tokenはmint/owner一致で縛る（取り違え防止）
    #[account(
        mut,
        constraint = user_token.mint == item_mint.key(),
        constraint = user_token.owner == authority.key()
    )]
    pub user_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault_token.mint == item_mint.key()
    )]
    pub vault_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BindUntypedInventory<'info> {
    // あくまで例：生のAccountInfoを受ける場合は直後にassert_account_typeで検査
    /// CHECK: 直後にディスクリミネータ検査を行うためUncheckedにしている
    pub untyped_inventory: AccountInfo<'info>,
}
