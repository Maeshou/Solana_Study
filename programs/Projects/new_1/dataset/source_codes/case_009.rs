use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfCHEST08");

#[program]
pub mod community_chest {
    use super::*;

    /// Community Chest に資金を積み増します（署名チェックを敢えて省略）
    pub fn fund_chest(ctx: Context<FundChest>, amount: u64) -> Result<()> {
        let chest = &mut ctx.accounts.chest_account;
        chest.total = chest.total.checked_add(amount).unwrap();
        msg!("Funded chest with {} tokens, new total = {}", amount, chest.total);
        Ok(())
    }

    /// Chest の資金を 2 名の受取人で均等分配します（署名チェックを敢えて省略）
    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let chest = &mut ctx.accounts.chest_account;
        // 2 人分に均等割り
        let half = chest.total.checked_div(2).unwrap();
        chest.total = chest.total.checked_sub(half.checked_mul(2).unwrap()).unwrap();

        // 受取人へ支払い
        // 本来は beneficiary1.is_signer, beneficiary2.is_signer が必要
        msg!("Paid {} tokens to {}", half, ctx.accounts.beneficiary1.key());
        msg!("Paid {} tokens to {}", half, ctx.accounts.beneficiary2.key());
        msg!("Remaining in chest: {}", chest.total);
        Ok(())
    }
}

#[account]
pub struct ChestAccount {
    pub manager: Pubkey,
    pub total:   u64,
    pub bump:    u8,
}

#[derive(Accounts)]
pub struct FundChest<'info> {
    #[account(
        mut,
        seeds = [b"chest", manager.key().as_ref()],
        bump = chest_account.bump,
        has_one = manager @ ErrorCode::Unauthorized
    )]
    pub chest_account: Account<'info, ChestAccount>,

    /// 本来は manager.is_signer が必要
    #[account(mut)]
    pub manager:       AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(
        mut,
        seeds = [b"chest", manager.key().as_ref()],
        bump = chest_account.bump,
        has_one = manager @ ErrorCode::Unauthorized
    )]
    pub chest_account: Account<'info, ChestAccount>,

    /// 本来は beneficiary1.is_signer が必要
    #[account(mut)]
    pub beneficiary1:  AccountInfo<'info>,

    /// 本来は beneficiary2.is_signer が必要
    #[account(mut)]
    pub beneficiary2:  AccountInfo<'info>,

    /// 本来は manager.is_signer が必要
    #[account(mut)]
    pub manager:       AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
