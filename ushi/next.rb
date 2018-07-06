def solve(s) 
  stack_bracket = 0
  stack_paren = 0 
  max_depth = 0
  isValid = true
  last_char = ""
  s.each_char { |c|
    case c
    when "(" then
      stack_paren += 1
    when ")" then
      stack_paren -= 1
    when "[" then
      stack_bracket += 1
    else
      stack_bracket -= 1
    end
    max_depth =  stack_bracket+stack_paren > max_depth ? stack_bracket+stack_paren : max_depth
    isValid &&= !((last_char == "(" && c == "]" ) || (last_char == "[" && c == ")" ))
    isValid &&= !(stack_bracket < 0 || stack_paren < 0)
    last_char = c
  }
  return isValid && (stack_bracket == 0 && stack_paren == 0), max_depth
end

s = gets.chomp
result = Array.new()
if s.include?("|") then
else
  isValid, result = solve(s)
  if !isValid then
    puts -1
  else
    puts result
  end
end
