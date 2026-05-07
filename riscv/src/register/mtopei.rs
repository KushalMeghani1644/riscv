//! `mtopei` register — Machine Top External Interrupt (0x35C)
//!
//! This CSR is part of the RISC-V Advanced Interrupt Architecture (AIA). Its layout mirrors
//! `mtopi`, exposing the interrupt identity and priority of the top pending machine external
//! interrupt.

read_write_csr! {
    /// Machine Top External Interrupt Register
    Mtopei: 0x35C,
    mask: 0x0FFF_00FF,
}

read_write_csr_field! {
    Mtopei,
    /// Interrupt ID (bits 16..27)
    ///
    /// Identifies the specific interrupt source. A value of 0 indicates no interrupt is pending.
    iid: [16:27],
}

read_write_csr_field! {
    Mtopei,
    /// Interrupt Priority ID (bits 0..7)
    ///
    /// Represents the priority level of the pending interrupt.
    /// Lower numerical values indicate higher priority interrupts.
    iprio: [0:7],
}

impl Mtopei {
    /// Returns true if there is a valid interrupt pending.
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        self.iid() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtopei_fields() {
        let mut mtopei = Mtopei::from_bits(0);
        test_csr_field!(mtopei, iid: [16, 27], 0x0);
        test_csr_field!(mtopei, iprio: [0, 7], 0x0);

        let mut mtopei = Mtopei::from_bits((0xB << 16) | 5);
        test_csr_field!(mtopei, iid: [16, 27], 0xB);
        test_csr_field!(mtopei, iprio: [0, 7], 0x5);

        let mut mtopei = Mtopei::from_bits((0xFFF << 16) | 0xFF);
        test_csr_field!(mtopei, iid: [16, 27], 0xFFF);
        test_csr_field!(mtopei, iprio: [0, 7], 0xFF);

        let mut mtopei = Mtopei::from_bits(1 << 16);
        test_csr_field!(mtopei, iid: [16, 27], 0x1);
        test_csr_field!(mtopei, iprio: [0, 7], 0x0);

        let mut mtopei = Mtopei::from_bits(1);
        test_csr_field!(mtopei, iid: [16, 27], 0x0);
        test_csr_field!(mtopei, iprio: [0, 7], 0x1);
    }

    #[test]
    fn test_mtopei_bitmask() {
        let mtopei = Mtopei::from_bits(usize::MAX);
        assert_eq!(mtopei.bits(), 0x0FFF_00FFusize);
    }
}
