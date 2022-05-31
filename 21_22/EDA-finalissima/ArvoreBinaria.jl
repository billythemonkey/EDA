"""
Author: Andrei Razvan Oproiu
Date: Tue May 31 2022
"""

module MyBinaryTree
    mutable struct BinaryTree
        root::Int

        vec_p::Array{Int}
        vec_l::Array{Int}
        vec_r::Array{Int}
        vec_key::Array

        function BinaryTree(N)
            new(1, ones(N), ones(N), ones(N), ones(N))
        end
    end

    function inOrderTreeWalk(t, x)
        NIL = 1
        if x != NIL
            inOrderTreeWalk(t, t.vec_l[x])
            println(t.vec_key[x])
            inOrderTreeWalk(t, t.vec_r[x])
        end
    end

    function fillNode!(t, z, key)
        t.vec_key[z] = key
    end

    function treeInsert!(t, z)
        NIL = 1
        y = NIL
        x = t.root

        while x != NIL
            y = x
            if t.vec_key[z] < t.vec_key[x]
                x = t.vec_l[x]
            else
                x = t.vec_r[x]
            end
        end

        t.vec_p[z] = y
        if y == NIL
            t.root = z
        elseif t.vec_key[z] < t.vec_key[y]
            t.vec_l[y] = z
        else
            t.vec_r[y] = z
        end

    end







    
end