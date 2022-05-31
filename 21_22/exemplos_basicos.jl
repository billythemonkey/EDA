include("medir_tempo.jl")

using Plots

i = 1
while i < 10
    println("Hellow World")
    global i += 1
end

for k  = 1:10:100
    println(k)
end

println()

a = [12, 3, 2, 14, 155, 2]



for k in a
    println(k)
end

function media(X)
    μ = 1/length(X) * sum(X)
end

μ = media(a)

println("média = $μ")