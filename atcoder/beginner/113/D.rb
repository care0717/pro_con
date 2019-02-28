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

def solve(n,m)
  sq = Math.sqrt(m).floor
  res = []
  (1..sq).each{|i|
    if m%i == 0
      pair = m/i
      if i >= n || pair >= n
        if i  >= n && pair >= n
          res.push([i,pair].max)
        else
          res.push([i,pair].min)
        end
      end  
    end
  }
  return res.max
end

n, m = li()
p solve(n,m)
