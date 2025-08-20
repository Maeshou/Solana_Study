use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("ArbMiXSu1te1111111111111111111111111111");

// 固定で叩く想定のID（安全寄りの対比用）
const FIXED_COUNTER_ID: Pubkey = pubkey!("FiXeDC0unTer1111111111111111111111111111");

#[program]
pub mod arb_mix_suite {
    use super::*;

    /* ───────────── helper / impl（すべて mod 内） ───────────── */

    // 共通: CpiContext を作って SPL Token を送るヘルパ
    fn transfer_tokens(
        program: &Program<Token>,
        from: &Account<TokenAccount>,
        to: &Account<TokenAccount>,
        auth: &AccountInfo,
        amount: u64,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                program.to_account_info(),
                Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: auth.clone(),
                },
            ),
            amount,
        )
    }

    // CraftMix 用のメソッド（impl も mod 内に配置）
    impl<'info> CraftMix<'info> {
        fn pay_with_tokens(&self, amount: u64) -> Result<()> {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.pool.to_account_info(),
                        to: self.user_token.to_account_info(),
                        authority: self.pool_authority.to_account_info(),
                    },
                ),
                amount,
            )
        }
    }

    // HatchMix 用のメソッド
    impl<'info> HatchMix<'info> {
        fn pay_owner(&self, amount: u64) -> Result<()> {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.chest.to_account_info(),
                        to: self.owner_token.to_account_info(),
                        authority: self.chest_authority.to_account_info(),
                    },
                ),
                amount,
            )
        }
    }

    /* ───────────── 1) reward_mix ───────────── */
    /// 固定ID invoke + 動的CPI（任意差し替え経路）+ CpiContext(token::transfer)
    pub fn reward_mix(ctx: Context<RewardMix>, stage: u64, payout: u64) -> Result<()> {
        // 固定ID（安全寄り）
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.fixed_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ],
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.fixed_hint.to_account_info(),
                ctx.accounts.fixed_slot.to_account_info(),
                ctx.accounts.actor.to_account_info(),
            ],
        )?;

        // 動的CPI（program_id を AccountInfo から採用）
        let mut prog_ai = ctx.accounts.report_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            prog_ai = ctx.remaining_accounts[0].clone();
            ctx.accounts.note.routes = ctx.accounts.note.routes.saturating_add(1);
        }
        let mut tag = stage.rotate_left(7);
        if tag < 10 {
            tag = tag.wrapping_add(10);
        }
        let mut dyn_data = stage.to_le_bytes().to_vec();
        dyn_data.extend_from_slice(&tag.to_le_bytes());

        let dyn_ix = Instruction {
            program_id: *prog_ai.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.report_pad.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ],
            data: dyn_data,
        };
        invoke(
            &dyn_ix,
            &[
                prog_ai,
                ctx.accounts.report_pad.to_account_info(),
                ctx.accounts.actor.to_account_info(),
            ],
        )?;

        // SPL Token transfer（内部でID固定）
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                },
            ),
            payout,
        )?;
        Ok(())
    }

    /* ───────────── 2) craft_mix ───────────── */
    /// impl メソッドで token::transfer、別途 動的CPI
    pub fn craft_mix(ctx: Context<CraftMix>, seed: u64, reward: u64) -> Result<()> {
        if seed > 100 {
            ctx.accounts.state.rolls = ctx.accounts.state.rolls.saturating_add(1);
        }
        if seed % 2 != 0 {
            ctx.accounts.state.odd = ctx.accounts.state.odd.wrapping_add(1);
        }

        // 固定ID
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.counter_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: seed.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.counter_hint.to_account_info(),
                ctx.accounts.counter_cell.to_account_info(),
                ctx.accounts.crafter.to_account_info(),
            ],
        )?;

        // 動的CPI
        let mut p = ctx.accounts.feed_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            p = ctx.remaining_accounts[0].clone();
            ctx.accounts.state.paths = ctx.accounts.state.paths.saturating_add(2);
        }
        let dyn_ix = Instruction {
            program_id: *p.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.feed_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.crafter.key(), false),
            ],
            data: reward.rotate_left(5).to_le_bytes().to_vec(),
        };
        invoke(
            &dyn_ix,
            &[
                p,
                ctx.accounts.feed_board.to_account_info(),
                ctx.accounts.crafter.to_account_info(),
            ],
        )?;

        // impl メソッド経由で SPL 送金
        ctx.accounts.pay_with_tokens(reward)
    }

    /* ───────────── 3) patrol_mix ───────────── */
    /// helper関数で CpiContext を構築、別途 動的CPI
    pub fn patrol_mix(ctx: Context<PatrolMix>, turn: u64, credit: u64) -> Result<()> {
        if turn % 3 != 0 {
            ctx.accounts.snap.missed = ctx.accounts.snap.missed.saturating_add(1);
        }
        if turn > 50 {
            ctx.accounts.snap.heavy = ctx.accounts.snap.heavy.wrapping_add(1);
        }

        // 固定ID
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.sync_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.agent.key(), false),
            ],
            data: turn.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.sync_hint.to_account_info(),
                ctx.accounts.sync_slot.to_account_info(),
                ctx.accounts.agent.to_account_info(),
            ],
        )?;

        // 動的CPI
        let mut rprog = ctx.accounts.router_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            rprog = ctx.remaining_accounts[0].clone();
        }
        let dyn_ix = Instruction {
            program_id: *rprog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.router_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.agent.key(), false),
            ],
            data: credit.wrapping_mul(7).to_le_bytes().to_vec(),
        };
        invoke(
            &dyn_ix,
            &[
                rprog,
                ctx.accounts.router_board.to_account_info(),
                ctx.accounts.agent.to_account_info(),
            ],
        )?;

        // helper で SPL 送金
        transfer_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.reserve,
            &ctx.accounts.agent_token,
            &ctx.accounts.reserve_authority,
            credit,
        )
    }

    /* ───────────── 4) grade_mix ───────────── */
    /// 固定ID + 動的CPI + CpiContext
    pub fn grade_mix(ctx: Context<GradeMix>, score: u64, gift: u64) -> Result<()> {
        if score > 90 {
            ctx.accounts.journal.gold = ctx.accounts.journal.gold.saturating_add(1);
        }
        if score < 60 {
            ctx.accounts.journal.red = ctx.accounts.journal.red.wrapping_add(1);
        }

        let mut data = score.to_le_bytes().to_vec();
        let mix = score.rotate_right(9);
        data.extend_from_slice(&mix.to_le_bytes());

        // 固定ID
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.gauge_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data,
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.gauge_hint.to_account_info(),
                ctx.accounts.gauge_cell.to_account_info(),
                ctx.accounts.student.to_account_info(),
            ],
        )?;

        // 動的CPI
        let mut sprog = ctx.accounts.signal_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            sprog = ctx.remaining_accounts[0].clone();
            ctx.accounts.journal.paths = ctx.accounts.journal.paths.saturating_add(3);
        }
        let dyn_ix = Instruction {
            program_id: *sprog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.signal_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.student.key(), false),
            ],
            data: gift.wrapping_add(111).to_le_bytes().to_vec(),
        };
        invoke(
            &dyn_ix,
            &[
                sprog,
                ctx.accounts.signal_board.to_account_info(),
                ctx.accounts.student.to_account_info(),
            ],
        )?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.reward_pool.to_account_info(),
                    to: ctx.accounts.student_token.to_account_info(),
                    authority: ctx.accounts.pool_authority.to_account_info(),
                },
            ),
            gift,
        )?;
        Ok(())
    }

    /* ───────────── 5) hatch_mix ───────────── */
    /// implメソッド + 動的CPI
    pub fn hatch_mix(ctx: Context<HatchMix>, seed: u64, grant: u64) -> Result<()> {
        if seed % 5 != 0 {
            ctx.accounts.inc.note = ctx.accounts.inc.note.saturating_add(1);
        }
        if seed > 999 {
            ctx.accounts.inc.large = ctx.accounts.inc.large.wrapping_add(1);
        }

        // 固定ID
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.hatch_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
            ],
            data: seed.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.hatch_hint.to_account_info(),
                ctx.accounts.hatch_cell.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;

        // 動的CPI
        let mut nprog = ctx.accounts.notice_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            nprog = ctx.remaining_accounts[0].clone();
        }
        let dyn_ix = Instruction {
            program_id: *nprog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.notice_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.owner.key(), false),
            ],
            data: grant.rotate_left(11).to_le_bytes().to_vec(),
        };
        invoke(
            &dyn_ix,
            &[
                nprog,
                ctx.accounts.notice_board.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;

        // impl メソッドで送金
        ctx.accounts.pay_owner(grant)
    }

    /* ───────────── 6) rail_mix ───────────── */
    /// 固定ID + 動的CPI + helperでトークン送金
    pub fn rail_mix(ctx: Context<RailMix>, steps: u64, tip: u64) -> Result<()> {
        if steps > 0 {
            ctx.accounts.trail.progress = ctx.accounts.trail.progress.saturating_add(steps);
        }
        if steps % 4 != 0 {
            ctx.accounts.trail.irregular = ctx.accounts.trail.irregular.wrapping_add(1);
        }

        // 固定ID
        let fixed_ix = Instruction {
            program_id: FIXED_COUNTER_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.step_slot.key(), false),
                AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
            ],
            data: steps.to_le_bytes().to_vec(),
        };
        invoke(
            &fixed_ix,
            &[
                ctx.accounts.step_hint.to_account_info(),
                ctx.accounts.step_slot.to_account_info(),
                ctx.accounts.rider.to_account_info(),
            ],
        )?;

        // 動的CPI
        let mut cprog = ctx.accounts.cast_hint.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            cprog = ctx.remaining_accounts[0].clone();
            ctx.accounts.trail.paths = ctx.accounts.trail.paths.saturating_add(2);
        }
        let dyn_ix = Instruction {
            program_id: *cprog.key,
            accounts: vec![
                AccountMeta::new(ctx.accounts.stage_board.key(), false),
                AccountMeta::new_readonly(ctx.accounts.rider.key(), false),
            ],
            data: tip.to_le_bytes().to_vec(),
        };
        invoke(
            &dyn_ix,
            &[
                cprog,
                ctx.accounts.stage_board.to_account_info(),
                ctx.accounts.rider.to_account_info(),
            ],
        )?;

        transfer_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.bank,
            &ctx.accounts.rider_token,
            &ctx.accounts.bank_authority,
            tip,
        )
    }
}

/* ───────────── Accounts / State（structは関数ではないのでモジュール外可） ───────────── */

#[derive(Accounts)]
pub struct RewardMix<'info> {
    #[account(mut)]
    pub note: Account<'info, LocalNote>,
    /// CHECK:
    pub fixed_slot: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub fixed_hint: AccountInfo<'info>,
    /// CHECK:
    pub report_pad: AccountInfo<'info>,
    /// CHECK:
    pub report_hint: AccountInfo<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct LocalNote {
    pub routes: u64,
}

#[derive(Accounts)]
pub struct CraftMix<'info> {
    #[account(mut)]
    pub state: Account<'info, CraftState>,
    /// CHECK:
    pub counter_cell: AccountInfo<'info>,
    /// CHECK:
    pub crafter: AccountInfo<'info>,
    /// CHECK:
    pub counter_hint: AccountInfo<'info>,
    /// CHECK:
    pub feed_board: AccountInfo<'info>,
    /// CHECK:
    pub feed_hint: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct CraftState {
    pub rolls: u64,
    pub odd: u64,
    pub paths: u64,
}

#[derive(Accounts)]
pub struct PatrolMix<'info> {
    #[account(mut)]
    pub snap: Account<'info, PatrolSnap>,
    /// CHECK:
    pub sync_slot: AccountInfo<'info>,
    /// CHECK:
    pub agent: AccountInfo<'info>,
    /// CHECK:
    pub sync_hint: AccountInfo<'info>,
    /// CHECK:
    pub router_board: AccountInfo<'info>,
    /// CHECK:
    pub router_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub agent_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub reserve_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct PatrolSnap {
    pub missed: u64,
    pub heavy: u64,
}

#[derive(Accounts)]
pub struct GradeMix<'info> {
    #[account(mut)]
    pub journal: Account<'info, GradeJournal>,
    /// CHECK:
    pub gauge_cell: AccountInfo<'info>,
    /// CHECK:
    pub student: AccountInfo<'info>,
    /// CHECK:
    pub gauge_hint: AccountInfo<'info>,
    /// CHECK:
    pub signal_board: AccountInfo<'info>,
    /// CHECK:
    pub signal_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reward_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct GradeJournal {
    pub gold: u64,
    pub red: u64,
    pub paths: u64,
}

#[derive(Accounts)]
pub struct HatchMix<'info> {
    #[account(mut)]
    pub inc: Account<'info, HatchNote>,
    /// CHECK:
    pub hatch_cell: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    pub hatch_hint: AccountInfo<'info>,
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub notice_hint: AccountInfo<'info>,
    #[account(mut)]
    pub chest: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub chest_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct HatchNote {
    pub note: u64,
    pub large: u64,
}

#[derive(Accounts)]
pub struct RailMix<'info> {
    #[account(mut)]
    pub trail: Account<'info, RailTrail>,
    /// CHECK:
    pub step_slot: AccountInfo<'info>,
    /// CHECK:
    pub rider: AccountInfo<'info>,
    /// CHECK:
    pub step_hint: AccountInfo<'info>,
    /// CHECK:
    pub stage_board: AccountInfo<'info>,
    /// CHECK:
    pub cast_hint: AccountInfo<'info>,
    #[account(mut)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub rider_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub bank_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
#[account]
pub struct RailTrail {
    pub progress: u64,
    pub irregular: u64,
    pub paths: u64,
}
