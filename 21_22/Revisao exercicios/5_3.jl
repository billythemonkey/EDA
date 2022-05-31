"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

include("Revisao com modulos/Stack.jl")


mutable struct ListaLigada
    head::Int
    a_key::Array{Int}
    a_prev::Array{Int}
    a_next::Array{Int}
end

function main()
    N = 10
    stack = MyStack.Stack(1, zeros(N))

    println(stack.memory)
end

main()



