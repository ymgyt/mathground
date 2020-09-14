/// equivalent to nCk
/// いわゆる二項係数を返す。
/// overflowするか引数が不正な場合Noneを返す。
/// パスカルの三角形の性質を利用してn=0から目的の箇所まで求めていく
/// 0 | 1
/// 1 | 1  1
/// 2 | 1  2  1
/// 3 | 1  3  3  1
/// 4 | 1  4  6  4  1
/// 5 | 1  5  10 10 5  1 (5C0, 5C1, 5C2, 5C3, 5C4, 5C5)
///
/// 対称性を利用して計算量を半分にできるらしい
pub fn binominal_coefficient(n: u64, k: u64) -> Option<u64> {
    if k == 0 || k == n {
        return Some(1);
    }
    // invalid arg
    if k > n {
        return None;
    }

    // n == 1のcaseでは必ずk == 0 || k == n に入るので考慮しなくてよい
    // n == 0 以外は必ず先頭と末尾が0でloopの前後に1をpushできるのでloopは1からはじめている
    let mut current = vec![1_u64];
    for _ in 1..n {
        let mut t = vec![1_u64];
        for w in current.windows(2) {
            match w[0].checked_add(w[1]) {
                Some(v) => t.push(v),
                None => return None,
            }
        }
        t.push(1);

        current = t;
        println!("{:#?}", current);
    }

    let k = k as usize;
    current[k - 1].checked_add(current[k])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binominal_coefficient_case_1() {
        struct Case {
            n: u64,
            k: u64,
            expect: Option<u64>,
        }
        macro_rules! case {
            (n: $n:expr, k: $k:expr, $expect:expr) => {{
                Case {
                    n: $n,
                    k: $k,
                    expect: $expect,
                }
            }};
        }
        let tests: Vec<Case> = vec![
            case!(n: 0, k: 0, Some(1)),
            case!(n: 3, k: 0, Some(1)),
            case!(n: 3, k: 3, Some(1)),
            case!(n: 4, k: 2, Some(6)),
            case!(n: 100, k: 50, None),
        ];

        for case in tests.into_iter() {
            let actual = binominal_coefficient(case.n, case.k);
            assert_eq!(
                actual, case.expect,
                "n: {} k: {} => {:?}",
                case.n, case.k, actual
            );
        }
    }
}
