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

def barger_p_num(n)
  2**(n+1)-1  
end

def barger_length(n)
  2**(n+2)-3
end

def eat_barger_level(n, x)
  (n+1).times{|i|
    len = 2**(n-i+2)-3
    if x-i >= len
      return n-i
    end
  }
end

def solve(n, x)
  if x <= n 
    return 0
  end
  level = eat_barger_level(n, x)
  eaten = barger_p_num(level)
  rest_eat_num = x-barger_length(level)-(n-level)
  if rest_eat_num > 0
    eaten+1+solve(level, rest_eat_num-1)
  else
    eaten
  end
end

n, x = li()
p solve(n,x)
