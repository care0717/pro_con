def s()
  gets().chomp
end
def i()
  gets.to_i
end
def li()
  gets.split.map(&:to_i)
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end

t = i()
qs = []
t.times{
  qs << li()
}

qs.each{|q|
  a, b, c, d = q
  diff = a%b
  if a-b < 0 || (diff < b && diff > c) || (b>d)
    puts "No"
    next
  end
  rui = d-d/b*b
  if (rui==0 && diff <= c) || b <= c+1
    puts "Yes"
    next
  elsif  rui==0 && diff > c
    puts "No"
    next
  end
  gcd = d.gcd(b)

  max = b - gcd + (a%gcd)

  if max > c
    puts "No"
  else
    puts "Yes"
  end

}