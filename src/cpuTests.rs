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
        if cpu.registers.Flags !=  expectedValues.Flags {println!("Flags error {:#08b} != Expected {:#08b}", cpu.registers.Flags, expectedValues.Flags);return false}

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
    #[test] 
    fn testSta(){
        //stores 0x50 at address 0x15
        assert_eq!(true, testIndividualInstruction(vec![0x85, 0x15], 
                                                   Instruction::STA(Mode::ZeroPage),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0x50, X:0, Y:0, Flags:0},
                                                   vec![Data{address: 0x15, value: 0x50}],
                                                   Answers{PC:2, SP:0, A:0x50, X:0, Y:0, Flags:0}));

        //stores 0x50 at address 0x15+5 register
        assert_eq!(true, testIndividualInstruction(vec![0x95, 0x15], 
                                                   Instruction::STA(Mode::ZeroPageX),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0x50, X:0x5, Y:0, Flags:0},
                                                   vec![Data{address: 0x1a, value: 0x50}],
                                                   Answers{PC:2, SP:0, A:0x50, X:0x5, Y:0, Flags:0}));

        //stores 0x50 at address 0x1234
        assert_eq!(true, testIndividualInstruction(vec![0x8D, 0x34, 0x12], 
                                                   Instruction::STA(Mode::Absolute),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0x50, X:0, Y:0, Flags:0},
                                                   vec![Data{address: 0x1234, value: 0x50}],
                                                   Answers{PC:3, SP:0, A:0x50, X:0, Y:0, Flags:0}));

        assert_eq!(true, testIndividualInstruction(vec![0x9D, 0x34, 0x12], 
                                                   Instruction::STA(Mode::AbsoluteX),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0x50, X:0x5, Y:0, Flags:0},
                                                   vec![Data{address: 0x1239, value: 0x50}],
                                                   Answers{PC:3, SP:0, A:0x50, X:0x5, Y:0, Flags:0}));

   }
    #[test] 
    fn testTax(){
        assert_eq!(true, testIndividualInstruction(vec![0xAA], 
                                                   Instruction::TAX(Mode::Implicit),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0x50, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:1, SP:0, A:0x50, X:0x50, Y:0, Flags:0})); 
    }
    #[test] 
    fn testPHA(){
        assert_eq!(true, testIndividualInstruction(vec![0x48], 
                                                   Instruction::PHA(Mode::Implicit),
                                                   vec![],
                                                   Registers{PC:0, SP:0x10, A:0x50, X:0, Y:0, Flags:0},
                                                   vec![Data{address:0x0110,value:0x50}],
                                                   Answers{PC:1, SP:0x0f, A:0x50, X:0, Y:0, Flags:0})); 
    }
    #[test] 
    fn testPLA(){
        assert_eq!(true, testIndividualInstruction(vec![0x68], 
                                                   Instruction::PLA(Mode::Implicit),
                                                   vec![Data{address:0x0111,value:0x50}],
                                                   Registers{PC:0, SP:0x11, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:1, SP:0x12, A:0x50, X:0, Y:0, Flags:0})); 
    }
 #[test] 
    fn testAND(){
        assert_eq!(true, testIndividualInstruction(vec![0x29,0b0000_0110], 
                                                   Instruction::AND(Mode::Immediate),
                                                   vec![],
                                                   Registers{PC:0, SP:0x0, A:0b0000_1111, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0b0000_0110, X:0, Y:0, Flags:0})); 
    }
    #[test] 
    fn testBIT(){
        assert_eq!(true, testIndividualInstruction(vec![0x24,0x20], 
                                                   Instruction::BIT(Mode::ZeroPage),
                                                   vec![Data{address: 0x20, value: 0b0110_0000}],
                                                   Registers{PC:0, SP:0x0, A:0b0110_0000, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0b0110_0000, X:0, Y:0, Flags:0b0110_0000})); 
    }
    #[test] 
    fn testADC(){
        assert_eq!(true, testIndividualInstruction(vec![0x65,0x20], 
                                                   Instruction::ADC(Mode::ZeroPage),
                                                   vec![Data{address: 0x20, value: 1}],
                                                   Registers{PC:0, SP:0x0, A:0b0000_1111, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:0b0001_0000, X:0, Y:0, Flags:0})); 
        //test overflow
        assert_eq!(true, testIndividualInstruction(vec![0x65,0x20], 
                                                   Instruction::ADC(Mode::ZeroPage),
                                                   vec![Data{address: 0x20, value: 0x01}],
                                                   Registers{PC:0, SP:0x0, A:0x7f, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:128, X:0, Y:0, Flags:0b00100000})); 
        assert_eq!(true, testIndividualInstruction(vec![0x65,0x20], 
                                                   Instruction::ADC(Mode::ZeroPage),
                                                   vec![Data{address: 0x20, value: 0x80}],
                                                   Registers{PC:0, SP:0x0, A:0xff, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:2, SP:0, A:127, X:0, Y:0, Flags:0b00100001})); 
    }


    #[test] 
    fn testJSR(){
        //pushes current pc to the stack
        //sets the pc to the target memory address
        assert_eq!(true, testIndividualInstruction(vec![0x20,0x34, 0x12], 
                                                   Instruction::JSR(Mode::Absolute),
                                                   vec![Data{address: 0x1234, value: 1}],
                                                   Registers{PC:0, SP:0x0, A:0, X:0, Y:0, Flags:0},
                                                   vec![Data{address:0x0100-1, value: 0x2}],
                                                   Answers{PC:0x1234, SP:0x2, A:0, X:0, Y:0, Flags:0})); 
    }

    #[test] 
    fn testRTS(){
        assert_eq!(true, testIndividualInstruction(vec![0x60], 
                                                   Instruction::RTS(Mode::Implicit),
                                                   vec![Data{address: 0x00fe, value: 5}],
                                                   Registers{PC:0, SP:0x2, A:0, X:0, Y:0, Flags:0},
                                                   vec![],
                                                   Answers{PC:0x4, SP:0x0, A:0, X:0, Y:0, Flags:0})); 
    }
    #[test] 
    fn testBCC(){
        assert_eq!(true, testIndividualInstruction(vec![0x90, !(0x02) + 1], 
                                                   Instruction::BCC(Mode::Relative),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0b0000_0000},
                                                   vec![],
                                                   Answers{PC:0x0, SP:0x0, A:0, X:0, Y:0, Flags:0b0000_0000})); 
        assert_eq!(true, testIndividualInstruction(vec![0x90, 0x10], 
                                                   Instruction::BCC(Mode::Relative),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0b0000_0000},
                                                   vec![],
                                                   Answers{PC:0x12, SP:0x0, A:0, X:0, Y:0, Flags:0b0000_0000})); 
    }
    #[test] 
    fn testNOP(){
        assert_eq!(true, testIndividualInstruction(vec![0xEA], 
                                                   Instruction::NOP(Mode::Implicit),
                                                   vec![],
                                                   Registers{PC:0, SP:0, A:0, X:0, Y:0, Flags:0b0000_0000},
                                                   vec![],
                                                   Answers{PC:0x1, SP:0x0, A:0, X:0, Y:0, Flags:0b0000_0000})); 

    }

    #[test] 
    fn testNESRom(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/nestest.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        cpu.run();
    }
    #[test] 
    fn testRun(){
        let f = File::open("/home/kai/Projects/nesVM/src/files/mario.nes").expect("File could not open");
        let mut cpu = CPU::new(f); 
        let snakeGame = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
        0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
        0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
        0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
        0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
        0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
        0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
        0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
        0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
        0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
        0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
        0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
        0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
        0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
        0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
        0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
        0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
        0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
        0xea, 0xca, 0xd0, 0xfb, 0x60
        ];
        cpu.program=snakeGame;
        cpu.run();
    }

}

