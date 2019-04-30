require 'benchmark'
require_relative './heap_tree'

def bubble_sort(list)
  copy = [*list]
  len = copy.size
  (len-1).times { |i|
    (i+1).upto(len-1) { |j|
      if copy[i] > copy[j]
        copy[i], copy[j] = copy[j], copy[i]
      end
    }
  }
  copy
end

def _merge(an, bn)
  res = []
  while !(an.empty? or bn.empty?)
    if an.first <= bn.first
      res.push(an.shift)
    else
      res.push(bn.shift)
    end
  end

  if an.empty?
    res.concat(bn)
  else
    res.concat(an)
  end

  res
end

def merge_sort(list)
  len = list.size
  if len == 1
    return list
  end

  list1 = list[0..(len/2-1)]
  list2 = list[(len/2)..-1]

  return _merge(merge_sort(list1), merge_sort(list2))
end

def heap_sort(list)
  ht = Heap_Tree.new()
  res = Array.new(list.length)

  list.each { |l|
    ht.up_heap(l)
  }

  (list.length-1).downto(0) { |i|
    res[i] = ht.down_heap
  }
  res
end

def _pivot(list)
  x = list[0]
  y = list[list.length/2]
  z = list[list.length-1]

  res = if x > y
          if y > z
            y
          elsif x > z
            z
          else
            x
          end
        else
          if x > z
            x
          elsif y > z
            z
          else
            y
          end
        end
  return res
end

def quick_sort(list)
  len = list.length
  if len == 1
    return list
  elsif len == 2
    if list[0] > list[1]
      return [list[1], list[0]]
    end
    return list
  end
  p = _pivot(list)
  left, right = list.partition { |l| l < p }

  return quick_sort(left) + quick_sort(right)
end

def benchmark(func_list, target, expect)

  func_list.each { |func|
    result = Benchmark.realtime do
      p func.call(target) == expect
    end
    puts "#{func.to_s}: #{result}s"
  }
end

def main
  random_list = [*(1..5000)].shuffle
  sorted_list = [*(1..5000)]

  sorts = [method(:bubble_sort), method(:merge_sort), method(:heap_sort), method(:quick_sort)]
  benchmark(sorts, random_list, sorted_list)
end

main()