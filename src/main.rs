use std::io::{Read, BufReader, BufRead};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone)]
struct World {
    pops: Popuration,
    max_pops: usize,
    generation: u64,
    crossover_rate: f64,
    mutation_rate: f64,
    rand: StdRng,
    nodes: Nodes,
}

impl World {
    fn new(rand: StdRng, fname: &str, max_pops: usize) -> World {
        let nodes = Nodes::new(std::fs::File::open(fname).unwrap());
        let pops = Popuration::new(rand.clone(), nodes.nodes.len(), max_pops);
        World{
            pops,
            max_pops,
            generation: 0,
            crossover_rate: 0.7,
            mutation_rate: 0.01,
            rand,
            nodes,
        }
    }
    fn crossover_rate(&mut self, rate: f64) -> Result<&mut World, &str> {
        if rate < 0.0 || 1.0 < rate {
            return Err("crossover rate shoud be between 0.0 to 1.0");
        };
        self.crossover_rate = rate;
        Ok(self)
    }
    fn mutation_rate(&mut self, rate: f64) -> Result<&mut World, &str> {
        if rate < 0.0 || 1.0 < rate {
            return Err("mutation rate shoud be between 0.0 to 1.0");
        };
        self.mutation_rate = rate;
        Ok(self)
    }
    fn select(&mut self) -> Gene {
        let nodes = self.nodes.clone();
        let total = self.pops.pops.iter().fold(0.0, |a, x|{a + nodes.clone().cost(x).unwrap()});
        let ps: Vec<f64> = self.pops.pops.iter().map(|x|{nodes.clone().cost(x).unwrap()/total}).collect();
        let q = self.rand.gen_range(0.0, 1.0);
        let mut p = 0.0;
        for i in 0..ps.len() {
            p += ps[i];
            if p > q {
                return self.pops.pops[i].clone();
            }
        }
        panic!("popuration exausted")
    }
    fn step(&mut self) {
        let mut new_pops = vec![];
        while new_pops.len() < self.max_pops {
            let q = self.rand.gen_range(0.0, 1.0);
            if q < self.crossover_rate {
                let mut x = self.select();
                let mut y = self.select();
                x.crossover(&mut self.rand, &mut y);
                new_pops.push(x);
                new_pops.push(y);
            } else if self.crossover_rate < q && q < self.crossover_rate + self.mutation_rate{
                let mut x = self.select();
                x.mutate(&mut self.rand);
                new_pops.push(x);
            } else {
                let x = self.select();
                new_pops.push(x);

            }
        }
    }
}

#[derive(Debug, Clone)]
struct Popuration {
    pops: Vec<Gene>,
}

impl Popuration {
    fn new(mut rand: StdRng, len: usize, max_pops: usize) -> Popuration {
        let pops = (0..max_pops).map(|_|{Gene::new(&mut rand, len)}).collect();
        Popuration{
            pops,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Gene {
    gene: Vec<usize>,
}

impl Gene {
    fn new(rand: &mut StdRng, x: usize) -> Gene {
        let mut gene: Vec<usize> = (0..x).collect();
        gene.shuffle(rand);
        Gene{gene}
    }
    fn crossover(&mut self, rand: &mut StdRng,  g1_: &mut Gene) {
        if self.gene.len() != g1_.gene.len() {
            panic!("different length")
        };
        let g0 = self.clone();
        let g1 = g1_.clone();
        let n = rand.gen_range(0, g0.gene.len());
        let (g0_head, g0_tail) = g0.gene.split_at(n);
        let (g1_head, g1_tail) = g1.gene.split_at(n);
        let g0_tail_ordered = order(g0_tail.to_vec());
        let g1_tail_ordered = order(g1_tail.to_vec());
        let mut g0_tail_res = vec![];
        let mut g1_tail_res = vec![];
        for i in 0..g0_tail.len() {
            let (k0, _) = g0_tail_ordered[i];
            let (k1, _) = g1_tail_ordered[i];
            let n0 = find(g0_tail_ordered.clone(), k1).unwrap();
            let n1 = find(g1_tail_ordered.clone(), k0).unwrap();
            g0_tail_res.push(g0_tail[n0]);
            g1_tail_res.push(g1_tail[n1]);
        };
        self.gene = [g0_head.to_vec(), g0_tail_res].concat();
        g1_.gene = [g1_head.to_vec(), g1_tail_res].concat();
    }
    fn mutate(&mut self, rand: &mut rand::rngs::StdRng) {
        let j = rand.gen_range(0, self.gene.len());
        let k = rand.gen_range(0, self.gene.len());
        self.gene.swap(j, k);
    }
}

fn find(vec: Vec<(usize, usize)>, n: usize) -> Option<usize> {
    for i in 0..vec.len() {
        let (j, _) = vec[i];
        if j == n {
           return Some(i);
        }
    };
    None
}

fn order(vec: Vec<usize>) -> Vec<(usize, usize)> {
    let mut v = vec.clone();
    v.sort();
    let mut order = vec![];
    for n in 0..v.len() {
        order.push((n, v[n]))
    };
    let mut res = vec![];
    for u in vec {
        let i = order.binary_search_by(|(_, x)| x.cmp(&u) ).unwrap();
        res.push(order[i]);
    }
    res
}


#[derive(Debug, Clone)]
struct Nodes {
    nodes: Vec<Node>,
}

impl Nodes {
    fn new<R: Read>(r: R) -> Nodes {
        let mut nodes = vec![];
        for line in BufReader::new(r).lines(){
            let l = line.unwrap();
            let coords: Vec<&str> = l.split(' ').collect();
            let x = coords[0].parse::<f64>().unwrap();
            let y = coords[1].parse::<f64>().unwrap();
            nodes.push(Node{x, y});
        };
        Nodes{nodes}
    }
    fn dist(self, n: usize, m: usize) -> Option<f64> {
        let l = self.nodes.len();
        if n > l || m > l {
            None
        } else {
            Some(self.nodes[n].dist_to(self.nodes[m]))
        }
    }
    fn cost(self, g: &Gene) -> Option<f64> {
        let mut sorted = g.clone().gene;
        sorted.sort();
        for n in 0..sorted.len() {
            if sorted[n] != n {
                return None
            }
        }
        let mut total = 0f64;
        for n in 0..g.gene.len() - 1{
            match self.clone().dist(g.gene[n], g.gene[n+1]) {
                None => return None,
                Some(v) => total += v,
            }
        };
        Some(total)
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: f64,
    y: f64,
}

impl Node {
    fn dist_to(self, b: Node) -> f64 {
        ((self.x - b.x).powi(2i32) + (self.y - b.y).powi(2i32)).sqrt()
    }
}

fn main() {
    let rand: rand::rngs::StdRng = rand::SeedableRng::from_seed([42u8; 32]);
    let mut world = World::new(rand, "./ulysses16.tsp", 10);
    world.crossover_rate(0.5).unwrap();
    for _ in 0..1000 {
        world.step();
    };
    for p in world.pops.pops {
        println!("{:?}", p);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn dist_node() {
        let a = Node{x: 1.0, y: 2.0};
        let b = Node{x: 3.0, y: 4.0};
        let d = a.dist_to(b);
        assert!(2.8 < d && d < 2.9, "dist");
    }

    #[test]
    fn dist_nodes() {
        let ns = Nodes{nodes: vec![
            Node{x:1.0, y:2.0},
            Node{x:3.0, y:4.0},
        ]};
        let d = ns.dist(0, 1).unwrap();
        assert!(2.8 < d && d < 2.9, "dist");
    }

    #[test]
    fn dist_nodes_not_exist() {
        let ns = Nodes{nodes: vec![
            Node{x:1.0, y:2.0},
            Node{x:3.0, y:4.0},
        ]};
        let d = ns.dist(0, 3);
        assert_eq!(d, None);
    }

    #[test]
    fn cost() {
        let ns = Nodes{nodes: vec![
            Node{x:1.0, y:2.0},
            Node{x:3.0, y:4.0},
        ]};
        let d = ns.cost(&Gene{gene: vec![0, 1]}).unwrap();
        assert!(2.8 < d && d < 2.9  , "cost");
    }

    #[test]
    fn cost_failed() {
        let ns = Nodes{nodes: vec![
            Node{x:1.0, y:2.0},
            Node{x:3.0, y:4.0},
        ]};
        let d = ns.cost(&Gene{gene: vec![1, 1]});
        assert_eq!(d, None);
    }

    #[test]
    fn new_gene() {
        let mut rand: rand::rngs::StdRng = rand::SeedableRng::from_seed([42u8; 32]);
        let a = Gene::new(&mut rand, 8);
        assert_eq!(a,Gene{gene: vec![5, 1, 4, 6, 7, 3, 0, 2]});
    }
    #[test]
    fn crossover_gene() {
        let mut rand: rand::rngs::StdRng = rand::SeedableRng::from_seed([12u8; 32]);
        let mut a = Gene::new(&mut rand, 8);
        let mut b = Gene::new(&mut rand, 8);
        a.crossover(&mut rand, &mut b);
        assert_eq!(a, Gene{gene: vec![5, 3, 1, 4, 2, 7, 0, 6]});
        assert_eq!(b, Gene{gene: vec![6, 5, 3, 1, 0, 4, 7, 2]});
            }
    #[test]
    fn mutate_gene() {
        let mut rand: rand::rngs::StdRng = rand::SeedableRng::from_seed([42u8; 32]);
        let mut a = Gene::new(&mut rand, 8);
        a.mutate(&mut rand);
        assert_eq!(a, Gene{gene: vec![6, 1, 4, 5, 7, 3, 0, 2]});
    }
    #[test]
    fn test_order() {
        let vec = vec![5, 3, 8, 7];
        assert_eq!(order(vec), vec![(1, 5), (0, 3), (3, 8), (2, 7)])
    }
}
