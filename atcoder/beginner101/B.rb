def digit_sum(n)
  n.to_s.each_char.map {|c| c.to_i }.reduce(:+)
end
N = gets.to_i
if  N % digit_sum(N) == 0
  puts "Yes"
else

  puts "No"
end
