"""
Author: Andrei Razvan Oproiu
Date: Tue May 10 2022
"""

module MyNaryTree
    mutable struct self
        vec_keys::Array
        vec_p::Array{Int}
        vec_prev::Array{Int}
        vec_next::Array{Int}
        vec_head::Array{Int}
    end

    function treeMaximum(t::self)
        
        while t.vec_next[] != 1

            lvl += 1
        end
    end

    function treeMinimum()
        
    end
end

