N, M = gets.split.map(&:to_i)
if N==1
  puts (M-2).abs
else
  if M==1
    puts N-2
  else
    puts N*M-2*N-2*M+4
  end
end