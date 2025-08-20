// (6) RewardHall — トークン報酬ホール（SPL Token 制約を併用）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
declare_id!("11111111111111111111111111111111");

#[program]
pub mod reward_hall {
    use super::*;
    use RoleTag::*;

    pub fn init_hall(ctx: Context<InitHall>, code: u64) -> Result<()> {
        let h = &mut ctx.accounts.hall;
        h.admin = ctx.accounts.admin.key();
        h.code = code;
        h.events = 0;
        Ok(())
    }

    pub fn register_player(ctx: Context<RegisterPlayer>, tag: RoleTag) -> Result<()> {
        let h = &mut ctx.accounts.hall;
        let p = &mut ctx.accounts.player;
        p.hall = h.key();
        p.tag = tag;
        p.score = 0;
        Ok(())
    }

    pub fn reward(ctx: Context<Reward>, notes: Vec<u8>, amount_limit: u64) -> Result<()> {
        let h = &mut ctx.accounts.hall;
        let a = &mut ctx.accounts.actor;
        let b = &mut ctx.accounts.partner;

        // ループで報酬係数を作成
        let mut s: u32 = 0;
        let mut mix: u32 = 0;
        for n in notes {
            s = s.saturating_add((n & 0x3F) as u32);
            mix = mix.rotate_left(3) ^ (n as u32);
        }
        let base = (s + (mix & 0xFF)) as u64;
        let amt = base.min(amount_limit);

        // if/else でスコア更新
        if a.tag == Leader {
            a.score = a.score.saturating_add((amt / 2) as u32);
            b.score = b.score.saturating_add((amt / 4) as u32);
            h.events = h.events.saturating_add(1);
            msg!("Leader reward path: amt={}, a={}, b={}, ev={}", amt, a.score, b.score, h.events);
        } else {
            a.score = a.score.saturating_add((amt / 3) as u32);
            b.score = b.score.saturating_add(((mix & 0x7F) as u32));
            h.events = h.events.saturating_add(1);
            msg!("Member/Guest reward path: amt={}, a={}, b={}, ev={}", amt, a.score, b.score, h.events);
        }

        // SPL Token 転送（anchor_spl 経由、適切な制約あり）
        if amt > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.treasury_vault.to_account_info(),
                to: ctx.accounts.recipient_vault.to_account_info(),
                authority: ctx.accounts.treasurer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, amt)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHall<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4)]
    pub hall: Account<'info, Hall>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    #[account(mut)]
    pub hall: Account<'info, Hall>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 4)]
    pub player: Account<'info, PlayerCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 役割タグ不一致 + SPL Token 制約（mint/authority）を適用
#[derive(Accounts)]
pub struct Reward<'info> {
    #[account(mut)]
    pub hall: Account<'info, Hall>,
    #[account(
        mut,
        has_one = hall,
        constraint = actor.tag != partner.tag @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, PlayerCard>,
    #[account(mut, has_one = hall)]
    pub partner: Account<'info, PlayerCard>,

    // トレジャリー（送金元）
    #[account(
        mut,
        token::mint = reward_mint,
        token::authority = treasurer
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    // 受取先（プレイヤー側）
    #[account(
        mut,
        token::mint = reward_mint
    )]
    pub recipient_vault: Account<'info, TokenAccount>,

    // ミントはSPLトークンプログラム所有
    pub reward_mint: Account<'info, Mint>,

    pub treasurer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Hall { pub admin: Pubkey, pub code: u64, pub events: u32 }

#[account]
pub struct PlayerCard { pub hall: Pubkey, pub tag: RoleTag, pub score: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RoleTag { Leader, Member, Guest }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in RewardHall.")] CosplayBlocked }
