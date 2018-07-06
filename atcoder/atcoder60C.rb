N,M = gets.split(" ").map(&:to_i)
t = gets.split(" ").map(&:to_i)
result = 0
(N-1).times {|i|
  if(t[i+1]-t[i] >= M) then
    result += M
  else
    result += t[i+1]-t[i]
  end
}
result += M
puts(result)
