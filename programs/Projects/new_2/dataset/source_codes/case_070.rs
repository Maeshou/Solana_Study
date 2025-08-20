use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqDaoVote01");

#[program]
pub mod dao_voting {
    use super::*;

    /// 提案に対して賛否を投じる  
    /// （`vote_power_account` の owner チェックを省略しているため、  
    ///  攻撃者が他人の投票権アカウントを指定して、不正に投票権を行使できます）
    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        support: bool,    // true=賛成、false=反対
    ) -> Result<()> {
        // 生の AccountInfo で受け取っているため owner==program_id の検証がスキップ
        let power_acc = &ctx.accounts.vote_power_account.to_account_info();
        // lamports を「投票重み」としてそのまま使用
        let weight = **power_acc.lamports.borrow();

        // 提案結果をまとめる提案アカウントのデータ領域を直接操作
        let data = &mut ctx.accounts.proposal_account.data.borrow_mut();
        // ── データレイアウト想定 ──
        // [0..8]   u64  proposal_id
        // [8..16]  u64  total_support
        // [16..24] u64  total_against

        if data.len() < 24 {
            return err!(ErrorCode::DataTooShort);
        }

        // 各フィールドをスライスで切り出し
        let (id_slice, rest)        = data.split_at_mut(8);
        let (support_slice, against_slice) = rest.split_at_mut(8);

        // 提案ID が一致しない場合は拒否
        let stored_id = u64::from_le_bytes(id_slice.try_into().unwrap());
        if stored_id != proposal_id {
            return err!(ErrorCode::InvalidProposal);
        }

        // 賛否カウントを更新
        if support {
            let mut total = u64::from_le_bytes(support_slice.try_into().unwrap());
            total = total.saturating_add(weight);
            support_slice.copy_from_slice(&total.to_le_bytes());
        } else {
            let mut total = u64::from_le_bytes(against_slice.try_into().unwrap());
            total = total.saturating_add(weight);
            against_slice.copy_from_slice(&total.to_le_bytes());
        }

        msg!(
            "Voted on proposal {}: support={} weight={} by {}",
            proposal_id,
            support,
            weight,
            ctx.accounts.voter.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub proposal_account:     AccountInfo<'info>,

    /// CHECK: owner == program_id の検証を省略している投票権アカウント
    #[account(mut)]
    pub vote_power_account:   AccountInfo<'info>,

    /// 投票者が署名していることのみ検証
    pub voter:                Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("提案IDが一致しません")]
    InvalidProposal,
}
