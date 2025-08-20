use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpGldMgmt01");

#[program]
pub mod guild_management {
    use super::*;

    /// ギルドにメンバーを追加する  
    /// (`guild_account` の owner チェックを一切行っていないため、  
    ///  悪意あるユーザーが他人のギルドアカウントを指定し、  
    ///  勝手にメンバーを追加できます)
    pub fn add_member(ctx: Context<ModifyGuild>, new_member: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.guild_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0]         : u8   現在のメンバー数 M
        // [1..1+32*max] : Pubkey × max メンバー数分

        const MAX_MEMBERS: usize = 16;
        let required = 1 + 32 * MAX_MEMBERS;
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        // メンバー数とリスト部を分割
        let (count_slice, list) = data.split_at_mut(1);
        let current = count_slice[0] as usize;
        if current >= MAX_MEMBERS {
            return err!(ErrorCode::GuildFull);
        }

        // 新メンバーの Pubkey を次のスロットに書き込み
        let start = 1 + current * 32;
        list[current * 32..current * 32 + 32]
            .copy_from_slice(&new_member.to_bytes());

        // メンバー数をインクリメント
        count_slice[0] = (current + 1) as u8;

        msg!(
            "Added member {} to guild {} (now {} members)",
            new_member,
            acct.key(),
            current + 1
        );
        Ok(())
    }

    /// ギルドからメンバーを削除する  
    /// (`guild_account` の owner チェックを省略しているため、  
    ///  悪意あるユーザーが他人のギルドアカウントを指定し、  
    ///  勝手にメンバーを追放できます)
    pub fn remove_member(ctx: Context<ModifyGuild>, member: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.guild_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        const MAX_MEMBERS: usize = 16;
        let required = 1 + 32 * MAX_MEMBERS;
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        // 分割して数とリスト取得
        let (count_slice, list) = data.split_at_mut(1);
        let mut current = count_slice[0] as usize;
        let mut found = false;

        // リストを走査し、見つかった要素以降を左に詰める
        for i in 0..current {
            let offset = i * 32;
            let pk = Pubkey::new(&list[offset..offset + 32]);
            if pk == member {
                found = true;
                // shift remaining entries left
                for j in i..current-1 {
                    let src_off = (j+1)*32;
                    let dst_off = j*32;
                    list[dst_off..dst_off+32].copy_from_slice(&list[src_off..src_off+32]);
                }
                break;
            }
        }
        if !found {
            return err!(ErrorCode::MemberNotFound);
        }

        // 最後の空きスロットをゼロクリア
        let last_off = (current-1)*32;
        list[last_off..last_off+32].fill(0);

        // メンバー数をデクリメント
        count_slice[0] = (current - 1) as u8;

        msg!(
            "Removed member {} from guild {} (now {} members)",
            member,
            acct.key(),
            current - 1
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyGuild<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub guild_account: AccountInfo<'info>,

    /// 操作実行者の署名のみ検証
    pub operator: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("ギルドアカウントのデータ長が不足しています")]
    DataTooShort,
    #[msg("ギルドにこれ以上メンバーを追加できません")]
    GuildFull,
    #[msg("指定したメンバーが見つかりません")]
    MemberNotFound,
}
