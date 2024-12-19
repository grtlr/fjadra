pub mod miserables {
    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub links: Vec<Edge>,
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct Node {
        pub name: String,
        pub group: usize,
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct Edge {
        pub source: usize,
        pub target: usize,
    }

    impl Graph {
        pub fn load() -> anyhow::Result<Self> {
            let file = std::fs::File::open("data/miserables.json")?;
            Ok(serde_json::from_reader(file)?)
        }
    }
}

pub mod random_tree {
    use rand::{seq::IteratorRandom as _, SeedableRng as _};
    use rand_chacha::ChaCha8Rng;

    pub fn generate(n: usize) -> (Vec<String>, Vec<(usize, usize)>) {
        let mut nodes = vec!["0".to_string()];
        let mut edges = Vec::new();

        let mut rng = ChaCha8Rng::seed_from_u64(42);

        for i in 1..n {
            let new_node = i.to_string();
            let existing_node_index = (0..nodes.len())
                .choose(&mut rng)
                .expect("`nodes` is guaranteed to contain at least one node");
            nodes.push(new_node);
            edges.push((existing_node_index, i));
        }

        (nodes, edges)
    }
}
