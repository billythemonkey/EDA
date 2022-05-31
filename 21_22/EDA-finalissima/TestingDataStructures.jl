"""
Author: Andrei Razvan Oproiu
Date: Tue May 31 2022
"""


include("ArvoreBinaria.jl")
include("Stack.jl")
using .MyStack
using .MyBinaryTree


function main()
    NIL = 1
    size = 10

    stack = MyStack.Stack(size)


    for i in 1:size
        MyStack.push!(stack, i)
    end


    tree = MyBinaryTree.BinaryTree(size)




    # tRoot = MyBinaryTree.root(NIL, NIL, NIL, 70)

    # tree.vec_key[1] = tRoot.key
    # tree.vec_p[1] = tRoot.p
    # tree.vec_l[1] = tRoot.l
    # tree.vec_r[1] = tRoot.r


    nextItem = MyStack.pop!(stack)

    MyBinaryTree.fillNode!(tree, nextItem, 70)

    println(nextItem)

    MyBinaryTree.treeInsert!(tree, nextItem)


    nextItem = MyStack.pop!(stack)

    MyBinaryTree.fillNode!(tree, nextItem, 53)

    println(nextItem)

    MyBinaryTree.treeInsert!(tree, nextItem)


    nextItem = MyStack.pop!(stack)

    MyBinaryTree.fillNode!(tree, nextItem, 60)

    println(nextItem)

    MyBinaryTree.treeInsert!(tree, nextItem)

    nextItem = MyStack.pop!(stack)

    MyBinaryTree.fillNode!(tree, nextItem, 90)

    println(nextItem)

    MyBinaryTree.treeInsert!(tree, nextItem)

    println(tree)

    MyBinaryTree.inOrderTreeWalk(tree, 10)

    
end

main()