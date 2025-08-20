use anchor_lang::prelude::*;

declare_id!("VulnVarX2000000000000000000000000000000002");

#[program]
pub mod example2 {
    pub fn merge_collections(ctx: Context<Ctx2>) -> Result<()> {
        // scratch_acc は unchecked
        let mut scratch = ctx.accounts.scratch_acc.data.borrow_mut().to_vec();
        // 一時的にバイトを反転
        for b in scratch.iter_mut() { *b = !*b; }
        // merged_set は has_one 検証済み
        let set = &mut ctx.accounts.merged_set;
        set.items.extend(scratch.iter().map(|b| *b as u64));
        set.merge_count = set.merge_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx2<'info> {
    /// CHECK: 一時スクラッチ、所有者検証なし
    #[account(mut)]
    pub scratch_acc: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub merged_set: Account<'info, MergedSet>,
    pub owner: Signer<'info>,
}

#[account]
pub struct MergedSet {
    pub owner: Pubkey,
    pub items: Vec<u64>,
    pub merge_count: u64,
}
