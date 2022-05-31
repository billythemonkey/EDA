"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 19 2022
"""

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
    return s.mem[s.top + 1]
end

function push!(s::Stack, x::Int64)
    s.mem[s.top] = x 
    s.top = s.top + 1
end


function main()
    N = 1000
    s = Stack(1, N, zeros(N))
    
    A = rand(Int64, 100)
    
    println(s)

    for v in A
        push!(s, v)
    end
    
    println(s)

    for n = 1:50 
        pop!(s)
    end

    println(s)
end

main()