use crate::gba::{clock::Time, cpu::Cpu, memory::MemoryBus};

pub(crate) struct Gba {
    cpu: Cpu,
    memory: MemoryBus,
    clock: Time,
}

impl Gba {
    pub(crate) fn startup() -> Self {
        Self {
            cpu: Cpu::startup(),
            memory: MemoryBus::startup(),
            clock: Time(0),
        }
    }
    pub(crate) fn reset(&mut self) {
        self.cpu.reset();
        self.clock.0 = 0;
    }
    pub(crate) fn run(&mut self) {
        loop {
            let cycles = self.cpu.step(&mut self.memory);
            let (next_clock, _overflow) = self.clock.overflowing_add(cycles);
            self.clock = next_clock;
        }
    }
}
