use anchor_lang::prelude::*;

declare_id!("Vfund4444444444444444444444444444444444444");

#[program]
pub mod insecure_vault_adjust {
    use super::*;

    pub fn modify_vault(ctx: Context<ModifyVault>, new_amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        // ヒストリー操作を挟む
        let entry = format!("prev={}", vault.value);
        vault.history.push(entry);
        vault.value = new_amount;
        vault.status = format!("set_{}", new_amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyVault<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 8 + 32 + (4 + 100) + (4 + 100),
        seeds = [b"vault", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub vault_account: Account<'info, VaultAccount4>,

    /// 真の署名チェック欠如
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultAccount4 {
    pub value: u64,
    pub status: String,
    pub owner: Pubkey,
    pub history: Vec<String>,
}
