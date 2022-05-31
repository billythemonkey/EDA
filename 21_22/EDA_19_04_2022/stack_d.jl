"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 19 2022
"""

mutable struct Stack

    top::Int64
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

    expression = "((2+2) * (1+3))"

    s = Stack(1, Array{Char}(undef, sizeof(expression)))
    
    println(s)

    for c in expression
        if c == '('
            push!(s, c)
        elseif c == ')'
            pop!(s)
        end
    end

    println(s)

end

main()