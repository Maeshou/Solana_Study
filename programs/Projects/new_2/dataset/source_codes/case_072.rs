use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqTourney01");

#[program]
pub mod tournament_entry {
    use super::*;

    /// トーナメント参加登録を行う  
    /// （`entry_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の参加リストアカウントを指定して、  
    ///  勝手に任意のプレイヤーを登録／滑り込ませることができます）
    pub fn register_participant(
        ctx: Context<RegisterParticipant>,
        new_player: Pubkey,      // 登録するプレイヤーの Pubkey
    ) -> Result<()> {
        let acct = &mut ctx.accounts.entry_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0]            : u8    現在の参加者数 N
        // [1..1+32*M]    : Pubkey × M (最大 M=64)
        const MAX_PARTICIPANTS: usize = 64;
        let required = 1 + 32 * MAX_PARTICIPANTS;
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        // 参加者数とリスト部を分割取得
        let (count_slice, list_slice) = data.split_at_mut(1);
        let current = count_slice[0] as usize;
        if current >= MAX_PARTICIPANTS {
            return err!(ErrorCode::TournamentFull);
        }

        // 新しい参加者 Pubkey を末尾に書き込み
        let offset = current * 32;
        list_slice[offset..offset + 32]
            .copy_from_slice(&new_player.to_bytes());

        // 参加者数をインクリメント
        count_slice[0] = (current + 1) as u8;

        msg!(
            "Player {} registered in tournament (total {} participants)",
            new_player,
            current + 1
        );
        Ok(())
    }

    /// トーナメントから参加解除する  
    /// （同様に owner チェックをしていないため、他人を勝手に追放できます）
    pub fn deregister_participant(
        ctx: Context<RegisterParticipant>,
        remove_player: Pubkey,   // 解除するプレイヤーの Pubkey
    ) -> Result<()> {
        let acct = &mut ctx.accounts.entry_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        const MAX_PARTICIPANTS: usize = 64;
        let required = 1 + 32 * MAX_PARTICIPANTS;
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        // 分割して数とリスト取得
        let (count_slice, list_slice) = data.split_at_mut(1);
        let mut current = count_slice[0] as usize;
        let mut found = false;

        // リスト走査：見つけた要素以降を前詰め
        for i in 0..current {
            let idx = i * 32;
            let pk = Pubkey::new(&list_slice[idx..idx + 32]);
            if pk == remove_player {
                found = true;
                for j in i..current - 1 {
                    let src = (j + 1) * 32;
                    let dst = j * 32;
                    list_slice[dst..dst + 32].copy_from_slice(&list_slice[src..src + 32]);
                }
                break;
            }
        }
        if !found {
            return err!(ErrorCode::PlayerNotFound);
        }
        // 最後のスロットをゼロクリア
        let last = (current - 1) * 32;
        list_slice[last..last + 32].fill(0);

        // 参加者数をデクリメント
        count_slice[0] = (current - 1) as u8;

        msg!(
            "Player {} deregistered from tournament (remaining {} participants)",
            remove_player,
            current - 1
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterParticipant<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub entry_account: AccountInfo<'info>,
    /// 申請者の署名のみ検証
    pub operator: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("エントリアカウントのデータが想定より短いです")]
    DataTooShort,
    #[msg("トーナメントの定員がいっぱいです")]
    TournamentFull,
    #[msg("指定したプレイヤーが見つかりません")]
    PlayerNotFound,
}
