def s()
  gets()
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

n, k = li()
ps = []
ruiseki = []

(2*k).times{
  ps << Array.new(2*k, 0)
  ruiseki << Array.new(2*k, 0)
}
n.times{
  temp = s().split
  if temp[2] == "B"
    ps[(temp[0].to_i+k)%(2*k)][temp[1].to_i%(2*k)] += 1
  else
    ps[temp[0].to_i%(2*k)][temp[1].to_i%(2*k)] += 1
  end
}
ruiseki[0][0] = ps[0][0]

1.upto(2*k-1){|i|
  ruiseki[0][i] = ps[0][i] + ruiseki[0][i-1]
  ruiseki[i][0] = ps[i][0] + ruiseki[i-1][0]
}

1.upto(2*k-1){|i|
  1.upto(2*k-1){|j|
    ruiseki[i][j] = ruiseki[i][j-1] + ruiseki[i-1][j] + ps[i][j] - ruiseki[i-1][j-1]
  }
}

p ruiseki
ans = 0
(0 .. k-1).each do |i|
  (0 .. k-1).each do |j|
    c = ruiseki[i + k][j + k] - ruiseki[i][j + k] - ruiseki[i + k][j] + ruiseki[i][j]
    puts c
    ans = c if c > ans
    c = n - c
    ans = c if c > ans
  end
end
puts ans
