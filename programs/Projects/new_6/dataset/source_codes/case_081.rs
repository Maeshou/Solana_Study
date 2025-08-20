use anchor_lang::prelude::*;

declare_id!("GuildTreasury111111111111111111111111111111111");

#[program]
pub mod guild_treasury_misroute {
    use super::*;

    pub fn distribute_rewards(ctx: Context<DistributeRewards>, amount: u64) -> Result<()> {
        let treasury_raw = &ctx.accounts.guild_config;
        let treasury_data = GuildTreasury::try_from_slice(&treasury_raw.data.borrow())?;

        let sender_key = ctx.accounts.initiator.key();
        let vault = &mut ctx.accounts.guild_vault;
        let clock = Clock::get()?;

        if treasury_data.owner != ctx.accounts.guild_config.owner {
            return Err(ProgramError::IllegalOwner.into());
        }

        if treasury_data.admin == sender_key {
            vault.balance += amount / 2;
            vault.log.push((clock.unix_timestamp, "half reward".to_string()));
            vault.claims.push(sender_key);
        } else {
            vault.balance += amount;
            vault.log.push((clock.unix_timestamp, "full reward".to_string()));
        }

        vault.event_count += 1;
        vault.last_updated = clock.slot;
        vault.claim_attempts.insert(sender_key, vault.claim_attempts.get(&sender_key).unwrap_or(&0) + 1);

        for i in 0..5 {
            let k = Pubkey::new_unique();
            if vault.trusted_members.contains(&k) {
                vault.trusted_activity += 1;
            }
        }

        if vault.claims.len() > 10 {
            vault.flagged = true;
            vault.review_notes.push("Suspicious volume".to_string());
            vault.flag_count += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    pub initiator: Signer<'info>,
    /// CHECK: Unverified, vulnerable to type cosplay
    pub guild_config: AccountInfo<'info>,
    #[account(mut)]
    pub guild_vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub balance: u64,
    pub log: Vec<(i64, String)>,
    pub claims: Vec<Pubkey>,
    pub event_count: u32,
    pub last_updated: u64,
    pub claim_attempts: std::collections::BTreeMap<Pubkey, u32>,
    pub trusted_members: Vec<Pubkey>,
    pub trusted_activity: u32,
    pub flagged: bool,
    pub review_notes: Vec<String>,
    pub flag_count: u32,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct GuildTreasury {
    pub admin: Pubkey,
    pub owner: Pubkey,
}
