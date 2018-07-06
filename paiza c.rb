input_lines = gets.split(" ")
if input_lines[0] == 'x'
    if input_lines[1] == '+'
        puts input_lines[4].to_i - input_lines[2].to_i
    else
        puts input_lines[4].to_i + input_lines[2].to_i
    end
elsif input_lines[2] == 'x'
    if input_lines[1] == '+'
        puts input_lines[4].to_i - input_lines[0].to_i
    else
        puts input_lines[0].to_i - input_lines[4].to_i
    end
else
    if input_lines[1] == '+'
        puts input_lines[2].to_i + input_lines[0].to_i
    else
        puts input_lines[0].to_i - input_lines[2].to_i
    end
end
