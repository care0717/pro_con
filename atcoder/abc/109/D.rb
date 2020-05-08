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
class Numeric
  def sign
    if self >= 0
      1
    else
      -1
    end
  end
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

def solve(top, left)
  y = left[0] - top[0]
  x =  left[1]-top[1]
  current = [*top]
  operations = []
  (1..y).each{
    next_pos = [current[0]+1, current[1]]
    operations << current.join(" ")+ " "+  next_pos.join(" ")
    current = [*next_pos]
  }
  (1..(x.abs)).each{
    next_pos = [current[0], current[1]+x.sign ]
    operations << current.join(" ")+ " "+  next_pos.join(" ")
    current = [*next_pos]
  }
  operations
end

h, w = li()
a = []
h.times{
  a << li()
}

oddPos = []
h.times{|i|
  w.times{|j|
    if a[i][j].odd? 
      oddPos << [i+1, j+1]
    end
  }
}

c = 0
res = []
while oddPos.length > 1 
  oddPos = oddPos.sort_by{|i, j| [-j, c+1]}.sort_by{|i, j| [-i, c+1]}
  top = oddPos.pop
  oddPos = oddPos.sort_by{|i, j| [-i, c+1]}.sort_by{|i, j| [-j, c+1]}
  left = oddPos.pop
  res << solve(top, left)
end
res.flatten!
puts res.length
res.each{|r|
  puts r
}
