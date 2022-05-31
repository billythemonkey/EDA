"""
Author: Andrei Razvan Oproiu
Date: Tue May 24 2022
"""

module ListaLigadaModule


mutable struct ListaLigada
    head::Int64
    vec_prev::Int64
    vec_next::Int64
    vec_key::Int64

    # function ListaLigada(size)
    #     new(1, zeros(size), zeros(size), zeros(size))
    # end
end

function listSearch(l, k)
    x = l.head
    while x != 1 && l.vec_key[x] != k
        x = l.vec_next[x]
    end
    return x
end

function listInsert(l, x)
    l.vec_next[x] = l.head
    if l.head != 1
        l.vec_prev[l.head] = x
    end

    l.head = x
    l.vec_prev[x] = 1
end
    
end