// (5) MapExplorer — 地図探索（探索者/探索者のロール不一致 + タイル盤）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod map_explorer {
    use super::*;
    use ExplorerTier::*;

    pub fn init_map(ctx: Context<InitMap>, seed: u32) -> Result<()> {
        let m = &mut ctx.accounts.map;
        m.owner = ctx.accounts.owner.key();
        m.seed = seed;
        m.tiles = 0;
        Ok(())
    }

    pub fn enlist(ctx: Context<Enlist>, tier: ExplorerTier, sign: u16) -> Result<()> {
        let m = &mut ctx.accounts.map;
        let e = &mut ctx.accounts.explorer;
        e.map = m.key();
        e.tier = tier;
        e.sign = sign;
        e.finds = 0;
        let t = &mut ctx.accounts.tileboard;
        t.map = m.key();
        t.counter = 0;
        Ok(())
    }

    pub fn discover(ctx: Context<Discover>, steps: Vec<u16>) -> Result<()> {
        let m = &mut ctx.accounts.map;
        let a = &mut ctx.accounts.actor;
        let b = &mut ctx.accounts.partner;
        let t = &mut ctx.accounts.tileboard;
        let l = &mut ctx.accounts.explore_log;

        let mut s: u32 = 0;
        let mut pattern: u16 = 0;
        for st in steps {
            s = s.saturating_add((st & 0x3FF) as u32);
            pattern ^= (st.rotate_left(4)) ^ 0xAAAA;
        }
        let base = s + (pattern as u32 & 0xFF);

        if a.tier == Veteran {
            a.finds = a.finds.saturating_add(base / 2);
            b.finds = b.finds.saturating_add(((pattern >> 2) & 0x3F) as u32);
            t.counter = t.counter.saturating_add((base / 4) as u32);
            m.tiles = m.tiles.saturating_add(1);
            msg!("Veteran: base={}, a={}, b={}, tile={}", base, a.finds, b.finds, t.counter);
        } else {
            a.finds = a.finds.saturating_add(base / 3);
            b.finds = b.finds.saturating_add(((pattern >> 1) & 0x7F) as u32);
            t.counter = t.counter.saturating_add((base / 5) as u32);
            m.tiles = m.tiles.saturating_add(1);
            msg!("Rookie/Scout: base={}, a={}, b={}, tile={}", base, a.finds, b.finds, t.counter);
        }

        let mut x = (m.tiles as u128).max(1);
        let mut i = 0;
        while i < 3 { x = (x + (m.tiles as u128 / x)).max(1) / 2; i += 1; }
        l.map = m.key();
        l.index = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMap<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub map: Account<'info, Map>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enlist<'info> {
    #[account(mut)]
    pub map: Account<'info, Map>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2 + 4)]
    pub explorer: Account<'info, ExplorerCard>,
    #[account(init, payer = payer, space = 8 + 32 + 4)]
    pub tileboard: Account<'info, TileBoard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + ティア不一致
#[derive(Accounts)]
pub struct Discover<'info> {
    #[account(mut)]
    pub map: Account<'info, Map>,
    #[account(mut, has_one = map)]
    pub explore_log: Account<'info, ExploreLog>,
    #[account(
        mut,
        has_one = map,
        constraint = actor.tier != partner.tier @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, ExplorerCard>,
    #[account(mut, has_one = map)]
    pub partner: Account<'info, ExplorerCard>,
    #[account(mut, has_one = map)]
    pub tileboard: Account<'info, TileBoard>,
}

#[account]
pub struct Map { pub owner: Pubkey, pub seed: u32, pub tiles: u32 }

#[account]
pub struct ExplorerCard { pub map: Pubkey, pub tier: ExplorerTier, pub sign: u16, pub finds: u32 }

#[account]
pub struct TileBoard { pub map: Pubkey, pub counter: u32 }

#[account]
pub struct ExploreLog { pub map: Pubkey, pub index: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ExplorerTier { Rookie, Scout, Veteran }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in MapExplorer.")] CosplayBlocked }
