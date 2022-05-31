"""
Author: Andrei Razvan Oproiu
Date: Wed May 04 2022
"""

module MyBinaryTree

    mutable struct BinaryTree
        head::Int
        vec_p::Array{Int}
        vec_l::Array{Int}
        vec_r::Array{Int}
        vec_keys::Array{Int}

        function BinaryTree(size)
            new(1, zeros(size), zeros(size), zeros(size), zeros(size))
        end
    end

    function inOrderTreeWalk(T, x)
        if x != 1
            inOrderTreeWalk(T.vec_l[x], x)
            println(T.vec_keys[x])
            inOrderTreeWalk(T.vec_r[x], x)
        end
    end

    function printTree(T,N)
    
        for a = 1:N
            print(" $a")
        end
        println("")
        for a = 1:N
            print(" $(T.vec_p[a])")
        end
        println("")
        for a = 1:N
            print(" $(T.vec_l[a])")
        end
        println("")
        for a = 1:N
            print(" $(T.vec_r[a])")
        end
        println("")
        for a = 1:N
            print(" $(T.vec_keys[a])")
        end
    end



    function treeMinimum(t)
        while t.vec_l[t.head] != 1
            t = t.vec_l[t.head]
        end
        return t.head
    end

    function treeMaximum(t)
        while t.vec_r[t.head] != 1
            t.head = t.vec_r[t.head]
        end
        return t.head
    end

    function treeInsert!(T, z, key)
        T.head = z
        y = 1
        x = T.vec_p[1]
        println("x = ", x)
        while x != 1
            y = x
            if T.vec_keys[T.head] < T.vec_keys[x]
                x = T.vec_l[x]
                T.vec_keys[x] = key
            else
                x = T.vec_r[x]
                T.vec_keys[x] = key
            end
        end
        T.vec_p[T.head] = y
        println("y = ", y)
        if y == 1
            z = T.vec_p[1]
            println("z = ", z)
            T.vec_keys[y] = key
        elseif T.vec_keys[z] < T.vec_keys[y]
            T.vec_l[y] = z
            T.vec_keys[y] = key
        else
            T.vec_r[y] = z
            T.vec_keys[y] = key
        end
    end

    

end