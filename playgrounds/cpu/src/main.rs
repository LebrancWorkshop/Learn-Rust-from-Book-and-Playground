type Memory = [u8; 0xFF];

#[derive(Debug)]
struct CPU {
    memory: Memory
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: [0; 0xFF]
        }
    }

    fn set_mem(&mut self, mem: usize, val: u8) {
        self.memory[mem] = val;
    }

    fn get_mem(&self, mem: usize) -> u8 {
        self.memory[mem]
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.set_mem(0, 1);

    println!("Mem 0: {}", cpu.get_mem(0));
}


#[cfg(test)]
mod cpu_tests {
    use super::*;
    #[test]
    fn test_mem() {
        let mut cpu = CPU::new();

        cpu.set_mem(0x01, 98);
        assert_eq!(cpu.get_mem(0x01), 98);
        assert_eq!(cpu.get_mem(0x02), 0);
    }

}
