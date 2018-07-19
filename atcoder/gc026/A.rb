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

n, as = ili()

res = {}
before = as[0]
j = 0
1.upto(n-1){|i|
  if before == as[i]
    if res[j] == nil
      res[j] = 2
    else
      res[j] += 1
    end
  else
    j+=1
  end
  before = as[i]
}

res = res.select{|k, v| v>=2}
sum = 0
res.each{|k,v|
  sum += v/2
}
puts sum
