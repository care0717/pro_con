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

def check_around(x, y, nex_x, nex_y, time)
  distance = (x-nex_x).abs + (y-nex_y).abs
  if distance == time
    return true
  elsif distance > time
    return false
  else
    return (time-distance)%2 == 0
  end
end

n = i()
as = [[0,0,0]]
n.times{
  as << li()
}
res = "Yes"
1.upto(n){|i|
  time = as[i][0] - as[i-1][0]
  if !check_around(as[i-1][1], as[i-1][2], as[i][1], as[i][2], time)
    res = "No"
  end
}
puts res