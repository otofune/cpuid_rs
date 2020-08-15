#![feature(asm)]

unsafe fn cpuid_vendor_id() -> (String, u32, u32, u32) {
    let ebx: u32;
    let edx: u32;
    let ecx: u32;
    asm!(
        "cpuid",
        in("eax") 0,
        lateout("ebx") ebx,
        lateout("edx") edx,
        lateout("ecx") ecx,
    );
    let mut vendor: Vec<u8> = vec![];
    vendor.extend_from_slice(&ebx.to_le_bytes());
    vendor.extend_from_slice(&edx.to_le_bytes());
    vendor.extend_from_slice(&ecx.to_le_bytes());
    if let Ok(r) = String::from_utf8(vendor) {
        return (r, ebx, edx, ecx);
    }
    panic!("not ok...")
}

unsafe fn cpuid_signature() -> u32 {
    let signature: u32;
    asm!(
        "cpuid",
        // EAX 1 selects the procesor information
        inout("eax") 1 as i32 => signature,
        lateout("edx") _, // flags
        lateout("ecx") _,
        lateout("ebx") _, // extra
    );
    signature
}

fn main() {
    unsafe {
        let (vendor, vendor_ebx, vendor_edx, vendor_ecx) = cpuid_vendor_id();
        println!("vendor = {}, ebx = {:0x}, edx = {:0x}, ecx = {:0x}", vendor, vendor_ebx, vendor_edx, vendor_ecx);
        let signature = cpuid_signature();
        println!("processor signature = {:0x}", signature);
    }
}
