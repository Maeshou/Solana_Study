// 3. セッション管理＋履歴（Clockなし）
use anchor_lang::prelude::*;
declare_id!("SESSAAAABBBBCCCCDDDDEEEEFFFF7777");

#[program]
pub mod misinit_session_no_clock {
    use super::*;

    pub fn init_session(
        ctx: Context<InitSession>,
        token: String,
    ) -> Result<()> {
        let ss = &mut ctx.accounts.session;
        ss.token = token;
        ss.active = true;
        ss.uses = 0;
        Ok(())
    }

    pub fn use_session(ctx: Context<InitSession>) -> Result<()> {
        let ss = &mut ctx.accounts.session;
        require!(ss.active, ErrorCode3::Inactive);
        ss.uses = ss.uses.checked_add(1).unwrap();
        if ss.uses >= 5 { ss.active = false; }
        Ok(())
    }

    pub fn log_try(ctx: Context<InitSession>, info: String) -> Result<()> {
        let log = &mut ctx.accounts.time_log;
        if log.tries.len() >= 20 { log.tries.remove(0); }
        log.tries.push(info);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(init, payer = signer, space = 8 + (4+64) + 1 + 1)]
    pub session: Account<'info, SessionData>,
    #[account(mut)] pub time_log: Account<'info, TimeLog>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SessionData { pub token:String, pub active:bool, pub uses:u8 }
#[account]
pub struct TimeLog { pub tries: Vec<String> }

#[error_code]
pub enum ErrorCode3 { #[msg("セッションが無効です。複数回使用されています。")] Inactive }
