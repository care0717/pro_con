A, B, C, X, Y = gets.split.map(&:to_i)
if (A+B) < 2*C
  puts A*X + B*Y
elsif A < 2*C && B < 2*C 
  if X>Y 
    puts 2*C*Y+(X-Y)*A
  else
    puts 2*C*X+(Y-X)*B
  end
elsif A > 2*C && B > 2*C
  puts [X, Y].max * 2*C
elsif A > 2*C && B < 2*C
  if X < Y
    puts  2*C*X+(Y-X)*B
  else
    puts  2*C*X
  end
else
  if X > Y
    puts  2*C*Y+(X-Y)*A
  else
    puts  2*C*Y
  end
end
