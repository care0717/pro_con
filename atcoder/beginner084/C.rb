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

c = []
s = []
f = []
(n-1).times{
  temp = li()
  c << temp[0]
  s << temp[1]
  f << temp[2]
}


(n-1).times{|i|
  ride_t = c[i]+s[i]
  (i+1).upto(n-2){|j|
    diff = ride_t-s[j]
    if diff <= 0
      ride_t = s[j]+c[j]
    else
      ride_t = s[j]+(diff.to_f/f[j]).ceil*f[j]+c[j]
    end

  }
  puts ride_t
}
puts 0