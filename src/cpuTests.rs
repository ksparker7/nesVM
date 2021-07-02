use std ::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io;
use crate::instructions::Instruction;
use crate::instructions::Mode;
use crate::memory::Memory;
use crate::cpu::CPU;
use crate::cpu::Registers;

#[cfg(test)]
mod cpuTests{
    use super::*;
    #[test]
    fn testCpuCreation(){
        let f = File::open("/home/kai/Projects/nesVM/src").expect("File could not open");
        let cpu = CPU::new(f); 
        assert_eq!(cpu.registers.PC, 0);
        assert_eq!(cpu.registers.SP, 0);
        assert_eq!(cpu.registers.A, 0);
        assert_eq!(cpu.registers.X, 0);
        assert_eq!(cpu.registers.Y, 0);
        assert_eq!(cpu.registers.Flags, 0);
    }

    #[test]
    fn testReadByteFromFile(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
         
        let mut cpu = CPU::new(f); 
        let byte = cpu.readByte();
        assert_eq!(0x4e, byte);
    }
    #[test]
    fn testLdaNonNegativeExecute(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
         
        let mut cpu = CPU::new(f); 
        cpu.program = [0xa9, 0b1010_1010].to_vec();
        let i = cpu.fetchAndDecodeInstruction();
        cpu.executeInstruction(i);
        assert_eq!(i, Instruction::LDA(Mode::Immediate));
        assert_eq!(2, cpu.registers.PC);
        //check a register is 0b1010_1010
        assert_eq!(cpu.registers.A, 0b1010_1010 );
        //check that the negative flag is set
        assert_eq!(cpu.registers.Flags, 0b0100_0000);
    }


    fn testIndividualInstruction(program: Vec<u8>, 
                                 instruction: Instruction,  
                                 writeData: Vec<Data>, 
                                 reg: Registers, 
                                 memoryExpectedValues: Vec<Data>,
                                 expectedValues: Answers) -> bool{

        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        cpu.program = program; 
        
        //set initial cpu registers
        cpu.registers.PC        = reg.PC;
        cpu.registers.SP        = reg.SP;
        cpu.registers.A         = reg.A; 
        cpu.registers.X         = reg.X;
        cpu.registers.Y         = reg.Y;
        cpu.registers.Flags     = reg.Flags;

        if !writeData.is_empty() {
            for memVal in writeData {
                cpu.memory.memoryWriteShort(memVal.address, memVal.value);
            }
        }


        let inst = cpu.fetchAndDecodeInstruction();
        cpu.executeInstruction(inst);

        //memoryWrite[0] is the address to write to on cpu
        //memoryWrite[1] is the value to write at the address
        //this conditional tests if the value passed

        //test all register pectedValues
        if inst != instruction                          {println!("Instruction error ");return false}
        if cpu.registers.PC    !=  expectedValues.PC    {println!("PC error {} != Expected {}",    cpu.registers.PC,    expectedValues.PC   );return false}
        if cpu.registers.SP    !=  expectedValues.SP    {println!("SP error {} != Expected {}",    cpu.registers.SP,    expectedValues.SP   );return false}
        if cpu.registers.A     !=  expectedValues.A     {println!("A error {} != Expected {}",     cpu.registers.A,     expectedValues.A    );return false}
        if cpu.registers.X     !=  expectedValues.X     {println!("X error {} != Expected {}",     cpu.registers.X,     expectedValues.X    );return false}
        if cpu.registers.Y     !=  expectedValues.Y     {println!("Y error {} != Expected {}",     cpu.registers.Y,     expectedValues.Y    );return false}
        if cpu.registers.Flags !=  expectedValues.Flags {println!("Flags error {} != Expected {}", cpu.registers.Flags, expectedValues.Flags);return false}

        //check expected memory values
        if !memoryExpectedValues.is_empty() {
            for memVal in memoryExpectedValues {
               if cpu.memory.memoryReadShort(memVal.address) != memVal.value {
                    println!("Memory value read error at location: {:x?}. Expected {:x} but found {:x}", memVal.address, memVal.value, cpu.memory.memoryReadShort(memVal.address));
                    return false;
               } 
            }
        }

        return true 
        
    }
    pub struct Data{
        address: u16,
        value: u16,
    }
    pub struct Answers{
        pub PC: u16,
        pub SP: u8,
        pub A: u8,
        pub X: u8,
        pub Y: u8,
        pub Flags: u8,
    }

    #[test] 
    fn testLda(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        assert_eq!(true, testIndividualInstruction(vec![0xa9, 0b1010_1010], 
                                                   Instruction::LDA(Mode::Immediate), 
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0b1010_1010, X:0, Y:0, Flags:0b0100_0000}));

        assert_eq!(true, testIndividualInstruction(vec![0xa5, 0x15], 
                                                   Instruction::LDA(Mode::ZeroPage),
                                                   vec![Data{address: 0x15, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0x44, X:0, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xb5, 0x15], 
                                                   Instruction::LDA(Mode::ZeroPageX),
                                                   vec![Data{address: 0x1a, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:5, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0x44, X:5, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xad, 0x34, 0x12], 
                                                   Instruction::LDA(Mode::Absolute),
                                                   vec![Data{address: 0x1234, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0x44, X:0, Y:0, Flags:0}));


        assert_eq!(true, testIndividualInstruction(vec![0xbd, 0x34, 0x12], 
                                                   Instruction::LDA(Mode::AbsoluteX),
                                                   vec![Data{address: 0x1234+0x10, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0x10, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0x44, X:0x10, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xb9, 0x34, 0x12], 
                                                   Instruction::LDA(Mode::AbsoluteY),
                                                   vec![Data{address: 0x1234+0x10, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0x10, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0x44, X:0, Y:0x10, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xa1, 0x20], 
                                                   Instruction::LDA(Mode::IndirectX),
                                                   vec![Data{address: 0x24, value: 0x7420}, Data{address:0x7420, value: 0x11}],
                                                   Registers{PC:0, SP:0, A:0, X:0x4, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0x11, X:0x4, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xb1, 0x86], 
                                                   Instruction::LDA(Mode::IndirectY),
                                                   vec![Data{address: 0x86, value: 0x7420}, Data{address:0x7424, value: 0x11}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0x4, Flags:0},
                                                   vec![Data{address: 0x7424, value: 0x11}],
                                                   Answers{PC:2, SP:0, A:0x11, X:0, Y:0x4, Flags:0}));
     }

    #[test] 
    fn testLdx(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        assert_eq!(true, testIndividualInstruction(vec![0xa2, 0b1010_1010], 
                                                   Instruction::LDX(Mode::Immediate), 
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:0b1010_1010, Y:0, Flags:0b0100_0000}));

        assert_eq!(true, testIndividualInstruction(vec![0xa6, 0x15], 
                                                   Instruction::LDX(Mode::ZeroPage),
                                                   vec![Data{address: 0x15, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:0x44, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xb6, 0x15], 
                                                   Instruction::LDX(Mode::ZeroPageY),
                                                   vec![Data{address: 0x1a, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:5, Y:5, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:0x44, Y:5, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xae, 0x34, 0x12], 
                                                   Instruction::LDX(Mode::Absolute),
                                                   vec![Data{address: 0x1234, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0, X:0x44, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xbe, 0x34, 0x12], 
                                                   Instruction::LDX(Mode::AbsoluteY),
                                                   vec![Data{address: 0x1234+0x10, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0x10, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0, X:0x44, Y:0x10, Flags:0}));


   }
    #[test] 
    fn testLdy(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        assert_eq!(true, testIndividualInstruction(vec![0xa0, 0b1010_1010], 
                                                   Instruction::LDY(Mode::Immediate), 
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:0, Y:0b1010_1010, Flags:0b0100_0000}));

        assert_eq!(true, testIndividualInstruction(vec![0xa4, 0x15], 
                                                   Instruction::LDY(Mode::ZeroPage),
                                                   vec![Data{address: 0x15, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:0, Y:0x44, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xb4, 0x15], 
                                                   Instruction::LDY(Mode::ZeroPageX),
                                                   vec![Data{address: 0x1a, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:5, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0, X:5, Y:0x44, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xac, 0x34, 0x12], 
                                                   Instruction::LDY(Mode::Absolute),
                                                   vec![Data{address: 0x1234, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0, X:0, Y:0x44, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0xbc, 0x34, 0x12], 
                                                   Instruction::LDY(Mode::AbsoluteX),
                                                   vec![Data{address: 0x1234+0x10, value: 0x44}],
                                                   Registers{PC:0, SP:0, A:0, X:0x10, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:3, SP:0, A:0, X:0x10, Y:0x44, Flags:0}));


   }
}

