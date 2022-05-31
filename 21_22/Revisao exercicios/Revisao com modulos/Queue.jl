"""
Author: Andrei Razvan Oproiu
Date: Tue May 03 2022
"""

module MyQueue

mutable struct new
    head::Int
    tail::Int
    memory::Array
end

function isEmpty(q::new)
    if q.head == q.tail
        return true
    end
    return false
end

function isFull(q::new)
    if q.head == q.tail + 1 || (q.head == 1 && q.tail == size(q.memory) + 1)
        return true
    end
    return false
end

function enqueue(q::new, value)
    if isFull(q)
        println("Enqueue Error: Queue is full!!")
    else
        if q.tail == size(q.memory) + 1
            q.tail = 1
        else
            q.memory[q.tail] = value
            q.tail += 1
        end
    end
end

function dequeue(q::new)
    if isEmpty(q)
        println("Dequeue Error: Queue is empty!!")
    else
        dequeueValue = q.memory[q.head]
        if q.head == q.memory[q.head] + 1
            q.head = 1
        else
            q.head += 1
        end
    end
    return dequeueValue
end

end