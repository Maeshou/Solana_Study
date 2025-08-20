use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("MaNuAlMiNt22222222222222222222222222222");

#[program]
pub mod pda_manual_mint {
    use super::*;

    pub fn mint_item(ctx: Context<MintItem>, qty: u16, bump_external: u8) -> Result<()> {
        // 危険: "mintSink" PDA を手動 bump で導出
        let seeds = &[b"mintSink", ctx.accounts.authority.key.as_ref(), &[bump_external]];
        let addr = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(MintErr::SinkMismatch))?;
        if addr != ctx.accounts.mint_sink.key() {
            return Err(error!(MintErr::SinkMismatch));
        }

        // 正常処理
        let p = &mut ctx.accounts.player_pool;
        if qty > 0 { p.minted = p.minted.saturating_add(qty as u32); }
        if qty > 500 { p.penalty = p.penalty.saturating_add(7); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintItem<'info> {
    #[account(mut, seeds=[b"pool", authority.key().as_ref()], bump)]
    pub player_pool: Account<'info, Pool>,
    /// CHECK: 危険な手動 PDA
    pub mint_sink: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Pool { pub owner: Pubkey, pub minted: u32, pub penalty: u32 }

#[error_code]
pub enum MintErr { #[msg("mint sink PDA mismatch")] SinkMismatch }
