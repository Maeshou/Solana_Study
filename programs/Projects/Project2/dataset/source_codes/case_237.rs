use anchor_lang::prelude::*;

declare_id!("ComboCnt0666666666666666666666666666666666");

#[program]
pub mod combo_counter {
    use super::*;

    pub fn count_combo(ctx: Context<CountCombo>, hits: [bool; 10]) -> Result<ComboResult> {
        let mut combo = 0u64;
        for &h in hits.iter() {
            combo = if h { combo + 1 } else { combo };
        }
        Ok(ComboResult { combo })
    }
}

#[derive(Accounts)]
pub struct CountCombo<'info> {
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ComboResult {
    pub combo: u64,
}
