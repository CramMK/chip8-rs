extern crate sdl2;

use rand::Rng;

use crate::fontset::FONT;
use crate::display::Display;
use crate::input::Input;

const OPCODE_SIZE: usize = 2;

pub struct Processor {
    memory: [u8; crate::MEMORY_SIZE],
    register: [u8; 16],
    index: usize, // used to store memory addresses
    pc: usize, // programm counter
    screen: [[u8; crate::SCREEN_WIDTH]; crate::SCREEN_HEIGHT],
    draw_flag: bool, // redraw screen if true
    delay_timer: usize,
    sound_timer: usize,
    stack: [usize; 16],
    sp: usize, // stack pointer
    key: [bool; 16],
    waiting_for_key: bool,
    waiting_key_location: usize,
}

impl Processor {
    pub fn new() -> Self {
	let mut mem = [0; crate::MEMORY_SIZE];
	// load font
	for (pos, &val) in FONT.iter().enumerate() {
	    mem[pos] = val;
	}

	Processor {
	    memory: mem,
	    register: [0; 16],
	    index: 0,
	    pc: crate::GAME_ENTRY,
	    screen: [[0; crate::SCREEN_WIDTH]; crate::SCREEN_HEIGHT],
	    draw_flag: false,
	    delay_timer: 0,
	    sound_timer: 0,
	    stack: [0; 16],
	    sp: 0,
	    key: [false; 16],
	    waiting_for_key: false,
	    waiting_key_location: 0,
	}
    }

    pub fn start(&mut self, game: &[u8]) {
	let sdl_ctx = sdl2::init().unwrap();
	let mut display = Display::new(&sdl_ctx);
	let mut input = Input::new(&sdl_ctx);

	self.load_game(&game);

	loop {
	    // get keyinput using sdl2
	    self.key = input.fetch().unwrap();

	    // emulate one cycle
	    self.cycle();

	    // draw to screen using sdl2
	    if self.draw_flag {
		display.draw(&self.screen);
	    }

	    // play sound using sdl2
	    // TODO

	    // add delay
	    // TODO
	}
    }

    pub fn cycle(&mut self) {
	// reset
	self.draw_flag = false;

	// opcode FX0A freezes the program, until a key is pressed
	if self.waiting_for_key {
	    for (pos, &val) in self.key.iter().enumerate() {
		if self.key[pos] {
		    self.waiting_for_key = false;
		    self.register[self.waiting_key_location] = val as u8;
		    break;
		}
	    }
	}
	else {
	    // decr both timers every cycle
	    if self.delay_timer > 0 {
		self.delay_timer -= 1;
	    }
	    if self.sound_timer > 0 {
		self.sound_timer -= 1;
	    }

	    // execute current opcode
	    let opcode = self.fetch_opcode();
	    self.decode_opcode(opcode);
	}
    }

    pub fn load_game(&mut self, game: &[u8]) {
	for (pos, &val) in game.iter().enumerate() {
	    let position = crate::GAME_ENTRY + pos;
	    if position < crate::MEMORY_SIZE { // don't go above mem limit
		self.memory[position] = val;
	    }
	    else {
		break;
	    }
	}
    }

    pub fn fetch_opcode(&self) -> u16 {
	// final opcode consists of 2 bytes
	(self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    pub fn decode_opcode(&mut self, opcode: u16) {
	// values from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0

	let nibbles = (
	    ((opcode & 0xF000) >> 12) as usize,
	    ((opcode & 0x0F00) >> 8) as usize,
	    ((opcode & 0x00F0) >> 4) as usize,
	    (opcode & 0x000F) as usize
	);

	// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
	let nnn = (opcode & 0x0FFF) as usize;

	// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
	let n = nibbles.3 as usize;

	// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
	let x = nibbles.1 as usize;

	// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
	let y = nibbles.2 as usize;

	//kk or byte - An 8-bit value, the lowest 8 bits of the instruction
	let kk = (opcode & 0x00FF) as u8;

	// match nibbles to opcodes => run funtion
	match nibbles {
	    (0x00, 0x00, 0x0e, 0x00) => self.code_00e0(),
	    (0x00, 0x00, 0x0e, 0x0e) => self.code_00ee(),
	    (0x01, _, _, _) => self.code_1nnn(nnn),
	    (0x02, _, _, _) => self.code_2nnn(nnn),
	    (0x03, _, _, _) => self.code_3xkk(x, kk),
	    (0x04, _, _, _) => self.code_4xkk(x, kk),
	    (0x05, _, _, 0x00) => self.code_5xy0(x, y),
	    (0x06, _, _, _) => self.code_6xkk(x, kk),
	    (0x07, _, _, _) => self.code_7xkk(x, kk),
	    (0x08, _, _, 0x00) => self.code_8xy0(x, y),
	    (0x08, _, _, 0x01) => self.code_8xy1(x, y),
	    (0x08, _, _, 0x02) => self.code_8xy2(x, y),
	    (0x08, _, _, 0x03) => self.code_8xy3(x, y),
	    (0x08, _, _, 0x04) => self.code_8xy4(x, y),
	    (0x08, _, _, 0x05) => self.code_8xy5(x, y),
	    (0x08, _, _, 0x06) => self.code_8xy6(x, y),
	    (0x08, _, _, 0x07) => self.code_8xy7(x, y),
	    (0x08, _, _, 0x0e) => self.code_8xye(x, y),
	    (0x09, _, _, 0x00) => self.code_9xy0(x, y),
	    (0x0a, _, _, _) => self.code_annn(nnn),
	    (0x0b, _, _, _) => self.code_bnnn(nnn),
	    (0x0c, _, _, _) => self.code_cxkk(x, kk),
	    (0x0d, _, _, _) => self.code_dxyn(x, y, n),
	    (0x0e, _, 0x09, 0x0e) => self.code_ex9e(x),
	    (0x0e, _, 0x0a, 0x01) => self.code_exa1(x),
	    (0x0f, _, 0x00, 0x07) => self.code_fx07(x),
	    (0x0f, _, 0x00, 0x0a) => self.code_fx0a(x),
	    (0x0f, _, 0x01, 0x05) => self.code_fx15(x),
	    (0x0f, _, 0x01, 0x08) => self.code_fx18(x),
	    (0x0f, _, 0x01, 0x0e) => self.code_fx1e(x),
	    (0x0f, _, 0x02, 0x09) => self.code_fx29(x),
	    (0x0f, _, 0x03, 0x03) => self.code_fx33(x),
	    (0x0f, _, 0x05, 0x05) => self.code_fx55(x),
	    (0x0f, _, 0x06, 0x05) => self.code_fx65(x),
	    _ => self.pc += OPCODE_SIZE
	};
    }

    // Clear screen
    fn code_00e0(&mut self) {
	self.screen = [[0; crate::SCREEN_WIDTH]; crate::SCREEN_HEIGHT];
	self.draw_flag = true;
	self.pc += OPCODE_SIZE;
    }

    // Return from subroutine
    fn code_00ee(&mut self) {
	self.sp -= 1;
	self.pc = self.stack[self.sp];
    }

    // Jump to location nnn
    fn code_1nnn(&mut self, nnn: usize) {
	self.pc = nnn;
    }

    // Call subroutine at nnn
    fn code_2nnn(&mut self, nnn: usize) {
	self.stack[self.sp] = self.pc;
	self.sp += 1;
	self.pc = nnn;
    }

    // Skip next instruction if Vx = kk
    fn code_3xkk(&mut self, x: usize, kk: u8) {
	if self.register[x] == kk {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Skip next instruction if Vx != kk
    fn code_4xkk(&mut self, x: usize, kk: u8) {
	if self.register[x] != kk {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Skip next instruction if Vx = Vy
    fn code_5xy0(&mut self, x: usize, y: usize) {
	if self.register[x] == self.register[y] {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Set Vx = kk
    fn code_6xkk(&mut self, x: usize, kk: u8) {
	self.register[x] = kk;
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx + kk
    fn code_7xkk(&mut self, x: usize, kk: u8) {
	let val = self.register[x] as u16;
	self.register[x] = (val + kk as u16) as u8;
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vy
    fn code_8xy0(&mut self, x: usize, y: usize) {
	self.register[x] = self.register[y];
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx OR Vy
    fn code_8xy1(&mut self, x: usize, y: usize) {
	self.register[x] = self.register[x] | self.register[y];
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx AND Vy
    fn code_8xy2(&mut self, x: usize, y: usize) {
	self.register[x] = self.register[x] & self.register[y];
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx XOR Vy
    fn code_8xy3(&mut self, x: usize, y: usize) {
	self.register[x] = self.register[x] ^ self.register[y];
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx + Vy, set VF = carry
    fn code_8xy4(&mut self, x: usize, y: usize) {
	let result = (self.register[x]) as usize + (self.register[y]) as usize;

	self.register[x] = result as u8; // write back lowest 8bit
	self.register[0x0f] = (result > 255) as u8; // set carry flag
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx - Vy, set VF = NOT borrow
    fn code_8xy5(&mut self, x: usize, y: usize) {
	let x_val = self.register[x];
	let y_val = self.register[y];
	
	self.register[0x0f] = (x_val > y_val) as u8;
	self.register[x] = x_val.wrapping_sub(y_val) as u8;
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx SHR 1
    fn code_8xy6(&mut self, x: usize, _y: usize) {
	self.register[0x0f] = self.register[x] & 1; // set if least significant bit == 1
	self.register[x] = self.register[x] / 2;
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vy - Vx, set VF = NOT borrow
    fn code_8xy7(&mut self, x: usize, y: usize) {
	self.register[0x0f] = (self.register[y] > self.register[x]) as u8;
	self.register[x] = self.register[y].wrapping_sub(self.register[x]);
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx SHL 1
    fn code_8xye(&mut self, x: usize, _y: usize) {
	self.register[0x0f] = (self.register[x] & 0b10000000) >> 7;
	self.register[x] <<= 1;
	self.pc += OPCODE_SIZE;
    }

    // Skip next instruction if Vx != Vy
    fn code_9xy0(&mut self, x: usize, y: usize) {
	if self.register[x] != self.register[y] {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Set I = nnn
    fn code_annn(&mut self, nnn: usize) {
	self.index = nnn;
	self.pc += OPCODE_SIZE;
    }

    // Jump to location nnn + V0
    fn code_bnnn(&mut self, nnn: usize) {
	self.pc = nnn + self.register[0x00] as usize;
    }

    // Set Vx = random byte AND kk
    fn code_cxkk(&mut self, x: usize, kk: u8) {
	let rng = rand::thread_rng().gen_range(0..255);
	self.register[x] = kk & rng;
	self.pc += OPCODE_SIZE;
    }

    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn code_dxyn(&mut self, x: usize, y: usize, n: usize) {
	self.register[0x0f] = 0;
	for byte in 0..n {
	    // get y coord, which we want to draw -> use modulo, so we don't overlap
	    let y = (self.register[y] as usize + byte) % crate::SCREEN_HEIGHT;
	    for bit in 0..8 {
		// get x coord, just as above
		let x = (self.register[x] as usize + bit) % crate::SCREEN_WIDTH;
		// bit hack to get every bit in a row
		let pixel_to_draw = (self.memory[self.index + byte] >> (7 - bit)) & 1;
		// check if we will overwrite an existing pixel
		self.register[0x0f] = self.register[0x0f] | (pixel_to_draw & self.screen[y][x]);
		self.screen[y][x] = self.screen[y][x] ^ pixel_to_draw; 
	    }
	}
	self.draw_flag = true;
	self.pc += OPCODE_SIZE;
    }

    // Skip next instruction if key with the value of Vx is pressed
    fn code_ex9e(&mut self, x: usize) {
	if self.key[self.register[x] as usize] {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Skip next instruction if key with the value of Vx is not pressed
    fn code_exa1(&mut self, x: usize) {
	if !(self.key[self.register[x] as usize]) {
	    self.pc += 2 * OPCODE_SIZE;
	}
	else {
	    self.pc += OPCODE_SIZE;
	}
    }

    // Set Vx = delay timer value
    fn code_fx07(&mut self, x: usize) {
	self.register[x] = self.delay_timer as u8;
	self.pc += OPCODE_SIZE;
    }

    // Wait for a key press, store the value of the key in Vx
    fn code_fx0a(&mut self, x: usize) {
	self.waiting_for_key = true;
	self.waiting_key_location = x; // safe for later
	self.pc += OPCODE_SIZE;
    }

    // Set delay timer = Vx
    fn code_fx15(&mut self, x: usize) {
	self.delay_timer = self.register[x] as usize;
	self.pc += OPCODE_SIZE;
    }

    // Set sound timer = Vx
    fn code_fx18(&mut self, x: usize) {
	self.sound_timer = self.register[x] as usize;
	self.pc += OPCODE_SIZE;
    }

    // Set I = I + Vx
    fn code_fx1e(&mut self, x: usize) {
	self.index += self.register[x] as usize;
	self.pc += OPCODE_SIZE;
    }

    // Set I = location of sprite for digit Vx
    fn code_fx29(&mut self, x: usize) {
	let sprite_name = self.register[x] as usize;
	let mem_position = sprite_name * 5; // single sprite is 5byte
	self.index = mem_position;
	self.pc += OPCODE_SIZE;
    }

    // Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn code_fx33(&mut self, x: usize) {
	let val = self.register[x];
	self.memory[self.index] = (val / 100) as u8;
	self.memory[self.index + 1] = ((val % 100) / 10) as u8;
	self.memory[self.index + 2] = (val % 10) as u8;
	self.pc += OPCODE_SIZE;
    }

    // Store registers V0 through Vx in memory starting at location I
    fn code_fx55(&mut self, x: usize) {
	// TODO offset correct?
	for reg_i in 0..x + 1 {
	    self.memory[self.index + reg_i] = self.register[reg_i];
	}
	self.pc += OPCODE_SIZE;
    }

    // Read registers V0 through Vx from memory starting at location I
    fn code_fx65(&mut self, x: usize) {
	for reg_i in 0..x + 1 {
	    self.register[reg_i] = self.memory[self.index + reg_i];
	}
	self.pc += OPCODE_SIZE;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn new_processor() -> Processor {
	let mut processor = Processor::new();
	processor.register = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8];
	processor
    }

    const ENTRY: usize = crate::GAME_ENTRY;
    const SKIP: usize = ENTRY + OPCODE_SIZE * 2;
    const NEXT: usize = ENTRY + OPCODE_SIZE;

    #[test]
    fn test_initial_state() {
	let processor = Processor::new();
	assert_eq!(processor.pc, 0x200);
	assert_eq!(processor.sp, 0);
	assert_eq!(processor.stack, [0; 16]);
	// Font loading
	assert_eq!(processor.memory[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    }

    #[test]
    fn test_load_game() {
	let mut processor = Processor::new();
	processor.load_game(&[1, 2, 3]);
	assert_eq!(processor.memory[crate::GAME_ENTRY], 1);
	assert_eq!(processor.memory[crate::GAME_ENTRY + 1], 2);
	assert_eq!(processor.memory[crate::GAME_ENTRY + 2], 3);
    }

    #[test]
    fn test_code_00e0() {
	let mut processor = Processor::new();
	processor.screen = [[1; crate::SCREEN_WIDTH]; crate::SCREEN_HEIGHT];
	processor.decode_opcode(0x00e0);
	for y in 0..crate::SCREEN_HEIGHT {
	    for x in 0..crate::SCREEN_WIDTH {
		assert_eq!(processor.screen[y][x], 0);
	    }
	}
    }

    #[test]
    fn test_code_00ee() {
	let mut processor = Processor::new();
	processor.sp = 3;
	processor.stack[2] = 0x1337;
	processor.decode_opcode(0x00ee);
	assert_eq!(processor.sp, 2);
	assert_eq!(processor.pc, 0x1337);
    }

    #[test]
    fn test_code_1nnn() {
	let mut processor = Processor::new();
	processor.decode_opcode(0x1222);
	assert_eq!(processor.pc, 0x0222);
    }

    #[test]
    fn test_code_2nnn() {
	let mut processor = new_processor();
	processor.sp = 0;
	let current = processor.pc;
	processor.decode_opcode(0x2333);
	assert_eq!(processor.sp, 1);
	assert_eq!(processor.pc, 0x0333);
	assert_eq!(processor.stack[0], current);
    }

    #[test]
    fn test_code_3xkk() {
	let mut processor = new_processor();
	processor.decode_opcode(0x3202);
	assert_eq!(processor.pc, SKIP);

	let mut processor = new_processor();
	processor.decode_opcode(0x3206);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_4xkk() {
	let mut processor = new_processor();
	processor.decode_opcode(0x3206);
	assert_eq!(processor.pc, NEXT);

	let mut processor = new_processor();
	processor.decode_opcode(0x3202);
	assert_eq!(processor.pc, SKIP);
    }

    #[test]
    fn test_code_5xy0() {
	let mut processor = new_processor();
	processor.decode_opcode(0x5010);
	assert_eq!(processor.pc, SKIP);

	let mut processor = new_processor();
	processor.decode_opcode(0x5070);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_6xkk() {
	let mut processor = new_processor();
	processor.decode_opcode(0x6133);
	assert_eq!(processor.register[1], 0x33);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_7xkk() {
	let mut processor = new_processor();
	processor.decode_opcode(0x7001);
	assert_eq!(processor.register[0], 0x02);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy0() {
	let mut processor = new_processor();
	processor.decode_opcode(0x8f00);
	assert_eq!(processor.register[0], 1);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy1() {
	let mut processor = new_processor();
	processor.decode_opcode(0x8011);
	assert_eq!(processor.register[0], 1);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy2() {
	let mut processor = new_processor();
	processor.decode_opcode(0x8142);
	assert_eq!(processor.register[1], 1);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy3() {
	let mut processor = new_processor();
	processor.decode_opcode(0x8143);
	assert_eq!(processor.register[2], 2);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy4() {
	// no carry
	let mut processor = new_processor();
	processor.decode_opcode(0x8124);
	assert_eq!(processor.register[1], 3);
	assert_eq!(processor.pc, NEXT);

	// carry
	let mut processor = new_processor();
	processor.register[2] = 254;
	processor.decode_opcode(0x8324);
	assert_eq!(processor.register[1], 1);
	assert_eq!(processor.register[0x0f], 1);
	assert_eq!(processor.pc, NEXT);
	
    }

    #[test]
    fn test_code_8xy5() {
	// set carry
	let mut processor = new_processor();
	processor.decode_opcode(0x8205);
	assert_eq!(processor.register[2], 1);
	assert_eq!(processor.register[0x0f], 1);
	assert_eq!(processor.pc, NEXT);

	// don't set carry
	let mut processor = new_processor();
	processor.decode_opcode(0x8065);
	assert_eq!(processor.register[0], 253);
	assert_eq!(processor.register[0x0f], 0);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy6() {
	// set VF
	let mut processor = new_processor();
	processor.decode_opcode(0x8416);
	assert_eq!(processor.register[0x0f], 1);
	assert_eq!(processor.register[4], 1);
	assert_eq!(processor.pc, NEXT);

	// don't set VF
	let mut processor = new_processor();
	processor.decode_opcode(0x8216);
	assert_eq!(processor.register[0x0f], 0);
	assert_eq!(processor.register[2], 1);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xy7() {
	// set VF
	let mut processor = new_processor();
	processor.decode_opcode(0x8937);
	assert_eq!(processor.register[0x0f], 0);
	assert_eq!(processor.register[9], 253);
	assert_eq!(processor.pc, NEXT);

	// don't set VF
	let mut processor = new_processor();
	processor.decode_opcode(0x8397);
	assert_eq!(processor.register[0x0f], 1);
	assert_eq!(processor.register[3], 3);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_8xye() {
	// set VF
	let mut processor = new_processor();
	processor.register[0] = 0b10000000;
	processor.decode_opcode(0x801e);
	assert_eq!(processor.register[0x0f], 1);
	assert_eq!(processor.register[0], 0);
	assert_eq!(processor.pc, NEXT);

	// don't set VF
	let mut processor = new_processor();
	processor.decode_opcode(0x801e);
	assert_eq!(processor.register[0x0f], 0);
	assert_eq!(processor.register[0], 2);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_9xy0() {
	// not equal, so skip
	let mut processor = new_processor();
	processor.decode_opcode(0x9020);
	assert_eq!(processor.pc, SKIP);

	// equal, so go next
	let mut processor = new_processor();
	processor.decode_opcode(0x9010);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_annn() {
	let mut processor = new_processor();
	processor.decode_opcode(0xa420);
	assert_eq!(processor.index, 0x420);
    }

    #[test]
    fn test_code_bnnn() {
	let mut processor = new_processor();
	processor.register[0] = 1;
	processor.decode_opcode(0xb111);
	assert_eq!(processor.pc, 0x112);
    }

    #[test]
    fn test_code_cxkk() {
	let mut processor = new_processor();
	// AND with 0 is zero
	processor.decode_opcode(0xc000);
	assert_eq!(processor.register[0], 0);
	// AND with 0 in register[0] is still 0
	processor.decode_opcode(0xc00f);
	assert_eq!(processor.register[0] & 0xf0, 0)
    }

    #[test]
    fn test_code_dxyn() {
	let mut processor = new_processor();
	processor.index = 0;
	processor.memory[0] = 0b11111111;
	processor.memory[1] = 0b00000000;
	processor.screen[0][0] = 1;
	processor.screen[0][1] = 0;
	processor.screen[1][0] = 1;
	processor.screen[1][1] = 0;
	processor.register[0] = 0;
	processor.decode_opcode(0xd002);

	// flip on/ off
	assert_eq!(processor.screen[0][0], 0);
	assert_eq!(processor.screen[0][1], 1);
	assert_eq!(processor.screen[1][0], 1);
	assert_eq!(processor.screen[1][1], 0);
	// update happened
	assert_eq!(processor.register[0x0f], 1);
	// capture screen update
	assert_eq!(processor.draw_flag, true);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_ex9e() {
	// skip if equal
	let mut processor = new_processor();
	processor.key[9] = true;
	processor.memory[3] = 9;
	processor.decode_opcode(0xe39e);
	assert_eq!(processor.pc, SKIP);

	// dont skip
	let mut processor = new_processor();
	processor.memory[3] = 9;
	processor.decode_opcode(0xe39e);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_exa1() {
	// skip if equal
	let mut processor = new_processor();
	processor.key[9] = true;
	processor.memory[3] = 9;
	processor.decode_opcode(0xe3a1);
	assert_eq!(processor.pc, NEXT);

	// dont skip
	let mut processor = new_processor();
	processor.memory[3] = 9;
	processor.decode_opcode(0xe3a1);
	assert_eq!(processor.pc, SKIP);
    }

    #[test]
    fn test_code_fx07() {
	let mut processor = new_processor();
	processor.delay_timer = 42;
	processor.decode_opcode(0xf207);
	assert_eq!(processor.register[2], 42);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx0a() {
	let mut processor = new_processor();
	processor.decode_opcode(0xf20a);
	assert_eq!(processor.waiting_for_key, true);
	// TODO: missing some checks here?
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx15() {
	let mut processor = new_processor();
	processor.register[2] = 42;
	processor.decode_opcode(0xf215);
	assert_eq!(processor.delay_timer, 42);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx18() {
	let mut processor = new_processor();
	processor.register[2] = 42;
	processor.decode_opcode(0xf218);
	assert_eq!(processor.sound_timer, 42);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx1e() {
	let mut processor = new_processor();
	processor.index = 2;
	processor.register[4] = 42;
	processor.decode_opcode(0xf41e);
	assert_eq!(processor.index, 44);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx29() {
	let mut processor = new_processor();
	processor.register[5] = 9;
	processor.decode_opcode(0xf529);
	assert_eq!(processor.index, 45);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx33() {
	let mut processor = new_processor();
	processor.register[2] = 123;
	processor.index = 420;
	processor.decode_opcode(0xf233);
	assert_eq!(processor.memory[420], 1);
	assert_eq!(processor.memory[420 + 1], 2);
	assert_eq!(processor.memory[420 + 2], 3);
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx55() {
	let mut processor = new_processor();
	processor.index = 100;
	processor.decode_opcode(0xff55);
	// 0 to f
	for mem in 0..16 {
	    assert_eq!(processor.memory[100 + mem], processor.register[mem]);
	}
	assert_eq!(processor.pc, NEXT);
    }

    #[test]
    fn test_code_fx65() {
	let mut processor = new_processor();
	processor.index = 100;
	// 0 to f
	for location in 0..16 {
	    processor.memory[100 + location] = location as u8;
	}
	processor.decode_opcode(0xff65);
	for mem in 0..16 {
	    assert_eq!(processor.register[mem], processor.memory[100 + mem]);
	}
	assert_eq!(processor.pc, NEXT);
    }
}
