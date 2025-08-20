use anchor_lang::prelude::*;

declare_id!("R5T9P8Q1V6U3W2Y7X4Z0A1B4C3D5E7F9G0H2I");

const MAX_BUFFER_SIZE: u64 = 1024 * 10; // 10KB
const MIN_RELAY_POWER_THRESHOLD: u64 = 500;
const POWER_COST_PER_PACKET: u64 = 50;

#[program]
pub mod orbital_relay {
    use super::*;

    pub fn init_relay(ctx: Context<InitRelay>, relay_id: u64, power_reserve: u64) -> Result<()> {
        let relay = &mut ctx.accounts.relay_core;
        relay.relay_id = relay_id << 2;
        relay.power_reserve = power_reserve;
        relay.total_packets_transferred = 0;
        relay.is_operational = relay.power_reserve > MIN_RELAY_POWER_THRESHOLD;
        msg!("Orbital Relay {} established with {} power.", relay.relay_id, relay.power_reserve);
        Ok(())
    }

    pub fn init_packet(ctx: Context<InitPacket>, packet_id: u64, data_size: u64) -> Result<()> {
        let packet = &mut ctx.accounts.data_packet;
        packet.parent_relay = ctx.accounts.relay_core.key();
        packet.packet_id = packet_id ^ 0x0123456789ABCDEF;
        packet.data_size = data_size;
        packet.is_processed = false;
        msg!("New data packet {} created with size {}.", packet.packet_id, packet.data_size);
        Ok(())
    }

    pub fn transfer_data(ctx: Context<TransferData>) -> Result<()> {
        let relay = &mut ctx.accounts.relay_core;
        let incoming_packet = &mut ctx.accounts.incoming_packet;
        let outgoing_packet = &mut ctx.accounts.outgoing_packet;

        if !relay.is_operational {
            return Err(ProgramError::Custom(1).into()); // Error: Relay is not operational
        }

        if incoming_packet.is_processed || outgoing_packet.is_processed {
            return Err(ProgramError::Custom(2).into()); // Error: One or both packets already processed
        }

        let total_size = incoming_packet.data_size.saturating_add(outgoing_packet.data_size);
        if total_size > MAX_BUFFER_SIZE {
            return Err(ProgramError::Custom(3).into()); // Error: Buffer capacity exceeded
        }

        relay.power_reserve = relay.power_reserve.saturating_sub(POWER_COST_PER_PACKET);
        incoming_packet.is_processed = true;
        outgoing_packet.is_processed = true;
        relay.total_packets_transferred = relay.total_packets_transferred.saturating_add(2);
        relay.is_operational = relay.power_reserve > MIN_RELAY_POWER_THRESHOLD;

        msg!("Transferred incoming packet {} and outgoing packet {}.", incoming_packet.packet_id, outgoing_packet.packet_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(relay_id: u64, power_reserve: u64)]
pub struct InitRelay<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8 + 1)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(packet_id: u64, data_size: u64)]
pub struct InitPacket<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 8 + 1)]
    pub data_packet: Account<'info, DataPacket>,
    #[account(mut)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferData<'info> {
    #[account(mut)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut, has_one = parent_relay)]
    pub incoming_packet: Account<'info, DataPacket>,
    #[account(mut, has_one = parent_relay)]
    pub outgoing_packet: Account<'info, DataPacket>,
    pub signer: Signer<'info>,
}

#[account]
pub struct RelayCore {
    relay_id: u64,
    power_reserve: u64,
    total_packets_transferred: u64,
    is_operational: bool,
}

#[account]
pub struct DataPacket {
    parent_relay: Pubkey,
    packet_id: u64,
    data_size: u64,
    is_processed: bool,
}