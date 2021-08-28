use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::option::Option;
use std::vec::Vec;

/// Reads and returns list of `i32` from input file as a vector.
fn read_input(file_name: &str) -> std::io::Result<Vec<i32>> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut nums: Vec<i32> = Vec::new();
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }
        let v = line.trim().parse::<i32>().unwrap();
        nums.push(v);
        line.clear();
    }
    return Ok(nums);
}

/// Performs two sum on vector `nums` with target sum `target`.
fn twosum(nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut cache = HashSet::new();
    for v in nums {
        cache.insert(v);
    }
    for v in nums {
        let other = target - v;
        if cache.contains(&other) {
            return Some((*v, other));
        }
    }
    return None;
}

/// Performs three sum on vector `nums` with target sum `target`.
fn threesum(nums: &mut Vec<i32>, target: i32) -> Option<(i32, i32, i32)> {
    nums.sort();
    for curr in 0..nums.len() {
        let pretarget = target - nums[curr];
        let mut p = curr + 1;
        let mut q = nums.len() - 1;
        while p < q {
            let s = nums[p] + nums[q];
            if s == pretarget {
                return Some((nums[curr], nums[p], nums[q]));
            }
            if s < pretarget {
                p += 1;
            } else {
                q -= 1;
            }
        }
    }
    return None;
}

fn main() -> std::io::Result<()> {
    let target: i32 = 2020;
    let mut nums = read_input("input")?;

    match twosum(&nums, target) {
        Some((v1, v2)) => {
            println!("{}", v1 * v2);
        }
        None => {
            println!("No result.");
        }
    }

    match threesum(&mut nums, target) {
        Some((v1, v2, v3)) => {
            println!("{}", v1 * v2 * v3);
        }
        None => {
            println!("No result.");
        }
    }

    Ok(())
}
