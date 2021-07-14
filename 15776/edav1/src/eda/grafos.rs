/*
EDA
José Jasnau Caeiro
2021-06-12

algoritmos com grafos para a unidade curricular de
Estruturas de Dados e Algoritmos
*/
use std::collections::{HashMap, VecDeque, HashSet, LinkedList};
use petgraph::graph::{NodeIndex, DiGraph, Node, UnGraph, EdgeIndex};
use petgraph::Direction::Outgoing;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;

#[derive(Eq, PartialOrd, PartialEq, Debug)]
enum CoresGrafo {
    White,
    Black,
    Gray,
}

pub struct ArvorePesquisa {
    color: HashMap<NodeIndex, CoresGrafo>,
    d: HashMap<NodeIndex, i32>,
    pi: HashMap<NodeIndex, NodeIndex>,
    t: HashMap<NodeIndex, i32>,
    time: i32,
    f: HashMap<NodeIndex, i32>,
    top: Vec<NodeIndex>,
}

pub struct ArvoreEnvergaduraMinima {
    vec_mst: Vec<HashSet<NodeIndex>>,
    vec_a: Vec<EdgeIndex>,
    hash_key: HashMap<NodeIndex, i32>,
    hash_pi: HashMap<NodeIndex,NodeIndex>
}

fn find_set(u: NodeIndex, vec_u: &Vec<HashSet<NodeIndex>>) -> (usize, HashSet<NodeIndex>) {
    let mut set_u: HashSet<NodeIndex> = HashSet::new();
    let mut count_u = 0;
    for set_s in vec_u.iter() {
        if set_s.contains(&u) {
            set_u = set_s.clone();
            break;
        }
        count_u = count_u + 1;
    }
    (count_u, set_u)
}

fn union(set_u: &HashSet<NodeIndex>, set_v: &HashSet<NodeIndex>) -> HashSet<NodeIndex>{
    let union_set_s = set_u.union(&set_v);
    let set_s: HashSet<&NodeIndex> = union_set_s.collect();
    let mut set_ss: HashSet<NodeIndex> = HashSet::new();
    for s in set_s.iter() {
        set_ss.insert(**s);
    }
    set_ss
}

impl ArvoreEnvergaduraMinima {
    pub fn new(g: &UnGraph<&str, i32>) -> ArvoreEnvergaduraMinima {
        let mut arv = ArvoreEnvergaduraMinima {
            vec_mst: Vec::new(),
            vec_a: Vec::new(),
            hash_key: HashMap::new(),
            hash_pi: HashMap::new()
        };

        for v in g.node_indices() {
            // para o algoritmos Kruskal
            let mut s: HashSet<NodeIndex> = HashSet::new();
            s.insert(v);
            arv.vec_mst.push(s);
            // para o algoritmo de Prim
            arv.hash_key.insert(v, i32::MAX);
            arv.hash_pi.insert(v,v);
        }
        arv
    }

    pub fn mst_prim(&mut self, g: &UnGraph<&str, i32>,
                    s: NodeIndex) {
        self.hash_key.insert(s, 0);

        // a fila de espera com prioridade tem capacidade
        // o número de nós do grafo
        let mut q = PriorityQueue::<NodeIndex, Reverse<i32>>::with_capacity(g.node_count());
        for n in g.node_indices() {
            q.push(n, Reverse(self.hash_key[&n]));
        }

        while !q.is_empty() {
            let u = q.pop().unwrap().0;
            for v in g.neighbors_undirected(u) {
                let e = g.find_edge(u, v).unwrap();
                let w = g.edge_weight(e).unwrap();
                // println!();
                // println!("peso = {:?}", w);
                match q.get(&v) {
                    Some(n) => {
                        // println!("{:?}", n);
                        if w < &self.hash_key[&v] {
                            // println!("w -> {:?} {:?}", w, &self.hash_key[&v]);
                            self.hash_key.insert(v, *w);
                            self.hash_pi.insert(v, u);
                            q.change_priority(&v, Reverse(*w));
                        }
                    },
                    None => (),
                } ;
                //println!("vv = {:?}", vv);
            }
        }
    }

    pub fn imprimir_prim(&mut self, g: &UnGraph<&str, i32>) {
        for v in g.node_indices() {
            let p = self.hash_pi[&v];
            let e = match g.find_edge(v,p) {
                Some(ee) => {
                    println!("({:?}, {:?}) => {:?}",
                             g.node_weight(p).unwrap(),
                             g.node_weight(v).unwrap(),
                             g.edge_weight(ee).unwrap() );
                },
                None => ()
            };
        }
    }

    pub fn mst_kruskal(&mut self, g: &UnGraph<&str, i32>) {
        struct Ramos {
            e: EdgeIndex,
            w: i32
        }
        impl Ramos {
            pub fn new(e: EdgeIndex, w: i32) -> Self {
                Ramos {
                    e,
                    w
                }
            }
        }

        // agora temos o procedimento de ordenação dos nós de acordo com
        // os tempos finais
        let mut ramos_ordenados: Vec<Ramos> = Vec::new();
        for e in g.edge_indices() {
            let w = g.edge_weight(e);
            let r: Ramos = Ramos::new(e, *w.unwrap());
            ramos_ordenados.push(r);
        }
        ramos_ordenados.sort_by(|a, b| a.w.cmp(&b.w));

        for r in ramos_ordenados.iter() {
            // find-set(u)
            let (u,v) = g.edge_endpoints(r.e).unwrap();
            let (ind_u, mut set_u) = find_set(u, &self.vec_mst);
            let (ind_v, mut set_v) = find_set(v, &self.vec_mst);
            if set_v != set_u {
                let set_ss = union(&set_u, &set_v);
                // println!("set_u = {:?} set_v = {:?} set_ss = {:?}", set_u, set_v, set_ss);
                // remover set_u e set_v de vec_mst...
                if ind_u > ind_v {
                    self.vec_mst.swap_remove(ind_u);
                    self.vec_mst.swap_remove(ind_v);
                } else {
                    self.vec_mst.swap_remove(ind_v);
                    self.vec_mst.swap_remove(ind_u);
                }

                self.vec_mst.push(set_ss);
                self.vec_a.push(r.e);
            }
        }
    }

    pub fn imprimir_kruskal(&self, g: &UnGraph<&str, i32>) {
        for e in self.vec_a.iter() {
            let (u, v) = g.edge_endpoints(*e).unwrap();
            println!(" ({:?},{:?}) w = {:?}",
                     g.node_weight(u).unwrap(),
                     g.node_weight(v).unwrap(),
                     g.edge_weight(*e).unwrap());
        }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
struct NosTemposFinais {
    no: NodeIndex,
    tf: i32
}

impl NosTemposFinais {
    pub fn new(no: NodeIndex, tf: i32) -> Self {
        NosTemposFinais {
            no,
            tf
        }
    }
}

impl ArvorePesquisa {
    pub fn new() -> ArvorePesquisa {
        let mut arv = ArvorePesquisa {
            color: HashMap::new(),
            d: HashMap::new(),
            pi: HashMap::new(),
            t: HashMap::new(),
            time: 0,
            f: HashMap::new(),
            top: Vec::new(),
        };
        arv
    }

    fn find_set(&self, vec_cc: &mut Vec<HashSet<NodeIndex>>, v: NodeIndex) -> bool {
        true
    }

    pub fn connected_components(&mut self, g: &mut UnGraph<&str, i32>) {
        // let mut vec_cc: Vec<HashSet<NodeIndex>> = Vec::new();
        let mut ll_cc: LinkedList<HashSet<NodeIndex>> = LinkedList::new();
        for v in g.node_indices() {
            // Make-Set
            let mut set_s: HashSet<NodeIndex> = HashSet::new();
            set_s.insert(v);
            // vec_cc.push(set_s);
            ll_cc.push_front(set_s);
        }

        for e in g.edge_indices() {
            let (u, v) = g.edge_endpoints(e).unwrap();
            // find-set
            let mut set_u: &HashSet<NodeIndex> = &HashSet::new();
            let mut count_u = 0;
            for set_s in ll_cc.iter() {
                if set_s.contains(&u) {
                    set_u = set_s;
                    break;
                }
                count_u = count_u + 1;
            }
            let mut set_v: &HashSet<NodeIndex> = &HashSet::new();
            let mut count_v = 0;
            for set_s in ll_cc.iter() {
                if set_s.contains(&v) {
                    set_v = set_s;
                    break;
                }
                count_v = count_v + 1;
            }
            if set_u != set_v {
                let union_set_s = set_u.union(set_v);
                let set_s: HashSet<&NodeIndex> = union_set_s.collect();
                let mut set_ss: HashSet<NodeIndex> = HashSet::new();
                for s in set_s.iter() {
                    set_ss.insert(**s);
                }
                //println!("set_u {:?} set_v {:?} set_s {:?} ", set_u, set_v, set_s);
                ll_cc.push_front(set_ss);
            }
        }
        for set_s in ll_cc.iter() {
            println!("set_s {:?}", set_s);
        }
    }

    pub fn strongly_connected_components(&mut self,
                                         g: &mut DiGraph<&str, i32>,
                                         s: NodeIndex) {
        self.dfs(&g, s);

        // agora temos o procedimento de ordenação dos nós de acordo com
        // os tempos finais
        let mut nos_ordenados : Vec<NosTemposFinais> = Vec::new();
        for v in g.node_indices() {
            let tf = self.f[&v];
            nos_ordenados.push(NosTemposFinais::new(v, tf));
        }
        for ntf in nos_ordenados.iter() {
            println!("{:?} {:?}",
                     g.node_weight(ntf.no).unwrap(),
                     ntf.tf);
        }
        nos_ordenados.sort_by(|a, b| b.tf.cmp(&a.tf));
        println!("ordenado");
        for ntf in nos_ordenados.iter() {
            println!("{:?} {:?}",
                     g.node_weight(ntf.no).unwrap(),
                     ntf.tf);
        }
        g.reverse();

        for ntf in nos_ordenados.iter() {
            let u = ntf.no;
            self.color.insert(u, CoresGrafo::White);
            self.pi.insert(u, s);
            self.d.insert(u, 0);
            self.f.insert(u, 0);
        }

        self.time = 0;

        for ntf in nos_ordenados.iter() {
            let u = ntf.no;
            if self.color[&u] == CoresGrafo::White {
                self.dfs_visit(&g, u);
            }
        }

    }

    pub fn imprimir_scc(&self, g: &DiGraph<&str, i32>) {
        // agora temos o procedimento de ordenação dos nós de acordo com
        // os tempos finais
        let mut nos_ordenados : Vec<NosTemposFinais> = Vec::new();
        for v in g.node_indices() {
            let tf = self.f[&v];
            nos_ordenados.push(NosTemposFinais::new(v, tf));
        }
        for ntf in nos_ordenados.iter() {
            let v = self.pi[&ntf.no];
            println!("{:?} {:?} {:?}",
                     g.node_weight(ntf.no).unwrap(),
                     ntf.tf,
                     g.node_weight(v).unwrap());
        }
        nos_ordenados.sort_by(|a, b| b.tf.cmp(&a.tf));
        println!("SCC");
        for ntf in nos_ordenados.iter() {
            let v = self.pi[&ntf.no];
            println!("{:?} {:?} {:?}",
                     g.node_weight(ntf.no).unwrap(),
                     ntf.tf,
                     g.node_weight(v).unwrap());
        }

    }
    pub fn dfs_visit(&mut self,
                     g: &DiGraph<&str, i32>,
                     u: NodeIndex) {
        self.time = self.time + 1;
        self.d.insert(u, self.time);
        self.color.insert(u, CoresGrafo::Gray);

        for v in g.neighbors_directed(u, Outgoing) {
            if self.color[&v] == CoresGrafo::White {
                self.pi.insert(v, u);
                // AQUI
                self.dfs_visit(&g, v);
            }
        }
        self.color.insert(u, CoresGrafo::Black);
        self.time = self.time + 1;
        self.f.insert(u, self.time);
        self.top.push(u);
    }

    pub fn dfs(&mut self,
               g: &DiGraph<&str, i32>,
               s: NodeIndex) {
        for u in g.node_indices() {
            self.color.insert(u, CoresGrafo::White);
            self.pi.insert(u, s);
            self.d.insert(u, 0);
            self.f.insert(u, 0);
        }

        self.time = 0;

        for u in g.node_indices() {
            if self.color[&u] == CoresGrafo::White {
                self.dfs_visit(&g, u);
            }
        }
        self.top.reverse();
    }
    pub fn bfs(&mut self,
               g: &DiGraph<&str, i32>,
               s: NodeIndex) {
        for u in g.node_indices() {
            if u != s {
                self.color.insert(u, CoresGrafo::White);
                self.d.insert(u, i32::MAX);
                self.pi.insert(u, s);
            } else {
                self.color.insert(u, CoresGrafo::Gray);
                self.d.insert(u, 0);
                self.pi.insert(u, s);
            }
        }
        let mut q: VecDeque<NodeIndex> = VecDeque::new();
        q.push_back(s);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            println!("u = {:?}", u);
            for v in g.neighbors_directed(u, Outgoing) {
                println!("v = {:?}", v);
                if self.color[&v] == CoresGrafo::White {
                    self.color.insert(v, CoresGrafo::Gray);
                    self.d.insert(v, self.d[&u] + 1);
                    self.pi.insert(v, u);
                    q.push_back(v);
                }
            }
            self.color.insert(u, CoresGrafo::Black);
        }
    }
    pub fn imprimir(&self, g: &DiGraph<&str, i32>) {
        println!("distances");
        for (item, v) in self.d.iter() {
            println!("no = {:?}, d = {:?}", g.node_weight(*item).unwrap(), v);
        }
        println!("predecessor nodes");
        for (item, v) in self.pi.iter() {
            println!("no = {:?}, pi = {:?}",
                     g.node_weight(*item).unwrap(),
                     g.node_weight(*v).unwrap());
        }
        println!("final discovery times");
        for (item, v) in self.f.iter() {
            println!("no = {:?}, tf = {:?}",
                     g.node_weight(*item).unwrap(),
                     *v);
        }
        println!("colors");
        for (item, v) in self.color.iter() {
            println!("no = {:?}, color = {:?}",
                     g.node_weight(*item).unwrap(),
                     *v);
        }
        println!("topological sorted nodes");
        for v in self.top.iter() {
            println!("no = {:?}", g.node_weight(*v).unwrap());
        }
    }
}

pub struct ArvoreCaminhosMaisCurtos {
    d: HashMap<NodeIndex, i32>,
    pi: HashMap<NodeIndex, NodeIndex>,
}

impl ArvoreCaminhosMaisCurtos {
    pub fn new() -> ArvoreCaminhosMaisCurtos {
        let mut arv = ArvoreCaminhosMaisCurtos {
            d: HashMap::new(),
            pi: HashMap::new(),
        };
        arv
    }

    pub fn initialize_single_source(&mut self,
                                    g: &DiGraph<&str, i32>,
                                    s: NodeIndex) {
        for n in g.node_indices() {
            self.d.insert(n, i32::max_value());
            self.pi.insert(n, n);
        }
        self.d.insert(s, 0);
    }


    pub fn relax(&mut self, u: NodeIndex, v: NodeIndex, w: i32,
                 q: &mut PriorityQueue::<NodeIndex, Reverse<i32>>) {
        let du = *self.d.get(&u).unwrap();
        let dv = *self.d.get(&v).unwrap();
        let d = if du <= i32::max_value() - w {
            du + w
        } else {
            du
        };

        if dv > d {
            // atualiza a distância na árvore
            self.d.insert(v, d);
            // atualiza o valor da prioridade na pq
            q.change_priority(&v, Reverse(d));
            self.pi.insert(v, u);
        }
    }

    pub fn relax_bf(&mut self, u: NodeIndex, v: NodeIndex, w: i32) {
        let du = *self.d.get(&u).unwrap();
        let dv = *self.d.get(&v).unwrap();
        let d = if du <= i32::max_value() - w {
            du + w
        } else {
            du
        };

        if dv > d {
            // atualiza a distância na árvore
            self.d.insert(v, d);
            self.pi.insert(v, u);
        }
    }

    pub fn bellman_ford(&mut self, g: &DiGraph<&str, i32>, s: NodeIndex) -> bool {
        self.initialize_single_source(g, s);
        for i in 0..g.node_count() - 1 {
            // println!();
            for e in g.edge_indices() {
                let (u, v) = g.edge_endpoints(e).unwrap();
                // println!("u = {:?}, v = {:?}", u, v);
                let w = g.edge_weight(e).unwrap();
                self.relax_bf(u, v, *w);
            }
        }
        for e in g.edge_indices() {
            let (u, v) = g.edge_endpoints(e).unwrap();
            let du = *self.d.get(&u).unwrap();
            let dv = *self.d.get(&v).unwrap();
            let w = g.edge_weight(e).unwrap();

            let d = if du <= i32::max_value() - w {
                du + w
            } else {
                du
            };
            if dv > d {
                return false;
            }
        }
        true
    }
    pub fn dijkstra(&mut self, g: &DiGraph<&str, i32>, s: NodeIndex) {
        self.initialize_single_source(g, s);
        // let set_s = HashSet::<NodeIndex>::new();

        // a fila de espera com prioridade tem capacidade
        // o número de nós do grafo
        let mut q = PriorityQueue::<NodeIndex, Reverse<i32>>::with_capacity(g.node_count());
        for n in g.node_indices() {
            q.push(n, Reverse(self.d[&n]));
        }

        while !q.is_empty() {
            let u = q.pop().unwrap().0;
            for v in g.neighbors_directed(u, Outgoing) {
                let e = g.find_edge(u, v).unwrap();
                let w = g.edge_weight(e).unwrap();
                self.relax(u, v, *w, &mut q);
            }
        }
    }

    pub fn imprimir(&self, g: &DiGraph<&str, i32>) {
        for (item, v) in self.d.iter() {
            println!("no = {:?}, d = {:?}", g.node_weight(*item).unwrap(), v);
        }
        println!();
        for (item, v) in self.pi.iter() {
            println!("no = {:?}, pi = {:?}",
                     g.node_weight(*item).unwrap(),
                     g.node_weight(*v).unwrap());
        }
    }
}
