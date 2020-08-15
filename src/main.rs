#![feature(asm)]

struct BrandPart {
    text: String,
    eax_in: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

unsafe fn cpuid_brand() -> Vec<BrandPart> {
    let mut brands: Vec<BrandPart> = vec![];
    for eax_in in 0x80000002 as u32..0x80000005 as u32 {
        let eax: u32;
        let ebx: u32;
        let ecx: u32;
        let edx: u32;
        asm!(
            "cpuid",
            inout("eax") eax_in => eax,
            out("ebx") ebx,
            out("ecx") ecx,
            out("edx") edx,
        );
        let mut brand_part: Vec<u8> = vec![];
        brand_part.extend_from_slice(&eax.to_le_bytes());
        brand_part.extend_from_slice(&ebx.to_le_bytes());
        brand_part.extend_from_slice(&ecx.to_le_bytes());
        brand_part.extend_from_slice(&edx.to_le_bytes());
        let brand_part = String::from_utf8(brand_part);
        match brand_part {
            Ok(r) => brands.push(BrandPart {
                text: r,
                eax_in,
                eax,
                ebx,
                ecx,
                edx,
            }),
            Err(r) => {
                println!("{}", r);
                panic!("not ok...");
            }
        }
    }
    brands
}

struct Vendor {
    text: String,
    ebx: u32,
    edx: u32,
    ecx: u32,
}

unsafe fn cpuid_vendor_id() -> Vendor {
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
        return Vendor {
            text: r,
            ebx,
            edx,
            ecx,
        };
    }
    panic!("not ok...")
}

unsafe fn cpuid_signature() -> u32 {
    let signature: u32;
    let extra: u32;
    asm!(
        "cpuid",
        // EAX 1 selects the procesor information
        inout("eax") 1 as i32 => signature,
        out("edx") _, // flags
        out("ecx") _,
        out("ebx") extra, // extra
    );
    println!("extra(ebx) = {:#b}", extra);
    signature
}

fn main() {
    unsafe {
        let vendor = cpuid_vendor_id();
        println!(
            "vendor = {} (ebx = {:#b}, edx = {:#b}, ecx = {:#b})",
            vendor.text, vendor.ebx, vendor.edx, vendor.ecx
        );
        let signature = cpuid_signature();
        println!("processor signature = eax = {:#b}", signature);
        let brand = cpuid_brand();
        for brand_part in &brand {
            println!("brand_part(eax_in = {:#x}, eax = {:#b}, ebx: {:#b}, ecx: {:#b}, edx: {:#b}) = '{}'", brand_part.eax_in, brand_part.eax, brand_part.ebx, brand_part.ecx, brand_part.edx, brand_part.text);
        }
    }
}
