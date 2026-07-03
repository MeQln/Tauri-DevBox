// UUID 生成器：v4（随机）/ v7（时间排序）。支持批量、大写、去连字符。
// v4 / v7 均基于 uuid crate 内置的 OS 熵源（getrandom）。
use uuid::Uuid;

const MAX_COUNT: usize = 1000;

#[tauri::command]
pub fn generate_uuids(version: u8, count: usize, uppercase: bool, hyphen: bool) -> Vec<String> {
    let n = count.clamp(1, MAX_COUNT);
    (0..n)
        .map(|_| {
            let u = match version {
                7 => Uuid::now_v7(),
                _ => Uuid::new_v4(),
            };
            let s = if hyphen { u.to_string() } else { u.simple().to_string() };
            if uppercase { s.to_uppercase() } else { s }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_hyphenated_is_36_chars() {
        let v = generate_uuids(4, 1, false, true);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].len(), 36);
        // 形如 xxxxxxxx-xxxx-4xxx-xxxx-xxxxxxxxxxxx（第 3 段首位为 4）
        assert_eq!(v[0].chars().nth(14), Some('4'));
        assert_eq!(v[0].matches('-').count(), 4);
    }

    #[test]
    fn no_hyphen_is_32_chars() {
        let v = generate_uuids(4, 1, false, false);
        assert_eq!(v[0].len(), 32);
        assert!(!v[0].contains('-'));
    }

    #[test]
    fn uppercase_works() {
        let v = generate_uuids(4, 1, true, false);
        assert!(v[0].chars().any(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn v7_has_correct_version_digit() {
        let v = generate_uuids(7, 1, false, true);
        // v7 的第 3 段首位为 7
        assert_eq!(v[0].chars().nth(14), Some('7'));
    }

    #[test]
    fn count_is_clamped() {
        // 0 → 至少 1；超大值 → 上限 MAX_COUNT
        assert_eq!(generate_uuids(4, 0, false, true).len(), 1);
        assert_eq!(generate_uuids(4, 9999, false, true).len(), MAX_COUNT);
    }

    #[test]
    fn batch_unique() {
        let v = generate_uuids(4, 100, false, true);
        let mut set: std::collections::HashSet<&String> = std::collections::HashSet::new();
        for s in &v {
            assert!(set.insert(s), "批量生成出现重复");
        }
    }

    #[test]
    fn v7_batch_unique() {
        // v7 同毫秒内批量生成：随机段应保证唯一
        let v = generate_uuids(7, 50, false, true);
        assert_eq!(v.len(), 50);
        let mut set: std::collections::HashSet<&String> = std::collections::HashSet::new();
        for s in &v {
            assert!(set.insert(s), "v7 批量生成出现重复");
        }
    }
}
