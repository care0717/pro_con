N = gets.to_i
d = []
N.times{
  d.push(gets.to_i)
}
d.sort_by!{|e| -e}
def hakimi(list)
  return false if list.select{|e| e < 0}.size > 0
  top = list.shift
  if top < 0
    return false
  elsif top == 0
    return true
  end
  top.times{|i|
    list[i] -= 1
  }
  hakimi(list)
end

if hakimi(d)
  puts "YES"
else
  index = d.index{|e| e == d.min}
  d[index[0]] += 1
  if hakimi(d)
    puts "NO"
  else
    puts "ABSOLUTELY NO"
  end
end
