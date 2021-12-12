use std::collections::{HashMap, HashSet};

fn main() {
    let mut g: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    include_str!("../input/12.txt").lines().for_each(|line| {
        if let Some((a, b)) = line.split_once('-') {
            g.entry(a).or_insert_with(HashSet::new).insert(b);
            g.entry(b).or_insert_with(HashSet::new).insert(a);
        }
    });

    println!("{}", go(&g, &mut HashMap::from([("start", 0)]), "start", 1));
    println!("{}", go(&g, &mut HashMap::from([("start", 0)]), "start", 2));

    fn go(
        graph: &HashMap<&'static str, HashSet<&'static str>>,
        path: &mut HashMap<&'static str, usize>,
        start: &'static str,
        max: usize,
    ) -> usize {
        // this check is only valid if we allow running through small caves more than once
        if max > 1 && path.values().filter(|&v| *v == 0).count() > 2 {
            return 0;
        }
        graph[start]
            .iter()
            .map(|&child| follow(child, path, graph, max))
            .sum()
    }

    fn follow(
        child: &'static str,
        path: &mut HashMap<&'static str, usize>,
        graph: &HashMap<&'static str, HashSet<&'static str>>,
        max: usize,
    ) -> usize {
        // at tne end - counts as 1 path
        if child == "end" {
            return 1;
        }
        // we have gone through small caves too many times - can't go down this child
        if path.get(child) == Some(&0) {
            return 0;
        }
        // set this before iteration and unset aftewards - is there a better pattern?
        if child.to_uppercase() != child {
            *path.entry(child).or_insert(max) -= 1;
        }
        let val = go(graph, path, child, max);
        if child.to_uppercase() != child {
            *path.get_mut(child).unwrap() += 1;
        }
        val
    }
}
