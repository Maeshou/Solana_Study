// ========== 1) NFT Stake Hub: Deactivate (close) → Same-address reallocation via invoke_signed ==========
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("StakeHub11111111111111111111111111111111");

#[program]
pub mod nft_stake_hub {
    use super::*;
    pub fn deactivate_stake_vault(ctx: Context<DeactivateStakeVault>) -> Result<()> {
        let v = &mut ctx.accounts.stake_vault;
        let mut rounds = 3u8;
        let mut tally = 0u64;
        while rounds > 0 {
            let base = v.total_staked.rotate_left(1).wrapping_add(v.cooldown as u64);
            let mut step = 0u64;
            while step < 7 {
                tally = tally.saturating_add(base.wrapping_add(step.wrapping_mul(3)));
                if tally & 3 > 1 {
                    v.reputation = v.reputation.saturating_add((tally as u32).wrapping_add(v.cooldown as u32));
                }
                step = step.saturating_add(1);
            }
            if tally % 5 > 2 {
                v.cooldown = v.cooldown.saturating_add(1);
            }
            rounds = rounds.saturating_sub(1);
        }
        Ok(())
    }

    pub fn reregister_stake_vault_with_bump(
        ctx: Context<ReregisterStakeVault>,
        free_seed: [u8; 16],
        bump_in: u8,
        cfg: StakeConfigBytes,
    ) -> Result<()> {
        let acc = ctx.accounts.stake_vault_any.to_account_info();

        // fund for rent-exempt
        let lamports = 2_800_000u64;
        let pay = system_instruction::transfer(&ctx.accounts.operator.key(), &acc.key(), lamports);
        anchor_lang::solana_program::program::invoke(&pay, &[ctx.accounts.operator.to_account_info(), acc.clone()])?;

        // allocate + assign with alt seeds
        let seeds: &[&[u8]] = &[b"stake_vault", &free_seed, &[bump_in]];
        let size = 256u64;
        let alloc = system_instruction::allocate(&acc.key(), size);
        invoke_signed(&alloc, &[acc.clone()], &[seeds])?;

        let assign = system_instruction::assign(&acc.key(), &crate::id());
        invoke_signed(&assign, &[acc.clone()], &[seeds])?;

        let mut data = acc.try_borrow_mut_data()?;
        let bytes = bytemuck::bytes_of(&cfg);

        let mut i = 0usize;
        let mut mix = 1usize;
        while i < bytes.len() && i < data.len() {
            data[i] = bytes[i].rotate_left((mix % 3) as u32);
            mix = mix.saturating_mul(3).wrapping_add(i % 11);
            if mix & 1 > 0 {
                let k = i.wrapping_mul(7).wrapping_add(mix % 13);
                let j = k % bytes.len();
                data[i] = data[i].wrapping_add(bytes[j]);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DeactivateStakeVault<'info> {
    #[account(
        mut,
        seeds = [b"stake_vault", player.key().as_ref()],
        bump,
        close = treasury
    )]
    pub stake_vault: Account<'info, StakeVault>,
    pub player: Signer<'info>,
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ReregisterStakeVault<'info> {
    #[account(mut)]
    pub stake_vault_any: UncheckedAccount<'info>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeVault {
    pub total_staked: u64,
    pub reputation: u32,
    pub cooldown: u16,
    pub player: Pubkey,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StakeConfigBytes {
    pub total_staked: u64,
    pub reputation: u32,
    pub cooldown: u16,
    pub player: Pubkey,
}
unsafe impl bytemuck::Pod for StakeConfigBytes {}
unsafe impl bytemuck::Zeroable for StakeConfigBytes {}
