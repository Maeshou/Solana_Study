use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf999mvTWf");

#[program]
pub mod process_metrics_003 {
    use super::*;

    // 指定値とdataを掛け算し、その結果が一定しきい値を超えていたら data を 1 に、それ以外は 0 に設定する
    pub fn scale_and_flag(ctx: Context<Ctx003>, multiplier: u64, threshold: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let result = ctx.accounts.storage.data.saturating_mul(multiplier);
        let flag = if result > threshold { 1 } else { 0 };
        ctx.accounts.storage.data = flag;

        msg!(
            "Multiplied: {} * {} = {}, Threshold: {}, Flag set to: {}",
            ctx.accounts.storage.data,
            multiplier,
            result,
            threshold,
            flag
        );

        Ok(())
    }

    // dataが偶数なら2で割り、奇数なら3を加える（Collatz的処理）
    pub fn transform_data_pattern(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let current = ctx.accounts.storage.data;
        let updated = if current % 2 == 0 {
            current / 2
        } else {
            current + 3
        };
        ctx.accounts.storage.data = updated;
        msg!("Transformed {} to {}", current, updated);

        Ok(())
    }

    // 安全な最小値に強制変更（例: 7以下なら全て7に）
    pub fn enforce_minimum(ctx: Context<Ctx003>, minimum: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let current = ctx.accounts.storage.data;
        if current < minimum {
            ctx.accounts.storage.data = minimum;
            msg!("Value {} was below minimum {}, adjusted.", current, minimum);
        } else {
            msg!("Value {} meets minimum {}", current, minimum);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
