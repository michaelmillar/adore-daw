pub struct AudioBlock<'a> { pub ch: &'a mut [&'a mut [f32]]; pub frames: usize }
#[derive(Copy, Clone)] pub struct Event  { pub frame: u32, pub kind: EventKind }
#[derive(Copy, Clone)] pub enum EventKind  { NoteOn{ch:u8, n:u8, v:u8}, Param{idx:u32, val:f32} }

pub trait Node: Send {
    fn prepare(&mut self, sr: u32, max_block: usize);
}