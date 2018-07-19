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

def get_bits(len)
  res =  []
  (2**len).times{|i|
    res << i.to_s(2).rjust(len, '0')
  }
  res
end

n = i()
fs = []
n.times{
  fs << s().gsub(" ", "")
}
ps = []
n.times{
  ps << li()
}

bits = get_bits(10)[1..-1]

max =-10000000000000000
bits.each{|b|
  sum = 0
  n.times{|i|
    pindex=(fs[i].to_i(2) & b.to_i(2)).to_s(2).count("1")
    sum += ps[i][pindex]
  }
  max = max > sum ? max : sum
}

puts max
