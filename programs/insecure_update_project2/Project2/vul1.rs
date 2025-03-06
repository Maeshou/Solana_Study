use anchor_lang::prelude::*;

#[program]
pub mod owner_check {
    use super::*;

    pub fn admin_instruction(ctx: Context<Unchecked>) -> Result<()> {
        let account_data = ctx.accounts.admin_config.try_borrow_data()?;
        let mut account_data_slice: &[u8] = &account_data;
        let account_state = AdminConfig::try_deserialize(&mut account_data_slice)?;

        // 脆弱性: admin_stateのadminキーはチェックされているが、
        // account_stateがこのプログラムによって所有されているかどうかは確認されていない。
        if account_state.admin != ctx.accounts.admin.key() {
            return Err(ProgramError::InvalidArgument.into());
        }

        msg!("Admin: {}", account_state.admin.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    pub admin_config: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminConfig {
    pub admin: Pubkey,
}
