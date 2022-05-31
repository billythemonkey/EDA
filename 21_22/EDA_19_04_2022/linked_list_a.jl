"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 19 2022
"""
mutable struct LinkedList
    head::Int64
    a_key::Array
    a_next::Array
    a_prev::Array
end

mutable struct Stack
    top::Int64
    sz:: Int64
    mem::Array
end

function isEmpty(s::Stack)
    if s.top == 1
        return true
    end
    return false
end

function pop!(s::Stack)
    if isEmpty(s)
        println("Underflow")
    else
        s.top = s.top - 1
    end
    return s.mem[s.top]
end

function push!(s::Stack, x::Int64)
    s.mem[s.top] = x 
    s.top = s.top + 1
end

function listSearch(L, key)
    p = L.head
    while L.a_next[p] != 0 && L.a_key[p] != key
        p = L.a_next[p]
    end
    return p
end

function listInsert!(L, head, key)
    L.a_next[head] = L.head
    if L.head != 1
        L.a_prev[L.head] = head
    end
    L.a_key[head] = key
    L.head = head
    L.a_prev[head] = 1
end

function listDelete!(L, position, mem)

    L.a_next[L.a_prev[position]] = L.a_next[position]
    L.a_prev[L.a_next[position]] = L.a_prev[position]
    push!(mem, position)
end

function printLinkedList(L,N)
    
    for a = 1:N
        print(" $a")
    end
    println("")
    for a = 1:N
        print(" $(L.a_key[a])")
    end
    println("")
    for a = 1:N
        print(" $(L.a_prev[a])")
    end
    println("")
    for a = 1:N
        print(" $(L.a_next[a])")
    end
end

function nextPosition(s::Stack)
    position = pop!(s)
    return position
end

function main()
    N = 9
    
    memoria_livre = Stack(1, N, zeros(Int64, N))
    index_array = zeros(Int64, N)
    for i = 1:N
        push!(memoria_livre, i) 
    end
    for i = 1:N
        index_array[i] = i
    end

    a = LinkedList(1, zeros(Int64,N), zeros(Int64,N), zeros(Int64,N))

    printLinkedList(a, N)

    position = nextPosition(memoria_livre)
    println(position)
    listInsert!(a, position, 88)

    println("")
    printLinkedList(a, N)

    position = nextPosition(memoria_livre)
    println(position)
    listInsert!(a, position, 77)

    println("")
    printLinkedList(a, N)

    position = nextPosition(memoria_livre)
    println(position)
    listInsert!(a, position, 66)

    println("")
    printLinkedList(a, N)

    position = nextPosition(memoria_livre)
    println(position)
    listInsert!(a, position, 33)

    println("")
    printLinkedList(a, N)

    listDelete!(a, 8, memoria_livre)
    println("")
    printLinkedList(a, N)

    
    
end

main()
