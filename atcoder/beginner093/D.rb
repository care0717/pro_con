
def less_div(a, b)
  if(a % b ==0)
    a/b-1
  else
    a/b
  end
end

Q = gets.to_i
as = []
Q.times{
  as << gets.split.map(&:to_i).sort
}

Q.times{|i|
  a, b = as[i]
  max = a*b
  root = Math.sqrt(max).floor
  first = root
  second = less_div(max, first)
  while second > 0
    first += 1
    nex = less_div(max, first)
    if(nex >= second)
      break
    else
      second = nex
    end
  end
  if a * b <= 2
    puts 0
  elsif a == b
    puts first+second-2
  else
    puts first+second-3
  end

}
