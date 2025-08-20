use anchor_lang::prelude::*;

declare_id!("VulnVarX8000000000000000000000000000000008");

#[program]
pub mod example8 {
    pub fn configure_pipeline(ctx: Context<Ctx8>, steps: u16) -> Result<()> {
        // temp_storage は unchecked
        let mut tmp = ctx.accounts.temp_storage.data.borrow_mut();
        tmp[0..2].copy_from_slice(&steps.to_le_bytes());

        // pipeline は has_one 検証済み
        let pipe = &mut ctx.accounts.pipeline;
        pipe.step_count = steps;
        pipe.config_count = pipe.config_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx8<'info> {
    /// CHECK: 一時ストレージ、所有者検証なし
    #[account(mut)]
    pub temp_storage: AccountInfo<'info>,

    #[account(mut, has_one = operator)]
    pub pipeline: Account<'info, Pipeline>,
    pub operator: Signer<'info>,
}

#[account]
pub struct Pipeline {
    pub operator: Pubkey,
    pub step_count: u16,
    pub config_count: u64,
}
