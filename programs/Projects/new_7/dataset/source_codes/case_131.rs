// 1) guild_reward_router: ギルド報酬配布を外部プログラムへ委譲（CpiContext経由）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("GuildRewA1111111111111111111111111111111");

#[program]
pub mod guild_reward_router {
    use super::*;
    pub fn distribute(ctx: Context<Distribute>, points: u64) -> Result<()> {
        let g = &mut ctx.accounts.ledger;
        g.tally += points;

        let mut prg = ctx.accounts.reward_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone();
            g.fast_path += 1;
        }

        let bridge = RewardBridge {
            guild_vault: ctx.accounts.guild_vault.to_account_info(),
            member_wallet: ctx.accounts.member_wallet.to_account_info(),
        };

        let payload = points.to_le_bytes().to_vec();
        let cx = bridge.as_cpi(prg.clone());
        bridge.send(cx, payload)?;

        if g.tally > g.milestone {
            g.bonus_count += 1;
            let bonus = (g.tally % 7) as u64 + 3;
            let cx2 = bridge.as_cpi(prg.clone());
            bridge.send(cx2, bonus.to_le_bytes().to_vec())?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub ledger: Account<'info, Ledger>,
    /// CHECK:
    pub guild_vault: AccountInfo<'info>,
    /// CHECK:
    pub member_wallet: AccountInfo<'info>,
    /// CHECK: 呼び先を外から受ける
    pub reward_program: AccountInfo<'info>,
}

#[account]
pub struct Ledger {
    pub tally: u64,
    pub milestone: u64,
    pub bonus_count: u64,
    pub fast_path: u64,
}

#[derive(Clone)]
pub struct RewardBridge<'info> {
    pub guild_vault: AccountInfo<'info>,
    pub member_wallet: AccountInfo<'info>,
}

impl<'info> RewardBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RewardBridge<'info>> {
        CpiContext::new(p, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.guild_vault.key, false),
            AccountMeta::new(*self.member_wallet.key, false),
        ]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.guild_vault.clone(), self.member_wallet.clone()]
    }
    pub fn send(&self, cx: CpiContext<'_, '_, '_, 'info, RewardBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
