use std::{
    char::from_u32,
    io::{stdin, stdout, Read, Write},
    process::exit,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,   //Malloc
    UnmapSegment, //Free
    Output,       //Print
    Input,        //Get
    LoadProgram,  //Goto
    LoadValue,    //Load Literal
}

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

#[inline]
fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

#[inline]
fn get(field: &Field, instruction: u32) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn run(instructions: Vec<u32>) {
    let mut pcounter: u32 = 0;
    let mut registers: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut memory: Vec<Vec<u32>> = vec![vec![0]];
    let mut id_pool: Vec<u32> = Vec::new();
    let mut max_id: u32 = 0;
    // let mut halt: bool = false;
    memory[0] = instructions;

    loop {
        let instruction = memory[0][pcounter as usize];
        match get(&OP, memory[0][pcounter as usize]) {
            o if o == Opcode::CMov as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                if registers[rc as usize] != 0 {
                    registers[ra as usize] = registers[rb as usize];
                }
                pcounter += 1;
            }
            o if o == Opcode::Load as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                registers[ra as usize] =
                    memory[registers[rb as usize] as usize][registers[rc as usize] as usize];
                pcounter += 1;
            }
            o if o == Opcode::Store as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                memory[registers[ra as usize] as usize][registers[rb as usize] as usize] =
                    registers[rc as usize];
                pcounter += 1;
            }
            o if o == Opcode::Add as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                registers[ra as usize] =
                    registers[rb as usize].wrapping_add(registers[rc as usize]);
                pcounter += 1;
            }
            o if o == Opcode::Mul as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                registers[ra as usize] =
                    registers[rb as usize].wrapping_mul(registers[rc as usize]);
                pcounter += 1;
            }
            o if o == Opcode::Div as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                registers[ra as usize] = registers[rb as usize] / registers[rc as usize];
                pcounter += 1;
            }
            o if o == Opcode::Nand as u32 => {
                let ra = get(&RA, instruction);
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                registers[ra as usize] = !(registers[rb as usize] & registers[rc as usize]);
                pcounter += 1;
            }
            o if o == Opcode::Halt as u32 => {
                halt();
            }
            o if o == Opcode::MapSegment as u32 => {
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                let address: u32;
                if id_pool.is_empty() {
                    max_id += 1;
                    address = max_id;
                    memory.push(vec![0; registers[rc as usize] as usize]);
                } else {
                    address = id_pool.pop().unwrap();
                    memory[address as usize] = vec![0; registers[rc as usize] as usize];
                }
                registers[rb as usize] = address;
                pcounter += 1;
            }
            o if o == Opcode::UnmapSegment as u32 => {
                let rc = get(&RC, instruction);
                id_pool.push(registers[rc as usize]);
                pcounter += 1;
            }
            o if o == Opcode::Output as u32 => {
                let rc = get(&RC, instruction);
                let value = registers[rc as usize];
                if value > 255 {
                    eprintln!("Invalid character");
                } else {
                    let charvalue = from_u32(value).unwrap() as u8;
                    stdout().write(&[charvalue]).unwrap();
                }
                pcounter += 1;
            }
            o if o == Opcode::Input as u32 => {
                let rc = get(&RC, instruction);
                match stdin().bytes().next() {
                    Some(value) => {
                        registers[rc as usize] = value.unwrap() as u32;
                    }
                    None => registers[rc as usize] = !0 as u32,
                };
                pcounter += 1;
            }
            o if o == Opcode::LoadProgram as u32 => {
                let rb = get(&RB, instruction);
                let rc = get(&RC, instruction);
                if registers[rb as usize] != 0 {
                    memory[0] = memory[registers[rb as usize] as usize].clone();
                }
                pcounter = registers[rc as usize];
            }
            o if o == Opcode::LoadValue as u32 => {
                let rl = get(&RL, instruction);
                let vl = get(&VL, instruction);
                registers[rl as usize] = vl;
                pcounter += 1;
            }

            _ => {
                halt();
            }
        }
    }
}
fn cmov(instruction: u32, registers: &mut [u32; 8]) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    if registers[rc as usize] != 0 {
        registers[ra as usize] = registers[rb as usize];
    }
}
fn load(instruction: u32, registers: &mut [u32; 8], memory: &mut Vec<Vec<u32>>) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    registers[ra as usize] =
        memory[registers[rb as usize] as usize][registers[rc as usize] as usize];
}
fn store(instruction: u32, registers: &mut [u32; 8], memory: &mut Vec<Vec<u32>>) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    memory[registers[ra as usize] as usize][registers[rb as usize] as usize] =
        registers[rc as usize];
}
fn add(instruction: u32, registers: &mut [u32; 8]) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    registers[ra as usize] = registers[rb as usize].wrapping_add(registers[rc as usize]);
}
fn mul(instruction: u32, registers: &mut [u32; 8]) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    registers[ra as usize] = registers[rb as usize].wrapping_mul(registers[rc as usize]);
}
fn div(instruction: u32, registers: &mut [u32; 8]) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    registers[ra as usize] = registers[rb as usize] / registers[rc as usize];
}
fn nand(instruction: u32, registers: &mut [u32; 8]) {
    let ra = get(&RA, instruction);
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    registers[ra as usize] = !(registers[rb as usize] & registers[rc as usize]);
}
fn halt() {
    exit(0);
}
fn map(
    instruction: u32,
    registers: &mut [u32; 8],
    memory: &mut Vec<Vec<u32>>,
    id_pool: &mut Vec<u32>,
    max_id: &mut u32,
) {
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);
    let address: u32;
    if id_pool.is_empty() {
        *max_id += 1;
        address = *max_id;
        memory.push(vec![0; registers[rc as usize] as usize]);
    } else {
        address = id_pool.pop().unwrap();
        memory[address as usize] = vec![0; registers[rc as usize] as usize];
    }
    registers[rb as usize] = address;
}
fn unmap(instruction: u32, registers: &mut [u32; 8], id_pool: &mut Vec<u32>) {
    let rc = get(&RC, instruction);
    id_pool.push(registers[rc as usize]);
}
fn output(instruction: u32, registers: &mut [u32; 8]) {
    let rc = get(&RC, instruction);
    let value = registers[rc as usize];
    if value > 255 {
        eprintln!("Invalid character");
    } else {
        let charvalue = from_u32(value).unwrap() as u8;
        stdout().write(&[charvalue]).unwrap();
    }
}
fn input(instruction: u32, registers: &mut [u32; 8]) {
    let rc = get(&RC, instruction);
    match stdin().bytes().next() {
        Some(value) => {
            registers[rc as usize] = value.unwrap() as u32;
        }
        None => registers[rc as usize] = !0 as u32,
    };
}
fn loadp(
    instruction: u32,
    registers: &mut [u32; 8],
    memory: &mut Vec<Vec<u32>>,
    pcounter: &mut u32,
) {
    let rb = get(&RB, instruction);
    let rc = get(&RC, instruction);

    if registers[rb as usize] != 0 {
        memory[0] = memory[registers[rb as usize] as usize].clone();
    }
    *pcounter = registers[rc as usize];
}
fn loadv(instruction: u32, registers: &mut [u32; 8]) {
    let rl = get(&RL, instruction);
    let vl = get(&VL, instruction);
    registers[rl as usize] = vl;
}
