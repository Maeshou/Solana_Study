use anchor_lang::prelude::*;

declare_id!("77777777777777777777777777777777");

#[program]
pub mod init_registry {
    use super::*;

    pub fn register_item(
        ctx: Context<RegisterItem>,
        key: String,
        value: u64,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.key = key;
        registry.value = value;
        registry.count = registry.count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterItem<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryData>,
    pub registrar: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegistryData {
    pub key: String,
    pub value: u64,
    pub count: u64,
}
