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

def eratosthenes(size)
  prime_list = []
  is_prime = Array.new(size, true)
  is_prime[0] = false
  is_prime[1] = false

  2.upto(size-1){|i|
    if is_prime[i]
      prime_list << i
      (2).upto((size.to_f/i).ceil-1){|j|
        is_prime[j*i] = false
      }
    end
  }
  [prime_list, is_prime]
end


sum = [0]

pl, is_p =  eratosthenes(100003)

like2017 = []
pl[1..-1].each{|i|
  if is_p[(i+1)/2]
    like2017 << i
  end
}

j=0
100003.times{|i|
  if like2017[j] == i
    sum << sum[i]+1
    j += 1
  else
    sum << sum[i]
  end
}
sum = sum[1..-1]

q = i()
lr = []
q.times{
  lr << li()
}

lr.each{|l, r|
  puts sum[r+1] - sum[l-1]
}
