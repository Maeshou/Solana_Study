use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("EnergyRouterA11111111111111111111111111111");

#[program]
pub mod energy_router_a {
    use super::*;

    pub fn init_station(ctx: Context<InitA>, chunk: u64, limit: u64) -> Result<()> {
        let st = &mut ctx.accounts.station;
        st.manager = ctx.accounts.manager.key();
        st.chunk = if chunk < 1 { 1 } else { chunk };
        st.daily_limit = if limit < st.chunk { st.chunk } else { limit };
        st.sent_today = 0;
        st.cool_level = 0;
        Ok(())
    }

    pub fn drip(ctx: Context<DripA>, bursts: u8) -> Result<()> {
        let st = &mut ctx.accounts.station;

        let mut loop_index: u8 = 0;
        while loop_index < bursts {
            let base = st.chunk;
            let scaled = if st.cool_level > 3 { base / 2 } else { base };
            let amount = if scaled < 1 { 1 } else { scaled };

            let projected = st.sent_today.saturating_add(amount);
            if projected > st.daily_limit {
                st.cool_level = st.cool_level.saturating_add(1);
                return Err(DripAErr::Reached.into());
            }

            // ─────────────────────────────────────────────────────────────────
            // ここが肝：分岐で使う「program」を切替。
            // use_alt が true の場合、型検証のない alt_program を CPI 先に採用。
            // （CpiContext は AccountInfo を受け取れるため、型での縛りを回避可能）
            // ─────────────────────────────────────────────────────────────────
            let use_alt = (st.cool_level % 3) == 0;
            let program_ai = if use_alt {
                ctx.accounts.alt_program.to_account_info()      // ← 未検証のプログラム
            } else {
                ctx.accounts.token_program.to_account_info()     // ← 型付き Token プログラム
            };

            token::approve(ctx.accounts.approve_ctx_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_ctx_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_ctx_with(program_ai))?;

            st.sent_today = projected;

            if st.sent_today % (st.chunk * 4) == 0 {
                if st.cool_level > 0 { st.cool_level -= 1; }
            }
            loop_index = loop_index.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitA<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub station: Account<'info, StationA>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DripA<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, StationA>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest_vault: Account<'info, TokenAccount>,
    /// CHECK: 外部から差し替え可能なプログラム
    pub alt_program: UncheckedAccount<'info>,   // ← これを CpiContext の program に使える
    pub token_program: Program<'info, Token>,   // 型付きの正規プログラム
}

impl<'info> DripA<'info> {
    pub fn approve_ctx_with(
        &self,
        program_ai: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let accs = Approve {
            to: self.source_vault.to_account_info(),
            delegate: self.dest_vault.to_account_info(), // 例示：転送先を一時的な委任先として使用
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program_ai, accs)
    }

    pub fn transfer_ctx_with(
        &self,
        program_ai: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.source_vault.to_account_info(),
            to: self.dest_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program_ai, accs)
    }

    pub fn revoke_ctx_with(
        &self,
        program_ai: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let accs = Revoke {
            source: self.source_vault.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program_ai, accs)
    }
}

#[account]
pub struct StationA {
    pub manager: Pubkey,
    pub chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
    pub cool_level: u64,
}

#[error_code]
pub enum DripAErr { #[msg("limit reached")] Reached }
