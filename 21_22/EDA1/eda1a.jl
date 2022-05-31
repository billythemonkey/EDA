"""
Author: Andrei Razvan Oproiu
Date: Tue Mar 15 2022
"""

using Random, Distributions, Plots

n = Normal(0.0, 1.0)
counter = 0
N1 = 1000
N2 = 10000
ΔN = 500
number_of_steps = convert( Int64, (N2-N1)/ΔN ) + 1
println(number_of_steps)
timings = zeros(number_of_steps, 2)


function bubble_sort(A::Array)
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




function medir_tempo(A, N, counter)
    x1 = rand( n, N)
    x2 = rand( n, N)
    local t1 = @elapsed bubble_sort(x1)
    local t2 = @elapsed insertion_sort(x2)
    A[counter, 1] = t1
    A[counter, 2] = t2
    println(A)
    return A
end

function media( X )
    μ = sum(X) / length(X) 
    μ
end

function desvio_padrao(X)
    μ = media(X)
    desvios = (X .- μ).*2
    σ = sum(desvios) / length(X)
end

function erro_relativo(μ, σ)
    ϵ =  σ / μ * 100.0
end

function n_medir_tempo(nr_samples,A,N,counter)
    x1 = rand( n, N)
    x2 = rand( n, N)

    A_t1 = zeros(nr_samples)
    A_t2 = zeros(nr_samples)

    for i = 1 : nr_samples
        t1 = @elapsed bubble_sort(x1)
        t2 = @elapsed insertion_sort(x2)

        A_t1[i] = t1
        A_t2[i] = t2

    end

    t1 = media(A_t1)
    t2 = media(A_t2)

    A[counter, 1] = t1
    A[counter, 2] = t2
    println(A)
    return A
end

for N = N1:ΔN:N2
    global counter += 1
    n_medir_tempo(50,timings, N, counter)
end


x = 1:number_of_steps
y = timings[:,1]
z = timings[:,2]
plot(x, y)
plot!(x, z.*10.0)

function main()
    M = 1000
    X = zeros(M)
    n = 10000
    
    for k = 1:M
        A = rand(n)
        Δt = @elapsed bubble_sort(A)
        X[k] = Δt
    end

    μ = media(X)
    σ = desvio_padrao(X)
    ϵ = erro_relativo(μ, σ)
end



