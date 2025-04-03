use anchor_lang::prelude::*;

#[program]
pub mod owner_check {
    use super::*;

    pub fn admin_instruction(ctx: Context<Unchecked>) -> Result<()> {
        let account_data = ctx.accounts.admin_config.try_borrow_data()?;
        let mut account_data_slice: &[u8] = &account_data;
        let account_state = AdminConfig::try_deserialize(&mut account_data_slice)?;

        // 脆弱性: admin_state の admin キーはチェックされているが、
        // account_state がこのプログラムによって所有されているかどうかは確認されていない。
        if account_state.admin != ctx.accounts.admin.key() {
            // false 側の処理：admin キーが一致しない場合に別関数を呼び出す
            handle_invalid_admin();
            return Err(ProgramError::InvalidArgument.into());
        } else {
            // true 側の処理：admin キーが一致している場合に別関数を呼び出す
            process_valid_admin();
        }

        Ok(())
    }

    // false 側の処理を担当する関数
    fn handle_invalid_admin() {
        msg!("Admin check failed: invalid admin key detected. Executing fallback routine.");
        // ここにさらに必要な処理を追加可能
    }

    // true 側の処理を担当する関数
    fn process_valid_admin() {
        msg!("Admin check succeeded: proceeding with valid admin operations.");
        // ここにさらに必要な関数呼び出しや処理を追加可能
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
