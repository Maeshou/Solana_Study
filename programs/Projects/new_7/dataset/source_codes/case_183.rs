// 2) forge_signal_mix.rs
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FoRgESiGnAlMiX1111111111111111111111111");

const COUNTER_ID: Pubkey = pubkey!("CoUnTerFiX000000000000000000000000000000");

#[program]
pub mod forge_signal_mix {
    use super::*;

    impl<'info> DoForge<'info> {
        fn pay_bounty(&self, amt: u64) -> Result<()> {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.bounty_bank.to_account_info(),
                        to: self.player_token.to_account_info(),
                        authority: self.bank_authority.to_account_info(),
                    }
                ),
                amt
            )
        }
    }

    pub fn run(ctx: Context<DoForge>, spark: u64, bounty: u64) -> Result<()> {
        if spark % 2 != 0 {
            ctx.accounts.state.odd = ctx.accounts.state.odd.saturating_add(1);
        }
        if spark > 1000 {
            ctx.accounts.state.big = ctx.accounts.state.big.wrapping_add(1);
        }

        // 固定ID
        let ix_fixed = Instruction {
            program_id: COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.counter_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.player.key(), false),
            ],
            data: spark.to_le_bytes().to_vec(),
        };
        invoke(&ix_fixed, &[
            ctx.accounts.counter_hint.to_account_info(),
            ctx.accounts.counter_slot.to_account_info(),
            ctx.accounts.player.to_account_info(),
        ])?;

        // 動的CPI
        let mut prg = ctx.accounts.signal_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prg = ctx.remaining_accounts[0].clone();
            ctx.accounts.state.routes = ctx.accounts.state.routes.saturating_add(2);
        }
        let ix_dyn = Instruction {
            program_id: *prg.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.signal_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.player.key(), false),
            ],
            data: bounty.wrapping_mul(3).to_le_bytes().to_vec(),
        };
        invoke(&ix_dyn, &[
            prg,
            ctx.accounts.signal_board.to_account_info(),
            ctx.accounts.player.to_account_info(),
        ])?;

        ctx.accounts.pay_bounty(bounty)
    }
}

#[derive(Accounts)]
pub struct DoForge<'info> {
    #[account(mut)]
    pub state: Account<'info, ForgeState>,
    /// CHECK:
    pub counter_slot: AccountInfo<'info>,
    /// CHECK:
    pub player: AccountInfo<'info>,
    /// CHECK:
    pub counter_hint: AccountInfo<'info>,
    /// CHECK:
    pub signal_board: AccountInfo<'info>,
    /// CHECK:
    pub signal_hint: AccountInfo<'info>,
    #[account(mut)]
    pub bounty_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct ForgeState { pub odd: u64, pub big: u64, pub routes: u64 }
