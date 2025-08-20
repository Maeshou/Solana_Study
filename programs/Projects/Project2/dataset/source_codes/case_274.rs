use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ReferralSys30303030303030303030303030303030");

#[program]
pub mod referral_system {
    use super::*;

    pub fn register_code(ctx: Context<AdminAction>, code: String, uses: u64) -> Result<()> {
        ctx.accounts.sys.codes.insert(code, uses);
        Ok(())
    }

    pub fn consume_code(ctx: Context<ConsumeCode>, code: String) -> Result<()> {
        let sys = &mut ctx.accounts.sys;
        if let Some(remaining) = sys.codes.get_mut(&code) {
            *remaining = remaining.saturating_sub(1);
            sys.used_codes.push(code.clone());
            if *remaining == 0 {
                sys.codes.remove(&code);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut)]
    pub sys: Account<'info, ReferralData>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ConsumeCode<'info> {
    #[account(mut)]
    pub sys: Account<'info, ReferralData>,
    pub user: Signer<'info>,
}

#[account]
pub struct ReferralData {
    pub codes: BTreeMap<String, u64>,
    pub used_codes: Vec<String>,
}
