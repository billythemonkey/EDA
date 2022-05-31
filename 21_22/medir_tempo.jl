"""
Author: Andrei Razvan Oproiu
Date: Tue Apr 05 2022
"""

include("sort_algorithms.jl")

module Timing
    function measure_once(A, N)
        sort_algorithm = Sort.get_function("quick_sort")
    end
    
    function measure_sample(A, N, nr_measurements)
        
    end
end