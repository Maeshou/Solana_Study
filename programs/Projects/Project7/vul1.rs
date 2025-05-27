#[program]
pub mod arbitrary_cpi_insecure {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>, amount: u64) -> ProgramResult {
        // 脆弱性: token_programがSPL Token Programであるかどうかを検証していない
        solana_program::program::invoke(
            &spl_token::instruction::transfer(
                ctx.accounts.token_program.key,
                ctx.accounts.source.key,
                ctx.accounts.destination.key,
                ctx.accounts.authority.key,
                &[],
                amount,
            )?,
            &[
                ctx.accounts.source.clone(),
                ctx.accounts.destination.clone(),
                ctx.accounts.authority.clone(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    pub source: UncheckedAccount<'info>,
    pub destination: UncheckedAccount<'info>,
    pub authority: UncheckedAccount<'info>,
    pub token_program: UncheckedAccount<'info>,
}