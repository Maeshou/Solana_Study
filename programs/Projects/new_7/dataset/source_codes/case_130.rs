use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NftArenaSettle11111111111111111111111111");

#[program]
pub mod nft_arena_settlement {
    use super::*;
    pub fn settle(ctx: Context<Settle>, winner_score: u64, loser_score: u64) -> Result<()> {
        let st = &mut ctx.accounts.arena;
        st.rounds += 1;

        let mut program = ctx.accounts.match_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.fast_lane += 1;
        } else {
            // 不戦敗・放棄などの管理
            st.forfeit_count += 1;
            st.abandon_log.push((Clock::get()?.slot as u32, (winner_score ^ loser_score) as u32));
            // ELO の軽い補正（乱数代わりにスロット下位ビット）
            let jitter = (Clock::get()?.slot & 7) as i64 - 3;
            st.elo = st.elo.saturating_add_signed(jitter);
            st.lose_streak += 1;
        }

        // アリーナ精算：winner/loser のNFTと金庫
        let br = ArenaBridge {
            winner_nft: ctx.accounts.winner_nft.to_account_info(),
            loser_nft: ctx.accounts.loser_nft.to_account_info(),
            arena_vault: ctx.accounts.arena_vault.to_account_info(),
        };

        // 2回送信：賞与と手数料
        let bonus = (winner_score.saturating_sub(loser_score) + 3) as u64;
        let fee   = ((winner_score + loser_score) / 10) as u64;

        let mut p1 = Vec::with_capacity(24);
        p1.extend_from_slice(&st.rounds.to_le_bytes());
        p1.extend_from_slice(&bonus.to_le_bytes());
        p1.extend_from_slice(&st.elo.to_le_bytes());
        let cx1 = br.as_cpi(program.clone());
        br.reward(cx1, p1)?;

        let mut p2 = Vec::with_capacity(16);
        p2.extend_from_slice(&fee.to_le_bytes());
        p2.extend_from_slice(&(st.fast_lane + st.forfeit_count).to_le_bytes());
        let cx2 = br.as_cpi(program.clone());
        br.fee(cx2, p2)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(init, payer = judge, space = 8 + 8 + 8 + 8 + 8 + 8 + 8 + (4 + 64*4))]
    pub arena: Account<'info, ArenaState>,
    #[account(mut)] pub judge: Signer<'info>,
    /// CHECK:
    pub winner_nft: AccountInfo<'info>,
    /// CHECK:
    pub loser_nft: AccountInfo<'info>,
    /// CHECK:
    pub arena_vault: AccountInfo<'info>,
    /// CHECK:
    pub match_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ArenaState {
    pub rounds: u64,
    pub fast_lane: u64,
    pub forfeit_count: u64,
    pub elo: i64,
    pub lose_streak: u64,
    pub pad: u64,
    pub abandon_log: Vec<(u32, u32)>,
}

#[derive(Clone)]
pub struct ArenaBridge<'info> { pub winner_nft: AccountInfo<'info>, pub loser_nft: AccountInfo<'info>, pub arena_vault: AccountInfo<'info> }
impl<'info> ArenaBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, ArenaBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas_reward(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(*self.winner_nft.key, false),
            AccountMeta::new_readonly(*self.loser_nft.key, false),
            AccountMeta::new(*self.arena_vault.key, false),
        ]
    }
    fn metas_fee(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.arena_vault.key, false),
            AccountMeta::new_readonly(*self.winner_nft.key, false),
            AccountMeta::new_readonly(*self.loser_nft.key, false),
        ]
    }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.winner_nft.clone(), self.loser_nft.clone(), self.arena_vault.clone()] }
    pub fn reward(&self, cx: CpiContext<'_, '_, '_, 'info, ArenaBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas_reward(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
    pub fn fee(&self, cx: CpiContext<'_, '_, '_, 'info, ArenaBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas_fee(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
