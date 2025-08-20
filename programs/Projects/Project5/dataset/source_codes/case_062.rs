// ============================================================================
// 10) Echo Archive — 回響記録（PDAなし / has_one + channel不一致）
// ============================================================================
declare_id!("ECHA10101010101010101010101010101010101010101");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum EchoState { Warmup, Recording, Cooling }

#[program]
pub mod echo_archive {
    use super::*;
    use EchoState::*;

    pub fn init_archive(ctx: Context<InitArchive>, bound: u32) -> Result<()> {
        let e = &mut ctx.accounts;
        e.room.owner = e.owner.key();
        e.room.bound = bound;
        e.room.state = Warmup;

        e.mic.room = e.room.key(); e.mic.channel = 2;
        e.amp.room = e.room.key(); e.amp.channel = 3;
        e.log.room = e.room.key(); e.log.channel = 9;
        Ok(())
    }

    pub fn record(ctx: Context<Record>, frames: u32) -> Result<()> {
        let e = &mut ctx.accounts;

        for k in 0..frames {
            // 逐次平均（clamp）
            e.log.count = e.log.count.saturating_add(1);
            let cnt = e.log.count.max(1);
            let sample = (e.mic.level as u64) + (k as u64 * 5);
            let delta = sample as i128 - e.log.mean as i128;
            e.log.mean = (e.log.mean as i128 + delta / cnt as i128).max(0) as u64;

            // 増幅は段階クリップ
            let step = if e.amp.gain < 100 { 3 } else { 1 };
            e.amp.gain = e.amp.gain.saturating_add(step);
        }

        if e.log.mean > e.room.bound as u64 {
            e.room.state = Cooling;
            e.log.flags = e.log.flags.saturating_add(2);
            e.amp.gain = e.amp.gain / 2 + 6;
            e.mic.level = e.mic.level / 2 + 4;
            msg!("cooling: flags+2, damp mic/amp");
        } else {
            e.room.state = Recording;
            e.log.count = e.log.count.saturating_add(3);
            e.mic.level = e.mic.level.saturating_add(7);
            e.amp.gain = e.amp.gain.saturating_add(5);
            msg!("recording: count+3, mic+7, amp+5");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArchive<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub room: Account<'info, EchoRoom>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub mic: Account<'info, MicState>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub amp: Account<'info, AmpState>,
    #[account(init, payer=payer, space=8+32+1+8+4)]
    pub log: Account<'info, EchoLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(mut, has_one=owner)]
    pub room: Account<'info, EchoRoom>,
    #[account(mut, has_one=room, constraint = mic.channel != amp.channel @ EaErr::Dup)]
    pub mic: Account<'info, MicState>,
    #[account(mut, has_one=room, constraint = amp.channel != log.channel @ EaErr::Dup)]
    pub amp: Account<'info, AmpState>,
    #[account(mut, has_one=room)]
    pub log: Account<'info, EchoLog>,
    pub owner: Signer<'info>,
}

#[account] pub struct EchoRoom { pub owner: Pubkey, pub bound: u32, pub state: EchoState }
#[account] pub struct MicState { pub room: Pubkey, pub channel: u8, pub level: u32 }
#[account] pub struct AmpState { pub room: Pubkey, pub channel: u8, pub gain: u32 }
#[account] pub struct EchoLog { pub room: Pubkey, pub channel: u8, pub count: u64, pub mean: u64, pub flags: u32 }
#[error_code] pub enum EaErr { #[msg("dup")] Dup }