//! FlexSPI Configuration Block (FCB) for the Teensy 4.1 (8MB flash).
//!
//! Derived from the upstream teensy4-fcb crate, with the flash size updated
//! to match the Teensy 4.1's 8MB QSPI.

#![no_std]

use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
use imxrt_boot_gen::serial_flash::*;

pub use nor::ConfigurationBlock;

/// Instructions for the Winbond W25Q64JV
/// SPI flash memory controller
mod winbond {
    pub const FAST_READ_QUAD_IO: u8 = 0xEB;
    pub const READ_STATUS_REGISTER_1: u8 = 0x05;
    pub const WRITE_ENABLE: u8 = 0x06;
    pub const SECTOR_ERASE: u8 = 0x20;
    pub const PAGE_PROGRAM: u8 = 0x02;
    pub const CHIP_ERASE: u8 = 0x60;
}

use winbond::*;

//
// Sequences for lookup table
//

// Common LUT operands (FlexSPI uses bytes/cycles or address-bits for these fields).
const ADDRESS_BITS_24: u8 = 0x18;
const TRANSFER_SIZE_4B: u8 = 0x04;
const DUMMY_CYCLES_6: u8 = 0x06;

const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
    .instr(Instr::new(RADDR, Pads::Four, ADDRESS_BITS_24))
    .instr(Instr::new(DUMMY, Pads::Four, DUMMY_CYCLES_6))
    .instr(Instr::new(READ, Pads::Four, TRANSFER_SIZE_4B))
    .build();

const SEQ_READ_STATUS: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, READ_STATUS_REGISTER_1))
    .instr(Instr::new(READ, Pads::One, TRANSFER_SIZE_4B))
    .build();

const SEQ_WRITE_ENABLE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, WRITE_ENABLE))
    .build();

const SEQ_ERASE_SECTOR: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, SECTOR_ERASE))
    .instr(Instr::new(RADDR, Pads::One, ADDRESS_BITS_24))
    .build();

const SEQ_PAGE_PROGRAM: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, PAGE_PROGRAM))
    .instr(Instr::new(RADDR, Pads::One, ADDRESS_BITS_24))
    .instr(Instr::new(WRITE, Pads::One, TRANSFER_SIZE_4B))
    .build();

const SEQ_CHIP_ERASE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, CHIP_ERASE))
    .build();

//
// Lookup table
//

const LUT: LookupTable = LookupTable::new()
    .command(Command::Read, SEQ_READ)
    .command(Command::ReadStatus, SEQ_READ_STATUS)
    .command(Command::WriteEnable, SEQ_WRITE_ENABLE)
    .command(Command::EraseSector, SEQ_ERASE_SECTOR)
    .command(Command::PageProgram, SEQ_PAGE_PROGRAM)
    .command(Command::ChipErase, SEQ_CHIP_ERASE);

//
// Common FlexSPI configuration block
//

const COMMON_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(LUT)
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x01)
        .cs_setup_time(0x02)
        .column_address_width(ColumnAddressWidth::OtherDevices)
        .device_mode_configuration(DeviceModeConfiguration::Disabled)
        .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
        .flash_size(SerialFlashRegion::A1, 0x0080_0000)
        .serial_clk_freq(SerialClockFrequency::MHz60)
        .serial_flash_pad_type(FlashPadType::Quad);

//
// Final serial NOR configuration block
//

/// Value for the serial NOR FlexSPI configuration block.
pub const SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(COMMON_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4096)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);

/// The FlexSPI configuration block.
#[no_mangle]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".fcb")]
pub static FLEXSPI_CONFIGURATION_BLOCK: nor::ConfigurationBlock = SERIAL_NOR_CONFIGURATION_BLOCK;
