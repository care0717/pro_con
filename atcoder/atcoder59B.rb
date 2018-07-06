a=gets.to_s
b=gets.to_s


def greaterThan(x, y, i)
  if(x[i].to_i > y[i].to_i)
    "GREATER"
  elsif(x[i].to_i < y[i].to_i)
    "LESS"
  elsif(i==x.size)
    "EQUAL"
  else
    greaterThan(x, y, i+1)
  end
end
if(a.size>b.size)then
  res = "GREATER"
elsif a.size<b.size then
  res = "LESS"
else
  res = greaterThan(a, b, 0)
end

puts(res)
