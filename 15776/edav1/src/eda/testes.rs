
use super::grafos::ArvoreCaminhosMaisCurtos;
use super::grafos::ArvorePesquisa;
use super::ordenacao::*;
use super::estruturas::*;

use super::grafos::ArvoreEnvergaduraMinima;
use petgraph::{Graph, Directed};
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::dot::Dot;
use petgraph::algo::dijkstra;

use rand::prelude::*;

/*
testes de estruturas de dados simples
 */

pub fn teste_memoria_listas_ligadas() {
    println!("teste de estruturas de dados simples");

    let mut memoria = MemoriaListasLigadas::new(10);
    memoria.imprimir();
    // reservar memoria
    let x = memoria.allocate_object();
    println!("reserva 1, x = {}", x);
    memoria.list_insert(x, 99);
    memoria.imprimir();

    let x = memoria.allocate_object();
    memoria.list_insert(x, 77);
    println!("reserva 2, x = {}", x);
    memoria.imprimir();

    let x = memoria.allocate_object();
    println!("reserva 3, x = {}", x);
    memoria.list_insert(x, 33);

    memoria.imprimir();

    let x = 7;
    memoria.free_object(x);
    println!("libertação 1, x = {}", x);
    memoria.imprimir();

    let x = 7;
    memoria.free_object(x);
    println!("libertação 2, x = {}", x);
    memoria.imprimir();

    let x = 9;
    memoria.free_object(x);
    println!("libertação 3, x = {}", x);
    memoria.imprimir();

    let x = memoria.allocate_object();
    println!("reserva 4, x = {}", x);
    memoria.list_insert(x, 44);
    memoria.imprimir();

    println!("pesquisa na lista");
    let k = 77;
    let x = memoria.list_search(k);
    println!("key {}, pos = {}", k, x);
}
/*
testes de ordenação
 */

pub fn criar_vetor_inteiro() -> Vec<i32> {
    let mut a: Vec<i32> = Vec::new();
    a.push(i32::MIN);

    a.push(2);
    a.push(8);
    a.push(7);
    a.push(1);
    a.push(3);
    a.push(5);
    a.push(6);
    a.push(4);

    a
}

pub fn criar_vetor_a() -> Vec<f64> {
    let mut a: Vec<f64> = Vec::new();
    a.push(f64::MIN);

    a.push(2.0);
    a.push(8.0);
    a.push(7.0);
    a.push(1.0);
    a.push(3.0);
    a.push(5.0);
    a.push(6.0);
    a.push(4.0);

    a
}

fn criar_vetor_aleatorio(n: usize) -> Vec<f64>{
    let mut a: Vec<f64> = Vec::new();
    let mut rng = thread_rng();
    for k in 0..n {
        let x: f64 = rng.gen_range(0.0..10.0);
        a.push(x);
    }
    a
}

fn criar_vetor_aleatorio_inteiro(n: usize) -> Vec<i32>{
    let mut a: Vec<i32> = Vec::new();
    let mut rng = thread_rng();
    for k in 0..n {
        let x: i32 = rng.gen_range(0..10);
        a.push(x);
    }
    a
}

pub fn teste_insertion_sort() {
    let mut a = criar_vetor_a();
    for v in &a {
        println!("A = {:?}", v);
    }
    insertion_sort(&mut a);
    for v in &a {
        println!("A = {:?}", v);
    }
}
pub fn teste_bubble_sort() {
    let mut a = criar_vetor_a();
    for v in &a {
        println!("A = {:?}", v);
    }
    bubble_sort(&mut a);
    for v in &a {
        println!("A = {:?}", v);
    }
}


pub fn teste_merge_sort() {
    let mut a = criar_vetor_aleatorio(10);
    println!("Merge-Sort");
    print_a_f64(&a);
    let p = 0;
    let r = a.len().clone()-1 ;
    merge_sort(&mut a, 1, r);
    println!();
    print_a_f64(&a);
}

pub fn teste_heap_sort() {
    let mut a = criar_vetor_aleatorio_inteiro(10);
    println!("Heap-Sort");
    print_a_i32(&a);
    let p = 0;
    let r = a.len().clone()-1 ;
    heap_sort(&mut a);
    println!();
    print_a_i32(&a);
}

pub fn teste_quick_sort() {
    let mut a = criar_vetor_inteiro();
    println!("Quick-Sort");
    print_a_i32(&a);
    let p = 1;
    let r = a.len()-1 ;
    quick_sort(&mut a, p, r);
    println!();
    print_a_i32(&a);
}

pub fn teste_bellman_ford() {
    // criação dum grafo orientado que obedece ao exemplo apresentado na aula
    let mut g_bf: Graph<&str, i32, Directed, u32> = DiGraph::<&str, i32>::new();

    // criação dos nós
    let s = g_bf.add_node("s");
    let t = g_bf.add_node("t");
    let x = g_bf.add_node("x");
    let y = g_bf.add_node("y");
    let z = g_bf.add_node("z");

    // criação dos ramos
    g_bf.add_edge(s, t, 6);
    g_bf.add_edge(s, y, 7);

    g_bf.add_edge(t, x, 5);
    g_bf.add_edge(t, y, 8);
    g_bf.add_edge(t, z, 4);

    g_bf.add_edge(x, t, 2);

    g_bf.add_edge(y, x, 3);
    g_bf.add_edge(y, z, 9);

    g_bf.add_edge(z, x, 7);
    g_bf.add_edge(z, s, 2);

    println!("algoritmo de Bellman Ford");
    println!("{:?}", Dot::with_config(&g_bf, &[]));

    let mut arv_bf = ArvoreCaminhosMaisCurtos::new();

    let correto = arv_bf.bellman_ford(&g_bf, s);
    if correto {
        println!("correto");
    } else {
        print!("não correto");
    }
    arv_bf.imprimir(&g_bf);
}

pub fn teste_dijkstra() {

    // criação dum grafo orientado que obedece ao exemplo apresentado na aula
    // para o algoritmo de Dijkstra
    let mut g_dij: Graph<&str, i32, Directed, u32> = DiGraph::<&str, i32>::new();

    // criação dos nós
    let s = g_dij.add_node("s");
    let t = g_dij.add_node("t");
    let x = g_dij.add_node("x");
    let y = g_dij.add_node("y");
    let z = g_dij.add_node("z");

    // criação dos ramos
    g_dij.add_edge(s, t, 10);
    g_dij.add_edge(s, y, 5);

    g_dij.add_edge(t, x, 1);
    g_dij.add_edge(t, y, 2);

    g_dij.add_edge(x, t, 4);

    g_dij.add_edge(y, x, 9);
    g_dij.add_edge(y, z, 2);
    g_dij.add_edge(y, t, 3);

    g_dij.add_edge(z, x, 6);
    g_dij.add_edge(z, s, 7);

    println!("algoritmo de Dijkstra");
    println!("{:?}", Dot::with_config(&g_dij, &[]));

    let mut arv_dij = ArvoreCaminhosMaisCurtos::new();

    arv_dij.dijkstra(&g_dij, s);
    arv_dij.imprimir(&g_dij);


    // aplicação do algoritmo de Dijkstra do crate petgraph
    let res = dijkstra(
        &g_dij,
        s,
        None,
        |e| {
            let w = e.weight();
            *w
        });

    // resultado da aplicação do algoritmo de Dijkstra do crate
    println!("{:?}", res);
}

pub fn teste_bfs() {
    // criação dum grafo orientado que obedece ao exemplo apresentado na aula
    let mut g_bf: Graph<&str, i32, Directed, u32> = DiGraph::<&str, i32>::new();

    // criação dos nós
    let s = g_bf.add_node("s");
    let t = g_bf.add_node("t");
    let x = g_bf.add_node("x");
    let y = g_bf.add_node("y");
    let z = g_bf.add_node("z");

    // criação dos ramos
    g_bf.add_edge(s, t, 6);
    g_bf.add_edge(s, y, 7);

    g_bf.add_edge(t, x, 5);
    g_bf.add_edge(t, y, 8);
    g_bf.add_edge(t, z, 4);

    g_bf.add_edge(x, t, 2);

    g_bf.add_edge(y, x, 3);
    g_bf.add_edge(y, z, 9);

    g_bf.add_edge(z, x, 7);
    g_bf.add_edge(z, s, 2);

    let mut arv_bfs = ArvorePesquisa::new();
    arv_bfs.bfs(&g_bf, s);
    println!("algoritmo Breadth First Search");
    arv_bfs.imprimir(&g_bf);

}

pub fn teste_dfs() {
    let mut arv_dfs = ArvorePesquisa::new();
    println!("algoritmo Depth First Search, e resultados da ordenação topológica");

    let mut g_top: Graph<&str, i32, Directed, u32> = DiGraph::<&str, i32>::new();

    // criação dos nós
    let undershorts = g_top.add_node("undershorts");
    let pants = g_top.add_node("pants");
    let belt = g_top.add_node("belt");
    let shirt = g_top.add_node("shirt");
    let tie = g_top.add_node("tie");
    let jacket = g_top.add_node("jacket");
    let socks = g_top.add_node("socks");
    let shoes = g_top.add_node("shoes");
    let watch = g_top.add_node("watch");

    // criação dos ramos
    g_top.add_edge(undershorts, pants, 1);
    g_top.add_edge(pants, belt, 1);
    g_top.add_edge(shirt, belt, 1);
    g_top.add_edge(undershorts, shoes, 1);
    g_top.add_edge(shirt, tie, 1);
    g_top.add_edge(tie, jacket, 1);
    g_top.add_edge(belt, jacket, 1);
    g_top.add_edge(socks, shoes, 1);
    g_top.add_edge(pants, shoes, 1);

    println!("{:?}", Dot::with_config(&g_top, &[]));

    arv_dfs.dfs(&g_top, undershorts);
    arv_dfs.imprimir(&g_top);
}

pub fn teste_scc() {
    let mut arv_scc = ArvorePesquisa::new();
    println!("algoritmo Strongly Connected Components");

    let mut g_scc: Graph<&str, i32, Directed, u32> = DiGraph::<&str, i32>::new();

    // criação dos nós
    let a = g_scc.add_node("a");
    let b = g_scc.add_node("b");
    let c = g_scc.add_node("c");
    let d = g_scc.add_node("d");
    let e = g_scc.add_node("e");
    let f = g_scc.add_node("f");
    let g = g_scc.add_node("g");
    let h = g_scc.add_node("h");

    // criação dos ramos
    g_scc.add_edge(a, b, 1);

    g_scc.add_edge(b, f, 1);
    g_scc.add_edge(b, e, 1);
    g_scc.add_edge(b, c, 1);

    g_scc.add_edge(c, d, 1);
    g_scc.add_edge(c, g, 1);

    g_scc.add_edge(d, h, 1);
    g_scc.add_edge(d, c, 1);

    g_scc.add_edge(e, a, 1);
    g_scc.add_edge(e, f, 1);

    g_scc.add_edge(f, g, 1);

    g_scc.add_edge(g, h, 1);
    g_scc.add_edge(g, f, 1);

    g_scc.add_edge(h, h, 1);

    // println!("{:?}", Dot::with_config(&g_scc, &[]));
    arv_scc.strongly_connected_components(&mut g_scc, a);
    // println!("{:?}", Dot::with_config(&g_scc, &[]));

    arv_scc.imprimir_scc(&g_scc);
}

pub fn teste_prim() {
    println!("algoritmo MST Prim");

    let mut g_mst: UnGraph<&str, i32> = UnGraph::new_undirected();

    // criação dos nós
    let a = g_mst.add_node("a");
    let b = g_mst.add_node("b");
    let c = g_mst.add_node("c");
    let d = g_mst.add_node("d");
    let e = g_mst.add_node("e");
    let f = g_mst.add_node("f");
    let g = g_mst.add_node("g");
    let h = g_mst.add_node("h");
    let i = g_mst.add_node("i");

    // criação dos ramos
    g_mst.add_edge(a, b, 4);
    g_mst.add_edge(a, h, 8);

    g_mst.add_edge(b, h, 11);
    g_mst.add_edge(b, c, 8);

    g_mst.add_edge(c, i, 2);
    g_mst.add_edge(h, i, 7);

    g_mst.add_edge(h, g, 1);
    g_mst.add_edge(i, g, 6);
    g_mst.add_edge(c, d, 7);
    g_mst.add_edge(c, f, 4);
    g_mst.add_edge(g, f, 2);

    g_mst.add_edge(f, e, 10);
    g_mst.add_edge(f, d, 14);
    g_mst.add_edge(d, e, 9);

    // println!("{:?}", Dot::with_config(&g_cc, &[]));
    let mut arv_mst = ArvoreEnvergaduraMinima::new(&g_mst);
    arv_mst.mst_prim(&mut g_mst, a);
    arv_mst.imprimir_prim(&g_mst);
    // println!("{:?}", Dot::with_config(&g_cc, &[]));
}

pub fn teste_kruskal() {
    println!("algoritmo MST Kruskal");

    let mut g_mst: UnGraph<&str, i32> = UnGraph::new_undirected();

    // criação dos nós
    let a = g_mst.add_node("a");
    let b = g_mst.add_node("b");
    let c = g_mst.add_node("c");
    let d = g_mst.add_node("d");
    let e = g_mst.add_node("e");
    let f = g_mst.add_node("f");
    let g = g_mst.add_node("g");
    let h = g_mst.add_node("h");
    let i = g_mst.add_node("i");

    // criação dos ramos
    g_mst.add_edge(a, b, 4);
    g_mst.add_edge(a, h, 8);

    g_mst.add_edge(b, h, 11);
    g_mst.add_edge(b, c, 8);

    g_mst.add_edge(c, i, 2);
    g_mst.add_edge(h, i, 7);

    g_mst.add_edge(h, g, 1);
    g_mst.add_edge(i, g, 6);
    g_mst.add_edge(c, d, 7);
    g_mst.add_edge(c, f, 4);
    g_mst.add_edge(g, f, 2);

    g_mst.add_edge(f, e, 10);
    g_mst.add_edge(f, d, 14);
    g_mst.add_edge(d, e, 9);

    // println!("{:?}", Dot::with_config(&g_cc, &[]));
    let mut arv_mst = ArvoreEnvergaduraMinima::new(&g_mst);
    arv_mst.mst_kruskal(&mut g_mst);
    arv_mst.imprimir_kruskal(&g_mst);
    // println!("{:?}", Dot::with_config(&g_cc, &[]));

}

pub fn teste_cc() {

    let mut arv_cc = ArvorePesquisa::new();
    println!("algoritmo MST Kruskal");

    let mut g_cc: UnGraph<&str, i32> = UnGraph::new_undirected();

    // criação dos nós
    let a = g_cc.add_node("a");
    let b = g_cc.add_node("b");
    let c = g_cc.add_node("c");
    let d = g_cc.add_node("d");
    let e = g_cc.add_node("e");
    let f = g_cc.add_node("f");
    let g = g_cc.add_node("g");
    let h = g_cc.add_node("h");
    let i = g_cc.add_node("i");

    // criação dos ramos
    g_cc.add_edge(a, b, 4);
    g_cc.add_edge(a, c, 1);

    g_cc.add_edge(c, b, 1);
    g_cc.add_edge(b, d, 1);

    g_cc.add_edge(e, f, 1);
    g_cc.add_edge(e, g, 1);

    g_cc.add_edge(h, i, 1);

    // println!("{:?}", Dot::with_config(&g_cc, &[]));
    arv_cc.connected_components(&mut g_cc);
    // println!("{:?}", Dot::with_config(&g_cc, &[]));

}

