"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

mutable struct Queue
    head::Int
    tail::Int
    memory::Array{Int}
end

function isEmpty(q::Queue)
    if q.tail == q.head
        return true
    end
    return false
end

function isFull(q::Queue)
    if q.head == q.tail + 1
        return true
    end
    return false
end

function enqueue(q::Queue, value::Int)
    if !isFull(q)
        q.memory[q.tail] = value
        if q.tail == size(q.memory)
            q.tail = 1
        else
            q.tail += 1
        end
    else
        println("Queue is Full !!")
    end
end

function dequeue(q::Queue)
    if isEmpty(q)
        println("Queue is Empty !!")
    else
        item = q.memory[q.head]
        if q.head == size(q.memory)
            q.head = 1
        else
            q.head += 1
        end
    end
    return item
end






function main()
    N = 10
    myQueue = Queue(1, 1, zeros(N))


    println(myQueue)

    enqueue(myQueue, 50)
    enqueue(myQueue, 30)

    println(myQueue)

    dequeue(myQueue)

    println(myQueue)

end

main()