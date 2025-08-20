use anchor_lang::prelude::*;

declare_id!("P8Z5Y7X4W1V0U9T2R6Q3E1F7G8H4J0K6L2M5N9");

#[program]
pub mod echoes_of_the_void {
    use super::*;

    pub fn init_void(ctx: Context<InitVoid>, void_seed: u64, max_echoes: u32) -> Result<()> {
        let void = &mut ctx.accounts.void_core;
        void.void_seed = void_seed.checked_sub(123456).unwrap_or(1);
        void.max_echoes = max_echoes.checked_mul(2).unwrap_or(u32::MAX);
        void.active_echoes = 0;
        void.void_state = VoidState::Initial;
        msg!("Void initialized with seed {}.", void.void_seed);
        Ok(())
    }

    pub fn init_echo(ctx: Context<InitEcho>, echo_id: u64, base_intensity: u32) -> Result<()> {
        let echo = &mut ctx.accounts.echo_data;
        echo.parent_void = ctx.accounts.void_core.key();
        echo.echo_id = echo_id.rotate_right(2);
        echo.intensity = base_intensity.checked_add(10).unwrap_or(u32::MAX);
        echo.life_span = 1000;
        echo.is_stable = true;
        msg!("New echo {} created with intensity {}.", echo.echo_id, echo.intensity);
        Ok(())
    }

    pub fn amplify_echoes(ctx: Context<AmplifyEchoes>, cycles: u32) -> Result<()> {
        let void = &mut ctx.accounts.void_core;
        let echo1 = &mut ctx.accounts.echo1;
        let echo2 = &mut ctx.accounts.echo2;
        let mut loop_counter = cycles;

        while loop_counter > 0 {
            // echo1の処理
            let intensity_gain1 = (void.void_seed as u32).checked_add(echo1.intensity).unwrap_or(u32::MAX);
            echo1.intensity = echo1.intensity.checked_add(intensity_gain1).unwrap_or(u32::MAX);
            echo1.life_span = echo1.life_span.checked_sub(1).unwrap_or(0);
            echo1.is_stable = echo1.life_span > 0 && echo1.intensity < 5000;

            // echo2の処理
            let intensity_gain2 = (void.void_seed as u32).checked_mul(2).unwrap_or(u32::MAX);
            echo2.intensity = echo2.intensity.checked_add(intensity_gain2).unwrap_or(u32::MAX);
            echo2.life_span = echo2.life_span.checked_sub(1).unwrap_or(0);
            echo2.is_stable = echo2.life_span > 0 && echo2.intensity < 8000;

            // VoidCoreの状態更新
            void.void_state = if echo1.is_stable && echo2.is_stable {
                VoidState::Amplifying
            } else {
                VoidState::Unstable
            };
            void.active_echoes = (echo1.is_stable as u32).checked_add(echo2.is_stable as u32).unwrap_or(0);

            loop_counter = loop_counter.checked_sub(1).unwrap_or(0);
        }

        msg!("Echoes amplified for {} cycles. Void state is now {:?}.", cycles, void.void_state);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(void_seed: u64, max_echoes: u32)]
pub struct InitVoid<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 4)]
    pub void_core: Account<'info, VoidCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(echo_id: u64, base_intensity: u32)]
pub struct InitEcho<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub echo_data: Account<'info, EchoData>,
    #[account(mut)]
    pub void_core: Account<'info, VoidCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct AmplifyEchoes<'info> {
    #[account(mut)]
    pub void_core: Account<'info, VoidCore>,
    #[account(mut, has_one = parent_void)]
    pub echo1: Account<'info, EchoData>,
    #[account(mut, has_one = parent_void)]
    pub echo2: Account<'info, EchoData>,
    pub signer: Signer<'info>,
}

#[account]
pub struct VoidCore {
    void_seed: u64,
    max_echoes: u32,
    active_echoes: u32,
    void_state: VoidState,
}

#[account]
pub struct EchoData {
    parent_void: Pubkey,
    echo_id: u64,
    intensity: u32,
    life_span: u32,
    is_stable: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoidState {
    Initial,
    Amplifying,
    Unstable,
}