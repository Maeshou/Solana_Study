use anchor_lang::prelude::*;

declare_id!("D9fJ6L3K2H5R8Q1T7Y4W0V5U2X1S3E6F9G8H0I");

#[program]
pub mod quantum_relay {
    use super::*;

    pub fn init_relay(ctx: Context<InitRelay>, relay_id: u64, max_packets: u32) -> Result<()> {
        let relay = &mut ctx.accounts.relay_core;
        relay.relay_id = relay_id ^ 0xABCDABCDABCDABCD;
        relay.max_packets = max_packets.checked_add(10).unwrap_or(u32::MAX);
        relay.total_packets_sent = 0;
        relay.is_online = true;
        msg!("Quantum Relay {} online with capacity for {} packets.", relay.relay_id, relay.max_packets);
        Ok(())
    }

    pub fn init_packet(ctx: Context<InitPacket>, packet_id: u64, priority: u8) -> Result<()> {
        let packet = &mut ctx.accounts.packet_data;
        packet.parent_relay = ctx.accounts.relay_core.key();
        packet.packet_id = packet_id.checked_add(999).unwrap_or(u64::MAX);
        packet.priority = priority.checked_add(10).unwrap_or(u8::MAX);
        packet.is_sent = false;
        packet.retries = 0;
        msg!("New packet {} created with priority {}.", packet.packet_id, packet.priority);
        Ok(())
    }

    pub fn transmit_packets(ctx: Context<TransmitPackets>, cycles: u32) -> Result<()> {
        let relay = &mut ctx.accounts.relay_core;
        let packet1 = &mut ctx.accounts.packet1;
        let packet2 = &mut ctx.accounts.packet2;
        let mut loop_counter = cycles;

        while loop_counter > 0 {
            // packet1の処理
            let transmission_successful1 = packet1.priority > 50 && relay.is_online;
            packet1.is_sent = transmission_successful1;
            packet1.retries = (transmission_successful1 == false) as u8;
            relay.total_packets_sent = relay.total_packets_sent.checked_add(transmission_successful1 as u64).unwrap_or(u64::MAX);

            // packet2の処理
            let transmission_successful2 = packet2.priority > 60 && relay.is_online;
            packet2.is_sent = transmission_successful2;
            packet2.retries = (transmission_successful2 == false) as u8;
            relay.total_packets_sent = relay.total_packets_sent.checked_add(transmission_successful2 as u64).unwrap_or(u64::MAX);

            // RelayCoreの状態更新
            relay.is_online = (packet1.retries + packet2.retries) < 5;

            loop_counter = loop_counter.checked_sub(1).unwrap_or(0);
        }

        msg!("Transmitted packets for {} cycles. Total packets sent: {}.", cycles, relay.total_packets_sent);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(relay_id: u64, max_packets: u32)]
pub struct InitRelay<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 1)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(packet_id: u64, priority: u8)]
pub struct InitPacket<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 1 + 1)]
    pub packet_data: Account<'info, PacketData>,
    #[account(mut)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct TransmitPackets<'info> {
    #[account(mut)]
    pub relay_core: Account<'info, RelayCore>,
    #[account(mut, has_one = parent_relay)]
    pub packet1: Account<'info, PacketData>,
    #[account(mut, has_one = parent_relay)]
    pub packet2: Account<'info, PacketData>,
    pub signer: Signer<'info>,
}

#[account]
pub struct RelayCore {
    relay_id: u64,
    max_packets: u32,
    total_packets_sent: u64,
    is_online: bool,
}

#[account]
pub struct PacketData {
    parent_relay: Pubkey,
    packet_id: u64,
    priority: u8,
    is_sent: bool,
    retries: u8,
}