"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 05 2022

This is module used just to get acccess to the sorting algorithms in each class of EDA.
How to use:
        - import file at the start of the working file. (ex: "import("name_of_file.jl")")
        - add using argument with "Sort"
        - implement the methods in this module.
"""

module Sort

    """
    This is the Insertion Sort algorithm that accepts and array as parameter
    """
    function insertion_sort!( A )
        for j = 2:length(A)
            key = A[j]
            i = j - 1
            while i > 0 && A[i]  > key
                A[i+1] = A[i]
                i = i - 1
            end
            A[i+1] = key
        end
    end

    """
    This is the Merge Sort algorith that has and array, index and end as parameters
    """
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

    """
    This is normal Quick-Sort algorith that receives the same parameters as Merge-Sort, array, index and end
    """
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



    function get_function(choice)
        println("Choose a sorting algorithm to use: ")
        
        if choice == 1
            
            insertion_sort!(A)
        end
    end

    function main()
        choice = parse(Int64, readline())
        get_function(choice)
    end
    
end