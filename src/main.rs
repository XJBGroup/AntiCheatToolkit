use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

use regex::internal::ExecBuilder;

fn calc_lcs(a: &[u8], b: &[u8]) -> f32 {
    let mut dp = vec![vec![0; b.len() + 5]; 2];
    let mut idx = 0;
    for i in 0..a.len() {
        idx ^= 1;
        dp[idx][0] = if a[i] == b[0] { 1 } else { 0 };
        for j in 1..b.len() {
            dp[idx][j] = max(dp[idx ^ 1][j], dp[idx][j - 1]);
            if a[i] == b[j] {
                dp[idx][j] = max(dp[idx][j], dp[idx ^ 1][j - 1] + 1);
            }
        }
    }
    dp[idx][b.len() - 1] as f32 / min(a.len(),b.len()) as f32
}

#[inline]
fn ksm(mut a: i64, mut b: i64, MOD: i64) -> i64 {
    let mut ret: i64 = 1;
    while b > 0 {
        if (b & 1) > 0 {
            ret = ret * a % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    ret
}

#[inline]
fn get_hash_segment(a: &Vec<u8>, k: usize, seed: i64, MOD: i64) -> Vec<i32> {
    assert!(a.len() >= k && k >= 1);
    let po = ksm(seed.clone(), (k - 1) as i64, MOD.clone());
    let mut ret = Vec::new();
    let mut now = 0;
    for i in 0..k - 1 {
        now = (now * seed + a[i] as i64) % MOD;
    }
    for i in k..a.len() {
        now = (now * seed + a[i] as i64) % MOD;
        ret.push(now as i32);
        now = (now - a[i - k + 1] as i64 * po % MOD) % MOD;
        if now < 0 {
            now += MOD;
        }
    }
    ret
}

#[inline]
fn monotone(a: &Vec<i32>, k: usize) -> Vec<i32> {
    let mut dQ: VecDeque<usize> = VecDeque::new();
    let mut ans = Vec::with_capacity(a.len() + 1 - k);
    for i in 0..a.len() {
        while !dQ.is_empty() && a[*dQ.back().unwrap()] >= a[i] {
            dQ.pop_back().unwrap();
        }
        dQ.push_back(i);
        if i >= k - 1 {
            ans.push(a[*dQ.front().unwrap()]);
            while !dQ.is_empty() && *dQ.front().unwrap() <= i + 1 - k {
                dQ.pop_front().unwrap();
            }
        }
    }
    ans
}

#[inline]
fn regular(a: &str) -> Vec<u8> {
    let re = ExecBuilder::new("\r|\n|\\s|/\\*(.|\n)*?\\*/|//(.|\n)*?\n").build().unwrap().into_regex();
    let seq: String = re.replace_all(a, "").into_owned();
    println!("{}", seq);
    seq.as_bytes().to_owned()
}

//k is hash windows
//m is monotone queue windows size
#[inline]
fn winnowing(_a: &str, _b: &str, k: usize, m: usize, seed: i64, MOD: i64) -> f32 {
    let a = regular(_a);
    let hash_a = get_hash_segment(&a, k.clone(), seed.clone(), MOD.clone());
    let pack_a = monotone(&hash_a, m.clone());

    let b = regular(_b);
    let hash_b = get_hash_segment(&b, k.clone(), seed.clone(), MOD.clone());
    let pack_b = monotone(&hash_b, m.clone());

    let mut similar = 0;
    let mut mp: HashMap<i32, i32> = HashMap::new();
    for i in pack_a.iter() {
        if mp.contains_key(i) {
            *mp.get_mut(i).unwrap() += 1;
        } else {
            mp.insert(i.clone(), 1);
        }
    }
    for i in pack_b.iter() {
        if mp.contains_key(i) {
            similar += mp.get(i).unwrap().clone();
        }
    }
    println!("{} {} {}", pack_a.len(), pack_b.len(), similar);
    similar as f32 / min(pack_a.len(),pack_b.len()) as f32
}

fn main() {

}
