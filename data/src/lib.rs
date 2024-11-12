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
