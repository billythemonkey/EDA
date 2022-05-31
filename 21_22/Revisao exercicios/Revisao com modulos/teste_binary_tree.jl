"""
Author: Andrei Razvan Oproiu
Date: Tue May 17 2022
"""


include("BinaryTree.jl")
include("MyModules/Stack.jl")

using .MyBinaryTree, .Stack_Mod

function main()

    N = 10

    memoria_livre = Stack_Mod.Stack(N)


    for i = 1:N
        Stack_Mod.push!(memoria_livre, i+1)
    end

    tree = MyBinaryTree.BinaryTree(N)

    tree.vec_p[1] = 1
    tree.vec_l[1] = 1
    tree.vec_r[1] = 1
    tree.vec_keys[1] = 1

    MyBinaryTree.printTree(tree, N)

    println("")
    println(memoria_livre)

    MyBinaryTree.treeInsert!(tree, 10, 100)
    MyBinaryTree.printTree(tree, N)
    MyBinaryTree.treeInsert!(tree, 9, 50)
    MyBinaryTree.printTree(tree, N)
    MyBinaryTree.treeInsert!(tree, 4, 30)

    MyBinaryTree.printTree(tree, N)

    
end

main()