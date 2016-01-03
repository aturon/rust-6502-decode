use super::{InterpretInstruction, Arg, InterpretArg};

pub fn decode_instruction<S>(state: &mut S) -> S::Output
where S: ByteStream + InterpretInstruction
{
    unimplemented!()
}

pub trait ByteStream {
    fn next(&mut self) -> u8;
}
