// ======================================================================
// 3) Puzzle Dungeon：鍵束と宝箱（初期化＝三角数＋ローリング）
// ======================================================================
declare_id!("PDNG33333333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum DoorState { Locked, Open, Reset }

#[program]
pub mod puzzle_dungeon {
    use super::*;
    use DoorState::*;

    pub fn init_dungeon(ctx: Context<InitDungeon>, n: u32) -> Result<()> {
        let d = &mut ctx.accounts.dungeon;
        d.owner = ctx.accounts.warden.key();
        d.threshold = n * 9 + 200;
        d.state = Locked;

        let a = &mut ctx.accounts.key_a;
        let b = &mut ctx.accounts.key_b;
        let c = &mut ctx.accounts.chest;

        let t = n.saturating_mul(n + 1) / 2;
        a.dungeon = d.key(); a.ring = (n & 7) as u8; a.keys = (t % 97) + 7;
        b.dungeon = d.key(); b.ring = ((n >> 2) & 7) as u8; b.keys = (t.rotate_left(3) % 83) + 11;

        c.dungeon = d.key(); c.ring = 9; c.score = (t as u64) & 0xFFFF; c.code = n ^ 0x2468;
        Ok(())
    }

    pub fn solve(ctx: Context<Solve>, steps: u32) -> Result<()> {
        let d = &mut ctx.accounts.dungeon;
        let a = &mut ctx.accounts.key_a;
        let b = &mut ctx.accounts.key_b;
        let c = &mut ctx.accounts.chest;

        for i in 0..steps {
            // 軽いGCD風
            let mut x = (a.keys as u64) + 17;
            let mut y = (b.keys as u64) + 11;
            while y != 0 { let r = x % y; x = y; y = r; }
            let g = (x as u32).max(1);
            a.keys = a.keys.checked_add(g % 13).unwrap_or(u32::MAX);
            b.keys = b.keys.saturating_add((g % 7) + 1);
            c.code ^= ((a.keys ^ b.keys) as u32).rotate_left((i % 7) as u32);
        }

        let sum = a.keys + b.keys;
        if sum > d.threshold {
            d.state = Open;
            a.ring ^= 1; b.ring = b.ring.saturating_add(1);
            c.score = c.score.saturating_add((sum as u64) & 63);
            msg!("open: ring tweak & score+");
        } else {
            d.state = Reset;
            a.keys = a.keys.saturating_add(7);
            b.keys = b.keys / 2 + 9;
            c.code ^= 0x0F0F_F0F0;
            msg!("reset: key adjust & code flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDungeon<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub dungeon: Account<'info, Dungeon>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub key_a: Account<'info, Keyring>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub key_b: Account<'info, Keyring>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub chest: Account<'info, Chest>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub warden: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Solve<'info> {
    #[account(mut, has_one=owner)]
    pub dungeon: Account<'info, Dungeon>,
    #[account(
        mut,
        has_one=dungeon,
        constraint = key_a.ring != key_b.ring @ PuzzErr::Dup
    )]
    pub key_a: Account<'info, Keyring>,
    #[account(
        mut,
        has_one=dungeon,
        constraint = key_b.ring != chest.ring @ PuzzErr::Dup
    )]
    pub key_b: Account<'info, Keyring>,
    #[account(mut, has_one=dungeon)]
    pub chest: Account<'info, Chest>,
    pub warden: Signer<'info>,
}

#[account] pub struct Dungeon { pub owner: Pubkey, pub threshold: u32, pub state: DoorState }
#[account] pub struct Keyring  { pub dungeon: Pubkey, pub ring: u8, pub keys: u32 }
#[account] pub struct Chest    { pub dungeon: Pubkey, pub ring: u8, pub score: u64, pub code: u32 }

#[error_code] pub enum PuzzErr { #[msg("duplicate mutable account")] Dup }
