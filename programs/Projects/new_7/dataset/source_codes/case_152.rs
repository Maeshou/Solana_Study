// 4) studio_publish_roll: 最初に固定IDrevoke→ループ→動的CPI→最後に分岐
// 固定IDrevoke→ループ→動的CPI→分岐
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Revoke, Token, TokenAccount};

declare_id!("StudioRoll111111111111111111111111111111");

#[program]
pub mod studio_publish_roll {
    use super::*;
    pub fn publish(ctx: Context<Publish>, content: u64) -> Result<()> {
        // 固定ID：revoke
        let r = Revoke {
            source: ctx.accounts.creator_token.to_account_info(),
            authority: ctx.accounts.creator_wallet.to_account_info(),
        };
        token::revoke(CpiContext::new(ctx.accounts.token_program.to_account_info(), r))?;

        // 先にループをはさむ
        for _ in 0..(content % 3 + 1) { ctx.accounts.stats.meter = ctx.accounts.stats.meter.wrapping_add(1); }

        // 動的CPI：公開
        let mut pubp = ctx.accounts.publish_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { pubp = ctx.remaining_accounts[0].clone(); ctx.accounts.stats.paths += 1; }
        let pb = PublishBridge { shelf: ctx.accounts.gallery_shelf.to_account_info(), author: ctx.accounts.creator_wallet.to_account_info() };
        pb.push(pb.as_cpi(pubp.clone()), content.to_le_bytes().to_vec())?;

        if content % 2 == 0 { ctx.accounts.stats.evens += 1; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Publish<'info> {
    #[account(mut)]
    pub stats: Account<'info, StudioStats>,

    // 固定ID
    #[account(mut)]
    pub creator_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub creator_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 動的
    /// CHECK:
    pub gallery_shelf: AccountInfo<'info>,
    /// CHECK:
    pub publish_program: AccountInfo<'info>,
}

#[account]
pub struct StudioStats { pub meter: u64, pub paths: u64, pub evens: u64 }

#[derive(Clone)]
pub struct PublishBridge<'info> { pub shelf: AccountInfo<'info>, pub author: AccountInfo<'info> }
impl<'info> PublishBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, PublishBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.shelf.key, false), AccountMeta::new_readonly(*self.author.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.shelf.clone(), self.author.clone()] }
    pub fn push(&self, cx: CpiContext<'_, '_, '_, 'info, PublishBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
