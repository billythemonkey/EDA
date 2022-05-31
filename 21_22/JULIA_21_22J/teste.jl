"""
Author: Andrei Razvan Oproiu
Date: Tue May 24 2022
"""

include("Lista_ligada.jl")

using .ListaLigadaModule

function main()
    
    l = ListaLigadaModule.ListaLigada(1, zeros(10), zeros(10), zeros(10))

    println(l)
end

main()