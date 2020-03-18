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


S = s()
len = S.length

min = len
(len-1).times{|i|
  if S[i+1] != S[i]
    max = (i+1) > (len-i-1) ? (i+1) : (len-i-1)
    min = max < min ? max : min
  end
}
puts min