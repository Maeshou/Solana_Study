use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("EnergyRouterC33333333333333333333333333333");

#[program]
pub mod energy_router_c {
    use super::*;

    pub fn configure(ctx: Context<ConfigureC>, base: u64, limit: u64, prefer_alt: bool) -> Result<()> {
        let st = &mut ctx.accounts.station;
        st.owner = ctx.accounts.owner.key();
        st.base = if base < 1 { 1 } else { base };
        st.limit = if limit < st.base { st.base } else { limit };
        st.today = 0;
        st.prefer_alt = prefer_alt;        // ルーティング方針を保存
        st.catalog = ctx.accounts.catalog_program.key(); // カタログ的に保存するだけ
        Ok(())
    }

    pub fn execute(ctx: Context<ExecuteC>, rounds: u8) -> Result<()> {
        let st = &mut ctx.accounts.station;
        let mut count = 0u8;

        while count < rounds {
            let amount = st.base;
            let new_total = st.today.saturating_add(amount);
            if new_total > st.limit {
                return Err(ExecCErr::Quota.into());
            }

            // ─────────────────────────────────────────────────────────────
            // ここが肝：impl 側で「保存値 prefer_alt とカタログ」を根拠に
            // program を選んで CpiContext を構築するが、実アカウントとの整合を検証していない。
            // external_router が catalog_program と一致しているか等も未検査。
            // ─────────────────────────────────────────────────────────────
            let chosen = if st.prefer_alt { 1u8 } else { 0u8 };
            if chosen == 1 {
                token::approve(ctx.accounts.approve_with_alt(), amount)?;
                token::transfer(ctx.accounts.transfer_with_alt(), amount)?;
                token::revoke(ctx.accounts.revoke_with_alt())?;
            } else {
                token::approve(ctx.accounts.approve_with_token(), amount)?;
                token::transfer(ctx.accounts.transfer_with_token(), amount)?;
                token::revoke(ctx.accounts.revoke_with_token())?;
            }

            st.today = new_total;
            if st.today % (st.base * 5) == 0 { st.prefer_alt = !st.prefer_alt; }
            count = count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureC<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 1 + 32)]
    pub station: Account<'info, StationC>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: 参照として保存するだけのプログラムキー
    pub catalog_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteC<'info> {
    #[account(mut, has_one = owner)]
    pub station: Account<'info, StationC>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub from_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_vault: Account<'info, TokenAccount>,
    /// CHECK: 実際に使う外部プログラム（catalog との一致検証なし）
    pub external_router: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ExecuteC<'info> {
    fn approve_with_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.from_vault.to_account_info(),
            delegate: self.to_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn transfer_with_token(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.from_vault.to_account_info(),
            to: self.to_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    fn revoke_with_token(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.from_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), r)
    }

    fn approve_with_alt(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.from_vault.to_account_info(),
            delegate: self.to_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.external_router.to_account_info(), a) // ← 未検証 program
    }
    fn transfer_with_alt(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.from_vault.to_account_info(),
            to: self.to_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.external_router.to_account_info(), t) // ← 未検証 program
    }
    fn revoke_with_alt(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.from_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.external_router.to_account_info(), r) // ← 未検証 program
    }
}

#[account]
pub struct StationC {
    pub owner: Pubkey,
    pub base: u64,
    pub limit: u64,
    pub today: u64,
    pub prefer_alt: bool,
    pub catalog: Pubkey,
}

#[error_code]
pub enum ExecCErr { #[msg("quota exceeded")] Quota }
