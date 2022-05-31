"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

"""
Stack
"""

mutable struct Stack
    top::Int
    memory::Array{Int}
end

function isEmpty(s::Stack)
    if s.top == 1
        return true
    end
    return false
end

function pop!(s::Stack)
    if isEmpty(s)
        println("Stack is empty!!")
    else
        s.top -= 1
    end

    return s.memory[s.top]
end

function push!(s::Stack, item::Int)
    s.memory[s.top] = item
    s.top += 1
end

function isFull(s::Stack)
    if s.top == length(s.memory) + 1
        return true
    end
    return false
end

function main()
    N = 10

    stack = Stack(1, zeros(N))

    randArray = rand(Int, N)

    for int in randArray
        push!(stack, int)
    end

    println(stack.memory)

end


main()


