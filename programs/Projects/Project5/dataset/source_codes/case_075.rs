// ======================================================================
// 8) Tower Lines：侵攻ルート（初期化=ロジスティック近似で初期強度）
// ======================================================================
declare_id!("TWRL88888888888888888888888888888888888888");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Wave { Prep, Hold, Surge }

#[program]
pub mod tower_lines {
    use super::*;
    use Wave::*;

    pub fn init_map(ctx: Context<InitMap>, pop: u32) -> Result<()> {
        let m = &mut ctx.accounts.map;
        m.owner = ctx.accounts.overseer.key();
        m.limit = pop.saturating_mul(3) + 100;
        m.phase = Prep;

        let p = &mut ctx.accounts.path_a;
        let q = &mut ctx.accounts.path_b;
        let g = &mut ctx.accounts.grid;

        // logistic-ish: x -> x + x*(1-x/K) を粗く
        let mut x = (pop as u64).max(1);
        let k = 1000u64;
        let delta = (x * (k - x)) / k;
        p.parent = m.key(); p.node = (pop & 7) as u8; p.strength = (x + delta) as u32 % 777 + 33;
        q.parent = m.key(); q.node = ((pop >> 2) & 7) as u8; q.strength = ((x / 2 + delta) as u32 % 701) + 21;

        g.parent = m.key(); g.node = 9; g.ticks = 0; g.hash = (x ^ delta) as u64;
        Ok(())
    }

    pub fn hold(ctx: Context<Hold>, t: u32) -> Result<()> {
        let m = &mut ctx.accounts.map;
        let p = &mut ctx.accounts.path_a;
        let q = &mut ctx.accounts.path_b;
        let g = &mut ctx.accounts.grid;

        for i in 0..t {
            let z = ((p.strength ^ q.strength) as u64).wrapping_mul(1469598103934665603);
            p.strength = p.strength.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            q.strength = q.strength.saturating_add((((z >> 6) & 63) as u32) + 5);
            g.ticks = g.ticks.saturating_add(1);
            g.hash ^= z.rotate_left((i % 19) as u32);
        }

        let total = p.strength + q.strength;
        if total > m.limit {
            m.phase = Surge;
            p.node ^= 0x1;
            q.node = q.node.saturating_add(1);
            g.ticks = g.ticks.saturating_add(10);
            msg!("surge: node tweak & ticks+10");
        } else {
            m.phase = Hold;
            p.strength = p.strength.saturating_add(7);
            q.strength = q.strength / 2 + 9;
            g.hash ^= 0x0FF0_FF0F;
            msg!("hold: adjust strengths & hash flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMap<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub map: Account<'info, MapCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub path_a: Account<'info, Path>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub path_b: Account<'info, Path>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub grid: Account<'info, GridTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub overseer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Hold<'info> {
    #[account(mut, has_one=owner)]
    pub map: Account<'info, MapCore>,
    #[account(
        mut,
        has_one=map,
        constraint = path_a.node != path_b.node @ TowerErr::Dup
    )]
    pub path_a: Account<'info, Path>,
    #[account(
        mut,
        has_one=map,
        constraint = path_b.node != grid.node @ TowerErr::Dup
    )]
    pub path_b: Account<'info, Path>,
    #[account(mut, has_one=map)]
    pub grid: Account<'info, GridTape>,
    pub overseer: Signer<'info>,
}

#[account] pub struct MapCore { pub owner: Pubkey, pub limit: u32, pub phase: Wave }
#[account] pub struct Path { pub parent: Pubkey, pub node: u8, pub strength: u32 }
#[account] pub struct GridTape { pub parent: Pubkey, pub node: u8, pub ticks: u64, pub hash: u64 }

#[error_code] pub enum TowerErr { #[msg("duplicate mutable account")] Dup }
