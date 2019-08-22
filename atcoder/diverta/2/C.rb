def s()
  gets().chomp
end
def i()
  gets.to_i
end
def li()
  gets.split.map(&:to_i)
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end
class Array
  def sum
    total = 0
    self.each do |item|
      total += item
    end
    total
  end
end

def eratosthenes(size)
  prime_list = []
  is_prime = Array.new(size, true)
  is_prime[0] = false
  is_prime[1] = false

  2.upto(size-1){|i|
    if is_prime[i]
      prime_list << i
      (2).upto((size.to_f/i).ceil-1){|j|
        is_prime[j*i] = false
      }
    end
  }
  [prime_list, is_prime]
end

def dijkstra(s, e, cost)
  size = cost.length
  if s==e
    return 0
  end
  dp = {}
  dp[s] = 0
  undefined_node = {}
  size.times{|i|
    if i != s
      undefined_node[i] = cost[s][i]
    end
  }
  while undefined_node.length != 0
    min = undefined_node.min_by{|_, v| v}
    stock = {}
    temp = undefined_node.clone
    temp.each{|k, v|
      if min[1] == v
        undefined_node.delete(k)
        if k == e
          return v
        end
        dp[k] = v
        stock[k] = v
      end
    }

    stock.each{|k, v|
      undefined_node.each{|k2, v2|
        undefined_node[k2] = v + cost[k][k2] < v2 ?  v + cost[k][k2] : v2
      }
    }
  end
end

def get_bits(len)
  res =  []
  (2**len).times{|i|
    res << i.to_s(2).rjust(len, '0')
  }
  res
end

def get_GCD(m, n)
  tmp = [n, m].max
  n = [n, m].min
  m = tmp
  while n != 0
    tmp = m % n
    m = n
    n = tmp
  end
  m
end


n = i()
an = li()
minus = an.select {|item| item  <  0 }
plus = an.select {|item| item  >=  0 }
res = []
if minus.length == 0
  min, index = plus.each_with_index.min
  plus.delete_at(index)
  base = plus.pop
  plus.each { |a|
    res.push(min.to_s+" "+a.to_s)
    min -= a
  }
  res.push(base.to_s+" "+min.to_s)
  p base - min
elsif plus.length == 0
  max, index = minus.each_with_index.max
  minus.delete_at(index)
  base = minus.pop
  res = []
  minus.each { |a|
    res.push(max.to_s+" "+a.to_s)
    max -= a
  }
  res.push(max.to_s+" "+base.to_s)
  p  max - base
else
  minus_base = minus.pop
  plus_base = plus.pop
  plus.each { |a|
    res.push(minus_base.to_s + " " + a.to_s)
    minus_base -= a
  }
  minus.push(minus_base)
  minus.each { |a|

    res.push(plus_base.to_s + " " + a.to_s)
    plus_base -= a
  }
  puts plus_base
end

res.each { |r|
  puts r
}