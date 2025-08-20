use anchor_lang::prelude::*;

declare_id!("Conf1G1111111111111111111111111111111111");

#[program]
pub mod safe_account_info {
    use super::*;
    pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
        let config_info = &ctx.accounts.config;

        // ★★★ 安全性の核心部分 ★★★
        // AccountInfoのownerを手動で検証する
        if *config_info.owner != *ctx.program_id {
            return err!(ErrorCode::InvalidOwner);
        }

        // 検証後、データをデシリアライズして利用
        let mut config_data = Config::try_from_slice(&config_info.data.borrow())?;
        config_data.admin = new_admin;

        let mut data_slice = &mut config_info.data.borrow_mut()[..];
        config_data.try_serialize(&mut data_slice)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    /// CHECK: We perform a manual owner check inside the instruction.
    #[account(mut, has_one = admin)]
    pub config: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub creation_fee: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided account is not owned by the correct program.")]
    InvalidOwner,
}