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

    

}
