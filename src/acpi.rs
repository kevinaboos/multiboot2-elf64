use super::Tag;

use core::{mem, slice};

#[derive(Debug)]
#[repr(packed)] // repr(C) would add unwanted padding before first_section
pub struct AcpiOldTag {
    typ: u32,
    size: u32,
    rsdp: RSDPv1,
}

impl AcpiOldTag {
    /// Returns the RSDPv1 or None if a valid RSDPv1 couldn't be found. 
    pub fn get_rsdp(&self) -> Option<&RSDPv1> {
        // sanity check that structure sizes match tag's size element
        if ((self.size as usize) - mem::size_of::<RSDPv1>()) != mem::size_of::<Tag>() {
            return None;
        }
        if self.rsdp.is_valid() {
            Some(&self.rsdp)
        }
        else {
            None
        }
    }
}


const RSDP_SIGNATURE: [u8; 8] = [b'R', b'S', b'D', b' ', b'P', b'T', b'R', b' '];


#[derive(Debug)]
#[repr(packed)]
/// The Root System Descriptor Pointer. 
pub struct RSDPv1 {
	signature: [u8; 8],
	checksum: u8,
	oemid: [u8; 6],
	revision: u8,
	rsdt_phys_addr: u32,
}

impl RSDPv1 {
    /// Returns the **physical address** of the Root System Descriptor Table. 
    pub fn rsdt_phys_addr(&self) -> u32 {
        self.rsdt_phys_addr
    }

	fn is_valid(&self) -> bool {
        (self.revision < 2)
		&& (self.signature == RSDP_SIGNATURE)
		&& (self.checksum() & 0xFF == 0)
	}

    /// Sums up every individual byte in the structure
    fn checksum(&self) -> usize {
        let ptr = self as *const RSDPv1 as *const u8;
        unsafe {
            let bytes = slice::from_raw_parts(ptr, mem::size_of::<RSDPv1>());
            bytes.iter().fold(0, |a, &b| { a + (b as usize) } )
        }
    }
}
