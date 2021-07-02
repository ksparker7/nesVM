use crate::instructions::Instruction;
use crate::instructions::Mode;


pub struct Memory{
    pub memory: Vec<u8>,
}

impl Memory{
    pub fn new(size: usize) -> Self{
        Memory{
            memory: vec![0; size],
        }
    }    
    
    pub fn memoryReadByte(&self, address: u16) -> u8{
        return self.memory[address as usize]; 
    }

    pub fn memoryReadShort(&mut self, address: u16) -> u16{
        
        let low = self.memory[address as usize] as u16; 
        let high = self.memory[(address+1) as usize] as u16;
        return (high << 8) | (low as u16)
    }

    pub fn memoryWriteShort(&mut self, address: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.memoryWriteByte(address, low);
        self.memoryWriteByte(address+1, high);
    }

       pub fn memoryWriteByte(&mut self, address: u16, val: u8) {
        //write a byte val to address in memory
        self.memory[address as usize] = val;
    }
    
}


#[cfg(test)]
mod cpuTests{
    use super::*;
    #[test]
    fn testReadMemory(){
        let mut mem = Memory::new(0x10);
        let mut buf = [0u8; 4];
        mem.memoryWriteByte(0x04, 0x60);
        assert_eq!(0x60, mem.memoryReadByte(0x04));
        
    }
    
}
