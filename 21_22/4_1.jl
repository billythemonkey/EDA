"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 05 2022
"""


using Plots

function quick_sort!(A, p, r)
    if p < r
        q = partition!(A, p, r)
        quick_sort!(A, p, q - 1)
        quick_sort!(A, q + 1, r)
    end

end
function partition!(A, p, r)
    x = A[r]
    
    i = p - 1
    for j = p:r-1 
        if A[j] <= x
            i = i + 1
            temp = A[i]
            A[i] = A[j]
            A[j] = temp
        end
    end
    temp = A[i+1]
    A[i+1] = A[r]
    A[r] = temp
    return i + 1
end

function merge_sort!(A, p, r)
    if p < r
        q = convert(Int64,floor((p+r)/2))
        ## q = convert(Int64, floor((p + r)/ 2))
        #merge_sort!(A, p, q)
        #merge_sort!(A, q + 1, r)
        merge!(A, p, q, r)
    end
end
function merge!(A, p, q, r)
    n1 = q - p + 1
    
    n2 = r - q
    
    L = zeros(n1 + 1)
    R = zeros(n2 + 1)
    for i = 2:n1
        L[i] = A[p + i - 1] 
    end
    for j = 2:n2 
        R[j] = A[q + j]
    end

    L[n1 + 1] = Inf
    R[n2 + 1] = Inf
    
    i = 1
    j = 1

    for k = p:r
        if L[i] <= R[j]
            A[k] = L[i]
            i = i+1
        else
            A[k] = R[j]
            j = j + 1
        end
    end
end

function medir_tempo!(A, N, counter, timings)
    A1 = A
    A2 = A
    t1 = @elapsed quick_sort!(A1, 1, length(A1))
    t2 = @elapsed merge_sort!(A2, 1, length(A2))
    timings[counter, 1] = N
    timings[counter, 2] = t1
    timings[counter, 3] = t2
end


function main()
    counter = 1
    N1 = 50000
    N2 = 120000
    ΔN = 10000
    steps = convert(Int64, ((N2 - N1) / ΔN) + 1)
    
    timings = zeros(steps, 3)

    for N = N1:ΔN:N2
        A = rand(Int, N)
        medir_tempo!(A, N, counter, timings)
        counter += 1
    end
    ##quick_sort!(A, 1, length(A))
    ##merge_sort!(A, 1, length(A))

    x = timings[:, 1]
    l1 = timings[:, 2]
    l2 = timings[:, 3]

    plot(x, l1, title = "4_1.jl - Quick-Sort vs. Merge-Sort", label = "quick-sort", lw = 3)
    plot!(x, l2 * 20,label = "merge-sort", lw = 3)


end

main()
