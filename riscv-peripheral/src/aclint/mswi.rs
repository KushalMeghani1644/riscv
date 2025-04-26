//! Machine-level Software Interrupt Device.

pub use super::HartIdNumber;
use crate::common::unsafe_peripheral;

/// MSWI peripheral.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MSWI {
    /// `MSIP` register for HART ID 0.  In multi-HART architectures,
    /// use [`MSWI::msip`] for accessing the `MSIP` of other HARTs.
    pub msip0: MSIP,
}

impl MSWI {
    /// Creates a new `MSWI` peripheral from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid `MSWI` peripheral.
    #[inline]
    pub const unsafe fn new(address: usize) -> Self {
        Self {
            msip0: MSIP::new(address),
        }
    }

    /// Returns the `MSIP` register for the HART which ID is `hart_id`.
    ///
    /// # Note
    ///
    /// For HART ID 0, you can simply use [`MSWI::msip0`].
    #[inline]
    pub fn msip<H: HartIdNumber>(&self, hart_id: H) -> MSIP {
        // SAFETY: `hart_id` is valid for the target
        unsafe { MSIP::new(self.msip0.get_ptr().add(hart_id.number()) as _) }
    }

    /// Returns the `MSIP` register for the current HART.
    ///
    /// # Note
    ///
    /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
    /// Thus, it can only be used in M-mode. For S-mode, use [`MSWI::msip`] instead.
    #[inline]
    pub fn msip_mhartid(&self) -> MSIP {
        let hart_id = riscv::register::mhartid::read();
        // SAFETY: `hart_id` is valid for the target and is the current hart
        unsafe { MSIP::new(self.msip0.get_ptr().add(hart_id) as _) }
    }
}

unsafe_peripheral!(MSIP, u32, RW);

impl MSIP {
    /// Returns `true` if a machine software interrupt is pending.
    #[inline]
    pub fn is_pending(self) -> bool {
        self.register.read() != 0
    }

    /// Writes to the register to trigger a machine software interrupt.
    #[inline]
    pub fn pend(self) {
        self.register.write(1);
    }

    /// Clears the register to unpend a machine software interrupt.
    #[inline]
    pub fn unpend(self) {
        self.register.write(0);
    }
}

#[cfg(test)]
mod test {
    use super::super::test::HartId;
    use super::*;

    #[test]
    fn test_mswi() {
        // slice to emulate the interrupt pendings register
        let raw_reg = [0u32; HartId::MAX_HART_ID_NUMBER + 1];
        // SAFETY: valid memory address
        let mswi = unsafe { MSWI::new(raw_reg.as_ptr() as _) };

        for (i, hart_id) in (0..raw_reg.len())
            .map(|i| HartId::from_number(i).unwrap())
            .enumerate()
        {
            let msip = mswi.msip(hart_id);
            assert!(!msip.is_pending());
            assert_eq!(raw_reg[i], 0);
            msip.pend();
            assert!(msip.is_pending());
            assert_ne!(raw_reg[i], 0);
            msip.unpend();
            assert!(!msip.is_pending());
            assert_eq!(raw_reg[i], 0);
        }
    }
}
