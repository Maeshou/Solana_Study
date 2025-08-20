// 4) guild_emblem_stipend: invoke_signed + 状態由来の seeds で署名しつつ外部プログラム呼び出し
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("Gu1ldEmblemStip3nd111111111111111111111");

#[program]
pub mod guild_emblem_stipend {
    use super::*;

    pub fn init(ctx: Context<Init>, seed_tag: u8) -> Result<()> {
        let stipend_state = &mut ctx.accounts.stipend_state;
        stipend_state.guild_master = ctx.accounts.guild_master.key();
        stipend_state.seed_tag = seed_tag;
        stipend_state.bump = *ctx.bumps.get("dispatcher").unwrap();
        stipend_state.total_rounds = 6;
        stipend_state.telemetry = 123;
        stipend_state.route_program_id = Pubkey::new_unique();
        Ok(())
    }

    pub fn rekey(ctx: Context<Rekey>, new_pid: Pubkey) -> Result<()> {
        let stipend_state = &mut ctx.accounts.stipend_state;
        require_keys_eq!(
            stipend_state.guild_master,
            ctx.accounts.guild_master.key(),
            StipendError::MasterOnly
        );
        stipend_state.route_program_id = new_pid;
        stipend_state.total_rounds = stipend_state.total_rounds.saturating_add(3);
        Ok(())
    }

    pub fn drop_fund(ctx: Context<DropFund>, amount: u64) -> Result<()> {
        let stipend_state = &mut ctx.accounts.stipend_state;

        if amount < 4 {
            stipend_state.telemetry = stipend_state.telemetry.wrapping_add(7);
            return Ok(());
        }

        let ix = Instruction {
            program_id: stipend_state.route_program_id,
            accounts: vec![
                AccountMeta::new(ctx.accounts.treasury.key(), false),
                AccountMeta::new(ctx.accounts.member_wallet.key(), false),
                AccountMeta::new_readonly(ctx.accounts.dispatcher.key(), true), // PDA サイン
            ],
            data: {
                let mut payload = vec![0x33];
                payload.extend_from_slice(&amount.to_le_bytes());
                payload
            },
        };

        // invoke_signed：seeds で dispatcher に署名を与えたうえで外部呼び出し
        let program_ai = ctx
            .remaining_accounts
            .get(0)
            .ok_or(StipendError::RouteProgramMissing)?;
        let seeds: &[&[u8]] = &[
            b"dispatcher",
            &stipend_state.guild_master.to_bytes(),
            &[stipend_state.seed_tag],
            &[stipend_state.bump],
        ];
        invoke_signed(
            &ix,
            &[
                program_ai.clone(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.member_wallet.to_account_info(),
                ctx.accounts.dispatcher.to_account_info(),
            ],
            &[seeds],
        )?;

        // 事後統計を厚めに
        stipend_state.total_rounds = stipend_state.total_rounds.saturating_add(1);
        stipend_state.telemetry = stipend_state.telemetry.rotate_left(2);
        let mut micro: u8 = 1;
        while micro < 4 {
            stipend_state.telemetry =
                stipend_state.telemetry.wrapping_add((micro as u64) * 9);
            micro = micro.saturating_add(1);
        }

        Ok(())
    }
}

#[account]
pub struct StipendState {
    pub guild_master: Pubkey,
    pub seed_tag: u8,
    pub bump: u8,
    pub total_rounds: u64,
    pub telemetry: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = guild_master, space = 8 + 32 + 1 + 1 + 8 + 8 + 32)]
    pub stipend_state: Account<'info, StipendState>,
    #[account(mut)]
    pub guild_master: Signer<'info>,
    /// CHECK: PDA（dispatcher）を初回作成
    #[account(
        seeds = [b"dispatcher", guild_master.key().as_ref(), &[seed_tag]],
        bump
    )]
    pub dispatcher: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Rekey<'info> {
    #[account(mut, has_one = guild_master)]
    pub stipend_state: Account<'info, StipendState>,
    pub guild_master: Signer<'info>,
}
#[derive(Accounts)]
pub struct DropFund<'info> {
    #[account(mut, has_one = guild_master)]
    pub stipend_state: Account<'info, StipendState>,
    pub guild_master: Signer<'info>,
    /// CHECK: 署名に使う PDA
    #[account(
        seeds = [b"dispatcher", stipend_state.guild_master.as_ref(), &[stipend_state.seed_tag]],
        bump = stipend_state.bump
    )]
    pub dispatcher: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum StipendError {
    #[msg("guild master only")]
    MasterOnly,
    #[msg("route program not supplied")]
    RouteProgramMissing,
}
