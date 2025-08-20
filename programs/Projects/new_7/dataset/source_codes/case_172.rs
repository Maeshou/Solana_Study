// 2) blend_emit: accounts をその場で配列リテラル、データは iterator で集約
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("B1endEmit111111111111111111111111111111");
const FIXED_BLEND_ID: Pubkey = pubkey!("BLeNdFixedBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB");

#[program]
pub mod blend_emit {
    use super::*;
    pub fn execute(ctx: Context<Execute>, step: u64, bonus: u64) -> Result<()> {
        let payload: Vec<u8> = [step, step.rotate_left(7)]
            .into_iter()
            .flat_map(|x| x.to_le_bytes())
            .collect();

        invoke(&Instruction{
                program_id: FIXED_BLEND_ID,
                accounts: [
                    AccountMeta::new(ctx.accounts.counter.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.user.key(), false),
                ].into(),
                data: payload.clone(),
            },
            &[
                ctx.accounts.fixed_hint.to_account_info(),
                ctx.accounts.counter.to_account_info(),
                ctx.accounts.user.to_account_info()
            ])?;

        let mut prog_ai = ctx.accounts.emit_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() { prog_ai = ctx.remaining_accounts[0].clone(); }

        invoke(&Instruction{
                program_id: *prog_ai.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.outbox.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.user.key(), false),
                ],
                data: bonus.to_le_bytes().to_vec(),
            },
            &[prog_ai,
              ctx.accounts.outbox.to_account_info(),
              ctx.accounts.user.to_account_info()])?;

        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.pool_authority.to_account_info()
                }),
            bonus
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info>{
    /// CHECK:
    pub counter: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub fixed_hint: AccountInfo<'info>,
    /// CHECK:
    pub outbox: AccountInfo<'info>,
    /// CHECK:
    pub emit_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info,TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info,TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info,Token>,
}
