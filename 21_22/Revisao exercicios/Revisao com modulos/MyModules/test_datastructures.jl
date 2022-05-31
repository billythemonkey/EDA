"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

include("LinkedList.jl")
include("Stack.jl")

using .Stack_Mod, .MyLinkedList

function main()
    N = 10
    memoria_livre = Stack_Mod.Stack(N - 1)
    for i = 1:N - 1
        Stack_Mod.push!(memoria_livre, i+1)
    end

    linkedList = MyLinkedList.LinkedList(N)
    MyLinkedList.defineNil(linkedList)

    println(memoria_livre)

    pos10 = MyLinkedList.nextPosition(memoria_livre)
    print("Next Position : -> ", pos10)
    println("")


    MyLinkedList.insert!(linkedList, pos10, 90)

    MyLinkedList.printLinkedList(linkedList, N)

    println("")

    pos9 = MyLinkedList.nextPosition(memoria_livre)
    print("Next Position : -> ", pos9)
    println("")


    MyLinkedList.insert!(linkedList, pos9, 780)

    MyLinkedList.printLinkedList(linkedList, N)

    println("")

    pos8 = MyLinkedList.nextPosition(memoria_livre)
    print("Next Position : -> ", pos8)
    println("")


    MyLinkedList.insert!(linkedList, pos8, 32)

    MyLinkedList.printLinkedList(linkedList, N)

    println("")
    println("Delete")
    MyLinkedList.delete!(linkedList, 780, memoria_livre)

    # searchItem = MyLinkedList.search(linkedList, 780)

    # println(searchItem)

    MyLinkedList.printLinkedList(linkedList, N)


    println("")

    println(memoria_livre)


    pos9 = MyLinkedList.nextPosition(memoria_livre)
    print("Next Position : -> ", pos9)
    println("")


    MyLinkedList.insert!(linkedList, pos9, 45)

    MyLinkedList.printLinkedList(linkedList, N)

    println("")
    

    

    


end

main()