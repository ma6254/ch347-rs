pub fn format_byte_unit<'a>(a: usize) -> String {
    let mut ret = String::new();

    if a < 1024 {
        ret = format!("{}B", a)
    } else if a < 1024 * 1024 {
        ret.push_str(&format!("{}KB", a / 1024));

        if (a % 1024) != 0 {
            ret.push_str(" ");
            ret.push_str(&format_byte_unit(a % 1024));
        }
    } else {
        ret.push_str(&format!("{}MB", a / (1024 * 1024)));

        if (a % (1024 * 1024)) != 0 {
            ret.push_str(" ");
            ret.push_str(&format_byte_unit(a % (1024 * 1024)));
        }
    }

    return ret;
}

pub fn format_byte_per_sec(a: f64) -> String {
    if a < (1024.0) {
        format!("{:.2} B/S ", a)
    } else if a < (1024.0 * 1024.0) {
        format!("{:.2} KB/S ", a / 1024.0)
    } else {
        format!("{:.2} MB/S ", a / 1024.0 / 1024.0)
    }
}
