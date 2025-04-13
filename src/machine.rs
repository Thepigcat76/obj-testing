use object::write::Object;

struct Frame {
    instructions: Vec<u8>,
}

pub struct MachineGenerator<'m> {
    obj: Object<'m>,
    frames: Vec<Frame>,
    frame_pointer: usize,
}

impl<'m> MachineGenerator<'m> {
    pub fn new() -> Self {
        Self {
            obj: Object::new(
                object::BinaryFormat::native_object(),
                object::Architecture::X86_64,
                object::Endianness::Little,
            ),
            frames: Vec::new(),
            frame_pointer: 0
        }
    }

    
}
