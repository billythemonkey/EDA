"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 19 2022
"""

mutable struct Queue
    head::Int64
    tail::Int64
    mem::Array
end

function isEmpty(q::Queue)
    if q.head == q.tail
        println("Underflow error")
        return true
    end
    return false
end

function isFull(q::Queue)
    if q.head == q.tail + 1
        println("Overflow error")
        return true
    end
    return false
end

function enqueue(q::Queue, v::Int64)
    if !isFull(q)
        q.mem[q.tail] = v
        if q.tail == size(q.mem)
            q.tail = 1
        else
            q.tail = q.tail + 1
        end
    end
end

function dequeue(q::Queue)
    if !isEmpty(q)
        x = q.mem[q.head]
        if q.head == size(q.mem)
            q.head = 1
        else
            q.head = q.head + 1
        end
        return x
    end
end


function main()
    q = Queue(1, 1, zeros(10))
    println(q)

    enqueue(q, 10)
    println(q)
    enqueue(q, 200)

    enqueue(q, 36)
    println(q)

    v = dequeue(q)

    println(q, v)

    enqueue(q, 77)
    println(q)
end

main()