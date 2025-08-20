use anchor_lang::prelude::*;
use std::collections::VecDeque;

declare_id!("OwnChkB5000000000000000000000000000000005");

#[program]
pub mod referral_system {
    pub fn redeem_code(ctx: Context<Redeem>, code: String) -> Result<()> {
        let sys = &mut ctx.accounts.sys;
        // has_one で owner チェック済み
        if let Some(count) = sys.codes.get_mut(&code) {
            *count = count.saturating_sub(1);
            sys.history.push_back((ctx.accounts.user.key(), code.clone()));
            if sys.history.len() > 50 {
                sys.history.pop_front();
            }
        }

        // history_acc は unchecked
        ctx.accounts.history_acc.data.borrow_mut().extend_from_slice(code.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut, has_one = owner)]
    pub sys: Account<'info, ReferralDataExt>,
    pub owner: Signer<'info>,
    pub user: Signer<'info>,
    /// CHECK: 履歴アカウント、所有者検証なし
    #[account(mut)]
    pub history_acc: AccountInfo<'info>,
}

#[account]
pub struct ReferralDataExt {
    pub owner: Pubkey,
    pub codes: std::collections::HashMap<String, u8>,
    pub history: VecDeque<(Pubkey, String)>,
}
