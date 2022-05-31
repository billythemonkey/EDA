"""
Author: Andrei Razvan Oproiu
Date: Tue Mar 15 2022
"""

using Random, Distributions, Plots

n = Normal(0.0, 1.0)

function media( X )
    μ = sum(X) / length(X) 
    μ
end

function desvio_padrao(X)
    μ = media(X)
    desvios = (X .- μ).*2
    σ = sqrt(sum(desvios) / length(X))
end

function erro_relativo(μ, σ)
    ϵ =  σ / μ * 100
end

function bubble_sort(A)
    len = length(A)
    for i = 1:len - 1
        for j = 2:len
            if A[j-1] > A[j]
                tmp = A[j-1]
                A[j-1] = A[j]
                A[j] = tmp
            end
        end
    end
end

function insertion_sort( A )
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

function merge_sort(A)

    
end

function medir_tempos(A, sample_size, N)
    


    for i = 1:sample_size
        A_insertion = rand( n, N)
        A_bubble = rand( n, N)
        A_merge = rand( n, N)


    end


end

function main()
    
    
    counter = 0
    
    n1 = 2000
    n2 = 3000
    Δn = 100

    

    steps = convert( Int64, (n2-n1)/Δn ) + 1
    sample_size = 300

    A1 = zeros(steps, 3)
    A2 = zeros(steps, 3)
    A3 = zeros(steps, 3)
    for i = n1:Δn:n2
        medir_tempos(steps, sample_size, i)
    end
   

end

main()