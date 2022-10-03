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

pub enum FindRegType<'a> {
    Reg(&'a ch347_rs::Register),
    RegItem(&'a ch347_rs::Register, usize),
}

pub fn find_reg_by_name<'a>(
    name: &str,
    reg_defines: &'a [ch347_rs::Register],
) -> Option<FindRegType<'a>> {
    let name = &name.to_lowercase();

    // search register name
    for r in reg_defines {
        if name.eq(&r.name.to_lowercase()) {
            return Some(FindRegType::Reg(r));
        }
    }

    // search register item name
    for r in reg_defines {
        let r_items = match r.items {
            None => continue,
            Some(a) => a,
        };

        for (k, ri) in r_items.iter().enumerate() {
            if name.eq(&ri.name.to_lowercase()) {
                return Some(FindRegType::RegItem(r, k));
            }
        }
    }

    // search register item alias
    for r in reg_defines {
        let r_items = match r.items {
            None => continue,
            Some(a) => a,
        };

        for (k, ri) in r_items.iter().enumerate() {
            for &ria in ri.alias {
                if name.eq(&ria.to_lowercase()) {
                    return Some(FindRegType::RegItem(r, k));
                }
            }
        }
    }

    None
}
