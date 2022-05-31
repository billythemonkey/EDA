"""
Author: Andrei Razvan Oproiu
Date: Wed May 04 2022
"""

module MyLinkedList

include("Stack.jl")
using .Stack_Mod

mutable struct LinkedList
    head::Int
    vec_prev::Array{Int}
    vec_keys::Array{Int}
    vec_next::Array{Int}
    
    function LinkedList(size)
        new(1, zeros(size), zeros(size), zeros(size))
    end
end

function defineNil(l::LinkedList)
    NIL = 1

    l.vec_next[NIL] = 1
    l.vec_prev[NIL] = 1
    l.vec_keys[NIL] = 1
end

function search(l::LinkedList, key)

    NIL = 1

    position = l.vec_next[NIL]
    while position != NIL && l.vec_keys[position] != key
        position = l.vec_next[position]
    end
    return position


    # p = l.head
    # while l.vec_next[p] != 1 && l.vec_keys[p] != key
    #     p = l.vec_next[p]
    # end
    # return p
end

function insert!(l::LinkedList, position::Int, key)

    NIL = 1
    # 1
    l.vec_next[position] = l.vec_next[NIL]

    # 2 
    l.vec_prev[ l.vec_next[NIL] ] = position


    l.vec_next[NIL] = position
    l.vec_prev[position] = NIL
    l.vec_keys[position] = key

    # l.vec_next[position] = l.head
    # if l.head != 1
    #     l.vec_prev[l.head] = position
    # end
    # l.vec_keys[position] = key
    # l.head = position
    # l.vec_prev[position] = 1
end

# l.vec_next[NIL]



function delete!(l::LinkedList, key, s)
    position = search(l, key)

    println(position)
    l.vec_next[l.vec_prev[position]] = l.vec_next[position]
    l.vec_prev[l.vec_next[position]] = l.vec_prev[position]

    Stack_Mod.push!(s, position)
end

function printLinkedList(l::LinkedList,N)
    
    for a = 1:N
        print(" $a")
    end
    println("")
    for a = 1:N
        print(" $(l.vec_keys[a])")
    end
    println("")
    for a = 1:N
        print(" $(l.vec_prev[a])")
    end
    println("")
    for a = 1:N
        print(" $(l.vec_next[a])")
    end
end

function nextPosition(s)
    position = Stack_Mod.pop!(s)
    return position
end

end