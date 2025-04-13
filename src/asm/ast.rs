pub enum Instruction {
    MovI2R(MovIns<Immediate, Register>),
    Syscall
}

pub struct Immediate(u32);

pub enum Register {
    Ra0,
    Ra1,
    Ra2,
    Ra3,
}

pub struct MovIns<V, D> {
    val: V,
    dest: D,
}
