"""
inserir.jl
Jose Jasnau Caeiro
8/03/2022

insertion sort
"""

include("sort_algorithms.jl")
using Sort


function media( X )
    μ = sum(X) / length(X)  
end


function desvio( x, μ )
    (x-μ)^2
end

function desvio_padrao( X )
    μ = media(X)
    Y = zeros(length(X))
    
    for k=1:length(X)
        Y[k] = desvio(X[k], μ)
    end
    
    sum(Y) / length(X)
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

function medir_tempo(A, N)
    numero_amostras = 50
    X = zeros(numero_amostras)
    for n = 1:numero_amostras
        local A = rand(N)
        local t1 = time()
        insertion_sort( A )
        local t2 = time()
        Δt = t2-t1
        X[n] = Δt
    end
    media(X), desvio_padrao(X)
end

N1 = 50000
N2 = 120000
ΔN = 10000


numero_pontos = convert( Int64, (N2-N1)/ΔN ) + 1
println(numero_pontos)
T = zeros(numero_pontos, 2)

k = 1
for N = N1:ΔN:N2

    t = medir_tempo(A, N)
    println( "$N -> $t" )
    #T[k,1] = N
    #T[k,2] = t
    global k += 1
end
