use rand::distributions::Uniform;
use rand::Rng;
use serde::de::{Error, MapAccess, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use std::prelude::rust_2015::Result::Err;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUMBER_OF_REGISTERS: usize = 16;
const FONT_SIZE: usize = 80;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const MEMORY_START_ADDRESS: u16 = 512;
const NUMBER_OF_KEYS: usize = 16;
const FONTS: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0xF0, 0xF0, 0xF0, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Copy, Clone, Debug)]
pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    i: u16,
    stack: [u16; STACK_SIZE],
    pc: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUMBER_OF_KEYS],
    v: [u8; NUMBER_OF_REGISTERS],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    fps: u32,
}

impl Serialize for Chip8 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer
            .serialize_struct("Chip8", 11)
            .expect("Couldn't serialize Chip8.");
        state.serialize_field("memory", &self.memory.to_vec())?;
        state.serialize_field("stack", &self.stack.to_vec())?;
        state.serialize_field("v", &self.v.to_vec())?;
        state.serialize_field("keys", &self.keys.to_vec())?;
        state.serialize_field("screen", &self.screen.to_vec())?;
        state.serialize_field("i", &self.i)?;
        state.serialize_field("pc", &self.pc)?;
        state.serialize_field("sp", &self.sp)?;
        state.serialize_field("fps", &self.fps)?;
        state.serialize_field("sound_timer", &self.sound_timer)?;
        state.serialize_field("delay_timer", &self.delay_timer)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Chip8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Memory,
            Stack,
            V,
            Keys,
            Screen,
            I,
            Pc,
            Sp,
            Fps,
            SoundTimer,
            DelayTimer,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`test`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                    E: Error, {
                        match v {
                            "pc" => Ok(Field::Pc),
                            "i" => Ok(Field::I),
                            "fps" => Ok(Field::Fps),
                            "sp" => Ok(Field::Sp),
                            "delay_timer" => Ok(Field::DelayTimer),
                            "sound_timer" => Ok(Field::SoundTimer),
                            "stack" => Ok(Field::Stack),
                            "v" => Ok(Field::V),
                            "memory" => Ok(Field::Memory),
                            "screen" => Ok(Field::Screen),
                            "keys" => Ok(Field::Keys),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }


        }

        struct Chip8Visitor;

        impl<'de> Visitor<'de> for Chip8Visitor {
            type Value = Chip8;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct Chip8")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
            where
                A: MapAccess<'de>,
            {
                let mut memory = None;
                let mut stack = None;
                let mut v = None;
                let mut keys = None;
                let mut screen = None;
                let mut i = None;
                let mut pc = None;
                let mut sp = None;
                let mut fps = None;
                let mut sound_timer = None;
                let mut delay_timer = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Memory => {
                            if memory.is_some() {
                                return Err(de::Error::duplicate_field("memory"));
                            }
                            memory = Some(map.next_value()?);
                        }
                        Field::Stack => {
                            if stack.is_some() {
                                return Err(de::Error::duplicate_field("stack"));
                            }
                            stack = Some(map.next_value()?);
                        }
                        Field::V => {
                            if v.is_some() {
                                return Err(de::Error::duplicate_field("v"));
                            }
                            v = Some(map.next_value()?);
                        }
                        Field::Keys => {
                            if keys.is_some() {
                                return Err(de::Error::duplicate_field("keys"));
                            }
                            keys = Some(map.next_value()?);
                        }
                        Field::Screen => {
                            if screen.is_some() {
                                return Err(de::Error::duplicate_field("keys"));
                            }
                            screen = Some(map.next_value()?);
                        }
                        Field::I => {
                            if i.is_some() {
                                return Err(de::Error::duplicate_field("i"));
                            }
                            i = Some(map.next_value()?);
                        }
                        Field::Pc => {
                            if pc.is_some() {
                                return Err(de::Error::duplicate_field("pc"));
                            }
                            pc = Some(map.next_value()?);
                        }
                        Field::Sp => {
                            if sp.is_some() {
                                return Err(de::Error::duplicate_field("sp"));
                            }
                            sp = Some(map.next_value()?);
                        }
                        Field::DelayTimer => {
                            if delay_timer.is_some() {
                                return Err(de::Error::duplicate_field("delay_timer"));
                            }
                            delay_timer = Some(map.next_value()?);
                        }
                        Field::SoundTimer => {
                            if sound_timer.is_some() {
                                return Err(de::Error::duplicate_field("sound_timer"));
                            }
                            sound_timer = Some(map.next_value()?);
                        }
                        Field::Fps => {
                            if fps.is_some() {
                                return Err(de::Error::duplicate_field("fps"));
                            }
                            fps = Some(map.next_value()?);
                        }
                    }
                }

                let memory = memory.ok_or_else(|| de::Error::missing_field("memory"))?;
                let stack = stack.ok_or_else(|| de::Error::missing_field("stack"))?;
                let v = v.ok_or_else(|| de::Error::missing_field("v"))?;
                let keys = keys.ok_or_else(|| de::Error::missing_field("keys"))?;
                let screen = screen.ok_or_else(|| de::Error::missing_field("screen"))?;
                let i = i.ok_or_else(|| de::Error::missing_field("i"))?;
                let pc = pc.ok_or_else(|| de::Error::missing_field("pc"))?;
                let sp = sp.ok_or_else(|| de::Error::missing_field("sp"))?;
                let fps = fps.ok_or_else(|| de::Error::missing_field("fps"))?;
                let sound_timer = sound_timer.ok_or_else(|| de::Error::missing_field("sound_timer"))?;
                let delay_timer = delay_timer.ok_or_else(|| de::Error::missing_field("delay_timer"))?;

                Ok(Chip8::from_deserialized_data(
                    memory,
                    i,
                    stack,
                    pc,
                    sp,
                    delay_timer,
                    sound_timer,
                    keys,
                    v,
                    screen,
                    fps,
                ))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "memory",
            "stack",
            "v",
            "screen",
            "keys",
            "i",
            "pc",
            "sp",
            "sound_timer",
            "delay_timer",
            "fps",
        ];
        deserializer.deserialize_struct("Chip8", FIELDS, Chip8Visitor)
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_chip8 = Self {
            memory: [0; MEMORY_SIZE],
            i: 0,
            stack: [0; STACK_SIZE],
            pc: MEMORY_START_ADDRESS,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; NUMBER_OF_KEYS],
            v: [0; NUMBER_OF_REGISTERS],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            fps: 60,
        };
        new_chip8.memory[..FONT_SIZE].copy_from_slice(&FONTS);
        new_chip8
    }

    pub fn load_file(&mut self, data: &[u8]) {
        let start = MEMORY_START_ADDRESS as usize;
        let end = (MEMORY_START_ADDRESS as usize) + data.len();
        self.memory[start..end].copy_from_slice(&data);
    }

    pub fn save_state(&self) {
        let mut _open_file = File::create("save-state.bin").expect("Couldn't create save state!");

        let serialized = serde_json::to_string(&self).unwrap();
        let deserialized: Chip8 = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", deserialized);

    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.decode(opcode);
    }

    pub fn get_display(&self) -> [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.screen
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    pub fn tick_timers(&mut self) -> bool {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer > 0 {
                return true;
            }
        }

        false
    }

    pub fn key_down(&mut self, key: Option<u8>, is_down: bool) {
        if let Some(key_index) = key {
            self.keys[key_index as usize] = is_down;
        }
    }

    fn from_deserialized_data(
        memory: Vec<u8>,
        i: u16,
        stack: Vec<u16>,
        pc: u16,
        sp: u8,
        delay_timer: u8,
        sound_timer: u8,
        keys: Vec<bool>,
        v: Vec<u8>,
        screen: Vec<bool>,
        fps: u32,
    ) -> Self {
        let mut memory_buf = [0; MEMORY_SIZE];
        memory_buf[0..].copy_from_slice(&memory);

        let mut stack_buf:[u16; STACK_SIZE] = [0; STACK_SIZE];
        stack_buf[0..STACK_SIZE].copy_from_slice(&stack);

        let mut v_buf: [u8; NUMBER_OF_REGISTERS] = [0; NUMBER_OF_REGISTERS];
        v_buf[0..NUMBER_OF_REGISTERS].copy_from_slice(&v);

        let mut screen_buf: [bool; SCREEN_HEIGHT * SCREEN_WIDTH] = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        screen_buf[0..(SCREEN_WIDTH * SCREEN_HEIGHT)].copy_from_slice(&screen);

        let mut keys_buf: [bool; NUMBER_OF_KEYS] = [false; NUMBER_OF_KEYS];
        keys_buf[0..NUMBER_OF_KEYS].copy_from_slice(&keys);
        Self {
            memory: memory_buf,
            stack: stack_buf,
            v: v_buf,
            keys: keys_buf,
            screen: screen_buf,
            pc,
            sp,
            delay_timer,
            sound_timer,
            i,
            fps,
        }

    }

    fn push(&mut self, instruction: u16) {
        self.stack[self.sp as usize] = instruction;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn fetch(&mut self) -> u16 {
        // Two successive byte stored one after the other.
        let higher_byte = self.memory[self.pc as usize] as u16;
        let lower_byte = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }

    fn decode(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let kk = opcode & 0x00FF;
        let n = opcode & 0x000F;

        let first_nibble = (opcode & 0xF000) >> 12;
        // Getting the lower 4 bits of the high byte of the instruction. EX:
        // instruction = 0x34ff -> in this example 4 represents the lower four bits of the high
        // byte. If we shift 8 bits (f -> 4, f -> 4) we get 0x0034.
        // if we perform bitwise and on 0x0034 and 0x00f we'll end with the lower 4 bytes:
        // 0x0034 & 0x000f = 0x0004.
        let x = (opcode & 0x0F00) >> 8;
        // Getting the higher 4 bits of the low byte instruction. We apply the same principle
        // explained above.
        let y = (opcode & 0x00F0) >> 4;

        println!(
            "Current instruction: {:#04x} {:#04x} {:#04x} {:#04x}",
            first_nibble, x, y, n
        );
        match (first_nibble, x, y, n) {
            // No OP opcode.
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            (1, _, _, _) => {
                self.pc = nnn;
            }
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = nnn;
            }
            (0, 0, 0xE, 0xE) => {
                let subroutine_value = self.pop();
                self.pc = subroutine_value;
            }
            (3, _, _, _) => {
                if self.v[x as usize] == (kk as u8) {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => {
                if self.v[x as usize] != (kk as u8) {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                self.v[x as usize] = kk as u8;
            }
            (7, _, _, _) => {
                // wrapping_add automatically performs wrap in case of overflow.
                // EX: 255 + 1 (u8) = 0.
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk as u8);
            }
            (8, _, _, 0) => {
                self.v[x as usize] = self.v[y as usize];
            }
            (8, _, _, 1) => {
                self.v[x as usize] |= self.v[y as usize];
            }
            (8, _, _, 2) => {
                self.v[x as usize] &= self.v[y as usize];
            }
            (8, _, _, 3) => {
                self.v[x as usize] ^= self.v[y as usize];
            }
            (8, _, _, 4) => {
                let (current_x, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                let new_vf = if overflow { 1 } else { 0 };
                self.v[x as usize] = current_x;
                self.v[0xF] = new_vf;
            }
            (8, _, _, 5) => {
                let (current_x, borrow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                let new_vf: u8 = if borrow { 0 } else { 1 };
                self.v[x as usize] = current_x;
                self.v[0xF] = new_vf;
            }
            (8, _, _, 6) => {
                let shifted_bit = self.v[x as usize] & 1;
                self.v[x as usize] >>= 1;
                self.v[0xF] = shifted_bit;
            }
            (8, _, _, 7) => {
                let (current_x, borrow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                let new_vf: u8 = if borrow { 0 } else { 1 };

                self.v[x as usize] = current_x;
                self.v[0xF] = new_vf;
            }
            (8, _, _, 0xE) => {
                let shifted_bit = (self.v[x as usize] >> 7) & 0x1;
                self.v[x as usize] <<= 1;
                self.v[0xF] = shifted_bit;
            }
            (9, _, _, 0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = (self.v[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                let range = Uniform::from(0..kk);
                let random_bit = rng.sample(range);
                self.v[x as usize] = (random_bit & kk) as u8;
            }
            (0xD, _, _, _) => {
                // Getting the coordinates out of their respective
                // registers.
                let x_coordinate = self.v[x as usize] as u16;
                let y_coordinate = self.v[y as usize] as u16;

                // The value of n represents the number of rows of our sprite.
                let number_of_rows = n;

                // Will allow us to check if any pixels were flipped.
                let mut flipped = false;

                // Here we are iterating over each row in the sprite.
                for y_line in 0..number_of_rows {
                    // Here we determine which memory address our rows data is
                    // stored in. Sprites are stored row by row, beginning at the
                    // address stored in I. So if we were to draw a 3px tall sprite
                    // the first row's data is stored in I, followed by I + 1, I + 2, etc.
                    // That's the main reason why all the sprites are 8 pixels wide; each
                    // row is assigned a byte, which is 8-bits.
                    let address = (self.i + y_line) as u16;
                    let pixels = self.memory[address as usize];
                    for x_line in 0..8 {
                        // Here we basically get the pixel located at a specific row.
                        // In this case, we want any value different than 0, since 0
                        // means the space is empty.
                        if (pixels & (0b10000000 >> x_line)) != 0 {
                            // Sprites wrap around the screen. We should use modulo division
                            // because of that.
                            let x = (x_coordinate + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coordinate + y_line) as usize % SCREEN_HEIGHT;

                            // Getting the index of the pixel from our 1D array.
                            let pixel_index = x + SCREEN_WIDTH * y;

                            // Checking if we are about to flip the pixel.
                            flipped |= self.screen[pixel_index];

                            // Set.
                            self.screen[pixel_index] ^= true;
                        }
                    }
                }

                // Populating the VF register.
                if flipped {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => {
                let key_index = self.v[x as usize];
                let is_pressed = self.keys[key_index as usize];
                if is_pressed {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let key_index = self.v[x as usize];
                let is_pressed = self.keys[key_index as usize];
                if !is_pressed {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                self.v[x as usize] = self.delay_timer;
            }
            (0xF, _, 1, 5) => {
                self.delay_timer = self.v[x as usize];
            }
            (0xF, _, 1, 8) => {
                self.sound_timer = self.v[x as usize];
            }
            (0xF, _, 1, 0xE) => {
                let (current_i, overflow) = self.i.overflowing_add(self.v[x as usize] as u16);
                self.i = current_i;
                if overflow {
                    self.v[0xF] = 1;
                }
            }
            (0xF, _, 0, 0xA) => {
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                };
            }
            (0xF, _, 2, 9) => {
                let vx = self.v[x as usize];
                self.i = (vx * 5) as u16;
            }
            (0xF, _, 3, 3) => {
                let vx = self.v[x as usize];
                let hundreds = ((vx as f32) / 100.0).floor() as u8;
                let tens = (((vx as f32) / 10.0) % 10.0).floor() as u8;
                let ones = ((vx as f32) % 10.0) as u8;

                self.memory[(self.i) as usize] = hundreds;
                self.memory[(self.i + 1) as usize] = tens;
                self.memory[(self.i + 2) as usize] = ones;
            }
            (0xF, _, 5, 5) => {
                for i in 0..=x {
                    self.memory[(self.i + i) as usize] = self.v[i as usize];
                }
            }
            (0xF, _, 6, 5) => {
                for i in 0..=x {
                    self.v[i as usize] = self.memory[(self.i + i) as usize];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode. Opcode: {}", opcode),
        };
    }
}
