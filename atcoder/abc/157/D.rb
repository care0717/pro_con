require 'set'

def s()
  gets().chomp
end
def i()
  gets.to_i
end
def ls()
  gets.split
end
def li()
  gets.split.map(&:to_i)
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end
def inli()
  n = gets.to_i
  as = []
  n.times {
    as << gets.split.map(&:to_i)
  }
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
def fact(n, m=0)
  (m+1..n).inject(1,:*)
end
def perm(n, r)
  fact(n, n-r)
end
def comb(n, m)
  fact(n, n-m)/fact(m)
end

def matrix(n, m=n, init=0)
  Array.new(n).map{Array.new(m,init)}
end
class UnionFind
  def initialize(size)
    @par = (0..size).to_a
    @size = Array.new(size+1, 1)
  end
  def root(x)
    while @par[x] != x
      @par[x] = @par[@par[x]]
      x = @par[x]
    end
    x
  end
  def merge(a, b)
    a = root(a)
    b = root(b)
    if a == b
      return false
    end
    if @size[a] < @size[b]
      tmp = a
      a = b
      b = tmp
    end
    @size[a] += @size[b]
    @par[b] = a
    true
  end

  def is_same(a, b)
    root(a) == root(b)
  end

  def size(a)
    @size[root(a)]
  end
end


n, m, k = li()
fs = []
m.times do
  fs << li()
end
bs = []
k.times do
  bs << li()
end

uf = UnionFind.new(n)
map = {}
fs.each do |a,b|
  if map.has_key?(a)
    map[a] << b
  else
    map[a] = Set.new([b])
  end
  if map.has_key?(b)
    map[b] << a
  else
    map[b] = Set.new([a])
  end
  uf.merge(a, b)
end

bs.each do |a,b|
  if map.has_key?(a)
    map[a] << b
  else
    map[a] = Set.new([b])
  end
  if map.has_key?(b)
    map[b] << a
  else
    map[b] = Set.new([a])
  end
end
res = []
1.upto(n) do |i|
  tmp = uf.size(i) - map.fetch(i, Set.new()).count{|j| uf.is_same(i, j)} - 1
  res << tmp
end
puts res.join(" ")
