use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuildTreasuryAAAA111111111111111111111111");

#[program]
pub mod guild_treasury {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, tier_seed: u64) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.leader = ctx.accounts.leader.key();
        g.tier = tier_seed.rotate_left(2).wrapping_add(17);
        g.power = 1;

        // 多少重めの初期化計算（固定値加算に偏らない）
        let mut rolling = g.tier.rotate_right(1).wrapping_add(9);
        let mut i = 0u8;
        while i < 5 {
            rolling = rolling.rotate_left(1).wrapping_mul(3).wrapping_add(11 + i as u64);
            let gain = (rolling % 23) as u32 + 1;
            g.power = g.power.saturating_add(gain);
            i += 1;
        }
        Ok(())
    }

    pub fn pay_from_guild(ctx: Context<PayFromGuild>, lamports: u64) -> Result<()> {
        // seeds/bump は Anchor の検証と同一の配列をそのまま再利用
        let bump = *ctx.bumps.get("vault").ok_or(error!(GuildErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"vault",
            ctx.accounts.leader.key.as_ref(),
            &ctx.accounts.guild.tier.to_le_bytes(),
            &[bump],
        ];

        // 送金前に複合的なメトリクス更新
        let g = &mut ctx.accounts.guild;
        let portion = (lamports % 97) as u32 + 3;
        g.power = g.power.saturating_add(portion);
        if g.power > 1_000_000 {
            let dec = (g.power % 997) + 13;
            g.power = g.power.saturating_sub(dec);
        }
        let mut steps = 0u8;
        while steps < 3 {
            g.power = g.power.saturating_add(((g.tier.rotate_left(steps as u32) % 29) as u32) + 2);
            steps += 1;
        }

        let ix = system_instruction::transfer(&ctx.accounts.vault.key(), &ctx.accounts.receiver.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(GuildPaid { to: ctx.accounts.receiver.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct GuildPaid { pub to: Pubkey, pub amount: u64 }

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(
        init,
        payer = leader,
        space = 8 + 32 + 8 + 4,
    )]
    pub guild: Account<'info, GuildState>,
    #[account(
        init,
        payer = leader,
        space = 8,
        seeds = [b"vault", leader.key().as_ref(), tier_seed.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub tier_seed: u64,
}

#[derive(Accounts)]
pub struct PayFromGuild<'info> {
    #[account(
        mut,
        seeds = [b"vault", leader.key().as_ref(), guild.tier.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub guild: Account<'info, GuildState>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub tier: u64,
    pub power: u32,
}

#[error_code]
pub enum GuildErr {
    #[msg("missing bump")] MissingBump,
}
