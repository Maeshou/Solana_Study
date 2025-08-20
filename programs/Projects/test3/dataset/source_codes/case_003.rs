use anchor_lang::prelude::*;

declare_id!("Fbal3333333333333333333333333333333333333");

#[program]
pub mod insecure_reserve_update {
    use super::*;

    pub fn update_reserve(ctx: Context<UpdateReserve>, amount: u64) -> Result<()> {
        let reserve = &mut ctx.accounts.reserve_acc;
        // 条件演算でタグ付け
        let total = reserve.reserve.checked_add(amount).unwrap();
        reserve.reserve = total;
        reserve.comment = if total % 2 == 0 {
            "even_reserve".to_string()
        } else {
            "odd_reserve".to_string()
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateReserve<'info> {
    #[account(
        init_if_needed,
        payer = payer_key,
        space = 8 + 8 + 32 + 12,
        seeds = [b"reserve", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub reserve_acc: Account<'info, ReserveAccount3>,

    /// 権限チェックを省略
    pub user: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer_key: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReserveAccount3 {
    pub reserve: u64,
    pub comment: String,
    pub user: Pubkey,
}
