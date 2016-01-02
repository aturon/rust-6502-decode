pub fn decode_instruction<S>(state: &mut S) -> S::Output
    where S: ByteStream + InterpretInstruction
{
    unimplemented!()
}

pub trait ByteStream {
    fn next(&mut self) -> u8;
}

pub trait InterpretInstruction {
    type Output;

    // Load and store

    /// Load A
    fn lda<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Load X
    fn ldx<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Load Y
    fn ldy<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Store A
    fn sta<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Store X
    fn stx<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Store Y
    fn sty<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Arithmetic

    /// Add with carry
    fn adc<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Subtract with carry
    fn sbc<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Increment and decrement

    /// Increment memory
    fn inc<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Increment X
    fn inx(&mut self) -> Self::Output;

    /// Increment Y
    fn iny(&mut self) -> Self::Output;

    /// Decrement memory
    fn dec<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Decrement X
    fn dex(&mut self) -> Self::Output;

    /// Decrement Y
    fn dey(&mut self) -> Self::Output;

    // Logical instructions

    /// Bitwise `and` with A
    fn and<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Bitwise `or` with A
    fn ora<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Bitwise exclusive `or`
    fn eor<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Jump, branch, and subroutine instructions

    /// Jump
    fn jmp<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Jump to subroutine
    fn jsr<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Return from interrupt
    fn rti(&mut self) -> Self::Output;

    /// Return from subroutine
    fn rts(&mut self) -> Self::Output;

    /// Branch on carry clear
    fn bcc<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on carry set
    fn bcs<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on equal to zero
    fn beq<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on not equal to zero
    fn bne<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on minus
    fn bmi<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on plus
    fn bpl<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on overflow set
    fn bvs<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Branch on overflow clear
    fn bvc<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Break
    fn brk(&mut self) -> Self::Output;

    // Comparison instructions:

    /// Compare A
    fn cmp<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Compare X registor
    fn cpx<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Compare Y registor
    fn cpy<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Bit testing

    /// Test bits
    fn bit<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Shifts and rotations:

    /// Arithmetic shift left
    fn asl<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Logical shift right
    fn lsr<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Rotate left
    fn rol<A: Arg>(&mut self, arg: A) -> Self::Output;

    /// Rotate right
    fn ror<A: Arg>(&mut self, arg: A) -> Self::Output;

    // Transfer instructions

    /// Transfer A to X
    fn tax(&mut self) -> Self::Output;

    /// Transfer A to Y
    fn tay(&mut self) -> Self::Output;

    /// Transfer X to A
    fn txa(&mut self) -> Self::Output;

    /// Transfer Y to A
    fn tya(&mut self) -> Self::Output;

    // Stack instructions:

    /// Transfer X to S
    fn txs(&mut self) -> Self::Output;

    /// Transfer S to X
    fn tsx(&mut self) -> Self::Output;

    /// Push A
    fn pha(&mut self) -> Self::Output;

    /// Pull A
    fn pla(&mut self) -> Self::Output;

    /// Push processor status
    fn php(&mut self) -> Self::Output;

    /// Pull processor status
    fn plp(&mut self) -> Self::Output;

    // Flag instructions

    /// Clear carry flag
    fn clc(&mut self) -> Self::Output;

    /// Clear decimal mode
    fn cld(&mut self) -> Self::Output;

    /// Clear intrerrupt disable
    fn cli(&mut self) -> Self::Output;

    /// Clear overflow flag
    fn clv(&mut self) -> Self::Output;

    /// Set carry
    fn sec(&mut self) -> Self::Output;

    /// Set decimal mode
    fn sed(&mut self) -> Self::Output;

    /// Set interrupt disable
    fn sei(&mut self) -> Self::Output;

    // Misc

    /// No operation
    fn nop(&mut self) -> Self::Output;

}

pub trait Arg {
    fn interpret<I: InterpretArg>(&self, interp: I) -> I::Output;
}

pub trait InterpretArg {
    type Output;

}