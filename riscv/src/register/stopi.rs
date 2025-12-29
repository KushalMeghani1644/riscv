//! stopi register — Supervisor Top Priority Interrupt (0xDB0)

read_only_csr! {
    /// Supervisor Top Priority Interrupt Register
    Stopi: 0xDB0,
    mask: 0x0FFF_00FF,
}

read_only_csr_field! {
    Stopi,
    /// Interrupt ID (bits 16..27)
    ///
    /// A value of 0 indicates no interrupt is pending.
    iid: [16:27],
}

read_only_csr_field! {
    Stopi,
    /// Interrupt Priority (bits 0..7)
    ///
    /// Lower values indicate higher priority.
    iprio: [0:7],
}

impl Stopi {
    /// Returns true if there is a valid interrupt pending
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        self.iid() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopi_fields() {
        let stopi = Stopi::from_bits(0);
        test_ro_csr_field!(stopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(stopi, iprio: [0, 7], 0x0);

        let stopi = Stopi::from_bits((0xB << 16) | 5);
        test_ro_csr_field!(stopi, iid: [16, 27], 0xB);
        test_ro_csr_field!(stopi, iprio: [0, 7], 0x5);

        let stopi = Stopi::from_bits((0xFFF << 16) | 0xFF);
        test_ro_csr_field!(stopi, iid: [16, 27], 0xFFF);
        test_ro_csr_field!(stopi, iprio: [0, 7], 0xFF);

        let stopi = Stopi::from_bits(1 << 16);
        test_ro_csr_field!(stopi, iid: [16, 27], 0x1);
        test_ro_csr_field!(stopi, iprio: [0, 7], 0x0);

        let stopi = Stopi::from_bits(1);
        test_ro_csr_field!(stopi, iid: [16, 27], 0x0);
        test_ro_csr_field!(stopi, iprio: [0, 7], 0x1);
    }

    #[test]
    fn test_stopi_bitmask() {
        let stopi = Stopi::from_bits(usize::MAX);
        assert_eq!(stopi.bits(), 0x0FFF_00FFusize);
    }
}
