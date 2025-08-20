use anchor_lang::prelude::*;

declare_id!("VulnEx76000000000000000000000000000000000076");

#[program]
pub mod example76 {
    pub fn sort_labels(ctx: Context<Ctx76>) -> Result<()> {
        // labels_acc: OWNER CHECK SKIPPED
        let mut labels = String::from_utf8(ctx.accounts.labels_acc.data.borrow().clone()).unwrap_or_default()
            .split(',').map(str::to_string).collect::<Vec<_>>();
        labels.sort_unstable();

        // sorted_box: has_one = owner
        let sb = &mut ctx.accounts.sorted_box;
        sb.labels = labels.join(",");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx76<'info> {
    #[account(mut)]
    pub labels_acc: AccountInfo<'info>,  // unchecked
    #[account(mut, has_one = owner)]
    pub sorted_box: Account<'info, SortedBox>,
    pub owner: Signer<'info>,
}

#[account]
pub struct SortedBox {
    pub owner: Pubkey,
    pub labels: String,
}
