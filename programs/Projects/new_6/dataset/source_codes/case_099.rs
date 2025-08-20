use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("TyPeCoSpLaY11111111111111111111111111111");

#[program]
pub mod guild_power_gate {
    use super::*;

    /// 「管理者だけが通れる」つもりの処理
    pub fn use_admin_power(ctx: Context<UseAdminPower>, _flag: u8) -> Result<()> {
        // 1) 所有者だけは確認している（これだけでは役割の取り違えは防げない）
        if ctx.accounts.cfg.owner != crate::ID {
            return Err(ProgramError::IllegalOwner.into());
        }

        // 2) 先頭8バイトのディスクリミネータを確認せず、そのまま中身を Admin 側の型でデコード
        //    → PlayerCard とレイアウトが同じため素通りする
        let data = ctx.accounts.cfg.data.borrow();
        let admin_data = GuildAdmin::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // 3) データ中の鍵と署名者が一致すればOK…（PlayerCardの user を admin と誤解釈できる）
        require_keys_eq!(admin_data.admin, ctx.accounts.signer.key(), ProgramError::MissingRequiredSignature);

        // 本来は管理者専用の更新や送金などに到達しうる
        msg!("granted: {:?}", admin_data.admin);
        Ok(())
    }

    /// デモ用：任意の32Bを cfg に書き込む（実運用ではこんな関数は置かない）
    /// これで PlayerCard を書いておくと、上の use_admin_power が通ってしまう
    pub fn write_player_card(ctx: Context<WritePlayerCard>, key: Pubkey) -> Result<()> {
        // cfg はこのプログラム所有・十分なサイズが割り当て済みである前提
        let card = PlayerCard { user: key };
        let bytes = card.try_to_vec().map_err(|_| ProgramError::InvalidInstructionData)?;
        let mut data = ctx.accounts.cfg.data.borrow_mut();
        // シンプルに先頭から上書き（ディスクリミネータ等は置かない）
        for (i, b) in bytes.iter().enumerate() {
            data[i] = *b;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseAdminPower<'info> {
    /// CHECK: 手動デコードするため UncheckedAccount で受けている（ここが落とし穴）
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WritePlayerCard<'info> {
    /// CHECK: 同上。ディスクリミネータなしの“生Borsh”を書けてしまう
    #[account(mut)]
    pub cfg: UncheckedAccount<'info>,
}

/// レイアウト: Pubkey 1個
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct GuildAdmin {
    pub admin: Pubkey,
}

/// レイアウト: Pubkey 1個（＝上と同じ並び）
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct PlayerCard {
    pub user: Pubkey,
}
