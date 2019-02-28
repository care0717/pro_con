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
def bin_search(s,  x)
  len = s.length
  left = 0 
  right = len-1
  if s[right] <= x
    return right
  elsif s[0] >= x
    return 0
  end

  while left != right-1
    mid = (left+right)/2
    if s[mid] == x
      return  mid
    elsif s[mid] > x 
      right = mid
    else 
      left = mid
    end
  end
  return left
end

def min_len(a, b, x)
  if a > b
    ma = a
    mi = b
  else
    ma = b
    mi = a
  end
  return [x-mi+ma-mi, ma-x+ma-mi].min
end

def solve(a1, a2, b1, b2, x)
  sum = 0
  res = []
  [a1,a2].each{|a|
    sum = 0
    if (x > b1 && b1 > a) ||  (x > b2 && b2 > a)
      sum = (x-a).abs
    else
      sum = [(b1-a).abs, (b2-a).abs].min +  (x-a).abs
    end
    res << sum
  }
  [b1,b2].each{|b|
    sum = 0
    if (x > a1 && a1 > b) ||  (x > a2 && a2 > b)
      sum = (x-b).abs
    else
      sum = [(a1-b).abs, (a2-b).abs].min +  (x-b).abs
    end
    res << sum
  }
  return res.min
end

a, b, q = li()
s = []
t = []
xs = []
a.times{
  s << i()
}
b.times{
  t << i()
}


q.times{
  xs << i()
}

ans = []
xs.each { |x|
  s_i = bin_search(s, x)
  t_i =  bin_search(t, x)
  if s_i == a-1 && t_i == b-1
    ans << solve(s[s_i], s[s_i], t[t_i], t[t_i], x)
  elsif s_i == a-1
    ans  << solve(s[s_i], s[s_i], t[t_i], t[t_i+1], x)
  elsif t_i == b-1
    ans  << solve(s[s_i], s[s_i+1], t[t_i], t[t_i], x)
  else
    ans  << solve(s[s_i], s[s_i+1], t[t_i], t[t_i+1], x)
  end
}
ans.each{|an|
  puts an
}

