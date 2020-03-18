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


def gen_abc(len)
  res =  []
  (3**len).times{|i|
    temp = i.to_s(3).rjust(len, '0')
    temp.gsub!("0", "A")
    temp.gsub!("1", "B")
    temp.gsub!("2", "C")
    res << temp
  }
  res
end

def count_bc(str)
  if str.length < 2 || !str.include?("B")
    return 0
  end
  b_index = str.index("B")
  string = str[(b_index+1)..-1]
  return string.count("C") + count_bc(string)
end

def count_abc(str)
  if str.length < 3 || !str.include?("A")
    return 0
  end
  a_index = str.index("A")
  string = str[(a_index+1)..-1]
  return count_bc(string) + count_abc(string)
end

S = s()

dp = Array.new(S.length+1)
len = (S.length)
(len+1).times{|i|
  dp[i] = Array.new(4)
}
dp[len][3] = 1
dp[len][2] = 0
dp[len][1] = 0
dp[len][0] = 0
MOD = 1000000007
abc = "ABC"

(len).times{|i|
  si = S[len-1-i]
  dp[len-1-i][3] = (if si == "?"
                     3*dp[len-i][3]
                   else
                     dp[len-i][3]
                   end)%MOD
  3.times{|j|
    m1 = if si == "?" then 3 else 1 end
    m2 = if si == "?" || si == abc[j] then 1 else 0 end
    dp[len-1-i][j] = (m1*dp[len-i][j] + m2*dp[len-i][j+1])%MOD
  }
}

p dp


