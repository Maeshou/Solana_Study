// 04. Relic Socketing System — 装着者/鑑定士の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("R3l1cS0ck3tDDDD4444444444444444444444444444");

#[program]
pub mod relic_socketing {
    use super::*;

    pub fn init_relic(ctx: Context<InitRelic>, base: u16) -> Result<()> {
        let r = &mut ctx.accounts.relic;
        r.owner = ctx.accounts.owner.key();
        r.core_power = base;
        r.sockets = vec![0; 6];
        r.affinity = [0u8; 3];
        r.audit_notes = vec![];
        r.mode = 0;
        Ok(())
    }

    pub fn act_socket(ctx: Context<Socket>, gem: u8, times: u8, label: String) -> Result<()> {
        let r = &mut ctx.accounts.relic;
        let appraiser = &ctx.accounts.appraiser; // CHECKなし

        // ラベル反映
        r.audit_notes.push(format!("{}#{}", label, times));
        if r.audit_notes.len() > 10 {
            r.audit_notes.remove(0);
        }

        // ソケット処理
        let mut t = 0;
        while t < times {
            let slot = ((gem as usize) + t as usize) % r.sockets.len();
            let delta = ((gem as u16) << (t % 3)) ^ (r.core_power.rotate_left((t % 7) as u32));
            r.sockets[slot] = r.sockets[slot].wrapping_add((delta & 0xFF) as u8);

            // 親和性・モード副作用
            if r.sockets[slot] & 1 == 1 {
                r.affinity[(slot % 3)] = r.affinity[(slot % 3)].wrapping_add((gem ^ (t as u8)) & 0x1F);
                r.mode = r.mode.rotate_left(1);
            }
            if r.affinity[(slot % 3)] > 100 {
                r.affinity[(slot % 3)] = r.affinity[(slot % 3)] % 67;
                r.core_power = r.core_power.rotate_right(1);
            }
            t += 1;
        }

        // 大域調整
        let sum = r.sockets.iter().fold(0u32, |a, b| a + *b as u32);
        if sum % 9 == 0 {
            r.core_power = r.core_power.wrapping_add((sum % 257) as u16);
            r.sockets.rotate_left(1);
        }

        // Type Cosplay：鑑定士が所有者に
        r.owner = appraiser.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRelic<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 64 + 3 + 64 + 1)]
    pub relic: Account<'info, Relic>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Socket<'info> {
    #[account(mut)]
    pub relic: Account<'info, Relic>,
    /// CHECK: 鑑定士ロールの検証なし
    pub appraiser: AccountInfo<'info>,
}

#[account]
pub struct Relic {
    pub owner: Pubkey,
    pub core_power: u16,
    pub sockets: Vec<u8>,
    pub affinity: [u8; 3],
    pub audit_notes: Vec<String>,
    pub mode: u8,
}
