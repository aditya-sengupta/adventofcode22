# I'm doing this one in Julia 
# because it wasn't gonna teach me much about Rust
# and all the parsing looked like a huge pain
# and this is a really good test case for multiple dispatch

compare(left::Number, right::Vector; internal=true) = compare([left], right; internal=internal)
compare(left::Vector, right::Number; internal=true) = compare(left, [right]; internal=internal)

function compare(left::Number, right::Number; internal=true)
    if left < right
        return true, false
    elseif right < left
        return false, false
    else 
        return true, true
    end
end

function compare(left::Vector, right::Vector; internal=false)
    for (l, r) in zip(left, right)
        comparison, check_next = compare(l, r; internal=true)
        if !check_next
            if internal
                return comparison, check_next
            else 
                return comparison
            end
        end
    end
    # reaching here because you ran out of elements
    ordered = length(left) <= length(right)
    if internal
        return ordered, false
    else
        return ordered
    end
end

function day13_1(fname = "./day13")
    lines = readlines(fname) .|> Meta.parse .|> eval
    j = 0
    total_inds = 0
    for i = 1:3:length(lines)
        j += 1
        if compare(lines[i], lines[i + 1])
            total_inds += j
        end
    end
    return total_inds
end

function day13_2(fname = "./day13")
    lines = readlines(fname) .|> Meta.parse .|> eval
    lines = filter(x -> !isnothing(x), lines)
    push!(lines, [[2]])
    push!(lines, [[6]])
    sorted_lines = sort(lines, lt=compare)
    p1 = findfirst(x->x==[[2]], sorted_lines)
    p2 = findfirst(x->x==[[6]], sorted_lines)
    println(p1 * p2)
end
