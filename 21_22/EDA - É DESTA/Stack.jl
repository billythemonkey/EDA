"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

module MyStack

mutable struct Stack
    top::Int
    memory::Array{Int}

    function Stack(size)
        new(1, zeros(size))
    end
end

function isEmpty(s)
    if s.top == 1
        return true
    end
    return false
end

function isFull(s)
    if s.top == (length(s.memory) + 1)
        return true
    end
    return false
end

function pop!(s)
    if isEmpty(s)
        println("Pop error: Stack Underflow.")
    else
        s.top -= 1
    end
    return s.memory[s.top]
end

function push!(s, value)
    if isFull(s)
        println("Push error: Stack Overflow.")
    else
        s.memory[s.top] = value
        s.top += 1
    end
end

end

