use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NoElseNoMatchNoAndAnd1111111111111111111");

#[program]
pub mod no_else_no_match_no_andand {
    use super::*;

    // A) 分岐→動的CPI→ループ（すべて単独 if／入れ子で複合）
    pub fn ex_a(ctx: Context<ExA>, n: u64) -> Result<()> {
        if n % 2 == 0 { ctx.accounts.state.evens += 1; }
        if n > 10 {
            if n % 3 == 0 { ctx.accounts.state.marks += 1; }
        }

        let mut prg = ctx.accounts.router.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone(); // ← 差し替え可能（Arbitrary CPI）
            ctx.accounts.state.switch += 1;
        }

        DynBridge {
            a: ctx.accounts.acc_a.to_account_info(),
            b: ctx.accounts.acc_b.to_account_info(),
        }
        .send(
            CpiContext::new(prg.clone(), DynBridge {
                a: ctx.accounts.acc_a.to_account_info(),
                b: ctx.accounts.acc_b.to_account_info(),
            }),
            n.to_le_bytes().to_vec(),
            true, // メタ順の切替フラグ（例）
        )?;

        for _ in 0..(n % 3 + 1) {
            ctx.accounts.state.tick = ctx.accounts.state.tick.wrapping_add(1);
        }
        Ok(())
    }

    // B) ループ→分岐→動的CPI（入れ子 if で複合条件）
    pub fn ex_b(ctx: Context<ExB>, seed: u64) -> Result<()> {
        for _ in 0..(seed % 4) {
            ctx.accounts.buf.mix ^= Clock::get()?.slot;
        }

        if seed > 0 {
            if seed & 1 == 1 { ctx.accounts.buf.odds += 1; }
        }

        let mut prg = ctx.accounts.exec_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone(); // ← 任意先CPI成立
            ctx.accounts.buf.paths += 1;
        }

        DynBridge {
            a: ctx.accounts.sheet.to_account_info(),
            b: ctx.accounts.user.to_account_info(),
        }
        .send(
            CpiContext::new(prg.clone(), DynBridge {
                a: ctx.accounts.sheet.to_account_info(),
                b: ctx.accounts.user.to_account_info(),
            }),
            (seed ^ 0xA5A5).to_le_bytes().to_vec(),
            false,
        )?;
        Ok(())
    }

    // C) 分岐（入れ子）→動的CPI→分岐→ループ
    pub fn ex_c(ctx: Context<ExC>, tag: u64) -> Result<()> {
        if tag >= 5 {
            if tag <= 100 { ctx.accounts.log.range += 1; }
        }

        let mut prg = ctx.accounts.notifier.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone(); // ← 任意先CPI成立
            ctx.accounts.log.routes += 1;
        }

        DynBridge {
            a: ctx.accounts.page.to_account_info(),
            b: ctx.accounts.actor.to_account_info(),
        }
        .send(
            CpiContext::new(prg.clone(), DynBridge {
                a: ctx.accounts.page.to_account_info(),
                b: ctx.accounts.actor.to_account_info(),
            }),
            tag.to_le_bytes().to_vec(),
            true,
        )?;

        if tag % 2 == 0 { ctx.accounts.log.evens += 1; }

        for _ in 0..(tag % 2 + 1) {
            ctx.accounts.log.bump = ctx.accounts.log.bump.wrapping_add(2);
        }
        Ok(())
    }
}

/* =========================
   Bridge（Arbitrary CPI 残る実装）
   ========================= */
#[derive(Clone)]
pub struct DynBridge<'info> {
    pub a: AccountInfo<'info>,
    pub b: AccountInfo<'info>,
}

impl<'info> DynBridge<'info> {
    pub fn send(
        &self,
        cx: CpiContext<'_, '_, '_, 'info, DynBridge<'info>>,
        data: Vec<u8>,
        ab_order: bool,
    ) -> Result<()> {
        let metas = if ab_order {
            vec![
                AccountMeta::new(*self.a.key, false),
                AccountMeta::new_readonly(*self.b.key, false),
            ]
        } else {
            vec![
                AccountMeta::new(*self.b.key, false),
                AccountMeta::new_readonly(*self.a.key, false),
            ]
        };

        let infos = vec![cx.program.clone(), self.a.clone(), self.b.clone()];
        let ix = Instruction {
            program_id: *cx.program.key, // ← 動的採用（ここが任意先CPIの肝）
            accounts: metas,
            data,
        };
        invoke(&ix, &infos)?;
        Ok(())
    }
}

/* =========================
   Accounts / State
   ========================= */
#[derive(Accounts)]
pub struct ExA<'info> {
    #[account(mut)] pub state: Account<'info, S1>,
    /// CHECK: 任意
    pub acc_a: AccountInfo<'info>,
    /// CHECK:
    pub acc_b: AccountInfo<'info>,
    /// CHECK:
    pub router: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExB<'info> {
    #[account(mut)] pub buf: Account<'info, S2>,
    /// CHECK:
    pub sheet: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub exec_prog: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExC<'info> {
    #[account(mut)] pub log: Account<'info, S3>,
    /// CHECK:
    pub page: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub notifier: AccountInfo<'info>,
}

#[account] pub struct S1 { pub evens: u64, pub marks: u64, pub switch: u64, pub tick: u64 }
#[account] pub struct S2 { pub mix: u64, pub odds: u64, pub paths: u64 }
#[account] pub struct S3 { pub range: u64, pub routes: u64, pub evens: u64, pub bump: u64 }
