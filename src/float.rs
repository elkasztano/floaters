use crate::Sign;

pub fn canonical(value: u64) -> f64 {
    (value >> 11) as f64 * 1.110223e-16 // = 0x1.0p-53 hex literal
}

pub fn noncanonical(mut value: u64) -> f64 {
    value |= u64::MAX << (56 + 2) >> 2; // set bits that should be 1
    value &= !(3u64 << 62 | 1u64 << 52); // clear bits that should be 0
    f64::from_bits(value)
}

pub fn with_params(mut value: u64, left_shift: i8, signed: Sign) -> f64 {
    let sign_mask = if signed == Sign::Signed { 1u64 } else { 3u64 };
    let ls = left_shift as usize;
    value |= u64::MAX << (ls + 2) >> 2;
    value &= !(sign_mask << 62 | 1u64 << 52);
    f64::from_bits(value)
}

pub fn exponent(mut value: u64, exponent: u16, signed: Sign) -> f64 {
    let exp = (exponent << 5 >> 5) as u64;
    if signed == Sign::Signed
    { value &= !(2047u64 << 52); } else { value &= !(4095u64 << 52); }
    value |= exp << 52;
    f64::from_bits(value)
}

pub fn canonical_tuple(value: u64) -> (f32, f32) {
    let (le, be) = u32_from_u64(value);
    ( (le >> 9) as f32 * 1.192093e-07, // = 0x1.0p-23 hex literal
    (be >> 9) as f32 * 1.192093e-07 )
}

pub fn noncanonical_tuple(value: u64) -> (f32, f32) {
    let (mut le, mut be) = u32_from_u64(value);
    ( f32_from_u32(&mut le, 26, Sign::Unsigned),
    f32_from_u32(&mut be, 26, Sign::Unsigned) )
}

pub fn tuple_with_params(value: u64, left_shift: i8, signed: Sign) -> (f32, f32) {
    let (mut le, mut be) = u32_from_u64(value);
    ( f32_from_u32(&mut le, left_shift, signed),
    f32_from_u32(&mut be, left_shift, signed) )
}

pub fn tuple_wild(value: u64) -> (f32, f32) {
    let (le, be) = u32_from_u64(value);
        ( f32::from_bits(le), f32::from_bits(be) )
}

pub fn tuple_exp(value: u64, exponent: u8, signed: Sign) -> (f32, f32) {
    let (mut le, mut be) = u32_from_u64(value);
    ( specified_exp_f32(&mut le, exponent, signed),
    specified_exp_f32(&mut be, exponent, signed) )
}


fn u32_from_u64(bits: u64) -> (u32, u32) {
    ( (bits << 32 >> 32) as u32,
    (bits >> 32) as u32 )
}

// reasonable values for left_shift: 26, 21..=29
fn f32_from_u32(bits: &mut u32, left_shift: i8, signed: Sign) -> f32 {
    let sign_mask = if signed == Sign::Signed { 1u32 } else { 3u32 };
    *bits |= u32::MAX << (left_shift + 2) >> 2;
    *bits &= !(sign_mask << 30 | 1u32 << 23);
    f32::from_bits(*bits)
}

fn specified_exp_f32(bits: &mut u32, exponent: u8, signed: Sign) -> f32 {
    if signed == Sign::Signed { *bits &= !(255u32 << 23); } 
    else { *bits &= !(511u32 << 23); }
    *bits |= (exponent as u32) << 23;
    f32::from_bits(*bits)
}