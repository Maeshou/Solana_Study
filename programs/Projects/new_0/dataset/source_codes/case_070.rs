use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfTitle01");

#[program]
pub mod activity_title_generator {
    use super::*;

    pub fn generate_title(ctx: Context<TitleContext>, activity_score: u64) -> Result<String> {
        let tier = activity_score / 100;
        let base = "Adventurer";
        let level = (tier + 1).to_string();
        let full_title = format!("{} Lv{}", base, level);
        msg!("Generated Title: {}", full_title);
        Ok(full_title)
    }
}

#[derive(Accounts)]
pub struct TitleContext<'info> {
    pub user: Signer<'info>,
}
