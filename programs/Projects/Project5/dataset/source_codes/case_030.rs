// ============================================================================
// 2) Echo Studio — サウンドNFTのミックス（PDAあり; seeds + constraint + has_one）
// ============================================================================
declare_id!("ECHO222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MixMode { Draft, Live, Master }

#[program]
pub mod echo_studio {
    use super::*;

    pub fn init_studio(ctx: Context<InitStudio>, ceiling: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.studio.producer = a.producer.key();
        a.studio.ceiling = ceiling;
        a.studio.mode = MixMode::Live;
        Ok(())
    }

    pub fn mix(ctx: Context<Mix>, ticks: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        // 追加ガード
        assert_ne!(a.bus.key(), a.track_a.key(), "bus/track_a must differ");
        assert_ne!(a.bus.key(), a.track_b.key(), "bus/track_b must differ");

        for _ in 0..ticks {
            a.track_a.gain = a.track_a.gain.saturating_add(5);
            a.track_b.gain = a.track_b.gain.saturating_add(7);
            a.bus.sum = a.bus.sum.saturating_add(9);
        }

        if a.bus.sum > a.studio.ceiling as u64 {
            a.studio.mode = MixMode::Master;
            a.bus.peaks = a.bus.peaks.saturating_add(3);
            a.track_b.flags = a.track_b.flags.saturating_add(2);
            msg!("ceiling exceeded; mastering, peaks+3 flagsB+2");
        } else {
            a.studio.mode = MixMode::Live;
            a.bus.peaks = a.bus.peaks.saturating_add(1);
            a.track_a.flags = a.track_a.flags.saturating_add(1);
            msg!("live mixing; peaks+1 flagsA+1");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer=payer, space=8+32+4+1)]
    pub track_a: Account<'info, Track>,
    #[account(init, payer=payer, space=8+32+4+1)]
    pub track_b: Account<'info, Track>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"bus", producer.key().as_ref()], bump)]
    pub bus: Account<'info, MixBus>,
    #[account(mut)] pub payer: Signer<'info>,
    pub producer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mix<'info> {
    #[account(mut, has_one=producer)]
    pub studio: Account<'info, Studio>,
    #[account(mut, constraint = track_a.key() != track_b.key(), error = EchoErr::Dup)]
    pub track_a: Account<'info, Track>,
    #[account(mut)]
    pub track_b: Account<'info, Track>,
    #[account(mut, seeds=[b"bus", producer.key().as_ref()], bump)]
    pub bus: Account<'info, MixBus>,
    pub producer: Signer<'info>,
}

#[account] pub struct Studio { pub producer: Pubkey, pub ceiling: u32, pub mode: MixMode }
#[account] pub struct Track { pub owner: Pubkey, pub gain: u32, pub flags: u8 }
#[account] pub struct MixBus { pub sum: u64, pub peaks: u32 }

#[error_code] pub enum EchoErr { #[msg("dup")] Dup }