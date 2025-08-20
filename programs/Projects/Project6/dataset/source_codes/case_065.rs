// (10) Quest Mission — クエスト検証（ランナー/ベリファイア・ボード）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod quest_mission {
    use super::*;
    use Role::*;

    pub fn init_hub(ctx: Context<InitHub>, code: u64) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        h.owner = ctx.accounts.owner.key();
        h.code = code;
        h.completed = 0;
        Ok(())
    }

    pub fn register(ctx: Context<Register>, role: Role, tag: u16) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        let r = &mut ctx.accounts.runner;
        r.hub = h.key();
        r.role = role;
        r.tag = tag;
        r.progress = 0;
        let v = &mut ctx.accounts.verifier;
        v.hub = h.key();
        v.role = Verifier;
        v.tag = tag ^ 0x55AA;
        v.approvals = 0;
        Ok(())
    }

    pub fn verify(ctx: Context<Verify>, steps: Vec<u16>) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        let r = &mut ctx.accounts.runner;
        let v = &mut ctx.accounts.verifier;
        let b = &mut ctx.accounts.board;

        let mut sum: u32 = 0;
        let mut mix: u16 = 0;
        for s in steps {
            sum = sum.saturating_add((s & 0x3FF) as u32);
            mix ^= (s.rotate_left(1)) ^ 0x1234;
        }
        let inc = (sum + (mix & 0xFF) as u32) as u32;

        if r.role == Runner {
            r.progress = r.progress.saturating_add(inc / 2);
            v.approvals = v.approvals.saturating_add((mix & 0x3F) as u16);
            h.completed = h.completed.saturating_add((inc / 8) as u32);
            msg!("Runner path: inc={}, prog={}, appr={}, done={}", inc, r.progress, v.approvals, h.completed);
        } else {
            r.progress = r.progress.saturating_add(inc / 3);
            v.approvals = v.approvals.saturating_add(((mix >> 2) & 0x3F) as u16);
            h.completed = h.completed.saturating_add((inc / 10) as u32);
            msg!("Verifier-as-actor path: inc={}, prog={}, appr={}, done={}", inc, r.progress, v.approvals, h.completed);
        }

        // sqrt 近似で指数
        let mut x = (h.completed as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (h.completed as u128 / x)).max(1) / 2;
            i += 1;
        }
        b.hub = h.key();
        b.index = (x as u32).min(2_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHub<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4)]
    pub hub: Account<'info, Hub>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub hub: Account<'info, Hub>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2 + 4)]
    pub runner: Account<'info, RoleCard>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2 + 2)]
    pub verifier: Account<'info, RoleCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 役割不一致 + 同一ハブ
#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub hub: Account<'info, Hub>,
    #[account(mut, has_one = hub)]
    pub board: Account<'info, Board>,
    #[account(
        mut,
        has_one = hub,
        constraint = runner.role != verifier.role @ ErrCode::CosplayBlocked
    )]
    pub runner: Account<'info, RoleCard>,
    #[account(mut, has_one = hub)]
    pub verifier: Account<'info, RoleCard>,
}
