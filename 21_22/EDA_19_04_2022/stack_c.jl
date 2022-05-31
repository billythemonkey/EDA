"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 19 2022
"""

mutable struct Stack

    top::Int64
    sz:: Int64
    mem

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

function push!(s::Stack, x::Char)
    s.mem[s.top] = x 
    s.top = s.top + 1
end


function main()
    N = 10
    s = Stack(1, N, Array{Char}(undef, N))
    
    
    println(s)
    push!(s, 'f')
    println(s)
    push!(s, 'r')
    println(s)
    c = pop!(s)
    println(s, c)
end

main()