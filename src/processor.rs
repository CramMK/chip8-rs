extern crate sdl2;

use rand::Rng;

use crate::fontset::FONT;

const MEMORY_SIZE: usize = 4096;
const GAME_ENTRY: usize = 0x200; // most games load into 0x200
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const OPCODE_SIZE: usize = 2;

pub struct Processor {
    memory: [u8; MEMORY_SIZE],
    register: [u8; 16],
    index: usize, // used to store memory addresses
    pc: usize, // programm counter
    screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
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
	let mut mem = [0; MEMORY_SIZE];
	// load font
	for (pos, &val) in FONT.iter().enumerate() {
	    mem[pos] = val;
	}

	Processor {
	    memory: mem,
	    register: [0; 16],
	    index: 0,
	    pc: GAME_ENTRY,
	    screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
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

    // TODO: cartridge needed
    pub fn start(&mut self) {
	let sdl_ctx = sdl2::init().unwrap();

	loop {
	    // get keyinput using sdl2

	    // emulate one cycle
	    self.cycle();

	    // draw to screen using sdl2
	    if self.draw_flag {
	    }

	    // play sound using sdl2

	    // add delay
	}
    }

    pub fn cycle(&mut self) {

	// reset
	self.draw_flag = false;

	// opcode FX0A holds the program, until a key is pressed
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
	// load game
	for (pos, &val) in game.iter().enumerate() {
	    let position = GAME_ENTRY + pos;
	    if position < MEMORY_SIZE { // don't go above mem limit
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

	// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
	let nibbles = (
	    (opcode & 0xF000 >> 12) as usize,
	    (opcode & 0x0F00 >> 8) as usize,
	    (opcode & 0x00F0 >> 4) as usize,
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
	self.screen = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
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
	self.sp += 1;
	self.stack[self.sp] = self.pc;
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
	self.register[x] = self.register[x] + kk;
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
	self.register[x] = (x_val - y_val) as u8;
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
	self.register[x] = self.register[y] - self.register[x];
	self.pc += OPCODE_SIZE;
    }

    // Set Vx = Vx SHL 1
    fn code_8xye(&mut self, x: usize, _y: usize) {
	self.register[0x0f] = (self.register[x] & 0b10000000) >> 7;
	self.register[x] = self.register[x] * 2;
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
	// TODO
	self.register[0x0f] = 0;
	for byte in 0..n {
	    // get y coord, which we want to draw -> use modulo, so we don't overlap
	    let y = (self.register[y] as usize + byte) % SCREEN_HEIGHT;
	    for bit in 0..8 {
		// get x coord, just as above
		let x = (self.register[x] as usize + bit) % SCREEN_WIDTH;
		// bit hack to get every bit in a row
		let pixel_to_draw = (self.memory[self.index + byte] >> (7 - bit)) & 1;
		// check if we will overwrite an existing pixel
		self.register[0x0f] = self.register[0x0f] | (pixel_to_draw & self.screen[x][y]);
		self.screen[x][y] = self.screen[x][y] ^ pixel_to_draw; 
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
	self.pc = OPCODE_SIZE;
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
	for reg_i in 0..x {
	    self.memory[self.index + reg_i] = self.register[reg_i];
	}
	self.pc += OPCODE_SIZE;
    }

    // Read registers V0 through Vx from memory starting at location I
    fn code_fx65(&mut self, x: usize) {
	for reg_i in 0..x {
	    self.register[reg_i] = self.memory[self.index + reg_i];
	}
	self.pc += OPCODE_SIZE;
    }
}
