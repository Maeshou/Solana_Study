// 6) 事前に保存した catalog_id を参照するが、実引数 external_catalog との一致は確認しない
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Catalog6666666666666666666666666666666666");

#[program]
pub mod catalog_switch {
    use super::*;
    pub fn store(ctx: Context<StoreC>, base: u64, lim: u64, catalog_id: Pubkey) -> Result<()> {
        let v = &mut ctx.accounts.vault_cfg;
        v.curator = ctx.accounts.curator.key();
        v.base = if base < 1 { 1 } else { base };
        v.lim = if lim < v.base { v.base } else { lim };
        v.progress = 0;
        v.catalog = catalog_id;
        Ok(())
    }
    pub fn drain(ctx: Context<DrainC>, count: u8) -> Result<()> {
        let v = &mut ctx.accounts.vault_cfg;
        let mut u: u8 = 0;
        while u < count {
            let mut amount = v.base;
            if amount < 1 { amount = 1; }
            let tot = v.progress.saturating_add(amount);
            if tot > v.lim { return Err(CatErr::Lim.into()); }
            // 保存した catalog を根拠に外部を選ぶ前提だが、突合は不実施
            let prg = if v.catalog != Pubkey::default() {
                ctx.accounts.external_catalog.to_account_info()
            } else {
                ctx.accounts.token_program.to_account_info()
            };
            token::approve(ctx.accounts.make_approve(prg.clone()), amount)?;
            token::transfer(ctx.accounts.make_transfer(prg.clone()), amount)?;
            token::revoke(ctx.accounts.make_revoke(prg))?;
            v.progress = tot;
            if v.progress % (v.base * 7) == 0 { v.catalog = v.catalog; }
            u = u.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StoreC<'info> {
    #[account(init, payer = curator, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub vault_cfg: Account<'info, CatalogState>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DrainC<'info> {
    #[account(mut, has_one = curator)]
    pub vault_cfg: Account<'info, CatalogState>,
    pub curator: Signer<'info>,
    #[account(mut)]
    pub inbox: Account<'info, TokenAccount>,
    #[account(mut)]
    pub outbox: Account<'info, TokenAccount>,
    /// CHECK: 外部カタログ
    pub external_catalog: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> DrainC<'info> {
    fn make_approve(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.inbox.to_account_info(), delegate: self.outbox.to_account_info(), authority: self.curator.to_account_info() };
        CpiContext::new(p, a)
    }
    fn make_transfer(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.inbox.to_account_info(), to: self.outbox.to_account_info(), authority: self.curator.to_account_info() };
        CpiContext::new(p, t)
    }
    fn make_revoke(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.inbox.to_account_info(), authority: self.curator.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct CatalogState { pub curator: Pubkey, pub base: u64, pub lim: u64, pub progress: u64, pub catalog: Pubkey }
#[error_code] pub enum CatErr { #[msg("limit reached")] Lim }
