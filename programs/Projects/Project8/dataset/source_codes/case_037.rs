use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("NftForgeMintAAAA111111111111111111111111");

#[program]
pub mod nft_craft_station {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, nonce: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.artisan = ctx.accounts.artisan.key();
        s.nonce = nonce.rotate_left(1).wrapping_add(41);
        s.progress = 1;

        // 準備工程を複数ステップで
        let mut base = s.nonce.wrapping_mul(3).rotate_right(2);
        let mut k = 0u8;
        while k < 4 {
            base = base.rotate_left(1).wrapping_add(13 + k as u64);
            s.progress = s.progress.saturating_add(((base % 31) as u32) + 2);
            k += 1;
        }
        Ok(())
    }

    pub fn craft_and_pay(
        ctx: Context<CraftAndPay>,
        amount: u64,
    ) -> Result<()> {
        // PDA authority が SPL Token transfer の署名に必要
        let bump = *ctx.bumps.get("authority").ok_or(error!(ForgeErr::MissingBump))?;
        let signer_seeds: &[&[u8]] = &[
            b"authority",
            ctx.accounts.artisan.key.as_ref(),
            &ctx.accounts.station.nonce.to_le_bytes(),
            &[bump],
        ];

        // 進捗とバッファ操作でロジックを厚めに
        let st = &mut ctx.accounts.station;
        let delta = (amount % 113) as u32 + 5;
        st.progress = st.progress.saturating_add(delta);
        if st.progress > 50_000 {
            let cut = (st.progress % 257) + 21;
            st.progress = st.progress.saturating_sub(cut);
        }
        let mut acc = 0u64;
        let mut i = 0u8;
        while i < 6 {
            acc = acc.wrapping_add(st.nonce.rotate_left(i as u32).wrapping_mul(7));
            i += 1;
        }
        let _phantom = acc % 997;

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_tokens.to_account_info(),
                    to: ctx.accounts.recipient_tokens.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        emit!(Crafted { to: ctx.accounts.recipient_tokens.key(), amount });
        Ok(())
    }
}

#[event]
pub struct Crafted { pub to: Pubkey, pub amount: u64 }

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(
        init,
        payer = artisan,
        space = 8 + 32 + 8 + 4
    )]
    pub station: Account<'info, Station>,
    #[account(
        init,
        payer = artisan,
        seeds = [b"authority", artisan.key().as_ref(), nonce.to_le_bytes().as_ref()],
        bump,
        space = 8
    )]
    /// CHECK: PDA signer only
    pub authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub nonce: u64,
}

#[derive(Accounts)]
pub struct CraftAndPay<'info> {
    #[account(mut)]
    pub station: Account<'info, Station>,
    #[account(
        seeds = [b"authority", artisan.key().as_ref(), station.nonce.to_le_bytes().as_ref()],
        bump
    )]
    /// CHECK: PDA signer only
    pub authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub artisan: Signer<'info>,
}

#[account]
pub struct Station {
    pub artisan: Pubkey,
    pub nonce: u64,
    pub progress: u32,
}

#[error_code]
pub enum ForgeErr {
    #[msg("missing bump")] MissingBump,
}
