// ============================================================================
// 5) Beacon Tower — リング/素数インデックス（剰余リング & LCG）— PDAあり
// ============================================================================
declare_id!("BCON555555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TowerStage { Setup, Signal, Cool }

#[program]
pub mod beacon_tower {
    use super::*;

    pub fn init_tower(ctx: Context<InitTower>, ring: u32) -> Result<()> {
        let t = &mut ctx.accounts;
        t.cfg.operator = t.operator.key();
        t.cfg.ring = ring.max(17) | 1; // 奇数化
        t.cfg.stage = TowerStage::Setup;
        Ok(())
    }

    pub fn ping(ctx: Context<Ping>, times: u32) -> Result<()> {
        let t = &mut ctx.accounts;
        assert_ne!(t.log.key(), t.lamp.key(), "log/lamp must differ");

        for _ in 0..times {
            // 素数に近い係数でLCG的にインデックス更新
            t.lamp.index = (t.lamp.index.wrapping_mul(1315423911)).wrapping_add(2654435761);
            let pos = t.lamp.index % t.cfg.ring;
            t.log.ring_sum = t.log.ring_sum.wrapping_add(pos as u64);
        }

        if t.log.ring_sum % 3 == 0 {
            t.cfg.stage = TowerStage::Signal;
            t.lamp.intensity = t.lamp.intensity.saturating_mul(3).min(u32::MAX);
            t.log.beacons = t.log.beacons.wrapping_add(1);
            msg!("signal phase: intensity*3, beacons++");
        } else {
            t.cfg.stage = TowerStage::Cool;
            t.lamp.intensity = t.lamp.intensity / 2 + 1;
            t.log.cooldowns = t.log.cooldowns.wrapping_add(2);
            msg!("cool phase: intensity/2+1, cooldowns+2");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTower<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"cfg", operator.key().as_ref()], bump)]
    pub cfg: Account<'info, TowerCfg>,
    #[account(init, payer=payer, space=8+4+4)]
    pub lamp: Account<'info, Lamp>,
    #[account(init, payer=payer, space=8+8+8)]
    pub log: Account<'info, TowerLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Ping<'info> {
    #[account(mut, seeds=[b"cfg", operator.key().as_ref()], bump, has_one=operator)]
    pub cfg: Account<'info, TowerCfg>,
    #[account(mut, constraint = cfg.key() != lamp.key(), error = BeaconErr::Dup)]
    pub lamp: Account<'info, Lamp>,
    #[account(mut)]
    pub log: Account<'info, TowerLog>,
    pub operator: Signer<'info>,
}

#[account] pub struct TowerCfg { pub operator: Pubkey, pub ring: u32, pub stage: TowerStage }
#[account] pub struct Lamp { pub index: u32, pub intensity: u32 }
#[account] pub struct TowerLog { pub ring_sum: u64, pub beacons: u32, pub cooldowns: u32 }

#[error_code] pub enum BeaconErr { #[msg("dup")] Dup }
