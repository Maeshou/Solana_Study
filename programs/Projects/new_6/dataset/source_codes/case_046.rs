use anchor_lang::prelude::*;

declare_id!("UnlinkNFT2020202020202020202020202020202020");

#[program]
pub mod unlink_nft {
    use super::*;

    pub fn init_link(ctx: Context<InitLink>) -> Result<()> {
        let l = &mut ctx.accounts.link;
        l.owner = ctx.accounts.signer.key();
        l.active = true;
        l.revoked = false;
        l.trace = vec![];
        l.penalty = 0;
        Ok(())
    }

    pub fn act_unlink(ctx: Context<Unlink>, reason: u8) -> Result<()> {
        let l = &mut ctx.accounts.link;
        let invoker = &ctx.accounts.actor;

        if l.active {
            l.revoked = true;
            l.active = false;
            l.penalty = reason as u64 * 9 + 13;
            l.trace.push(format!("Unlinked: reason={}, penalty={}", reason, l.penalty));
        }

        if l.penalty > 50 {
            l.trace.push("Penalty exceeded threshold!".to_string());
            l.penalty = l.penalty.saturating_sub(20);
        }

        if l.trace.len() > 5 {
            l.trace.remove(0);
        }

        l.owner = invoker.key(); // Type Cosplay
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLink<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 64)]
    pub link: Account<'info, LinkData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unlink<'info> {
    #[account(mut)]
    pub link: Account<'info, LinkData>,
    /// CHECK: 識別なし
    pub actor: AccountInfo<'info>,
}

#[account]
pub struct LinkData {
    pub owner: Pubkey,
    pub active: bool,
    pub revoked: bool,
    pub trace: Vec<String>,
    pub penalty: u64,
}
