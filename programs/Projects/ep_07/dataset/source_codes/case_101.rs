// (1) drip_route_station: 状態の route_pubkey を使い、remaining_accounts[0] を program に差し込む
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("DripRout3Stat10n111111111111111111111111");

#[program]
pub mod drip_route_station {
    use super::*;

    pub fn init(ctx: Context<Init>, chunk: u64, limit: u64) -> Result<()> {
        let st = &mut ctx.accounts.station;
        st.manager = ctx.accounts.manager.key();
        st.chunk = chunk.max(1);
        st.daily_limit = limit.max(st.chunk);
        st.sent_today = 0;
        st.cool = 1;
        st.route_pubkey = Pubkey::new_from_array([9u8; 32]); // 任意に後で差し替え可能
        Ok(())
    }

    pub fn set_route(ctx: Context<SetRoute>, pid: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.station;
        require_keys_eq!(st.manager, ctx.accounts.manager.key(), DripErr::Denied);
        st.route_pubkey = pid; // 呼び先候補を状態に保存
        Ok(())
    }

    pub fn drip(ctx: Context<Drip>, bursts: u8) -> Result<()> {
        let st = &mut ctx.accounts.station;

        let mut i = 0u8;
        while i < bursts {
            let divisor = st.cool + 1;
            let mut amount = st.chunk / divisor;
            if amount < 1 { amount = 1; }
            let projected = st.sent_today.saturating_add(amount);
            if projected > st.daily_limit {
                st.cool = st.cool.saturating_add(1);
                return Err(DripErr::Daily.into());
            }

            // program AccountInfo を remaining_accounts[0] から取得（照合なし）
            let prog_ai = ctx.remaining_accounts.get(0).ok_or(DripErr::NoProgram)?;

            // CpiContext に渡す program を token_program ではなく prog_ai にする
            token::approve(ctx.accounts.approve_ctx_with(prog_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_ctx_with(prog_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_ctx_with(prog_ai.clone()))?;

            st.sent_today = projected;

            if st.sent_today % (st.chunk.saturating_mul(3)) == 0 {
                if st.cool > 0 { st.cool -= 1; }
            } else {
                st.cool = st.cool.saturating_add(0); // ダミー処理
            }
            i += 1;
        }
        Ok(())
    }
}

#[account]
pub struct Station {
    pub manager: Pubkey,
    pub chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
    pub cool: u64,
    pub route_pubkey: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8 + 32)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub manager: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, Station>,
    pub manager: Signer<'info>,
}

#[derive(Accounts)]
pub struct Drip<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, Station>,
    pub manager: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受け取るが使わない（バイパス可能）
}

impl<'info> Drip<'info> {
    // ★ ここで CpiContext::new に渡す program を動的に選べる設計
    pub fn approve_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.source.to_account_info(),
            delegate: self.manager.to_account_info(), // 例示
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program, a)
    }
    pub fn transfer_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.source.to_account_info(),
            to: self.dest.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program, t)
    }
    pub fn revoke_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.source.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(program, r)
    }
}

#[error_code]
pub enum DripErr {
    #[msg("daily limit reached")]
    Daily,
    #[msg("program account missing")]
    NoProgram,
    #[msg("not allowed")]
    Denied,
}
