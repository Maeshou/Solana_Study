use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqFriendGift01");

#[program]
pub mod nft_friend_gift_history {
    use super::*;

    /// フレンドを追加する  
    /// (`friend_list_account` の owner チェックを省略しているため、  
    ///  攻撃者が他人のフレンドリストを指定し、勝手に友達登録できます)
    pub fn add_friend(ctx: Context<ModifyFriendList>, new_friend: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.friend_list_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [0]         : u8   現在の友達数 N
        // [1..1+32*M] : Pubkey × M (最大 M=32)

        const MAX_FRIENDS: usize = 32;
        let required = 1 + 32 * MAX_FRIENDS;
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        let (cnt, list) = data.split_at_mut(1);
        let current = cnt[0] as usize;
        if current >= MAX_FRIENDS {
            return err!(ErrorCode::ListFull);
        }

        // 新しい Pubkey を末尾に書き込み
        let offset = current * 32;
        list[offset..offset + 32].copy_from_slice(&new_friend.to_bytes());
        cnt[0] = (current + 1) as u8;

        msg!(
            "Added friend {} to list {} (total {})",
            new_friend,
            acct.key(),
            current + 1
        );
        Ok(())
    }

    /// ギフト履歴にエントリを追加する  
    /// (`gift_history_account` の owner チェックを省略しているため、  
    ///  攻撃者が他人の履歴を指定し、任意の贈与記録を追加できます)
    pub fn record_gift(ctx: Context<ModifyGiftHistory>, recipient: Pubkey, item: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.gift_history_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        //  各ギフト記録は 32(recipient)+32(item)+8(ts) = 72 バイトずつ連続
        if data.len() < 72 {
            return err!(ErrorCode::DataTooShort);
        }

        // 末尾 72 バイト分を取得し上書き（古い記録は順次上書き）
        let start = data.len() - 72;
        let now = Clock::get()?.unix_timestamp as u64;
        data[start..start+32].copy_from_slice(recipient.as_ref());
        data[start+32..start+64].copy_from_slice(item.as_ref());
        data[start+64..start+72].copy_from_slice(&now.to_le_bytes());

        msg!(
            "Recorded gift of item {} to {} in history {} at {}",
            item,
            recipient,
            acct.key(),
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyFriendList<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub friend_list_account: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModifyGiftHistory<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub gift_history_account: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短すぎます")]
    DataTooShort,
    #[msg("フレンドリストが上限に達しています")]
    ListFull,
}
