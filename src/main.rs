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
    asm!(
        "cpuid",
        // EAX 1 selects the procesor information
        inlateout("eax") 1 as u32 => signature,
        out("edx") _, // flags
        out("ecx") _,
        out("ebx") _, // extra
    );
    signature
}

fn brand_string(src: &Vec<BrandPart>) -> String {
    let mut s = String::new();
    for part in src {
        s = s + &part.text;
    }
    s
}

fn mask(mut s: String, m: &str, r: Vec<std::ops::Range<usize>>) -> String {
    for range in r {
        let (size, _) = range.size_hint();
        s.replace_range(range, &m.repeat(size));
    }
    s
}

fn main() {
    unsafe {
        let vendor = cpuid_vendor_id();
        let signature = cpuid_signature();
        let brand = cpuid_brand();

        eprintln!("# cpuid to vmx");
        eprintln!("# vendor = {}", vendor.text);
        // TODO: DRY
        println!(r#"cpuid.0.ebx = "{:032b}""#, vendor.ebx);
        println!(r#"cpuid.0.edx = "{:032b}""#, vendor.edx);
        println!(r#"cpuid.0.ecx = "{:032b}""#, vendor.ecx);

        eprintln!("# signature");
        println!(
            r#"cpuid.1.eax = "{}""#,
            mask(
                format!("{:032b}", signature),
                "-",
                vec![(31 - 31)..(31 - 27), (31 - 15)..(31 - 13)]
            )
        );
        eprintln!(r#"# cpuid.brandstring = "{}""#, brand_string(&brand));
        for brand_part in &brand {
            let left = format!(r#"cpuid.{}"#, brand_part.eax_in);
            println!(r#"{}.eax = "{:032b}""#, left, brand_part.eax);
            println!(r#"{}.ebx = "{:032b}""#, left, brand_part.ebx);
            println!(r#"{}.ecx = "{:032b}""#, left, brand_part.ecx);
            println!(r#"{}.edx = "{:032b}""#, left, brand_part.edx);
        }
    }
}
