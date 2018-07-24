#![feature(test)]

extern crate louds;
extern crate rand;
extern crate test;

use louds::Louds;
use rand::{Rng, SeedableRng, StdRng};
use std::cmp;
use std::collections::VecDeque;
use test::Bencher;

const TRIALS: usize = 10000;

fn generate_tree(n: usize, max: usize) -> Louds {
    let mut louds = Louds::new();
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

    let mut queue = VecDeque::new();
    let mut remain = n - 1;
    let d = rng.gen_range(0, cmp::min(remain, max));
    queue.push_back(d);
    remain = remain - d;
    while let Some(d) = queue.pop_front() {
        louds.push_node(d);
        for _ in 0..d {
            let dd = rng.gen_range(0, cmp::min(remain, max));
            queue.push_back(dd);
            remain = remain - dd;
        }
    }
    louds
}

#[bench]
fn first_child_1000000_narrow(b: &mut Bencher) {
    bench_first_child(1000000, 10, b);
}

#[bench]
fn first_child_1000000_normal(b: &mut Bencher) {
    bench_first_child(1000000, 1000, b);
}

#[bench]
fn first_child_1000000_wide(b: &mut Bencher) {
    bench_first_child(1000000, 100000, b);
}

#[bench]
fn first_child_100000000_narrow(b: &mut Bencher) {
    bench_first_child(100000000, 10, b);
}

#[bench]
fn first_child_100000000_normal(b: &mut Bencher) {
    bench_first_child(100000000, 1000, b);
}

#[bench]
fn first_child_100000000_wide(b: &mut Bencher) {
    bench_first_child(100000000, 100000, b);
}

fn bench_first_child(n: usize, m: usize, b: &mut Bencher) {
    let louds = generate_tree(n, m);
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
    let indices = (0..TRIALS)
        .map(|_| rng.gen_range(0, n))
        .collect::<Vec<usize>>();
    b.iter(|| {
        for i in &indices {
            louds.first_child(*i);
        }
    })
}
