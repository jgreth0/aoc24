
// https://adventofcode.com/2024/day/17

use scanf::sscanf;

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
struct Regfile {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Bit {
    A(u8),
    AInv(u8),
    Zero,
    One,
}

impl Bit {
    fn inv(&self) -> Bit {
        match self {
            Bit::A(b) => Bit::AInv(*b),
            Bit::AInv(b) => Bit::A(*b),
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BitVec {
    bits: [Bit; 128],
}

impl BitVec {
    fn from_const(val: u128) -> BitVec {
        let mut bits = [Bit::Zero; 128];
        let mut val = val;
        let mut offset = 0;
        while val != 0 {
            if val & 1 == 1 {
                bits[offset] = Bit::One;
            }
            offset += 1;
            val >>= 1;
        }
        BitVec {
            bits,
        }
    }

    fn export(&self) -> Result<u8, u8> {
        let mut val = 0;
        for bit in self.bits.iter().rev() {
            val <<= 1;
            match bit {
                Bit::One => { val |= 1; },
                Bit::Zero => {},
                Bit::A(i) => { return Err(*i); },
                Bit::AInv(i) => { return Err(*i); },
            }
        }
        Ok(val)
    }

    fn shift_right(&self, shift: &BitVec) -> Result<Self, u8> {
        let mut shift = shift.export()?;
        let mut bits = [Bit::Zero; 128];
        let mut offset = 0;
        for bit in self.bits.iter() {
            if shift != 0 {
                shift -= 1;
                continue;
            }
            bits[offset] = *bit;
            offset += 1;
        }
        Ok(BitVec { bits, })
    }

    fn xor(&self, op: &BitVec) -> Result<Self, u8> {
        let mut bits = [Bit::Zero; 128];
        for (i, bit) in bits.iter_mut().enumerate() {
            match (self.bits[i], op.bits[i]) {
                (Bit::Zero, b) => *bit = b,
                (b, Bit::Zero) => *bit = b,
                (Bit::One, b) => *bit = b.inv(),
                (b, Bit::One) => *bit = b.inv(),
                (Bit::A(my_offset), Bit::A(op_offset)) |
                        (Bit::AInv(my_offset), Bit::AInv(op_offset)) => {
                    if my_offset == op_offset {
                        *bit = Bit::Zero;
                    } else {
                        return Err(std::cmp::min(my_offset, op_offset));
                    }
                },
                (Bit::AInv(my_offset), Bit::A(op_offset)) |
                        (Bit::A(my_offset), Bit::AInv(op_offset)) => {
                    if my_offset == op_offset {
                        *bit = Bit::One;
                    } else {
                        return Err(std::cmp::min(my_offset, op_offset));
                    }
                },
            }
        }
        Ok(BitVec { bits, })
    }

    fn mask7(&self) -> Self {
        let mut bits = [Bit::Zero; 128];
        bits[..3].copy_from_slice(&self.bits[..3]);
        BitVec { bits, }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BitRegfile {
    a: BitVec,
    b: BitVec,
    c: BitVec,
    pc: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ComboOperand {
    L0,
    L1,
    L2,
    L3,
    RegA,
    RegB,
    RegC,
    Rsv,
}

impl ComboOperand {
    fn from(c: char) -> Self {
        match c {
            '0' => ComboOperand::L0,
            '1' => ComboOperand::L1,
            '2' => ComboOperand::L2,
            '3' => ComboOperand::L3,
            '4' => ComboOperand::RegA,
            '5' => ComboOperand::RegB,
            '6' => ComboOperand::RegC,
            '7' => ComboOperand::Rsv,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn load(&self, rf: &Regfile) -> u64 {
        match self {
            ComboOperand::L0 => 0,
            ComboOperand::L1 => 1,
            ComboOperand::L2 => 2,
            ComboOperand::L3 => 3,
            ComboOperand::RegA => rf.a,
            ComboOperand::RegB => rf.b,
            ComboOperand::RegC => rf.c,
            ComboOperand::Rsv => panic!("Invalid combo operand"),
        }
    }

    fn load_bits(&self, rf: &BitRegfile) -> BitVec {
        match self {
            ComboOperand::L0 => BitVec::from_const(0),
            ComboOperand::L1 => BitVec::from_const(1),
            ComboOperand::L2 => BitVec::from_const(2),
            ComboOperand::L3 => BitVec::from_const(3),
            ComboOperand::RegA => rf.a,
            ComboOperand::RegB => rf.b,
            ComboOperand::RegC => rf.c,
            ComboOperand::Rsv => panic!("Invalid combo operand"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LiteralOperand {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
}

impl LiteralOperand {
    fn from(c: char) -> Self {
        match c {
            '0' => LiteralOperand::L0,
            '1' => LiteralOperand::L1,
            '2' => LiteralOperand::L2,
            '3' => LiteralOperand::L3,
            '4' => LiteralOperand::L4,
            '5' => LiteralOperand::L5,
            '6' => LiteralOperand::L6,
            '7' => LiteralOperand::L7,
            _ => panic!("Invalid literal operand"),
        }
    }
    fn load(&self) -> u64 {
        match self {
            LiteralOperand::L0 => 0,
            LiteralOperand::L1 => 1,
            LiteralOperand::L2 => 2,
            LiteralOperand::L3 => 3,
            LiteralOperand::L4 => 4,
            LiteralOperand::L5 => 5,
            LiteralOperand::L6 => 6,
            LiteralOperand::L7 => 7,
        }
    }
    fn load_bits(&self) -> BitVec {
        match self {
            LiteralOperand::L0 => BitVec::from_const(0),
            LiteralOperand::L1 => BitVec::from_const(1),
            LiteralOperand::L2 => BitVec::from_const(2),
            LiteralOperand::L3 => BitVec::from_const(3),
            LiteralOperand::L4 => BitVec::from_const(4),
            LiteralOperand::L5 => BitVec::from_const(5),
            LiteralOperand::L6 => BitVec::from_const(6),
            LiteralOperand::L7 => BitVec::from_const(7),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    // jump from pointers and operands
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn from(i: char, o: char) -> Self {
        match i {
            '0' => Instruction::Adv(ComboOperand::from(o)),
            '1' => Instruction::Bxl(LiteralOperand::from(o)),
            '2' => Instruction::Bst(ComboOperand::from(o)),
            '3' => Instruction::Jnz(LiteralOperand::from(o)),
            '4' => Instruction::Bxc,
            '5' => Instruction::Out(ComboOperand::from(o)),
            '6' => Instruction::Bdv(ComboOperand::from(o)),
            '7' => Instruction::Cdv(ComboOperand::from(o)),
            _ => panic!("Invalid instruction"),
        }
    }

    fn exec(&self, rf: &mut Regfile) -> Option<char> {
        rf.pc += 2;
        match self {
            Instruction::Adv(o) => {
                rf.a >>= o.load(rf);
            },
            Instruction::Bxl(o) => {
                rf.b ^= o.load();
            },
            Instruction::Bst(o) => {
                rf.b = o.load(rf) % 8;
            },
            Instruction::Jnz(o) => {
                if rf.a != 0 {
                    rf.pc = o.load() as usize;
                }
            },
            Instruction::Bxc => {
                rf.b ^= rf.c;
            },
            Instruction::Out(o) => {
                return char::from_digit((o.load(rf) % 8) as u32, 10);
            },
            Instruction::Bdv(o) => {
                rf.b = rf.a >> o.load(rf);
            },
            Instruction::Cdv(o) => {
                rf.c = rf.a >> o.load(rf);
            },
        }
        None
    }

    fn exec_brf(&self, rf: &mut BitRegfile) -> Result<Option<[Bit; 3]>, u8> {
        rf.pc += 2;
        match self {
            Instruction::Adv(o) => {
                rf.a = rf.a.shift_right(&o.load_bits(rf))?;
            },
            Instruction::Bxl(o) => {
                rf.b = rf.b.xor(&o.load_bits())?;
            },
            Instruction::Bst(o) => {
                rf.b = o.load_bits(rf).mask7();
            },
            Instruction::Jnz(o) => {
                // Handle the condition externally
                rf.pc = o.load() as usize;
            },
            Instruction::Bxc => {
                rf.b = rf.b.xor(&rf.c)?;
            },
            Instruction::Out(o) => {
                let mut res = [Bit::Zero; 3];
                res.copy_from_slice(&o.load_bits(rf).bits[..3]);
                return Ok(Some(res));
            },
            Instruction::Bdv(o) => {
                rf.b = rf.a.shift_right(&o.load_bits(rf))?;
            },
            Instruction::Cdv(o) => {
                rf.c = rf.a.shift_right(&o.load_bits(rf))?;
            },
        }
        Ok(None)
    }
}

struct Simulator {
    regfile: Regfile,
    program: Vec<Instruction>,
    program_str: Vec<u8>,
}

impl Simulator {
    pub fn from(input: &str) -> Self {
        let mut regfile = Regfile::default();
        let mut program = Vec::new();
        let mut program_str: Vec<u8> = Vec::new();
        let mut lines = input.lines();

        let mut r = 0;
        sscanf!(lines.next().unwrap(), "Register A: {}", r).expect("Failed to parse");
        regfile.a = r;
        sscanf!(lines.next().unwrap(), "Register B: {}", r).expect("Failed to parse");
        regfile.b = r;
        sscanf!(lines.next().unwrap(), "Register C: {}", r).expect("Failed to parse");
        regfile.c = r;

        lines.next();
        let mut chars = lines.next().unwrap().chars();
        for c in chars.by_ref() {
            if c == ' ' {
                break;
            }
        }
        let mut i = chars.next().unwrap();
        program_str.push(i.to_digit(10).unwrap() as u8);
        for o in chars {
            if !o.is_ascii_digit() {
                continue;
            }
            program.push(Instruction::from(i, o));
            program_str.push(o.to_digit(10).unwrap() as u8);
            i = o;
        }

        Simulator {
            regfile,
            program,
            program_str,
        }
    }

    fn exec(&mut self) -> String {
        let mut out = String::new();
        let mut rf = self.regfile;
        while let Some(i) = self.program.get(rf.pc) {
            if let Some(c) = i.exec(&mut rf) {
                if !out.is_empty() {
                    out.push(',');
                }
                out.push(c);
            }
        }
        out
    }

    // Given RegA which is partially populated with bits, populate the remaining bits in a way that solves the problem.
    fn try_find_a(&self, a: [Bit; 128]) -> Option<u128> {
        let mut a_pop = a;
        let mut brf = BitRegfile {
            a: BitVec { bits : a, },
            b: BitVec::from_const(self.regfile.b as u128),
            c: BitVec::from_const(self.regfile.c as u128),
            pc: self.regfile.pc,
        };

        let mut offset = 0;
        while let Some(i) = self.program.get(brf.pc) {
            match i.exec_brf(&mut brf) {
                Err(b) => {
                    if a == a_pop {
                        // There was no change in this attempt. Now we must use
                        // trial and error.
                        debug_assert!(a_pop[b as usize] == Bit::A(b));
                        a_pop[b as usize] = Bit::Zero;
                        let res1 = self.try_find_a(a_pop);
                        a_pop[b as usize] = Bit::One;
                        let res2 = self.try_find_a(a_pop);
                        match (res1, res2) {
                            (Some(r1), Some(r2)) => {
                                return Some(std::cmp::min(r1, r2));
                            },
                            (Some(r1), _) => {
                                return Some(r1);
                            }
                            (_, Some(r2)) => {
                                return Some(r2);
                            },
                            _ => { 
                                return None;
                            },
                        }
                    } else {
                        // More info has been gathered. Try again with the new info.
                        return self.try_find_a(a_pop);
                    }
                },
                Ok(Some(bits)) => {
                    match self.program_str.get(offset) {
                        None => return None,
                        Some(byte) => {
                            let mut byte = *byte;
                            for i in 0..3 {
                                match (byte & 1, bits[i as usize]) {
                                    (0, Bit::One) | (1, Bit::Zero) => { return None; },
                                    (0, Bit::A(p)) | (1, Bit::AInv(p)) => {
                                        if a_pop[p as usize] == Bit::One { return None; }
                                        a_pop[p as usize] = Bit::Zero;
                                    },
                                    (1, Bit::A(p)) | (0, Bit::AInv(p)) => {
                                        if a_pop[p as usize] == Bit::Zero { return None; }
                                        a_pop[p as usize] = Bit::One;
                                    },
                                    _ => {},
                                }
                                byte >>= 1;
                            }
                        }
                    }
                    offset += 1;
                },
                Ok(None) => {
                    if let Instruction::Jnz(_) = *i {
                        // TODO: Handle the general case. This only works when jnz is the last instruction.
                        if offset == self.program_str.len() {
                            let mut res = 0;
                            for (i, b) in a_pop.iter().enumerate() {
                                if *b == Bit::One {
                                    res += 1 << i;
                                }
                            }
                            return Some(res);
                        }
                    }
                },
            }

        }
        Some(0)
    }

    fn find_a(&self) -> u128 {
        let mut a = [Bit::Zero; 128];
        for i in 0..128 {
            a[i as usize] = Bit::A(i);
        }
        self.try_find_a(a).unwrap()
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    Simulator::from(input).exec()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u128 {
    Simulator::from(input).find_a()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0";

    static TEST_INPUT_B: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(TEST_INPUT_A));

        assert_eq!("7,0,3,1,2,6,3,7,1", part1(include_str!("../input/2024/day17.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(117440, part2(TEST_INPUT_B));

        assert_eq!(109020013201563, part2(include_str!("../input/2024/day17.txt")));
    }
}
