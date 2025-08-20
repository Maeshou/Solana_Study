// (2) PubkeyMappedProgram: 状態に保存した Pubkey を remaining_accounts から引き当て
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("PkMapProg2222222222222222222222222222222");

#[program]
pub mod pubkey_mapped_program {
    use super::*;
    pub fn set_route(ctx: Context<SetRoute>, program_hint: Pubkey) -> Result<()> {
        let st = &mut ctx.accounts.router;
        st.admin = ctx.accounts.admin.key();
        st.program_hint = program_hint; // 呼び先候補を Pubkey として保存
        Ok(())
    }
    pub fn hop(ctx: Context<Hop>, amount: u64) -> Result<()> {
        // ★ hint と一致する remaining_accounts の AccountInfo を探して使う
        let want = ctx.accounts.router.program_hint;
        let mut chosen = None;
        for ai in ctx.remaining_accounts.iter() {
            if ai.key() == want { chosen = Some(ai.clone()); break; }
        }
        let p = chosen.ok_or(ErrorCode::NoRoute)?;

        token::transfer(
            CpiContext::new(
                p, // ← Program<Token> ではなく AccountInfo
                Transfer {
                    from: ctx.accounts.from_ata.to_account_info(),
                    to: ctx.accounts.to_ata.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32)]
    pub router: Account<'info, RouteCfg>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Hop<'info> {
    #[account(mut, has_one = admin)]
    pub router: Account<'info, RouteCfg>,
    pub admin: Signer<'info>,
    #[account(mut)] pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)] pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct RouteCfg { pub admin: Pubkey, pub program_hint: Pubkey }
#[error_code] pub enum ErrorCode { #[msg("route not found")] NoRoute }
