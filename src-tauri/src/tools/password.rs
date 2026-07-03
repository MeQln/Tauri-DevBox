// 密码生成器：用 OsRng（密码学安全熵源）从字符池随机采样。
// 保证每个启用类别至少出现一个字符（长度允许时），结果 shuffle 打乱顺序。
// exclude_ambiguous 剔除 Il1O0o 等肉眼易混字符。支持批量生成（1–10 个）。
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGIT: &str = "0123456789";
const SYMBOL: &str = "!@#$%^&*()-_=+[]{};:,.?/";
const AMBIGUOUS: &str = "Il1O0o";
const MAX_COUNT: usize = 10;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOptions {
    pub length: usize,
    pub upper: bool,
    pub lower: bool,
    pub digit: bool,
    pub symbol: bool,
    pub exclude_ambiguous: bool,
}

fn strip_ambiguous(s: &str) -> String {
    s.chars().filter(|c| !AMBIGUOUS.contains(*c)).collect()
}

// 收集启用的类别池（已按需剔除易混淆字符）。返回 (各类别池, 合并池)。
fn build_pools(opts: &PasswordOptions) -> (Vec<String>, String) {
    let pick = |on: bool, base: &str| -> Option<String> {
        if !on {
            return None;
        }
        Some(if opts.exclude_ambiguous {
            strip_ambiguous(base)
        } else {
            base.to_string()
        })
    };
    let cats: Vec<String> = [pick(opts.upper, UPPER), pick(opts.lower, LOWER), pick(opts.digit, DIGIT), pick(opts.symbol, SYMBOL)]
        .into_iter()
        .flatten()
        .collect();
    let full: String = cats.concat();
    (cats, full)
}

fn gen_one(opts: &PasswordOptions, cats: &[String], full: &str) -> String {
    let full_chars: Vec<char> = full.chars().collect();
    let mut rng = OsRng;
    let mut out: Vec<char> = Vec::with_capacity(opts.length);

    // 每个启用类别先各取一个，保证覆盖（长度不足时取到 length 为止）。
    for pool in cats {
        if out.len() >= opts.length {
            break;
        }
        let pc: Vec<char> = pool.chars().collect();
        if let Some(c) = pc.choose(&mut rng) {
            out.push(*c);
        }
    }
    // 剩余从合并池随机填充。
    while out.len() < opts.length {
        if let Some(c) = full_chars.choose(&mut rng) {
            out.push(*c);
        }
    }
    out.shuffle(&mut rng);
    out.into_iter().collect()
}

#[tauri::command]
pub fn generate_passwords(opts: PasswordOptions, count: usize) -> Result<Vec<String>, String> {
    if opts.length == 0 {
        return Err("长度必须大于 0".to_string());
    }
    let (cats, full) = build_pools(&opts);
    if full.is_empty() {
        return Err("至少选择一个字符类别".to_string());
    }
    let n = count.clamp(1, MAX_COUNT);
    Ok((0..n).map(|_| gen_one(&opts, &cats, &full)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(length: usize, upper: bool, lower: bool, digit: bool, symbol: bool, excl: bool) -> PasswordOptions {
        PasswordOptions { length, upper, lower, digit, symbol, exclude_ambiguous: excl }
    }

    // 单条快捷：count=1 取第一个。
    fn one(o: PasswordOptions) -> String {
        generate_passwords(o, 1).unwrap().pop().unwrap()
    }

    #[test]
    fn length_matches_request() {
        let p = one(opts(20, true, true, true, true, false));
        assert_eq!(p.chars().count(), 20);
    }

    #[test]
    fn chars_stay_within_enabled_pools() {
        let p = one(opts(50, false, true, true, false, false));
        for c in p.chars() {
            let ok = LOWER.contains(c) || DIGIT.contains(c);
            assert!(ok, "生成了未启用的字符: {c}");
        }
    }

    #[test]
    fn each_category_represented() {
        // 长度足够时，四个启用类别都应至少出现一个字符
        let p = one(opts(40, true, true, true, true, false));
        assert!(p.chars().any(|c| UPPER.contains(c)));
        assert!(p.chars().any(|c| LOWER.contains(c)));
        assert!(p.chars().any(|c| DIGIT.contains(c)));
        assert!(p.chars().any(|c| SYMBOL.contains(c)));
    }

    #[test]
    fn exclude_ambiguous_drops_il1o0() {
        let p = one(opts(200, true, true, true, false, true));
        for c in p.chars() {
            assert!(!AMBIGUOUS.contains(c), "出现了易混淆字符: {c}");
        }
    }

    #[test]
    fn zero_length_errors() {
        assert!(generate_passwords(opts(0, true, true, false, false, false), 1).is_err());
    }

    #[test]
    fn no_category_errors() {
        assert!(generate_passwords(opts(12, false, false, false, false, false), 1).is_err());
    }

    #[test]
    fn batch_count_clamped() {
        // 0 → 1；超大值 → 上限 10
        assert_eq!(generate_passwords(opts(16, true, true, false, false, false), 0).unwrap().len(), 1);
        assert_eq!(generate_passwords(opts(16, true, true, false, false, false), 99).unwrap().len(), MAX_COUNT);
    }

    #[test]
    fn batch_each_correct_length() {
        let v = generate_passwords(opts(24, true, true, true, true, false), 5).unwrap();
        assert_eq!(v.len(), 5);
        for p in &v {
            assert_eq!(p.chars().count(), 24);
        }
    }
}
