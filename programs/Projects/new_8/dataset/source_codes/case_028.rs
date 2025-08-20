use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ShAd0wSwAp11111111111111111111111111111");

#[program]
pub mod pda_shadow_swap {
    use super::*;

    // 安全: main_vault は #[account(seeds, bump)] で検証
    // 危険: "shadow" PDA を user 提供の bump で手動導出して検証に使う（Case 1）
    pub fn swap(ctx: Context<Swap>, amount: u64, shadow_bump: u8) -> Result<()> {
        // --- 手動 PDA（危険ポイント） ---
        let seeds = &[b"shadow", ctx.accounts.user.key.as_ref(), &[shadow_bump]];
        let shadow_addr = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(Errx::BadShadow))?;
        if shadow_addr != ctx.accounts.shadow_cell.key() {
            return Err(error!(Errx::BadShadow));
        }

        // --- 正常処理（例） ---
        if amount == 0 { return Ok(()); }
        let v = &mut ctx.accounts.main_vault;
        if v.balance < amount { v.flags = v.flags.saturating_add(1); }
        if v.balance >= amount { v.balance = v.balance.saturating_sub(amount); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, seeds=[b"vault", user.key().as_ref()], bump)]
    pub main_vault: Account<'info, Vault>,
    /// CHECK: 手動 bump で検証してしまう影響点
    pub shadow_cell: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub flags: u32,
}

#[error_code]
pub enum Errx { #[msg("shadow PDA mismatch")] BadShadow }
