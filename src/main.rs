use std::{
    fmt::Display,
    //ops::Range,
    thread::sleep,
    time::{Duration, Instant},
};

use bitflags::bitflags;
use log::{debug, trace};
use rand::Rng;

use clap::Parser;
use sdl2::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Texture, WindowCanvas},
};

use std::path;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(value_name = "cartridge")]
    cartridge: Option<path::PathBuf>,

    #[arg(long, short, default_value_t = 100)]
    clock_micros: u64,
}

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GRAY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn string_to_err(s: String) -> anyhow::Error {
    anyhow::anyhow!(s)
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let sdl_context = sdl2::init().map_err(string_to_err)?;
    let video_subsystem = sdl_context.video().map_err(string_to_err)?;
    let window = video_subsystem
        .window("Snake Game", 32 * 10, 32 * 10)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_scale(10.0, 10.0).map_err(string_to_err)?;
    //let mut event_pump = sdl_context.event_pump().map_err(string_to_err)?;
    let creator = canvas.texture_creator();
    let texture = creator.create_texture_target(PixelFormatEnum::RGB24, 32, 32)?;
    let event_pump = sdl_context.event_pump().map_err(string_to_err)?;
    let cli = Cli::parse();
    /*let test_code = vec![
        0xa9, 0x03, 0x4c, 0x08, 0x06, 0x00, 0x00, 0x00, 0x8d, 0x00, 0x02, 0xFF
    ];*/
    let test_code = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02,
        0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9,
        0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85,
        0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20,
        0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c,
        0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0,
        0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85,
        0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0, 0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01,
        0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05,
        0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00,
        0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20,
        0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11,
        0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60,
        0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
        0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9, 0x20,
        0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10,
        0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5,
        0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe,
        0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10,
        0x60, 0xa2, 0x00, 0xea, 0xea, 0xca, 0xd0, 0xfb, 0x60, 0xFF,
    ];
    /*let test_code = vec![
        0x20, 0x09, 0x06, 0x20, 0x0c, 0x06, 0x20, 0x12, 0x06, 0xa2, 0x00, 0x60, 0xe8, 0xe0, 0x05, 0xd0, 0xfb, 0x60, 0xFF
    ];*/
    //let test_code = vec![0xa5, 0xfe, 0xa2, 0x0c, 0xFF ];
    let mut machine = Machine::new(cli.clock_micros, texture, canvas, event_pump);
    machine.load_jmp(0x0600, &test_code)?;
    machine.boot()?;
    machine.reset();
    //machine.dump_memory(0..0x600)?;

    Ok(())
}

enum Operand {
    Value(u8),
    Address(usize),
}

const MEMORY_SIZE: usize = 2048;
struct Machine<'a> {
    running: bool,
    clk: Duration,
    display_buffer: [u8; 32 * 3 * 32],
    display_dirty: bool,
    event_pump: EventPump,
    acc: u8,
    x: u8,
    y: u8,
    flags: Flags,
    sp: usize,
    pc: usize,
    bpc: usize,
    texture: Texture<'a>,
    canvas: WindowCanvas,
    memory: [u8; MEMORY_SIZE],
}

const STACK: usize = 0x100;
const BIT7: u8 = 0x80;
const BIT6: u8 = 0x40;
const BIT0: u8 = 0x01;

bitflags! {
    pub struct Flags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

const IRQ_VECTOR: usize = 0xFFFE;

impl<'a> Machine<'a> {
    fn new(
        clk_micros: u64,
        texture: Texture<'a>,
        canvas: WindowCanvas,
        event_pump: EventPump,
    ) -> Self {
        Machine {
            running: false,
            clk: Duration::from_micros(clk_micros),
            display_buffer: [0; 32 * 3 * 32],
            display_dirty: false,
            event_pump,
            acc: 0,
            x: 0,
            y: 0,
            flags: Flags::empty(),
            sp: 0xff,
            pc: 0,
            bpc: 0,
            texture,
            canvas,
            memory: [0; MEMORY_SIZE],
        }
    }

    /*
    fn dump_memory(&self, r: Range<usize>) -> anyhow::Result<()> {
        let per_row = 16;
        let mut row_cursor = 0;
        for i in r {
            if row_cursor == 0 {
                print!("{i:0>4x}: ");
            }
            let byte = self.read_memory(i)?;
            print!("{:0>2x} ", byte);
            row_cursor += 1;
            if row_cursor == per_row {
                row_cursor = 0;
                println!();
            }
        }

        Ok(())
    }
    */

    fn reset(&mut self) {
        self.x = 0;
        self.acc = 0;
        self.display_buffer = [0; 32 * 3 * 32];
        self.display_dirty = true;
        self.sp = 0xff;
        self.pc = 0x0;
        self.bpc = 0x0;
        self.memory = [0; MEMORY_SIZE]
    }

    fn display(&mut self) -> anyhow::Result<()> {
        if !self.display_dirty {
            return Ok(());
        }
        let mut frame_i = 0;
        for i in 0x200..0x600 {
            let mem_val = self.read_memory(i)?;
            let (r, g, b) = color(mem_val).rgb();
            self.display_buffer[frame_i] = r;
            self.display_buffer[frame_i + 1] = g;
            self.display_buffer[frame_i + 2] = b;
            frame_i += 3;
        }
        self.texture.update(None, &self.display_buffer, 32 * 3)?;
        self.canvas
            .copy(&self.texture, None, None)
            .map_err(string_to_err)?;
        self.canvas.present();
        trace!("[display] buffer displayed");
        self.display_dirty = false;
        Ok(())
    }

    fn handle_key(&mut self) -> anyhow::Result<()> {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => self.write_memory(0xff, 0x77)?,
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => self.write_memory(0xff, 0x73)?,
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => self.write_memory(0xff, 0x61)?,
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => self.write_memory(0xff, 0x64)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn set_acc(&mut self, value: u8) {
        self.acc = value;
        self.update_zero_and_negative_flags(self.acc);
    }

    fn set_x(&mut self, value: u8) {
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
    }

    fn set_y(&mut self, value: u8) {
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
    }

    #[inline]
    fn is_carry(&self) -> bool {
        self.flags.contains(Flags::CARRY)
    }

    #[inline]
    fn set_carry(&mut self) {
        self.flags.insert(Flags::CARRY);
    }

    #[inline]
    fn cls_carry(&mut self) {
        self.flags.remove(Flags::CARRY);
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.flags.contains(Flags::ZERO)
    }

    #[inline]
    fn set_zero(&mut self) {
        self.flags.insert(Flags::ZERO)
    }

    #[inline]
    fn cls_zero(&mut self) {
        self.flags.remove(Flags::ZERO)
    }

    //fn is_interrupt_disable(&self) -> bool {
    //    self.p & INTERRUPT_DISABLE_BIT != 0
    //}

    #[inline]
    fn set_interrupt_disable(&mut self) {
        self.flags.insert(Flags::INTERRUPT_DISABLE);
    }

    #[inline]
    fn cls_interrupt_disable(&mut self) {
        self.flags.remove(Flags::INTERRUPT_DISABLE);
    }

    #[inline]
    fn set_decimal(&mut self) {
        self.flags.insert(Flags::DECIMAL);
    }

    #[inline]
    fn cls_decimal(&mut self) {
        self.flags.remove(Flags::DECIMAL);
    }

    /*fn is_break(&self) -> bool {
        self.p & BREAK_BIT != 0
    }*/

    /*fn set_break(&mut self) {
        self.p |= BREAK_BIT;
    }*/

    /*fn cls_break(&mut self) {
        self.p &= BREAK_MASK;
    }*/

    #[inline]
    fn is_overflow(&self) -> bool {
        self.flags.contains(Flags::OVERFLOW)
    }

    #[inline]
    fn set_overflow(&mut self) {
        self.flags.insert(Flags::OVERFLOW);
    }

    #[inline]
    fn cls_overflow(&mut self) {
        self.flags.remove(Flags::OVERFLOW);
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.flags.contains(Flags::NEGATIVE)
    }

    #[inline]
    fn set_negative(&mut self) {
        self.flags.insert(Flags::NEGATIVE);
    }

    #[inline]
    fn cls_negative(&mut self) {
        self.flags.remove(Flags::NEGATIVE);
    }

    fn write_memory(&mut self, addr: usize, value: u8) -> anyhow::Result<()> {
        if self.check_addr(addr) {
            if (0x200..0x600).contains(&addr) && self.memory[addr] != value {
                trace!("[display] display buffer dirty");
                self.display_dirty = true;
            }
            self.memory[addr] = value;
            Ok(())
        } else {
            anyhow::bail!("write memory overflow addr: {:x}", addr);
        }
    }

    fn read_memory(&self, addr: usize) -> anyhow::Result<u8> {
        match addr {
            0xFE => Ok(rand::rng().random_range(1..16)),
            other => {
                if self.check_addr(other) {
                    Ok(self.memory[other])
                } else {
                    anyhow::bail!("get memory overflow addr:{:x}", other);
                }
            }
        }
    }

    fn read_memory_u16(&mut self, addr: usize) -> anyhow::Result<u16> {
        let lsb = self.read_memory(addr)?;
        let msb_addr = addr
            .checked_add(1)
            .ok_or(anyhow::anyhow!("invalid addr in get memory u16"))?;
        let msb = self.read_memory(msb_addr)?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }

    #[inline]
    fn check_addr(&self, addr: usize) -> bool {
        addr < MEMORY_SIZE
    }

    /*fn store_flag(&mut self) -> anyhow::Result<()> {
        self.stack_push(self.p)
    }*/

    #[inline]
    fn store_flag_with(&mut self, flags: Flags) -> anyhow::Result<()> {
        self.stack_push(self.flags.bits() | flags.bits())
    }

    fn restore_flag(&mut self) -> anyhow::Result<()> {
        let bits = self.stack_pop()?;
        self.flags = Flags::from_bits_truncate(bits);
        Ok(())
    }

    fn store_pc(&mut self) -> anyhow::Result<()> {
        self.stack_push_u16(self.pc as u16)
    }

    fn restore_pc(&mut self) -> anyhow::Result<()> {
        let pc = self.stack_pop_u16()?;
        self.pc = pc as usize;
        self.bpc = pc as usize;
        Ok(())
    }

    fn stack_push(&mut self, value: u8) -> anyhow::Result<()> {
        if self.sp > 0 {
            self.write_memory(STACK + self.sp, value)?;
            self.sp -= 1;
            Ok(())
        } else {
            anyhow::bail!("unable to push stack due to memory overflow");
        }
    }

    fn stack_pop(&mut self) -> anyhow::Result<u8> {
        if self.sp < 0xFF {
            self.sp += 1;
            self.read_memory(STACK + self.sp)
        } else {
            anyhow::bail!("stack pointer overflow {}", self.sp);
        }
    }

    fn stack_push_u16(&mut self, value: u16) -> anyhow::Result<()> {
        self.stack_push(((value >> 8) & 0xFF) as u8)?;
        self.stack_push((value & 0xFF) as u8)?;
        Ok(())
    }

    fn stack_pop_u16(&mut self) -> anyhow::Result<u16> {
        let lsb = self.stack_pop()?;
        let msb = self.stack_pop()?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }

    fn load(&mut self, addr: usize, data: &[u8]) -> anyhow::Result<()> {
        if addr + data.len() >= MEMORY_SIZE {
            anyhow::bail!("insufficient memory for loading");
        }
        self.memory[addr..addr + data.len()].clone_from_slice(data);
        Ok(())
    }

    fn load_jmp(&mut self, addr: usize, data: &[u8]) -> anyhow::Result<()> {
        self.load(addr, data)?;
        self.bpc = addr;
        self.pc = addr;
        Ok(())
    }

    fn boot(&mut self) -> anyhow::Result<()> {
        self.running = true;
        let mut cycle_start;
        while self.running {
            cycle_start = Instant::now();
            match parse_opcode(self)? {
                Some(op) => {
                    debug!("{:x}: {}", self.bpc, op);
                    let status = self.step(op)?;
                    debug!(
                        "- acc:{:x}, x:{:x}, y:{:x}, sp:{:x}, p:{:0>8b} -",
                        self.acc,
                        self.x,
                        self.y,
                        self.sp,
                        self.flags.bits()
                    );
                    if matches!(status, Status::Halt) {
                        return Ok(());
                    }
                    self.display()?;
                    self.handle_key()?;
                    let elapsed = cycle_start.elapsed();
                    if elapsed < self.clk {
                        sleep(self.clk - elapsed);
                    }
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }

    fn goto(&mut self, addr: usize) -> anyhow::Result<()> {
        if self.check_addr(addr) {
            self.pc = addr;
            self.bpc = addr;
            Ok(())
        } else {
            anyhow::bail!("Invalid goto addr {}", addr);
        }
    }

    fn carry_bit(&self) -> u8 {
        if self.is_carry() { 1 } else { 0 }
    }

    fn advance(&mut self) {
        self.bpc = self.pc;
    }

    fn get_operand(&self, mode: AddressingMode) -> anyhow::Result<Operand> {
        use AddressingMode::*;
        use Operand::*;
        let val = match mode {
            Immediate(n) => Value(n),
            ZeroPage(a, idx) => match idx {
                Index::None => Address(a as usize),
                Index::X => Address(
                    (a as usize)
                        .checked_add(self.x as usize)
                        .ok_or(anyhow::anyhow!("invalid ZeroPage, X"))?,
                ),
                Index::Y => Address(
                    (a as usize)
                        .checked_add(self.y as usize)
                        .ok_or(anyhow::anyhow!("invalid ZeroPage, Y"))?,
                ),
            },
            Relative(ra) => Address(
                self.pc
                    .checked_add_signed(ra as isize)
                    .ok_or(anyhow::anyhow!("failed to calc relative address"))?,
            ),
            Absolute(a, idx) => match idx {
                Index::None => Address(a as usize),
                Index::X => Address(
                    (a as usize)
                        .checked_add(self.x as usize)
                        .ok_or(anyhow::anyhow!("failed to calc Absolute, X"))?,
                ),
                Index::Y => Address(
                    (a as usize)
                        .checked_add(self.y as usize)
                        .ok_or(anyhow::anyhow!("failed to calc Absolute, X"))?,
                ),
            },
            Indirect(a) => {
                let msb_addr = (a as usize)
                    .checked_add(1)
                    .ok_or(anyhow::anyhow!("invalid Indirect {}", a))?;
                let lsb = self.read_memory(a as usize)?;
                let msb = self.read_memory(msb_addr)?;
                Address(u16::from_le_bytes([lsb, msb]) as usize)
            }
            IndexedIndirect(a) => {
                let addr = a.wrapping_add(self.x) as usize;
                let addr_msb = addr
                    .checked_add(1)
                    .ok_or(anyhow::anyhow!("invalid indexedIndirect {}", addr))?;
                let lsb = self.read_memory(addr)?;
                let msb = self.read_memory(addr_msb)?;
                Address(u16::from_le_bytes([lsb, msb]) as usize)
            }
            IndirectIndexed(a) => {
                let a_msb = (a as usize)
                    .checked_add(1)
                    .ok_or(anyhow::anyhow!("invalid IndirectIndexed {}", a))?;
                let addr_lsb = self.read_memory(a as usize)?;
                let addr_msb = self.read_memory(a_msb)?;
                let addr = u16::from_le_bytes([addr_lsb, addr_msb]) as usize;
                Address(
                    addr.checked_add(self.y as usize)
                        .ok_or(anyhow::anyhow!("invalid IndirectIndexed {}", addr))?,
                )
            }
            _ => anyhow::bail!("unsupported addressing mode"),
        };
        Ok(val)
    }

    fn get_operand_value(&self, mode: AddressingMode) -> anyhow::Result<u8> {
        let operand = self.get_operand(mode)?;
        use Operand::*;
        match operand {
            Address(addr) => {
                if addr < MEMORY_SIZE {
                    Ok(self.read_memory(addr)?)
                } else {
                    anyhow::bail!("memory overflow {}", addr);
                }
            }
            Value(v) => Ok(v),
        }
    }

    fn update_zero_and_negative_flags(&mut self, val: u8) {
        if val == 0 {
            self.set_zero();
        } else {
            self.cls_zero();
        }
        // N
        if is_negative(val) {
            self.set_negative();
        } else {
            self.cls_negative();
        }
    }

    fn step(&mut self, op: Operation) -> anyhow::Result<Status> {
        use AddressingMode::*;
        use Operand::*;
        use Operation::*;
        match op {
            Adc(mode) => {
                let mem_val = self.get_operand_value(mode)?;
                let acc = self.acc;
                let carry = self.carry_bit();
                let (result, overflow) = acc.overflowing_add(mem_val);
                let (result_c, overflow_c) = result.overflowing_add(carry);
                self.set_acc(result_c);
                if overflow || overflow_c {
                    self.set_carry();
                } else {
                    self.cls_carry();
                }
                let result_sign = sign_bit(result_c);
                if result_sign != sign_bit(mem_val) && result_sign != sign_bit(acc) {
                    self.set_overflow();
                } else {
                    self.cls_overflow();
                }
                self.advance();
            }
            And(mode) => {
                self.set_acc(self.acc & self.get_operand_value(mode)?);
                self.advance();
            }
            Asl(mode) => {
                if let Accumulator = mode {
                    let sign_bit = sign_bit(self.acc);
                    self.set_acc(self.acc << 1);
                    if sign_bit != 0 {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                } else if let Address(addr) = self.get_operand(mode)? {
                    let mem_val = self.read_memory(addr)?;
                    if is_negative(mem_val) {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    let new_val = mem_val << 1;
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Asl instruction");
                }
                self.advance();
            }
            Bit(mode) => {
                let mem_val = self.get_operand_value(mode)?;
                let result = self.acc & mem_val;
                if result == 0 {
                    self.set_zero();
                } else {
                    self.cls_zero();
                }
                if is_negative(mem_val) {
                    self.set_negative();
                } else {
                    self.cls_negative();
                }
                if mem_val & BIT6 == BIT6 {
                    self.set_overflow();
                } else {
                    self.cls_overflow();
                }
                self.advance();
            }
            Bpl(mode) => {
                if !self.is_negative() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bpl");
                    }
                } else {
                    self.advance();
                }
            }
            Bmi(mode) => {
                if self.is_negative() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Nmi");
                    }
                } else {
                    self.advance();
                }
            }
            Bvc(mode) => {
                if !self.is_overflow() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bvc");
                    }
                } else {
                    self.advance();
                }
            }
            Bvs(mode) => {
                if self.is_overflow() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bvc");
                    }
                } else {
                    self.advance();
                }
            }
            Bcc(mode) => {
                if !self.is_carry() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bcc");
                    }
                } else {
                    self.advance();
                }
            }
            Bcs(mode) => {
                if self.is_carry() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bcs");
                    }
                } else {
                    self.advance();
                }
            }
            Bne(mode) => {
                if !self.is_zero() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Bne");
                    }
                } else {
                    self.advance();
                }
            }
            Beq(mode) => {
                if self.is_zero() {
                    if let Address(addr) = self.get_operand(mode)? {
                        self.goto(addr)?;
                    } else {
                        anyhow::bail!("invalid address in Beq");
                    }
                } else {
                    self.advance();
                }
            }
            Brk => {
                self.store_pc()?;
                self.store_flag_with(Flags::BREAK)?;
                self.set_interrupt_disable();
                let addr = self.read_memory_u16(IRQ_VECTOR)? as usize;
                self.goto(addr)?;
            }
            Cmp(mode) => {
                let val = self.get_operand_value(mode)?;
                if self.acc > val {
                    self.set_carry();
                    self.cls_zero();
                } else if self.acc == val {
                    self.set_carry();
                    self.set_zero();
                } else {
                    self.cls_carry();
                    self.cls_zero();
                }
                if is_negative(self.acc.wrapping_sub(val)) {
                    self.set_negative();
                } else {
                    self.cls_negative();
                }
                self.advance();
            }
            Cpx(mode) => {
                let val = self.get_operand_value(mode)?;
                if self.x > val {
                    self.set_carry();
                    self.cls_zero();
                } else if self.x == val {
                    self.set_carry();
                    self.set_zero();
                } else {
                    self.cls_zero();
                    self.cls_carry();
                }
                if is_negative(self.x.wrapping_sub(val)) {
                    self.set_negative();
                } else {
                    self.cls_negative();
                }
                self.advance();
            }
            Cpy(mode) => {
                let val = self.get_operand_value(mode)?;
                if self.y > val {
                    self.set_carry();
                    self.cls_zero();
                } else if self.y == val {
                    self.set_carry();
                    self.set_zero();
                } else {
                    self.cls_zero();
                    self.cls_carry();
                }
                if is_negative(self.y.wrapping_sub(val)) {
                    self.set_negative();
                } else {
                    self.cls_negative();
                }
                self.advance();
            }
            Dec(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    let mem_val = self.read_memory(addr)?;
                    let new_val = mem_val.wrapping_sub(1);
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Dec instruction");
                }
                self.advance();
            }
            Eor(mode) => {
                self.set_acc(self.acc ^ self.get_operand_value(mode)?);
                self.advance();
            }
            Clc => {
                self.cls_carry();
                self.advance();
            }
            Sec => {
                self.set_carry();
                self.advance();
            }
            Cli => {
                self.cls_interrupt_disable();
                self.advance();
            }
            Sei => {
                self.set_interrupt_disable();
                self.advance();
            }
            Clv => {
                self.cls_overflow();
                self.advance();
            }
            Cld => {
                self.cls_decimal();
                self.advance();
            }
            Sed => {
                self.set_decimal();
                self.advance();
            }
            Inc(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    let mem_val = self.read_memory(addr)?;
                    let new_val = mem_val.wrapping_add(1);
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Inc instruction");
                }
                self.advance();
            }
            Jmp(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    self.goto(addr)?;
                } else {
                    anyhow::bail!("invalid jump instruction");
                }
            }
            Jsr(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    self.store_pc()?;
                    self.goto(addr)?;
                } else {
                    anyhow::bail!("invalid Jsr instruction");
                }
            }
            Lda(mode) => {
                let val = self.get_operand_value(mode)?;
                self.set_acc(val);
                self.advance();
            }
            Ldx(mode) => {
                self.set_x(self.get_operand_value(mode)?);
                self.advance();
            }
            Ldy(mode) => {
                self.set_y(self.get_operand_value(mode)?);
                self.advance();
            }
            Lsr(mode) => {
                if let Accumulator = mode {
                    if self.acc & 1 != 0 {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    self.set_acc(self.acc >> 1);
                } else if let Address(addr) = self.get_operand(mode)? {
                    let val = self.read_memory(addr)?;
                    if val & 1 != 0 {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    let new_val = val >> 1;
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Lsr instruction");
                }
                self.advance();
            }
            Nop => self.advance(),
            Ora(mode) => {
                self.set_acc(self.acc | self.get_operand_value(mode)?);
                self.advance();
            }
            Tax => {
                self.x = self.acc;
                self.advance();
            }
            Txa => {
                self.acc = self.x;
                self.advance();
            }
            Dex => {
                self.set_x(self.x.wrapping_sub(1));
                self.advance();
            }
            Inx => {
                self.set_x(self.x.wrapping_add(1));
                self.advance();
            }
            Tay => {
                self.set_y(self.acc);
                self.advance();
            }
            Tya => {
                self.set_acc(self.y);
                self.advance();
            }
            Dey => {
                self.set_y(self.y.wrapping_sub(1));
                self.advance();
            }
            Iny => {
                self.set_y(self.y.wrapping_add(1));
                self.advance();
            }
            Rol(mode) => {
                let carry = self.carry_bit();
                if let Accumulator = mode {
                    if is_negative(self.acc) {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    self.set_acc((self.acc << 1) | carry);
                } else if let Address(addr) = self.get_operand(mode)? {
                    let mem_val = self.read_memory(addr)?;
                    if is_negative(mem_val) {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    let new_val = (mem_val << 1) | carry;
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Ror instruction");
                }
            }
            Ror(mode) => {
                let high_bit: u8 = if self.is_carry() { BIT7 } else { 0 };
                if let Accumulator = mode {
                    if self.acc & BIT0 == BIT0 {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    self.set_acc((self.acc >> 1) | high_bit);
                } else if let Address(addr) = self.get_operand(mode)? {
                    let mem_val = self.read_memory(addr)?;
                    if mem_val & BIT0 == BIT0 {
                        self.set_carry();
                    } else {
                        self.cls_carry();
                    }
                    let new_val = (mem_val >> 1) | high_bit;
                    self.write_memory(addr, new_val)?;
                    self.update_zero_and_negative_flags(new_val);
                } else {
                    anyhow::bail!("invalid Ror instruction");
                }
                self.advance();
            }
            Rti => {
                self.restore_flag()?;
                self.restore_pc()?;
            }
            Rts => {
                self.restore_pc()?;
            }
            Sbc(mode) => {
                let mem_val = self.get_operand_value(mode)?;
                let acc = self.acc;
                let inv_carry = if self.is_carry() { 0u8 } else { 1u8 };
                let (result, underflow) = acc.overflowing_sub(mem_val);
                let (result_c, underflow_c) = result.overflowing_sub(inv_carry);
                self.set_acc(result_c);
                if underflow || underflow_c {
                    self.cls_carry();
                } else {
                    self.set_carry();
                }
                let result_sign = sign_bit(result_c);
                if result_sign != sign_bit(acc) && result_sign == sign_bit(mem_val) {
                    self.set_overflow();
                } else {
                    self.cls_overflow();
                }
                self.advance();
            }
            Sta(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    self.write_memory(addr, self.acc)?;
                } else {
                    anyhow::bail!("invalid operaand in Sta");
                }
                self.advance();
            }
            Txs => {
                self.sp = self.x as usize;
                self.advance();
            }
            Tsx => {
                self.x = self.sp as u8;
                self.advance();
            }
            Pha => {
                self.stack_push(self.acc)?;
                self.advance();
            }
            Pla => {
                let acc_val = self.stack_pop()?;
                self.set_acc(acc_val);
                self.advance();
            }
            Php => {
                self.store_flag_with(Flags::BREAK)?;
                self.advance();
            }
            Plp => {
                self.restore_flag()?;
                self.advance();
            }
            Stx(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    self.write_memory(addr, self.x)?;
                } else {
                    anyhow::bail!("invalid operand in Stx");
                }
                self.advance();
            }
            Sty(mode) => {
                if let Address(addr) = self.get_operand(mode)? {
                    self.write_memory(addr, self.y)?;
                } else {
                    anyhow::bail!("invalid operaand in Sty");
                }
                self.advance();
            }
            Halt => {
                return Ok(Status::Halt);
            }
        };
        Ok(Status::Cont)
    }
}

enum Status {
    Halt,
    Cont,
}

impl Iterator for Machine<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pc < MEMORY_SIZE {
            let pc = self.pc;
            self.pc += 1;
            Some(self.read_memory(pc).unwrap())
        } else {
            None
        }
    }
}

impl<T> Cursor for T where T: Iterator<Item = u8> {}

#[derive(Debug)]
pub enum Index {
    None,
    X,
    Y,
}

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate(u8),
    ZeroPage(u8, Index),
    Relative(i8),
    Absolute(u16, Index),
    Indirect(u16),
    /// Indexed Indirect | LDA ($40, X) -> *(val(X) + $40)
    IndexedIndirect(u8),
    /// Indirect Indexed | LDA ($40), Y -> *($0040) + val(Y)
    IndirectIndexed(u8),
}

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AddressingMode::*;
        match self {
            Implied => write!(f, ""),
            Accumulator => write!(f, "A"),
            Immediate(n) => write!(f, "#${:0>2x}", *n),
            ZeroPage(n, idx) => match idx {
                Index::None => write!(f, "${:0>2x}", *n),
                Index::X => write!(f, "${:0>2x},X", *n),
                Index::Y => write!(f, "${:0>2x},Y", *n),
            },
            Relative(n) => write!(f, "${:0>2x}", *n),
            Absolute(n, idx) => match idx {
                Index::None => write!(f, "${:0>4x}", *n),
                Index::X => write!(f, "${:0>4x},X", *n),
                Index::Y => write!(f, "${:0>4x},Y", *n),
            },
            Indirect(n) => write!(f, "(${:0>4x})", *n),
            IndexedIndirect(n) => write!(f, "(${:0>2x},X)", *n),
            IndirectIndexed(n) => write!(f, "(${:0>2x}),Y", *n),
        }
    }
}

type Mode = AddressingMode;

pub trait Cursor: Iterator<Item = u8> {
    fn next_2(&mut self) -> Option<(u8, u8)> {
        match (self.next(), self.next()) {
            (Some(lsb), Some(msb)) => Some((lsb, msb)),
            _ => None,
        }
    }

    fn need_u8(&mut self) -> anyhow::Result<u8> {
        if let Some(b) = self.next() {
            Ok(b)
        } else {
            anyhow::bail!("insufficient byte");
        }
    }

    fn need_i8(&mut self) -> anyhow::Result<i8> {
        if let Some(b) = self.next() {
            Ok(b as i8)
        } else {
            anyhow::bail!("insufficient byte");
        }
    }

    fn need_u16(&mut self) -> anyhow::Result<u16> {
        if let Some((lsb, msb)) = self.next_2() {
            Ok(u16::from_le_bytes([lsb, msb]))
        } else {
            anyhow::bail!("insufficient bytes for u16")
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    /// Add with carry
    Adc(Mode),
    /// Bitwise And
    And(Mode),
    /// Arithmetic shift left
    Asl(Mode),
    /// Branch if carry clear
    Bcc(Mode),
    /// Branch if carry set
    Bcs(Mode),
    /// Branch if equal
    Beq(Mode),
    /// Bit Test
    Bit(Mode),
    /// Branch if Minus
    Bmi(Mode),
    /// Branch if not euqal
    Bne(Mode),
    /// Branch if plus
    Bpl(Mode),
    /// Break (software IRQ)
    Brk,
    /// Branch if overflow clear
    Bvc(Mode),
    /// Branch if overflow set
    Bvs(Mode),
    /// Clear carray
    Clc,
    /// Clear decimal
    Cld,
    /// Clear interrupt disable
    Cli,
    /// Clear overflow
    Clv,
    /// Compare A
    Cmp(Mode),
    /// Compare X
    Cpx(Mode),
    /// Compare Y
    Cpy(Mode),
    /// Decrement Memory (M=M-1)
    Dec(Mode),
    /// Decrement X (X=X-1)
    Dex,
    /// Decrement Y (Y=Y-1)
    Dey,
    /// Bitwise Exclusive or
    Eor(Mode),
    /// Increment memory
    Inc(Mode),
    /// Increment X
    Inx,
    /// Increment Y
    Iny,
    /// Jump
    Jmp(Mode),
    /// Jump to subroutine
    Jsr(Mode),
    /// Load A
    Lda(Mode),
    /// Load X
    Ldx(Mode),
    /// Load Y
    Ldy(Mode),
    /// Logical shift right
    Lsr(Mode),
    /// No oepration
    Nop,
    /// Bitwise or
    Ora(Mode),
    /// Push A
    Pha,
    /// Push processor status
    Php,
    /// pull A
    Pla,
    /// Pull processor status
    Plp,
    /// Rotate left
    Rol(Mode),
    /// Rotate right
    Ror(Mode),
    /// Return from interrupt
    Rti,
    /// Return from Subroutine
    Rts,
    /// Substract with carry
    Sbc(Mode),
    /// Set Carry
    Sec,
    /// Set Decimal
    Sed,
    /// Set interrupt disable
    Sei,
    /// Store A
    Sta(Mode),
    /// Store X
    Stx(Mode),
    /// Store Y
    Sty(Mode),
    /// Transfer A to X
    Tax,
    /// Transfer A to Y
    Tay,
    /// Transfer Stack Pointer to X
    Tsx,
    /// Transfer X to A
    Txa,
    /// Transfer X to Stack Pointer
    Txs,
    /// Transfer Y to A
    Tya,

    Halt,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation::*;
        match self {
            Adc(mode) => write!(f, "ADC {}", mode),
            And(mode) => write!(f, "AND {}", mode),
            Asl(mode) => write!(f, "ASL {}", mode),
            Bit(mode) => write!(f, "BIT {}", mode),
            Bpl(mode) => write!(f, "BPL {}", mode),
            Bmi(mode) => write!(f, "BMI {}", mode),
            Bvc(mode) => write!(f, "BVC {}", mode),
            Bvs(mode) => write!(f, "BVS {}", mode),
            Bcc(mode) => write!(f, "BCC {}", mode),
            Bcs(mode) => write!(f, "BCS {}", mode),
            Bne(mode) => write!(f, "BNE {}", mode),
            Beq(mode) => write!(f, "BEQ {}", mode),
            Brk => write!(f, "BRK"),
            Cmp(mode) => write!(f, "CMP {}", mode),
            Cpx(mode) => write!(f, "CPX {}", mode),
            Cpy(mode) => write!(f, "CPY {}", mode),
            Dec(mode) => write!(f, "DEC {}", mode),
            Eor(mode) => write!(f, "EOR {}", mode),
            Clc => write!(f, "CLC"),
            Sec => write!(f, "SEC"),
            Cli => write!(f, "CLI"),
            Sei => write!(f, "SEI"),
            Clv => write!(f, "CLV"),
            Cld => write!(f, "CLD"),
            Sed => write!(f, "SED"),
            Inc(mode) => write!(f, "INC {}", mode),
            Jmp(mode) => write!(f, "JMP {}", mode),
            Jsr(mode) => write!(f, "JSR {}", mode),
            Lda(mode) => write!(f, "LDA {}", mode),
            Ldx(mode) => write!(f, "LDX {}", mode),
            Ldy(mode) => write!(f, "LDY {}", mode),
            Lsr(mode) => write!(f, "LSR {}", mode),
            Nop => write!(f, "NOP"),
            Ora(mode) => write!(f, "ORA {}", mode),
            Tax => write!(f, "TAX"),
            Txa => write!(f, "TXA"),
            Dex => write!(f, "DEX"),
            Inx => write!(f, "INX"),
            Tay => write!(f, "TAY"),
            Tya => write!(f, "TYA"),
            Dey => write!(f, "DEY"),
            Iny => write!(f, "INY"),
            Rol(mode) => write!(f, "ROL {}", mode),
            Ror(mode) => write!(f, "ROR {}", mode),
            Rti => write!(f, "RTI"),
            Rts => write!(f, "RTS"),
            Sbc(mode) => write!(f, "SBC {}", mode),
            Sta(mode) => write!(f, "STA {}", mode),
            Txs => write!(f, "TXS"),
            Tsx => write!(f, "TSX"),
            Pha => write!(f, "PHA"),
            Pla => write!(f, "PLA"),
            Php => write!(f, "PHP"),
            Plp => write!(f, "PLP"),
            Stx(mode) => write!(f, "STX {}", mode),
            Sty(mode) => write!(f, "STY {}", mode),
            Halt => write!(f, "HALT"),
        }
    }
}

fn parse_opcode<T: Cursor>(cursor: &mut T) -> anyhow::Result<Option<Operation>> {
    let operator = match cursor.next() {
        Some(operator) => operator,
        None => return Ok(None),
    };
    use AddressingMode::*;
    use Operation::*;
    let operation = match operator {
        // Adc
        0x69 => Adc(Immediate(cursor.need_u8()?)),
        0x65 => Adc(ZeroPage(cursor.need_u8()?, Index::None)),
        0x75 => Adc(ZeroPage(cursor.need_u8()?, Index::X)),
        0x6D => Adc(Absolute(cursor.need_u16()?, Index::None)),
        0x7D => Adc(Absolute(cursor.need_u16()?, Index::X)),
        0x79 => Adc(Absolute(cursor.need_u16()?, Index::Y)),
        0x61 => Adc(IndexedIndirect(cursor.need_u8()?)),
        0x71 => Adc(IndirectIndexed(cursor.need_u8()?)),
        // And
        0x29 => And(Immediate(cursor.need_u8()?)),
        0x25 => And(ZeroPage(cursor.need_u8()?, Index::None)),
        0x35 => And(ZeroPage(cursor.need_u8()?, Index::X)),
        0x2D => And(Absolute(cursor.need_u16()?, Index::None)),
        0x3D => And(Absolute(cursor.need_u16()?, Index::X)),
        0x39 => And(Absolute(cursor.need_u16()?, Index::Y)),
        0x21 => And(IndexedIndirect(cursor.need_u8()?)),
        0x31 => And(IndirectIndexed(cursor.need_u8()?)),
        // Asl
        0x0A => Asl(Accumulator),
        0x06 => Asl(ZeroPage(cursor.need_u8()?, Index::None)),
        0x16 => Asl(ZeroPage(cursor.need_u8()?, Index::X)),
        0x0E => Asl(Absolute(cursor.need_u16()?, Index::None)),
        0x1E => Asl(Absolute(cursor.need_u16()?, Index::X)),
        // Bit
        0x24 => Bit(ZeroPage(cursor.need_u8()?, Index::None)),
        0x2C => Bit(Absolute(cursor.need_u16()?, Index::None)),
        // Branch
        0x10 => Bpl(Relative(cursor.need_i8()?)),
        0x30 => Bmi(Relative(cursor.need_i8()?)),
        0x50 => Bvc(Relative(cursor.need_i8()?)),
        0x70 => Bvs(Relative(cursor.need_i8()?)),
        0x90 => Bcc(Relative(cursor.need_i8()?)),
        0xB0 => Bcs(Relative(cursor.need_i8()?)),
        0xD0 => Bne(Relative(cursor.need_i8()?)),
        0xF0 => Beq(Relative(cursor.need_i8()?)),
        // Break
        0x00 => {
            let _ = cursor.need_u8()?; // ignore the following byte
            Brk
        }
        // Cmp
        0xC9 => Cmp(Immediate(cursor.need_u8()?)),
        0xC5 => Cmp(ZeroPage(cursor.need_u8()?, Index::None)),
        0xD5 => Cmp(ZeroPage(cursor.need_u8()?, Index::X)),
        0xCD => Cmp(Absolute(cursor.need_u16()?, Index::None)),
        0xDD => Cmp(Absolute(cursor.need_u16()?, Index::X)),
        0xD9 => Cmp(Absolute(cursor.need_u16()?, Index::Y)),
        0xC1 => Cmp(IndexedIndirect(cursor.need_u8()?)),
        0xD1 => Cmp(IndirectIndexed(cursor.need_u8()?)),
        // Cpx
        0xE0 => Cpx(Immediate(cursor.need_u8()?)),
        0xE4 => Cpx(ZeroPage(cursor.need_u8()?, Index::None)),
        0xEC => Cpx(Absolute(cursor.need_u16()?, Index::None)),
        // Cpy
        0xC0 => Cpy(Immediate(cursor.need_u8()?)),
        0xC4 => Cpy(ZeroPage(cursor.need_u8()?, Index::None)),
        0xCC => Cpy(Absolute(cursor.need_u16()?, Index::None)),
        // Dec
        0xC6 => Dec(ZeroPage(cursor.need_u8()?, Index::None)),
        0xD6 => Dec(ZeroPage(cursor.need_u8()?, Index::X)),
        0xCE => Dec(Absolute(cursor.need_u16()?, Index::None)),
        0xDE => Dec(Absolute(cursor.need_u16()?, Index::X)),
        // Eor
        0x49 => Eor(Immediate(cursor.need_u8()?)),
        0x45 => Eor(ZeroPage(cursor.need_u8()?, Index::None)),
        0x55 => Eor(ZeroPage(cursor.need_u8()?, Index::X)),
        0x4D => Eor(Absolute(cursor.need_u16()?, Index::None)),
        0x5D => Eor(Absolute(cursor.need_u16()?, Index::X)),
        0x59 => Eor(Absolute(cursor.need_u16()?, Index::Y)),
        0x41 => Eor(IndexedIndirect(cursor.need_u8()?)),
        0x51 => Eor(IndirectIndexed(cursor.need_u8()?)),
        // Flag
        0x18 => Clc,
        0x38 => Sec,
        0x58 => Cli,
        0x78 => Sei,
        0xB8 => Clv,
        0xD8 => Cld,
        0xF8 => Sed,
        // Inc
        0xE6 => Inc(ZeroPage(cursor.need_u8()?, Index::None)),
        0xF6 => Inc(ZeroPage(cursor.need_u8()?, Index::X)),
        0xEE => Inc(Absolute(cursor.need_u16()?, Index::None)),
        0xFE => Inc(Absolute(cursor.need_u16()?, Index::X)),
        // Jmp
        0x4C => Jmp(Absolute(cursor.need_u16()?, Index::None)),
        0x6C => Jmp(Indirect(cursor.need_u16()?)),
        // Jsr
        0x20 => Jsr(Absolute(cursor.need_u16()?, Index::None)),
        // Lda
        0xA9 => Lda(Immediate(cursor.need_u8()?)),
        0xA5 => Lda(ZeroPage(cursor.need_u8()?, Index::None)),
        0xB5 => Lda(ZeroPage(cursor.need_u8()?, Index::X)),
        0xAD => Lda(Absolute(cursor.need_u16()?, Index::None)),
        0xBD => Lda(Absolute(cursor.need_u16()?, Index::X)),
        0xB9 => Lda(Absolute(cursor.need_u16()?, Index::Y)),
        0xA1 => Lda(IndexedIndirect(cursor.need_u8()?)),
        0xB1 => Lda(IndirectIndexed(cursor.need_u8()?)),
        // Ldx
        0xA2 => Ldx(Immediate(cursor.need_u8()?)),
        0xA6 => Ldx(ZeroPage(cursor.need_u8()?, Index::None)),
        0xB6 => Ldx(ZeroPage(cursor.need_u8()?, Index::Y)),
        0xAE => Ldx(Absolute(cursor.need_u16()?, Index::None)),
        0xBE => Ldx(Absolute(cursor.need_u16()?, Index::Y)),
        // Ldy
        0xA0 => Ldy(Immediate(cursor.need_u8()?)),
        0xA4 => Ldy(ZeroPage(cursor.need_u8()?, Index::None)),
        0xB4 => Ldy(ZeroPage(cursor.need_u8()?, Index::X)),
        0xAC => Ldy(Absolute(cursor.need_u16()?, Index::None)),
        0xBC => Ldy(Absolute(cursor.need_u16()?, Index::X)),
        // Lsr
        0x4A => Lsr(Accumulator),
        0x46 => Lsr(ZeroPage(cursor.need_u8()?, Index::None)),
        0x56 => Lsr(ZeroPage(cursor.need_u8()?, Index::X)),
        0x4E => Lsr(Absolute(cursor.need_u16()?, Index::None)),
        0x5E => Lsr(Absolute(cursor.need_u16()?, Index::X)),
        // Nop
        0xEA => Nop,
        // Ora
        0x09 => Ora(Immediate(cursor.need_u8()?)),
        0x05 => Ora(ZeroPage(cursor.need_u8()?, Index::None)),
        0x15 => Ora(ZeroPage(cursor.need_u8()?, Index::X)),
        0x0D => Ora(Absolute(cursor.need_u16()?, Index::None)),
        0x1D => Ora(Absolute(cursor.need_u16()?, Index::X)),
        0x19 => Ora(Absolute(cursor.need_u16()?, Index::Y)),
        0x01 => Ora(IndexedIndirect(cursor.need_u8()?)),
        0x11 => Ora(IndirectIndexed(cursor.need_u8()?)),
        // Register
        0xAA => Tax,
        0x8A => Txa,
        0xCA => Dex,
        0xE8 => Inx,
        0xA8 => Tay,
        0x98 => Tya,
        0x88 => Dey,
        0xC8 => Iny,
        // Rol
        0x2A => Rol(Accumulator),
        0x26 => Rol(ZeroPage(cursor.need_u8()?, Index::None)),
        0x36 => Rol(ZeroPage(cursor.need_u8()?, Index::X)),
        0x2E => Rol(Absolute(cursor.need_u16()?, Index::None)),
        0x3E => Rol(Absolute(cursor.need_u16()?, Index::X)),
        // Ror
        0x6A => Rol(Accumulator),
        0x66 => Rol(ZeroPage(cursor.need_u8()?, Index::None)),
        0x76 => Rol(ZeroPage(cursor.need_u8()?, Index::X)),
        0x6E => Rol(Absolute(cursor.need_u16()?, Index::None)),
        0x7E => Rol(Absolute(cursor.need_u16()?, Index::X)),
        // Rti
        0x40 => Rti,
        // Rts
        0x60 => Rts,
        // Sbc
        0xE9 => Sbc(Immediate(cursor.need_u8()?)),
        0xE5 => Sbc(ZeroPage(cursor.need_u8()?, Index::None)),
        0xF5 => Sbc(ZeroPage(cursor.need_u8()?, Index::X)),
        0xED => Sbc(Absolute(cursor.need_u16()?, Index::None)),
        0xFD => Sbc(Absolute(cursor.need_u16()?, Index::X)),
        0xF9 => Sbc(Absolute(cursor.need_u16()?, Index::Y)),
        0xE1 => Sbc(IndexedIndirect(cursor.need_u8()?)),
        0xF1 => Sbc(IndirectIndexed(cursor.need_u8()?)),
        // Sta
        0x85 => Sta(ZeroPage(cursor.need_u8()?, Index::None)),
        0x95 => Sta(ZeroPage(cursor.need_u8()?, Index::X)),
        0x8D => Sta(Absolute(cursor.need_u16()?, Index::None)),
        0x9D => Sta(Absolute(cursor.need_u16()?, Index::X)),
        0x99 => Sta(Absolute(cursor.need_u16()?, Index::Y)),
        0x81 => Sta(IndexedIndirect(cursor.need_u8()?)),
        0x91 => Sta(IndirectIndexed(cursor.need_u8()?)),
        // Stack
        0x9A => Txs,
        0xBA => Tsx,
        0x48 => Pha,
        0x68 => Pla,
        0x08 => Php,
        0x28 => Plp,
        // Stx
        0x86 => Stx(ZeroPage(cursor.need_u8()?, Index::None)),
        0x96 => Stx(ZeroPage(cursor.need_u8()?, Index::X)),
        0x8E => Stx(Absolute(cursor.need_u16()?, Index::None)),
        // Stx
        0x84 => Sty(ZeroPage(cursor.need_u8()?, Index::None)),
        0x94 => Sty(ZeroPage(cursor.need_u8()?, Index::X)),
        0x8C => Sty(Absolute(cursor.need_u16()?, Index::None)),
        0xFF => Halt,
        _ => {
            anyhow::bail!("unknown operator {:x}", operator)
        }
    };
    Ok(Some(operation))
}

fn sign_bit(b: u8) -> u8 {
    (b & BIT7) >> 7
}

fn is_negative(b: u8) -> bool {
    sign_bit(b) == 1
}
