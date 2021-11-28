struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // program counterのこと. u16ではなくusizeなのは，Rustではインデックスはusizeだから
    memory: [u8; 0x1000],      // 4KB(4096bit)のRAM
    stack: [u16; 16],          // スタックは16段命令がある位置.
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2; // u8を2つ分とばす

            let c = ((opcode & 0xF000) >> 12) as u8; // 上位バイトの上位ニブル以外を0にして，12bit右にシフト
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x0F0) >> 4) as u8;
            let d = ((opcode & 0x00F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y), // 真ん中8bitにオペランド（レジスタのインデックス）がある
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // carry flag
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow")
        }

        stack[sp] = self.position_in_memory as u16; // callの次の命令がある位置
        self.stack_pointer += 1; //
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x21; mem[1] = 0x00; // 0x100の関数をコールする
    mem[2] = 0x21; mem[3] = 0x00;
    mem[4] = 0x00; mem[5] = 0x00; // 終了

    // 関数をメモリに置く
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
