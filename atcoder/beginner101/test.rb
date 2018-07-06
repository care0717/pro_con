def digit_sum(n)
  n.to_s.each_char.map {|c| c.to_i }.reduce(:+)
end
K = gets.to_i
res  = []
15.times{|i|
  a = 10**(i+1)+10**i-1
  b = 10**(i+1)+10**i-1-10**(i-1)
  puts a/digit_sum(a) > b/digit_sum(b)
}
