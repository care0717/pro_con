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

n = i()
as = li()
res = {}
as.each{|a|
  if res[a.to_s] == nil
    res[a.to_s] = 1
  else
    res[a.to_s] += 1
  end
}
sum = 0
res.each{|key, val|
  key_i = key.to_i
  if key_i < val
    sum += val - key_i
  elsif key_i > val
    sum += val
  end
}
puts sum