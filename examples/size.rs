extern crate louds;
extern crate rand;

use louds::Louds;
use rand::{Rng, SeedableRng, StdRng};
use std::cmp;
use std::collections::VecDeque;

fn generate_tree(n: usize, max: usize) -> Louds {
    let mut louds = Louds::new();
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

    let mut queue = VecDeque::new();
    let mut remain = n - 1;
    let mut inserted = 1;
    let d = rng.gen_range(0, cmp::min(remain, max));
    queue.push_back(d);
    remain = remain - d;
    while let Some(d) = queue.pop_front() {
        louds.push_node(d);
        inserted += 1;
        for _ in 0..d {
            let dd = rng.gen_range(0, cmp::min(remain, max));
            queue.push_back(dd);
            remain = remain - dd;
        }
    }
    assert_eq!(inserted, n, "# of inserted nodes should be {}", n);

    louds
}

fn main() {
    let test_cases = &[
        (1000000, 10),
        (1000000, 1000),
        (1000000, 100000),
        (100000000, 10),
        (100000000, 1000),
        (100000000, 100000),
    ];

    println!("n: # of nodes, m: maximum # of degree\n");

    for &(n, m) in test_cases {
        let tree = generate_tree(n, m);
        let size = tree.size();
        let rate = 8.0 * size as f64 / n as f64;
        println!(
            "n = {}, m = {}: {} bytes ({} bit / node)",
            n, m, size, rate
        );
    }
}
