type Gene = Vec<usize>;

#[derive(Debug, Clone)]
struct Nodes {
    nodes: Vec<Node>,
}

impl Nodes {
    fn dist(self, n: usize, m: usize) -> Option<f64> {
        let l = self.nodes.len();
        if n > l || m > l {
            None
        } else {
            Some(self.nodes[n].dist_to(self.nodes[m]))
        }
    }
    fn cost(self, g: Gene) -> Option<f64> {
        let mut sorted = g.clone();
        sorted.sort();
        for n in 0..sorted.len() {
            if sorted[n] != n {
                return None
            }
        }
        let mut total = 0f64;
        for n in 0..g.len() - 1{
            match self.clone().dist(g[n], g[n+1]) {
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
    println!("Hello, world!");
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
        let d = ns.cost(vec![0, 1]).unwrap();
        assert!(2.8 < d && d < 2.9  , "cost");
    }

    #[test]
    fn cost_failed() {
        let ns = Nodes{nodes: vec![
            Node{x:1.0, y:2.0},
            Node{x:3.0, y:4.0},
        ]};
        let d = ns.cost(vec![1, 1]);
        assert_eq!(d, None);
    }

}
