use std ::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io;
use crate::instructions::Instruction;
use crate::instructions::Mode;
use crate::memory::Memory;





//Flags register
//Carry Flag (C):             0000_0001
//Zero Flag (Z):              0000_0010
//Interrupt Disable Flag (I): 0000_0100
//Decimal Mode Flag (D):      0000_1000
//Break Command (B):          0001_0000
//Overflow Flag (V):          0010_0000
//Negative Flag (N):          0100_0000
pub struct Registers{
    pub PC: u16,
    pub SP: u8,
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub Flags: u8,
}

impl Registers{
    pub fn new() -> Self{
        Self{
            PC: 0,
            SP: 0,
            A:  0,
            X:  0,
            Y:  0,
            Flags:  0,
        }
    }
}

pub struct CPU{ 
    pub registers: Registers,
    pub program: Vec<u8>,
    pub memory: Memory,
    pub STACK: u16,
    
}


impl CPU{
    pub fn new(f: File) -> Self{
        let mut reader = BufReader::new(f);
        let mut codeV  = Vec::new();
        
        reader.read_to_end(&mut codeV);
        return Self{
            registers: Registers::new(), 
            program: codeV,
            memory: Memory::new(0xFFFF),
            STACK: 0x0100,
        }
    }
    pub fn reset(&mut self){
       self.registers.PC=0;
       self.registers.SP=0;
       self.registers.A=0;
       self.registers.X=0;
       self.registers.Y=0;
       self.registers.Flags=0;
       self.memory.memory = vec![0; 0xffff];
    }
    //sets the carry flag if val is >= 0
    fn setCarryFlag(&mut self, val:u8){
        if val >= 0 {
            self.registers.Flags = self.registers.Flags | 0b0000_0001;
        } else{
            //clear carry flag and keep everything else the same
            self.registers.Flags = self.registers.Flags & 0b1111_1110;
        }
    }
    
    fn setZeroFlag(&mut self, val: u8){
        if val == 0 {
            self.registers.Flags = self.registers.Flags | 0b0000_0010;
        } else{
            //clear zero flag and keep everything else the same
            self.registers.Flags = self.registers.Flags & 0b1111_1101;
        }
    }

    fn setInterruptDisable(&mut self){
        self.registers.Flags = self.registers.Flags | 0b0000_0100;
    }

    fn setDecimalModeFlag(&mut self){
        self.registers.Flags = self.registers.Flags | 0b0000_1000;
    }

    fn setBreakCommand(&mut self){
        self.registers.Flags = self.registers.Flags | 0b0001_0000;
    }

    fn setOverflowFlag(&mut self, f: bool){
        self.registers.Flags = self.registers.Flags | 0b0010_0000;
    }

    fn setNegativeFlag(&mut self, val: u8){
        if val & 0b1000_0000 != 0 {
            self.registers.Flags = self.registers.Flags | 0b0100_0000;
        }else{
            self.registers.Flags = self.registers.Flags & 0b1011_1111;
        }
    }

    pub fn fetchAndDecodeInstruction(&mut self) -> Instruction{    
        let opcode = self.readByte();
        println!("Instruction opcode: {:x}", opcode);
        match opcode {
            //LDA
            0xA9 => return Instruction::LDA(Mode::Immediate),
            0xA5 => return Instruction::LDA(Mode::ZeroPage),
            0xB5 => return Instruction::LDA(Mode::ZeroPageX),
            0xAD => return Instruction::LDA(Mode::Absolute),
            0xBD => return Instruction::LDA(Mode::AbsoluteX),
            0xB9 => return Instruction::LDA(Mode::AbsoluteY),
            0xA1 => return Instruction::LDA(Mode::IndirectX),
            0xB1 => return Instruction::LDA(Mode::IndirectY),
            //LDX 
            0xA2 => return Instruction::LDX(Mode::Immediate), 
            0xA6 => return Instruction::LDX(Mode::ZeroPage), 
            0xB6 => return Instruction::LDX(Mode::ZeroPageY), 
            0xAE => return Instruction::LDX(Mode::Absolute), 
            0xBE => return Instruction::LDX(Mode::AbsoluteY), 
            //LDY
            0xA0 => return Instruction::LDY(Mode::Immediate),
            0xA4 => return Instruction::LDY(Mode::ZeroPage), 
            0xB4 => return Instruction::LDY(Mode::ZeroPageX),
            0xAC => return Instruction::LDY(Mode::Absolute), 
            0xBC => return Instruction::LDY(Mode::AbsoluteX),
            //STA
            0x85 => return Instruction::STA(Mode::ZeroPage),
            0x95 => return Instruction::STA(Mode::ZeroPageX),
            0x8D => return Instruction::STA(Mode::Absolute),
            0x9D => return Instruction::STA(Mode::AbsoluteX),
            0x99 => return Instruction::STA(Mode::AbsoluteY),
            0x81 => return Instruction::STA(Mode::IndirectX),
            0x91 => return Instruction::STA(Mode::IndirectY),
            //STX
            0x86 => return Instruction::STX(Mode::ZeroPage),
            0x96 => return Instruction::STX(Mode::ZeroPageY),
            0x8E => return Instruction::STX(Mode::Absolute),
            //STY
            0x84 => return Instruction::STY(Mode::ZeroPage),
            0x94 => return Instruction::STY(Mode::ZeroPageX),
            0x8C => return Instruction::STY(Mode::Absolute),
            //TAX TAY
            0xAA => return Instruction::TAX(Mode::Implicit),
            0xA8 => return Instruction::TAY(Mode::Implicit),
            //TXA TXY TXS TSX TYA 
            0x8A => return Instruction::TXA(Mode::Implicit),
            0x98 => return Instruction::TYA(Mode::Implicit),
            0xBA => return Instruction::TSX(Mode::Implicit),
            0x9A => return Instruction::TXS(Mode::Implicit),
            //PHA PHP PLA PLP
            0x48 => return Instruction::PHA(Mode::Implicit),
            0x08 => return Instruction::PHP(Mode::Implicit),
            0x68 => return Instruction::PLA(Mode::Implicit),
            0x28 => return Instruction::PLP(Mode::Implicit),
            //AND
            0x29 => return Instruction::AND(Mode::Immediate),
            0x25 => return Instruction::AND(Mode::ZeroPage),
            0x35 => return Instruction::AND(Mode::ZeroPageX),
            0x2D => return Instruction::AND(Mode::Absolute),
            0x3D => return Instruction::AND(Mode::AbsoluteX),
            0x39 => return Instruction::AND(Mode::AbsoluteY),
            0x21 => return Instruction::AND(Mode::IndirectX),
            0x31 => return Instruction::AND(Mode::IndirectY),
            //EOR
            0x49 => return Instruction::EOR(Mode::Immediate),
            0x45 => return Instruction::EOR(Mode::ZeroPage),
            0x55 => return Instruction::EOR(Mode::ZeroPageX),
            0x4D => return Instruction::EOR(Mode::Absolute),
            0x5D => return Instruction::EOR(Mode::AbsoluteX),
            0x59 => return Instruction::EOR(Mode::AbsoluteY),
            0x41 => return Instruction::EOR(Mode::IndirectX),
            0x51 => return Instruction::EOR(Mode::IndirectY),
            //ORA
            0x09 => return Instruction::ORA(Mode::Immediate),
            0x05 => return Instruction::ORA(Mode::ZeroPage),
            0x15 => return Instruction::ORA(Mode::ZeroPageX),
            0x0D => return Instruction::ORA(Mode::Absolute),
            0x1D => return Instruction::ORA(Mode::AbsoluteX),
            0x19 => return Instruction::ORA(Mode::AbsoluteY),
            0x01 => return Instruction::ORA(Mode::IndirectX),
            0x11 => return Instruction::ORA(Mode::IndirectY),
            //BIT
            0x24 => return Instruction::BIT(Mode::ZeroPage),
            0x2C => return Instruction::BIT(Mode::Absolute),
            //ADC
            0x69 => return Instruction::ADC(Mode::Immediate),
            0x65 => return Instruction::ADC(Mode::ZeroPage),
            0x75 => return Instruction::ADC(Mode::ZeroPageX),
            0x6D => return Instruction::ADC(Mode::Absolute),
            0x7D => return Instruction::ADC(Mode::AbsoluteX),
            0x79 => return Instruction::ADC(Mode::AbsoluteY),
            0x61 => return Instruction::ADC(Mode::IndirectX),
            0x71 => return Instruction::ADC(Mode::IndirectY),
            //SBC
            0xE9 => return Instruction::SBC(Mode::Immediate),
            0xE5 => return Instruction::SBC(Mode::ZeroPage),
            0xF5 => return Instruction::SBC(Mode::ZeroPageX),
            0xED => return Instruction::SBC(Mode::Absolute),
            0xFD => return Instruction::SBC(Mode::AbsoluteX),
            0xF9 => return Instruction::SBC(Mode::AbsoluteY),
            0xE1 => return Instruction::SBC(Mode::IndirectX),
            0xF1 => return Instruction::SBC(Mode::IndirectY),
            //CMP
            0xC9 => return Instruction::CMP(Mode::Immediate),
            0xC5 => return Instruction::CMP(Mode::ZeroPage),
            0xD5 => return Instruction::CMP(Mode::ZeroPageX),
            0xCD => return Instruction::CMP(Mode::Absolute),
            0xDD => return Instruction::CMP(Mode::AbsoluteX),
            0xD9 => return Instruction::CMP(Mode::AbsoluteY),
            0xC1 => return Instruction::CMP(Mode::IndirectX),
            0xD1 => return Instruction::CMP(Mode::IndirectY),
            //CPX
            0xE0 => return Instruction::CPX(Mode::Immediate),
            0xE4 => return Instruction::CPX(Mode::ZeroPage),
            0xEC => return Instruction::CPX(Mode::Absolute),
            //CPY
            0xC0 => return Instruction::CPY(Mode::Immediate),
            0xC4 => return Instruction::CPY(Mode::ZeroPage),
            0xCC => return Instruction::CPY(Mode::Absolute),
            //INC
            0xE6 => return Instruction::INC(Mode::ZeroPage),
            0xF6 => return Instruction::INC(Mode::ZeroPageX),
            0xEE => return Instruction::INC(Mode::Absolute),
            0xFE => return Instruction::INC(Mode::AbsoluteX),
            //INX
            0xE8 => return Instruction::INX(Mode::Implicit),
            //INY
            0xC8 => return Instruction::INY(Mode::Implicit),
            //DEC
            0xC6 => return Instruction::DEC(Mode::ZeroPage),
            0xD6 => return Instruction::DEC(Mode::ZeroPageX),
            0xCE => return Instruction::DEC(Mode::Absolute),
            0xDE => return Instruction::DEC(Mode::AbsoluteX),
            //DEX
            0xCA => return Instruction::DEX(Mode::Implicit),
            //DEY
            0x88 => return Instruction::DEY(Mode::Implicit),
            //ASL
            0x0A => return Instruction::ASL(Mode::Accumulator),
            0x06 => return Instruction::ASL(Mode::ZeroPage),
            0x16 => return Instruction::ASL(Mode::ZeroPageX),
            0x0E => return Instruction::ASL(Mode::Absolute),
            0x1E => return Instruction::ASL(Mode::AbsoluteX),
            //LSR  
            0x4A => return Instruction::LSR(Mode::Accumulator),
            0x46 => return Instruction::LSR(Mode::ZeroPage),
            0x56 => return Instruction::LSR(Mode::ZeroPageX),
            0x4E => return Instruction::LSR(Mode::Absolute),
            0x5E => return Instruction::LSR(Mode::AbsoluteX),
            //ROL
            0x2A => return Instruction::ROL(Mode::Accumulator),
            0x26 => return Instruction::ROL(Mode::ZeroPage),
            0x36 => return Instruction::ROL(Mode::ZeroPageX),
            0x2E => return Instruction::ROL(Mode::Absolute),
            0x3E => return Instruction::ROL(Mode::AbsoluteX),
            //ROR
            0x6A => return Instruction::ROR(Mode::Accumulator),
            0x66 => return Instruction::ROR(Mode::ZeroPage),
            0x76 => return Instruction::ROR(Mode::ZeroPageX),
            0x6E => return Instruction::ROR(Mode::Absolute),
            0x7E => return Instruction::ROR(Mode::AbsoluteX),
            //JMP
            0x4C => return Instruction::JMP(Mode::Absolute),
            0x6C => return Instruction::JMP(Mode::Indirect),
            //JSR
            0x20 => return Instruction::JSR(Mode::Absolute),
            //RTS
            0x60 => return Instruction::RTS(Mode::Implicit),
            //BCC
            0x90 => return Instruction::BCC(Mode::Relative),
            //BCS
            0xB0 => return Instruction::BCS(Mode::Relative),
            //BEQ
            0xF0 => return Instruction::BEQ(Mode::Relative),
            //BMI
            0x30 => return Instruction::BMI(Mode::Relative),
            //BNE
            0xD0 => return Instruction::BNE(Mode::Relative),
            //BPL
            0x10 => return Instruction::BPL(Mode::Relative),
            //BVC
            0x50 => return Instruction::BVC(Mode::Relative),
            //BVS
            0x70 => return Instruction::BVS(Mode::Relative),
            //CLC
            0x18 => return Instruction::CLC(Mode::Implicit),
            //CLD
            0xD8 => return Instruction::CLD(Mode::Implicit),
            //CLI
            0x58 => return Instruction::CLI(Mode::Implicit),
            //CLV
            0xB8 => return Instruction::CLV(Mode::Implicit),
            //SEC
            0x38 => return Instruction::SEC(Mode::Implicit),
            //SED
            0xF8 => return Instruction::SED(Mode::Implicit),
            //SEI
            0x78 => return Instruction::SEI(Mode::Implicit),
            //BRK
            0x00 => return Instruction::BRK(Mode::Implicit),
            //NOP
            0xEA => return Instruction::NOP(Mode::Implicit),
            //RTI
            0x40 => return Instruction::RTI(Mode::Implicit),
            _ => {
                panic!("Illegal instruction");
            }
        }
    }
    pub fn readByte(&mut self) -> u8 {
        let b = self.program[self.registers.PC as usize];
        self.registers.PC+=1;
        return b    
    }    

    pub fn readShort(&mut self) -> u16 {
        let low = self.program[self.registers.PC as usize] as u16;
        let high = self.program[(self.registers.PC+1) as usize] as u16;
        self.registers.PC+=2;
        return (high << 8) | (low as u16)
    }    
    fn stackPushByte(&mut self, value: u8){
        //write the value to stack+SP
        self.memory.memoryWriteByte(self.STACK-(self.registers.SP as u16), value); 
        //subtract 1 from the stack pointer
        self.registers.SP = self.registers.SP.wrapping_add(1);
    }

    fn stackPushShort(&mut self, value: u16){
        let hi = (value >> 8) as u8;
        let low = (value & 0xff) as u8;
        self.stackPushByte(hi);
        self.stackPushByte(low);
    }

    fn stackPopByte(&mut self) -> u8{
        //read byte from stack
        let val = self.memory.memoryReadByte(self.STACK-(self.registers.SP as u16));
        //add 1 to stack pointer
        self.registers.SP = self.registers.SP.wrapping_sub(1);
        //return popped value
        val
    }

    fn stackPopShort(&mut self) -> u16{
        let l = self.stackPopByte() as u16;
        let h = self.stackPopByte() as u16; 
        return h<<8 | l

    }
    //calls the mode specific function depending on whatever mode is passed
    fn modeHandler(&mut self, mode : Mode) -> u16{
        match mode {
            Mode::Implicit	  => return 0,
            Mode::Accumulator => return 0,
            Mode::ZeroPage	  => self.ZeroPageHandler()  ,
            Mode::ZeroPageX	  => self.ZeroPageXHandler() ,
            Mode::ZeroPageY	  => self.ZeroPageYHandler() ,
            //remember to cast as i8 since only used in branch operations
            Mode::Relative	  => self.RelativeHandler()  ,
            Mode::Absolute	  => self.AbsoluteHandler()  ,
            Mode::AbsoluteX	  => self.AbsoluteXHandler() ,
            Mode::AbsoluteY	  => self.AbsoluteYHandler() ,
            Mode::Indirect	  => self.IndirectHandler(),
            Mode::IndirectX	  => self.IndirectXHandler() ,
            Mode::IndirectY	  => self.IndirectYHandler() ,
            _           =>{
                return 0xff
            }
        } 
    }
    //reads next byte and returns it as the value
    fn ImmediateHandler(&mut self) -> u8{
        self.readByte()
    }      
    //reads byte at memory address specified
    fn ZeroPageHandler(&mut self) -> u16{
        let mut next = self.readByte();
        next as u16
    }
    fn ZeroPageXHandler(&mut self) -> u16{
        let mut next = self.readByte() as u16;
        next.wrapping_add(self.registers.X as u16)
    }
    fn ZeroPageYHandler(&mut self) -> u16{
        let mut next = self.readByte() as u16;
        next.wrapping_add(self.registers.Y as u16)
    }
    fn RelativeHandler(&mut self) -> u16{
        self.readByte() as u16
    }
    fn AbsoluteHandler(&mut self) -> u16{
        let mut next = self.readShort();
        next
    }
    
    fn AbsoluteXHandler(&mut self) -> u16{
        let next = self.readShort();
        let low  = ((next & 0x00ff) as u8).wrapping_add(self.registers.X);
        let high = (next >> 8) as u8;
        (high as u16) << 8 | (low as u16)
        
    }
    fn AbsoluteYHandler(&mut self) -> u16{
        let next = self.readShort();
        let low  = ((next & 0x00ff) as u8).wrapping_add(self.registers.Y);
        let high = (next >> 8) as u8;
        (high as u16) << 8 | (low as u16)
    }
    fn IndirectHandler(&mut self) ->u16{
        let next = self.readShort();
        self.memory.memoryReadShort(next)
       
    }
    fn IndirectXHandler(&mut self) -> u16{        
        //get byte + X as index
        let mut index = self.readByte().wrapping_add(self.registers.X);
        //get the short at memory byte+X and store in memVal
        let memVal = self.memory.memoryReadShort(index as u16);
        memVal
        
    }
    fn IndirectYHandler(&mut self) -> u16{
        //get byte
        let mut index = self.readByte();
        //get the short at memory byte and add Y to it
        let memVal = self.memory.memoryReadShort(index as u16).wrapping_add(self.registers.Y as u16);
        memVal
    }

    fn LDA(&mut self, mode: Mode){
        //reads 1 or two bytes
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.A = val;
        self.setZeroFlag(val);
        self.setNegativeFlag(val);
    } 
    fn LDX(&mut self, mode: Mode){ 
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.X = val as u8;
        self.setZeroFlag(val as u8);
        self.setNegativeFlag(val as u8);
    } 
    fn LDY(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.Y = val as u8;
        self.setZeroFlag(val as u8);
        self.setNegativeFlag(val as u8);
    } 
    fn STA(&mut self, mode: Mode){ 
        let address = self.modeHandler(mode);
        self.memory.memoryWriteByte(address, self.registers.A);
    } 
    fn STX(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        self.memory.memoryWriteByte(address, self.registers.X);
    } 
    fn STY(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        self.memory.memoryWriteByte(address, self.registers.Y);
    } 
    fn TAX(&mut self, mode: Mode){
        self.registers.X = self.registers.A;
        self.setZeroFlag(self.registers.X);
        self.setNegativeFlag(self.registers.X);
    } 
    fn TAY(&mut self, mode: Mode){
        self.registers.Y = self.registers.A;
        self.setZeroFlag(self.registers.Y);
        self.setNegativeFlag(self.registers.Y);
    } 
    fn TXA(&mut self, mode: Mode){
        self.registers.A = self.registers.X;
        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);
    } 
    fn TYA(&mut self, mode: Mode){
        self.registers.A = self.registers.Y;
        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);
    } 
    fn TSX(&mut self, mode: Mode){
        self.registers.X = self.registers.SP;
        self.setZeroFlag(self.registers.X);
        self.setNegativeFlag(self.registers.X);
    } 
    fn TXS(&mut self, mode: Mode){
        self.registers.SP = self.registers.X;
    } 
    //pushes A register to the stack and subs 1 from SP
    //in the nes cpu the stack starts at 0x0100 and goes to 0x1ff
    //so it is indexed by 1 byte
    fn PHA(&mut self, mode: Mode){
        //gets absolute address of stack pointer
        let absoluteAddress = 0x0100 | (self.registers.SP as u16);
        //writes register A to this stack address
        self.memory.memoryWriteByte(absoluteAddress, self.registers.A);
        //DECREMENTS stack pointer
        //not sure if the subtraction is wrapping but we will find out
        self.registers.SP = self.registers.SP.wrapping_sub(1);

    } 
    fn PHP(&mut self, mode: Mode){
        let absoluteAddress = 0x0100 | (self.registers.SP as u16);
        self.memory.memoryWriteByte(absoluteAddress, self.registers.Flags);
        self.registers.SP = self.registers.SP.wrapping_sub(1);
    } 
    //pull
    fn PLA(&mut self, mode: Mode){
        let absoluteAddress = 0x0100 | (self.registers.SP as u16);
        self.registers.A = self.memory.memoryReadByte(absoluteAddress);
        self.registers.SP = self.registers.SP.wrapping_add(1);

        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);     
    } 
    fn PLP(&mut self, mode: Mode){
        let absoluteAddress = 0x0100 | (self.registers.SP as u16);
        self.registers.Flags = self.memory.memoryReadByte(absoluteAddress);
        self.registers.SP = self.registers.SP.wrapping_add(1);
    } 
    fn AND(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.A = self.registers.A & val;
        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);
    } 
    fn EOR(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.A = self.registers.A ^ val;
        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);
       

    } 
    fn ORA(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address),};
        self.registers.A = self.registers.A | val;
        self.setZeroFlag(self.registers.A);
        self.setNegativeFlag(self.registers.A);
    } 

    fn BIT(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let memVal = self.memory.memoryReadByte(address);
        self.setZeroFlag(self.registers.A & memVal);

        //if bit 6 of memVal is set then set self.registers.Flags
        if memVal & 0b0010_0000 != 0 {
            self.registers.Flags = self.registers.Flags | 0b0010_0000;
        } else{
            self.registers.Flags = self.registers.Flags & 0b1101_1111;
        }

        //if bit 7 of memVal is set then set self.registers.Flags
        if memVal & 0b0100_0000 != 0 {
            self.registers.Flags = self.registers.Flags | 0b0100_0000;
        } else{
            self.registers.Flags = self.registers.Flags & 0b1011_1111;
        }
    } 

    fn ADC(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address)};
        //adds together A, the value, and the carry in bit
        let sum: u16 = (self.registers.A as u16) + (val as u16) + ((self.registers.Flags & 0b0000_0001) as u16);
        

        //tests if overflow occurs and sets carry flag
        if sum > 0xff {self.registers.Flags = self.registers.Flags | 0b0000_0001;}
        else          {self.registers.Flags = self.registers.Flags & 0b1111_1110;}

                

        
        if (val ^ (sum as u8)) & ((sum as u8) ^ self.registers.A) & 0x80 != 0 {
            self.registers.Flags = self.registers.Flags | 0b0010_0000;
        } else {
            self.registers.Flags = self.registers.Flags & 0b1101_1111;
        }
        
        self.registers.A = sum as u8;
    } 

    fn SBC(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address)};
        //A-M-1+C
        val = ((val as i8).wrapping_neg().wrapping_sub(1))as u8;
        //adds together Add, the value, and the carry in bit
        let sum: u16 = (self.registers.A as u16) + (val as u16) + ((self.registers.Flags & 0b0000_0001) as u16);
        

        //tests if overflow occurs and sets carry flag
        if sum > 0xff {self.registers.Flags = self.registers.Flags | 0b0000_0001;}
        else          {self.registers.Flags = self.registers.Flags & 0b1111_1110;}

                

        
        if (val ^ (sum as u8)) & ((sum as u8) ^ self.registers.A) & 0x80 != 0 {
            self.registers.Flags = self.registers.Flags | 0b0010_0000;
        } else {
            self.registers.Flags = self.registers.Flags & 0b1101_1111;
        }
        
        self.registers.A = sum as u8;
    } 
    fn compare(&mut self, mode: Mode, value: u8){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address)};
        let result = value-val;
        self.setCarryFlag(result);
        self.setZeroFlag(result);
        self.setNegativeFlag(result);
    }

    fn CMP(&mut self, mode: Mode){
        self.compare(mode, self.registers.A);
    } 
    fn CPX(&mut self, mode: Mode){
        self.compare(mode, self.registers.X);
    } 
    fn CPY(&mut self, mode: Mode){
        self.compare(mode, self.registers.Y);
    } 
    fn INC(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address)};
        let result = val.wrapping_add(1);
        self.memory.memoryWriteByte(address,result);
        self.setZeroFlag(result);
        self.setNegativeFlag(result);
    } 
    fn INX(&mut self, mode: Mode){
        self.registers.X.wrapping_add(1);
        self.setZeroFlag(self.registers.X);
        self.setNegativeFlag(self.registers.X);
    } 
    fn INY(&mut self, mode: Mode){
        self.registers.Y.wrapping_add(1);
        self.setZeroFlag(self.registers.Y);
        self.setNegativeFlag(self.registers.Y);
    } 
    fn DEC(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Immediate => self.ImmediateHandler(),
                                 _ => self.memory.memoryReadByte(address)};
        let result = val.wrapping_sub(1);
        self.memory.memoryWriteByte(address,result);
        self.setZeroFlag(result);
        self.setNegativeFlag(result);
    } 
    fn DEX(&mut self, mode: Mode){
        self.registers.X.wrapping_sub(1);
        self.setZeroFlag(self.registers.X);
        self.setNegativeFlag(self.registers.X);
    } 
    fn DEY(&mut self, mode: Mode){
        self.registers.Y.wrapping_sub(1);
        self.setZeroFlag(self.registers.Y);
        self.setNegativeFlag(self.registers.Y);
    } 
    fn ASL(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Accumulator => self.registers.A,
                                 _ => self.memory.memoryReadByte(address)};
        //set carry bit to the 7th bit of val
        let result = val << 1;
        if (val >> 7) == 1 {self.registers.Flags=self.registers.Flags | 0b0000_0001;}
        else               {self.registers.Flags=self.registers.Flags & 0b1111_1110;}
        
        match mode {
            Mode::Accumulator => self.registers.A = result,
            _                 => self.memory.memoryWriteByte(address,result),
        }

        self.setZeroFlag(result);
        self.setNegativeFlag(result);
        
    } 
    fn LSR(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Accumulator => self.registers.A,
                                 _ => self.memory.memoryReadByte(address)};
        //set carry bit to the 7th bit of val
        let result = val >> 1;
        if (val & 0b0000_0001) == 1 {self.registers.Flags=self.registers.Flags | 0b0000_0001;}
        else                        {self.registers.Flags=self.registers.Flags & 0b1111_1110;}
        
        match mode {
            Mode::Accumulator => self.registers.A = result,
            _                 => self.memory.memoryWriteByte(address,result),
        }

        self.setZeroFlag(result);
        self.setNegativeFlag(result);
    } 
    fn ROL(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Accumulator => self.registers.A,
                                 _ => self.memory.memoryReadByte(address)};
        let oldCarry = self.registers.Flags & 0b0000_0001;

        //set carry bit to the 7th bit of val
        let mut result = val << 1;
        if (val >> 7) == 1 {self.registers.Flags=self.registers.Flags | 0b0000_0001;}
        else               {self.registers.Flags=self.registers.Flags & 0b1111_1110;}
         
        result = result | oldCarry;
        
        match mode {
            Mode::Accumulator => self.registers.A = result,
            _                 => self.memory.memoryWriteByte(address,result),
        }

        self.setZeroFlag(result);
        self.setNegativeFlag(result);
       
    } 
    fn ROR(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        let mut val = match mode{Mode::Accumulator => self.registers.A,
                                 _ => self.memory.memoryReadByte(address)};

        let oldCarry = self.registers.Flags & 0b0000_0001;

        //set carry bit to the 1st bit of val
        let mut result = val >> 1;
        if (val & 0b0000_0001) == 1 {self.registers.Flags=self.registers.Flags | 0b0000_0001;}
        else                        {self.registers.Flags=self.registers.Flags & 0b1111_1110;}
         
        result = result | oldCarry<<7;
        
        match mode {
            Mode::Accumulator => self.registers.A = result,
            _                 => self.memory.memoryWriteByte(address,result),
        }
        self.setZeroFlag(result);
        self.setNegativeFlag(result);

    } 
    fn JMP(&mut self, mode: Mode){
        let address = self.modeHandler(mode);
        //let val = self.memory.memoryReadShort(address);
        self.registers.PC = address;
    } 
    //pushes the current PC to the stack
    //sets the new PC to the target memory address
    fn JSR(&mut self, mode: Mode){
        //target memory address
        let address = self.modeHandler(mode);
        println!("address {:x}", address); 
        //returnAddress equals the address of the NEXT instruction
        self.stackPushShort(self.registers.PC-1);
        //push the first byte of the return address onto the stack
        
        println!("pc {:x}", self.registers.PC); 
        //set PC to absolute address
        self.registers.PC = address;
        
    } 
    fn branch(&mut self, condition: bool, offset: i8){
        if condition {self.registers.PC = self.registers.PC.wrapping_add(offset as u16);}
    }
    fn RTS(&mut self, mode: Mode){
        self.registers.PC = self.stackPopShort()-1;
    } 
    fn BCC(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0000_0001) == 0;
        self.branch(condition, offset as i8);
    } 
    fn BCS(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0000_0001) == 1;
        self.branch(condition, offset as i8);
    } 
    fn BNE(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0000_0010) == 0;
        self.branch(condition, offset as i8);
    } 
    fn BEQ(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0000_0010) == 1;
        self.branch(condition, offset as i8);
    } 
    fn BPL(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0100_0000) == 0;
        self.branch(condition, offset as i8);
    } 
    fn BMI(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0100_0000) == 1;
        self.branch(condition, offset as i8);
    } 
    fn BVC(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0010_0000) == 0;
        self.branch(condition, offset as i8);
    } 
    fn BVS(&mut self, mode: Mode){
        let offset = self.modeHandler(mode);
        let condition = (self.registers.Flags & 0b0010_0000) == 1;
        self.branch(condition, offset as i8);
    } 
    fn CLC(&mut self, mode: Mode){
        self.registers.Flags & 0b1111_1110;
    } 
    fn CLD(&mut self, mode: Mode){
        self.registers.Flags & 0b1111_0111;
    } 
    fn CLI(&mut self, mode: Mode){
        self.registers.Flags & 0b1111_1011;
    } 
    fn CLV(&mut self, mode: Mode){
        self.registers.Flags & 0b1101_1111;
    } 
    fn SEC(&mut self, mode: Mode){
        self.registers.Flags | 0b0000_0001;
    } 
    fn SED(&mut self, mode: Mode){
        self.registers.Flags | 0b0000_1000;
    } 
    fn SEI(&mut self, mode: Mode){
        self.registers.Flags | 0b0000_0100;
    } 
    
    fn BRK(&mut self, mode: Mode){
        //self.stackPushByte(self.registers.PC);
        //self.stackPushByte(self.registers.Flags);
        //self.registers.Flags | 0b0001_0000;
    } 
    fn NOP(&mut self, mode: Mode){
        
    } 
    fn RTI(&mut self, mode: Mode){
        self.registers.Flags = self.stackPopByte();
        self.registers.PC = self.stackPopShort();
    }

    pub fn executeInstruction(&mut self, inst: Instruction) {
        match inst{
            Instruction::LDA(mode) => self.LDA(mode),
            Instruction::LDX(mode) => self.LDX(mode), 
            Instruction::LDY(mode) => self.LDY(mode),
            Instruction::STA(mode) => self.STA(mode),
            Instruction::STX(mode) => self.STX(mode),
            Instruction::STY(mode) => self.STY(mode),
            Instruction::TAX(mode) => self.TAX(mode),
            Instruction::TAY(mode) => self.TAY(mode),
            Instruction::TXA(mode) => self.TXA(mode),
            Instruction::TYA(mode) => self.TYA(mode),
            Instruction::TSX(mode) => self.TSX(mode),
            Instruction::TXS(mode) => self.TXS(mode),
            Instruction::PHA(mode) => self.PHA(mode),
            Instruction::PHP(mode) => self.PHP(mode),
            Instruction::PLA(mode) => self.PLA(mode),
            Instruction::PLP(mode) => self.PLP(mode),
            Instruction::AND(mode) => self.AND(mode),
            Instruction::EOR(mode) => self.EOR(mode),
            Instruction::ORA(mode) => self.ORA(mode),
            Instruction::BIT(mode) => self.BIT(mode),
            Instruction::ADC(mode) => self.ADC(mode),
            Instruction::SBC(mode) => self.SBC(mode),
            Instruction::CMP(mode) => self.CMP(mode),
            Instruction::CPX(mode) => self.CPX(mode),
            Instruction::CPY(mode) => self.CPY(mode),
            Instruction::INC(mode) => self.INC(mode),
            Instruction::INX(mode) => self.INX(mode),
            Instruction::INY(mode) => self.INY(mode),
            Instruction::DEC(mode) => self.DEC(mode),
            Instruction::DEX(mode) => self.DEX(mode),
            Instruction::DEY(mode) => self.DEY(mode),
            Instruction::ASL(mode) => self.ASL(mode),
            Instruction::LSR(mode) => self.LSR(mode),
            Instruction::ROL(mode) => self.ROL(mode),
            Instruction::ROR(mode) => self.ROR(mode),
            Instruction::JMP(mode) => self.JMP(mode),
            Instruction::JSR(mode) => self.JSR(mode),
            Instruction::RTS(mode) => self.RTS(mode),
            Instruction::BCC(mode) => self.BCC(mode),
            Instruction::BCS(mode) => self.BCS(mode),
            Instruction::BEQ(mode) => self.BEQ(mode),
            Instruction::BMI(mode) => self.BMI(mode),
            Instruction::BNE(mode) => self.BNE(mode),
            Instruction::BPL(mode) => self.BPL(mode),
            Instruction::BVC(mode) => self.BVC(mode),
            Instruction::BVS(mode) => self.BVS(mode),
            Instruction::CLC(mode) => self.CLC(mode),
            Instruction::CLD(mode) => self.CLD(mode),
            Instruction::CLI(mode) => self.CLI(mode),
            Instruction::CLV(mode) => self.CLV(mode),
            Instruction::SEC(mode) => self.SEC(mode),
            Instruction::SED(mode) => self.SED(mode),
            Instruction::SEI(mode) => self.SEI(mode),
            Instruction::BRK(mode) => self.BRK(mode),
            Instruction::NOP(mode) => self.NOP(mode),
            Instruction::RTI(mode) => self.RTI(mode),
            _ => {
                panic!("Illegal instruction");
            }
        }
    } 

    pub fn run(&mut self){
        loop {
            let inst = self.fetchAndDecodeInstruction();
            self.executeInstruction(inst);
        }
    }
    
    pub fn runWithCallbacak<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        let ref opcodes: HashMap<u8, &
    }
}
