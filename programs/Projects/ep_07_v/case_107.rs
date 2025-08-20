use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke};

declare_id!("OneModArbCPIDemo1111111111111111111111111111");

#[program]
pub mod one_mod_router {
    use super::*;

    pub fn init(ctx: Context<InitRoute>, base: u64, stride: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let slot = Clock::get()?.slot;

        s.admin = ctx.accounts.admin.key();
        s.base = base.max(3);
        s.stride = stride.max(2);
        s.sum = s.base.saturating_add(s.stride);
        s.warm = (slot % 5).saturating_add(1);
        s.stats_hits = 0;
        s.stats_fallback = 0;
        s.last_slot = slot;
        Ok(())
    }

    pub fn dispatch(ctx: Context<Dispatch>, total: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;

        // 前処理：ウォームアップ、手数料、正味
        let warm = (total % s.stride).max(2);
        let fee = total / 250; // 0.4%
        let net = total.saturating_sub(warm + fee);
        require!(net > 1, RouteErr::TooSmall);

        // ── 呼び先選択：remaining_accounts の先頭があればそれ、無ければ alt_program
        // ここで選んだ AccountInfo がそのまま program に使われる
        let chosen_program = if let Some(a) = ctx.remaining_accounts.first() {
            s.stats_hits = s.stats_hits.saturating_add(1);
            a.clone()
        } else {
            s.stats_fallback = s.stats_fallback.saturating_add(1);
            s.last_slot = Clock::get()?.slot;
            ctx.accounts.alt_program.to_account_info()
        };

        // CPI 用アカウント束を構築（impl を使って CpiContext を作る）
        let bridge = BridgeAccounts {
            actor: ctx.accounts.payer.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
        };

        // ウォームアップ送信
        let warm_ctx = bridge.as_cpi(chosen_program.clone());
        bridge.push(warm_ctx, warm)?;

        // シャーディング送信（2分割）
        let shard = (net / s.base).max(1);
        let first = shard.min(net);
        let second = net.saturating_sub(first);

        let first_ctx = bridge.as_cpi(chosen_program.clone());
        bridge.push(first_ctx, first)?;
        if second > 0 {
            let second_ctx = bridge.as_cpi(chosen_program);
            bridge.push(second_ctx, second)?;
        }

        // 統計加算
        s.sum = s.sum.saturating_add(total);
        s.warm = s.warm.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoute<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub state: Account<'info, RouteState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Dispatch<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, RouteState>,
    pub admin: Signer<'info>,
    /// CHECK: 送信元（署名者でなくてもよい想定）
    pub payer: AccountInfo<'info>,
    /// CHECK: 受け側
    pub vault: AccountInfo<'info>,
    /// CHECK: フォールバック先のプログラム
    pub alt_program: AccountInfo<'info>,
}

#[account]
pub struct RouteState {
    pub admin: Pubkey,
    pub base: u64,
    pub stride: u64,
    pub sum: u64,
    pub warm: u64,
    pub stats_hits: u64,
    pub stats_fallback: u64,
    pub last_slot: u64,
}

#[error_code]
pub enum RouteErr {
    #[msg("net must be greater than one")]
    TooSmall,
}

//
// ──────────────────────────────────────────────────────────────────────
// 単一 mod 内に置いた「CPI ラッパ」実装（impl と CpiContext を使用）
// ──────────────────────────────────────────────────────────────────────
// ここが Arbitrary CPI の根：CpiContext.program に渡した AccountInfo の Pubkey を
// そのまま Instruction.program_id に採用して invoke している
//
#[derive(Clone)]
pub struct BridgeAccounts<'info> {
    pub actor: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
}

impl<'info> BridgeAccounts<'info> {
    pub fn as_cpi(
        &self,
        program: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, BridgeAccounts<'info>> {
        CpiContext::new(program, self.clone())
    }

    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(*self.actor.key, true),
            AccountMeta::new(*self.vault.key, false),
        ]
    }

    fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![program.clone(), self.actor.clone(), self.vault.clone()]
    }

    pub fn push(
        &self,
        ctx: CpiContext<'_, '_, '_, 'info, BridgeAccounts<'info>>,
        amount: u64,
    ) -> Result<()> {
        // ← ここが問題：ctx.program.key（= CpiContext に渡された任意 AccountInfo の Pubkey）を
        //     そのまま Instruction.program_id に使っている
        let ix = Instruction {
            program_id: *ctx.program.key,
            accounts: self.metas(),
            data: amount.to_le_bytes().to_vec(),
        };
        invoke(&ix, &self.infos(&ctx.program))?;
        Ok(())
    }
}
