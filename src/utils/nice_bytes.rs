pub enum Size {
  BYTES(u32),
  KB(u32),
  MB(u32),
  GB(u32),
}

pub fn nice_bytes(x: f64) -> String {
    let units = vec![
        "bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB",
    ];
    let mut l = 0;
    let mut n = x;

    while n >= 1024.0 && l < units.len() - 1 {
        n /= 1024.0;
        l += 1;
    }

    format!(
        "{:.1} {}",
        n,
        if n < 10.0 && l > 0 {
            units[l]
        } else {
            units[l]
        }
    )
}
