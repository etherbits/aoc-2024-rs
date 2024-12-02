use std::collections::HashMap;

use crate::utils;

pub fn p1() {
    let lines = utils::get_file_lines("src/d1/p1.txt");
    let mut l_nums: Vec<i32> = Vec::new();
    let mut r_nums: Vec<i32> = Vec::new();
    let mut diff = 0;

    lines.iter().for_each(|ln| {
        let nums: Vec<&str> = ln.split("   ").collect();

        let l_num = nums[0].parse::<i32>().unwrap();
        let r_num = nums[1].parse::<i32>().unwrap();

        l_nums.push(l_num);
        r_nums.push(r_num);
    });

    l_nums.sort();
    r_nums.sort();

    for (i, l_num) in l_nums.iter().enumerate() {
        diff += (l_num - r_nums[i]).abs();
    }

    println!("{}", diff);
}

pub fn p2() {
    let lines = utils::get_file_lines("src/d1/p2.txt");
    let mut l_nums: Vec<i32> = Vec::new();
    let mut mults: HashMap<i32, i32> = HashMap::new();

    lines.iter().for_each(|ln| {
        let nums: Vec<&str> = ln.split("   ").collect();

        let l_num = nums[0].parse::<i32>().unwrap();
        let r_num = nums[1].parse::<i32>().unwrap();

        l_nums.push(l_num);
        let mult = match mults.get(&r_num) {
            Some(m) => m,
            None => &0,
        };

        mults.insert(r_num, mult + 1);
    });

    let score = l_nums.into_iter().fold(0, |acc, num| {
        acc + num
            * match mults.get(&num) {
                Some(m) => m,
                None => &0,
            }
    });

    println!("{}", score);
}
