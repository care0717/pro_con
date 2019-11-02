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

def calc_divisor(n)
  hash = {}
  PRIMES.each{ |p|
    count = 0
    if n%p == 0
      m = n
      while(m%p == 0)
        count += 1
        m /= p
      end
    end
    hash[p] = count
  }
  return hash
end

def factorial(n, r)
  res = 1
  ((n-r+1)..n).each{|i|
    res *= i
  }
  res
end
def combination(n,r)
  factorial(n,r)/factorial(r,r)
end

class Node
  attr_accessor :done, :cost, :previous
  def initialize(done, cost, previous = nil)
    @done = done
    @cost = cost
    @previous = previous
  end
end

def dijkstra(inf, start, goal, fields)
  n = fields.length
  is_opens = Array.new(n).map{Array.new(n, false)}
  fields[start[0]][start[1]] =  Node.new(false, 0)
  opens = [[[start[0], start[1]], Node.new(false, 0)]]
  is_opens[start[0]][start[1]] = true
  while true
    # コストが一番小さいやつをさがす
    min = inf - 1
    pos = nil
    index = 0
    opens.length.times do |i|
      y, x = opens[i][0]
      node = opens[i][1]
      if node.cost < min
        min = node.cost
        pos = [y, x]
        index = i
      end
    end
    if pos == nil
      break
    end
    opens.delete_at(index)
    min_y, min_x = pos
    min_node = fields[min_y][min_x]
    min_node.done = true
    edge_poss = []
    dd = [[0, 1], [1, 0], [0, -1], [-1, 0]]
    dd.each do |dy, dx|
      if min_y + dy < 0 || n <= min_y + dy || min_x + dx < 0 || n <= min_x + dx
        next
      end
      edge_poss << [min_y + dy, min_x + dx]
    end

    edge_poss.each do |y, x|
      if fields[y][x].done
        next
      end

      cost = min_node.cost + 1
      if cost < fields[y][x].cost
        fields[y][x].cost = cost
        fields[y][x].previous = [min_y, min_x]
      end
      unless is_opens[y][x]
        opens << [[y, x], fields[y][x]]
        is_opens[y][x] = true
      end
    end
  end
  res = []
  pre_pos = fields[goal[0]][goal[1]].previous
  while pre_pos != nil
    res << pre_pos
    pre_pos = fields[pre_pos[0]][pre_pos[1]].previous
  end
  res.reverse << goal
end

def solve()
  n, m, b = li()
  goal = li()
  rs = []
  m.times {
    tmp = gets.split
    rs << [tmp[0].to_i, tmp[1].to_i, tmp[2]]
  }
  inf = 1000000000000
  fields = Array.new(n).map{Array.new(n)}
  n.times do |i|
    n.times do |j|
      fields[i][j] = Node.new(false, inf)
    end
  end
  bs = []
  b.times do
    tmp = li()
    fields[tmp[0]][tmp[1]] = Node.new(true, inf)
    bs << tmp
  end
  results = Array.new(n).map{Array.new(n, nil)}
  rs.each do |start|
    poss = dijkstra(inf, start, goal, Marshal.load(Marshal.dump(fields)))
    if poss.size < 2
      next
    end
    poss = tanshuku(poss)
    (poss.size-1).times do |i|

      if i == 0 && direction(poss[i], poss[i+1]) == start[2]
        next
      end
      y = poss[i][0]
      x = poss[i][1]
      if results[y][x] != nil
        next
      end
      results[y][x] = direction(poss[i], poss[i+1])
    end
  end

  ans = []
  n.times do |i|
    n.times do |j|
      if results[i][j] != nil
        ans << "#{i} #{j} #{results[i][j]}"
      end
    end
  end
  puts ans.size
  ans.each do |an|
    puts an
  end
end

def tanshuku(poss)
  new_poss = [poss[0]]
  dir = direction(poss[0], poss[1])
  1.upto(poss.size-2) do |i|
    if dir != direction(poss[i], poss[i+1])
      new_poss << poss[i]
      dir = direction(poss[i], poss[i+1])
    end
  end
  new_poss << poss.last
end

def direction(pos1, pos2)
  y = pos1[0]
  x = pos1[1]
  dy = pos2[0] - y
  dx = pos2[1] - x
  if dx < 0
    "L"
  elsif dx > 0
    "R"
  elsif dy < 0
    "U"
  elsif dy > 0
    "D"
  end

end


def test()
  n = 5
  inf = 100000000
  start = [0, 0]
  goal = [1, 1]
  fields = Array.new(n).map{Array.new(n)}
  n.times do |i|
    n.times do |j|
      fields[i][j] = Node.new(false, inf)
    end
  end
  p dijkstra(inf, start, goal, Marshal.load(Marshal.dump(fields)))


end

solve()
#test()