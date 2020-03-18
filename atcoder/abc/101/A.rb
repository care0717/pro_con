s = gets.chop
count = 0
s.each_char{|c|
  if c=="+"
    count += 1
  else
    count -= 1
  end
}
puts count