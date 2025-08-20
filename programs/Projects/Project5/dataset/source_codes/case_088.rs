use anchor_lang::prelude::*;

// ======================================================================
// 1) Arcade Kiosk：トークン販売機（初期化＝メータ先行→親→ホッパA/B、LCG & ビット混色）
// ======================================================================
declare_id!("ARCD111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ArcadeState { Offline, Serving, Locked }

#[program]
pub mod arcade_kiosk {
    use super::*;
    use ArcadeState::*;

    pub fn init_kiosk(ctx: Context<InitKiosk>, seed: u32) -> Result<()> {
        // 並びを崩す：メータ→親→子A→子B
        let m = &mut ctx.accounts.meter;
        m.arcade = ctx.accounts.arcade.key(); // 後で親を確定し直す
        m.lane = 9;
        m.dispensed = 0;
        m.entropy = seed.rotate_left(9) ^ 0xA55A_33CC;

        let a = &mut ctx.accounts.arcade;
        a.owner = ctx.accounts.operator.key();
        a.float = (seed as u64) * 3 + 500;
        a.state = Offline;

        m.arcade = a.key(); // 正しい親で上書き

        let mut s64 = (seed as u64)
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        let h1 = &mut ctx.accounts.hopper_a;
        h1.arcade = a.key();
        h1.chute = (s64 as u8) & 7;
        h1.coins = ((s64 & 255) as u32) + 80;

        s64 = s64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let h2 = &mut ctx.accounts.hopper_b;
        h2.arcade = a.key();
        h2.chute = ((s64 >> 3) as u8) & 7;
        h2.coins = (((s64 >> 8) & 255) as u32) + 75;

        Ok(())
    }

    pub fn vend(ctx: Context<Vend>, presses: u32) -> Result<()> {
        let a = &mut ctx.accounts.arcade;
        let h1 = &mut ctx.accounts.hopper_a;
        let h2 = &mut ctx.accounts.hopper_b;
        let m = &mut ctx.accounts.meter;

        for i in 0..presses {
            let mix = ((h1.coins ^ h2.coins) as u64).wrapping_mul(780291637);
            let take = ((mix & 7) + 1) as u32;
            h1.coins = h1.coins.saturating_sub(take.min(h1.coins));
            h2.coins = h2.coins.checked_add(take + (i & 3)).unwrap_or(u32::MAX);
            m.dispensed = m.dispensed.saturating_add(take as u64);
            m.entropy ^= (mix as u32).rotate_left((i % 11) as u32);
        }

        let total = h1.coins as u64 + h2.coins as u64 + m.dispensed;
        if total > a.float {
            a.state = Locked;
            h1.chute ^= 1;
            h2.chute = h2.chute.saturating_add(1);
            m.lane = m.lane.saturating_add(1);
            msg!("locked: lanes tweaked, meter lane++");
        } else {
            a.state = Serving;
            h1.coins = h1.coins.saturating_add(9);
            h2.coins = h2.coins / 2 + 11;
            m.entropy ^= 0x0F0F_F0F0;
            msg!("serving: coin adjust & entropy flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitKiosk<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub arcade: Account<'info, ArcadeCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub hopper_a: Account<'info, Hopper>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub hopper_b: Account<'info, Hopper>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub meter: Account<'info, Meter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vend<'info> {
    #[account(mut, has_one=owner)]
    pub arcade: Account<'info, ArcadeCore>,
    #[account(
        mut,
        has_one=arcade,
        constraint = hopper_a.chute != hopper_b.chute @ ArcadeErr::Dup
    )]
    pub hopper_a: Account<'info, Hopper>,
    #[account(
        mut,
        has_one=arcade,
        constraint = hopper_b.chute != meter.lane @ ArcadeErr::Dup
    )]
    pub hopper_b: Account<'info, Hopper>,
    #[account(mut, has_one=arcade)]
    pub meter: Account<'info, Meter>,
    pub operator: Signer<'info>,
}

#[account] pub struct ArcadeCore { pub owner: Pubkey, pub float: u64, pub state: ArcadeState }
#[account] pub struct Hopper { pub arcade: Pubkey, pub chute: u8, pub coins: u32 }
#[account] pub struct Meter { pub arcade: Pubkey, pub lane: u8, pub dispensed: u64, pub entropy: u32 }

#[error_code] pub enum ArcadeErr { #[msg("duplicate mutable account")] Dup }
