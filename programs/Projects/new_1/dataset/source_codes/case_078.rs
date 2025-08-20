use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, MintTo, mint_to};

declare_id!("Fg6PaFpoGXkYsidMpWxCONTENTREWARDINSEC000");

#[program]
pub mod content_reward_insecure {
    use super::*;

    /// 新しいコンテンツを「公開」し、PDA にメタ情報を保存します。
    /// - `title`: コンテンツタイトル  
    /// - `base_reward`: 1 視聴あたりの付与トークン量  
    /// 署名チェックは一切行われません。
    pub fn publish_content(
        ctx: Context<PublishContent>,
        title: String,
        base_reward: u64,
    ) {
        let info = &mut ctx.accounts.content_meta;
        info.author       = *ctx.accounts.user.key;
        info.title        = title;
        info.base_reward  = base_reward;
        info.total_views  = 0;
        info.total_rewards = 0;
    }

    /// コンテンツを「視聴」した際に呼び出し、視聴カウントをインクリメント。
    /// - `views`: 今回の視聴回数  
    /// 署名チェックなし、分岐・ループも使いません。
    pub fn record_views(ctx: Context<RecordViews>, views: u64) {
        let meta = &mut ctx.accounts.content_meta;
        meta.total_views   = meta.total_views.saturating_add(views);
        // 同時に累積報酬見込みを更新
        let add = views.checked_mul(meta.base_reward).unwrap_or(0);
        meta.total_rewards = meta.total_rewards.saturating_add(add);
    }

    /// 蓄積された報酬をミントしてクリエイターに付与します。
    /// - `mint_amount`: 今回ミントする量  
    pub fn mint_rewards(ctx: Context<MintRewards>, mint_amount: u64) {
        // CPI でトークンをミント（署名チェック omitted intentionally）
        let cpi_accounts = MintTo {
            mint:      ctx.accounts.reward_mint.to_account_info(),
            to:        ctx.accounts.author_token_acc.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let seeds = &[&[b"mint_auth", &[*ctx.bumps.get("mint_authority").unwrap()]]];
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                seeds,
            ),
            mint_amount,
        ).unwrap();

        // PDA に実際に付与した量を累積
        let meta = &mut ctx.accounts.content_meta;
        meta.distributed = meta.distributed.saturating_add(mint_amount);
    }

    /// コンテンツのメタ情報をイベントで通知します。
    pub fn fetch_meta(ctx: Context<FetchMeta>) {
        let m = &ctx.accounts.content_meta;
        emit!(ContentMetaEvent {
            author:        m.author,
            title:         m.title.clone(),
            total_views:   m.total_views,
            total_rewards: m.total_rewards,
            distributed:   m.distributed,
        });
    }
}

#[derive(Accounts)]
pub struct PublishContent<'info> {
    /// コンテンツ公開者（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    /// メタ情報を保持する PDA（init_if_needed OK）
    #[account(init_if_needed, payer = payer, seeds = [b"meta", user.key().as_ref()], bump, space = 8 + 32 + (4+100) + 8 + 8 + 8 + 8)]
    pub content_meta:    Account<'info, ContentMeta>,

    #[account(mut)]
    pub payer:           Signer<'info>,
    pub system_program:  Program<'info, System>,
    pub rent:            Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RecordViews<'info> {
    /// 利用者（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    /// 既存のメタ情報 PDA
    #[account(mut, seeds = [b"meta", content_meta.author.as_ref()], bump)]
    pub content_meta:    Account<'info, ContentMeta>,
}

#[derive(Accounts)]
pub struct MintRewards<'info> {
    /// クリエイター（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    /// トークンミント権限を持つ PDA（署名チェック omitted）
    #[account(seeds = [b"mint_auth"], bump)]
    pub mint_authority:  AccountInfo<'info>,

    /// 付与用トークンミント
    pub reward_mint:     Account<'info, Mint>,

    /// クリエイター受け取り先 TokenAccount
    #[account(mut)]
    pub author_token_acc: Account<'info, TokenAccount>,

    /// メタ情報 PDA
    #[account(mut, seeds = [b"meta", content_meta.author.as_ref()], bump)]
    pub content_meta:     Account<'info, ContentMeta>,

    pub token_program:    Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FetchMeta<'info> {
    /// メタ情報 PDA
    #[account(seeds = [b"meta", content_meta.author.as_ref()], bump)]
    pub content_meta:    Account<'info, ContentMeta>,

    pub user:            AccountInfo<'info>,
}

#[account]
pub struct ContentMeta {
    pub author:         Pubkey,
    pub title:          String,
    pub base_reward:    u64,
    pub total_views:    u64,
    pub total_rewards:  u64,
    pub distributed:    u64,
}

#[event]
pub struct ContentMetaEvent {
    pub author:         Pubkey,
    pub title:          String,
    pub total_views:    u64,
    pub total_rewards:  u64,
    pub distributed:    u64,
}
