// 5) craft_boost_router: 状態ルート使用＋後処理で別系の演算・小ループを複数回
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("CraftBo0stRout3r11111111111111111111111");

#[program]
pub mod craft_boost_router {
    use super::*;

    pub fn init(ctx: Context<Init>, limit: u64) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        router_state.creator = ctx.accounts.creator.key();
        router_state.limit = limit;
        router_state.cycles = 9;
        router_state.signal = limit.rotate_right(2);
        router_state.route_program_id = Pubkey::new_from_array([7u8; 32]);
        Ok(())
    }

    pub fn rebind(ctx: Context<Rebind>, p: Pubkey) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;
        require_keys_eq!(
            router_state.creator,
            ctx.accounts.creator.key(),
            CraftBoostError::CreatorOnly
        );
        router_state.route_program_id = p;
        router_state.cycles = router_state.cycles.saturating_add(2);
        Ok(())
    }

    pub fn boost(ctx: Context<Boost>, energy: u64, hops: u8) -> Result<()> {
        let router_state = &mut ctx.accounts.router_state;

        if energy == 3 {
            router_state.signal = router_state.signal ^ 0xAA;
            let mut local = 1u8;
            while local < 3 {
                router_state.cycles = router_state.cycles.saturating_add(1);
                router_state.signal = router_state.signal.wrapping_add(5);
                local = local.saturating_add(1);
            }
            return Ok(());
        }

        if energy > router_state.limit {
            router_state.signal = router_state.signal.wrapping_add(energy ^ 0x1F);
            return Err(CraftBoostError::OverLimit.into());
        }

        let mut remain = energy;
        let mut hop_index: u8 = 0;
        while hop_index < hops {
            let part = (remain / 3).max(4);
            if part >= remain {
                break;
            }

            let ix = token_ix::transfer(
                &router_state.route_program_id,
                &ctx.accounts.workshop_vault.key(),
                &ctx.accounts.player_wallet.key(),
                &ctx.accounts.creator.key(),
                &[],
                part,
            )?;
            let program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(CraftBoostError::ProgramMissing)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.workshop_vault.to_account_info(),
                    ctx.accounts.player_wallet.to_account_info(),
                    ctx.accounts.creator.to_account_info(),
                ],
            )?;

            remain = remain.saturating_sub(part);
            router_state.cycles = router_state.cycles.saturating_add(1);
            router_state.signal = router_state.signal.wrapping_add(part ^ 0x0A);

            // 後処理：二段ネストの補正
            if router_state.signal % 3 == 0 {
                let mut rep: u8 = 1;
                while rep < 4 {
                    router_state.signal =
                        router_state.signal.rotate_left((rep % 2) as u32);
                    rep = rep.saturating_add(1);
                }
            } else {
                router_state.signal = router_state.signal.wrapping_add(13);
                let mut sweep: u8 = 1;
                while sweep < 3 {
                    router_state.cycles = router_state.cycles.saturating_add(1);
                    sweep = sweep.saturating_add(1);
                }
            }

            hop_index = hop_index.saturating_add(1);
        }

        if remain > 2 {
            let finalize = token_ix::transfer(
                &router_state.route_program_id,
                &ctx.accounts.workshop_vault.key(),
                &ctx.accounts.player_wallet.key(),
                &ctx.accounts.creator.key(),
                &[],
                remain - 2,
            )?;
            let program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(CraftBoostError::ProgramMissing)?;
            invoke(
                &finalize,
                &[
                    program_ai.clone(),
                    ctx.accounts.workshop_vault.to_account_info(),
                    ctx.accounts.player_wallet.to_account_info(),
                    ctx.accounts.creator.to_account_info(),
                ],
            )?;
            router_state.signal =
                router_state.signal.wrapping_add(remain - 2).rotate_right(1);
        }
        Ok(())
    }
}

#[account]
pub struct RouterState {
    pub creator: Pubkey,
    pub limit: u64,
    pub cycles: u64,
    pub signal: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub router_state: Account<'info, RouterState>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub workshop_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Rebind<'info> {
    #[account(mut, has_one = creator)]
    pub router_state: Account<'info, RouterState>,
    pub creator: Signer<'info>,
}
#[derive(Accounts)]
pub struct Boost<'info> {
    #[account(mut, has_one = creator)]
    pub router_state: Account<'info, RouterState>,
    pub creator: Signer<'info>,
    #[account(mut)]
    pub workshop_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum CraftBoostError {
    #[msg("creator only operation")]
    CreatorOnly,
    #[msg("external program account missing")]
    ProgramMissing,
    #[msg("requested energy exceeds limit")]
    OverLimit,
}
