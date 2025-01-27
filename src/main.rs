use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug, Default)]
struct Graph {
    red: HashMap<u32, HashSet<u32>>,
    black: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    pub fn from_gr<R: BufRead>(reader: &mut R) -> std::io::Result<Self> {
        let mut graph = Self::new();
        reader.lines().try_for_each(|line| {
            line.and_then(|line| {
                let elements: Vec<_> = line.split(' ').collect();
                match elements[0] {
                    "c" | "p" => {}
                    _ => {
                        let u = elements[0].parse::<u32>().map_err(|e: ParseIntError| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                        })?;
                        let v = elements[1].parse::<u32>().map_err(|e: ParseIntError| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                        })?;
                        graph.add_black_edge(u, v);
                    }
                }
                Ok(())
            })
        })?;
        Ok(graph)
    }

    fn new() -> Self {
        Self {
            red: Default::default(),
            black: Default::default(),
        }
    }

    fn add_red_edge(&mut self, u: u32, v: u32) {
        self.red.entry(u).or_default().insert(v);
        self.red.entry(v).or_default().insert(u);
        self.black.entry(u).or_default();
        self.black.entry(v).or_default();
    }

    fn add_black_edge(&mut self, u: u32, v: u32) {
        self.black.entry(u).or_default().insert(v);
        self.black.entry(v).or_default().insert(u);
        self.red.entry(u).or_default();
        self.red.entry(v).or_default();
    }

    fn delete_vertex(&mut self, u: u32) {
        if let Some((_, black)) = self.black.remove_entry(&u) {
            for v in black {
                self.black.get_mut(&v).unwrap().remove(&u);
            }
        }
        if let Some((_, red)) = self.red.remove_entry(&u) {
            for v in red {
                self.red.get_mut(&v).unwrap().remove(&u);
            }
        }
    }

    // merges v to u. label u remains in the graph, label v is removed.
    fn merge(&mut self, u: u32, v: u32) {
        assert!(self.black.contains_key(&u));
        assert!(self.black.contains_key(&v));
        assert!(self.red.contains_key(&u));
        assert!(self.red.contains_key(&v));

        let u_red: HashSet<u32> = self.red.remove(&u).unwrap();
        let u_black: HashSet<u32> = self.black.remove(&u).unwrap();
        let v_red: HashSet<u32> = self.red.remove(&v).unwrap();
        let v_black: HashSet<u32> = self.black.remove(&v).unwrap();

        let mut symmetric_difference: HashSet<u32> =
            u_black.symmetric_difference(&v_black).copied().collect();
        symmetric_difference.remove(&u);
        symmetric_difference.remove(&v);

        let mut red_union: HashSet<u32> = u_red.union(&v_red).copied().collect();
        red_union.remove(&u);
        red_union.remove(&v);

        let new_red: Vec<u32> = symmetric_difference.union(&red_union).copied().collect();
        let new_black: Vec<u32> = u_black.intersection(&v_black).copied().collect();

        self.delete_vertex(u);
        self.delete_vertex(v);

        for v in new_red {
            self.add_red_edge(u, v);
        }
        for v in new_black {
            self.add_black_edge(u, v);
        }
    }

    fn count_merge(&mut self, u: u32, v: u32) -> (u32, u32) {
        assert!(self.black.contains_key(&u));
        assert!(self.black.contains_key(&v));
        assert!(self.red.contains_key(&u));
        assert!(self.red.contains_key(&v));

        let u_red = self.red.get(&u).unwrap();
        let old_red_degree = u_red.len();
        let u_black = self.black.get(&u).unwrap();
        let v_red = self.red.get(&v).unwrap();
        let v_black = self.black.get(&v).unwrap();

        let mut symmetric_difference: HashSet<u32> =
            u_black.symmetric_difference(&v_black).copied().collect();
        symmetric_difference.remove(&u);
        symmetric_difference.remove(&v);

        let mut red_union: HashSet<u32> = u_red.union(&v_red).copied().collect();
        red_union.remove(&u);
        red_union.remove(&v);

        (old_red_degree as u32, red_union.len() as u32)
    }

    fn greedy(mut self) -> Sequence {
        let mut width = 0;
        let mut contractions = vec![];

        let mut available_vertex: HashSet<u32> = self.black.keys().copied().collect();

        let mut best: Option<(u32, u32)> = None;
        let mut best_value: Option<u32> = None;
        while available_vertex.len() > 1 {
            for u in &available_vertex {
                for v in &available_vertex {
                    if u < v {
                        let (_, value) = self.count_merge(*u, *v);
                        if best_value == None || value < best_value.unwrap() {
                            best = Some((*u, *v));
                            best_value = Some(value);
                        }
                    }
                }
            }
            let (u, v) = best.unwrap();
            let value = best_value.unwrap();
            self.merge(u, v);
            contractions.push((u, v));
            width = width.max(value);

            best = None;
            best_value = None;
            available_vertex.remove(&v);
        }

        Sequence {
            contractions,
            width,
        }
    }
}

struct Sequence {
    contractions: Vec<(u32, u32)>,
    width: u32,
}

fn main() -> std::io::Result<()> {
    let mut reader = BufReader::new(stdin());
    let graph: Graph = Graph::from_gr(&mut reader)?;

    let sequence = graph.greedy();
    let width = sequence.width;



    println!("c tww: {width}");
    for (u, v) in sequence.contractions {
        println!("{u} {v}");
    }

    Ok(())
}
