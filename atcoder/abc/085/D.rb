def s()
  gets()
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

n, h = li()
as = []
max = 0
n.times{
  temp =  li()
  max = temp[0] > max ? temp[0] : max
  as << temp
}
as.sort_by!{|i| -i[1]}

strong_list = as.map{|i| i[1]}.select{|i| i > max}
sum = 0
strong_list.each{|s|
  sum += s
}

if h <= sum
  strong_list.length.times{|i|
    h -= strong_list[i]
    if (h <= 0)
      puts i+1
      break
    end
  }
else
  puts strong_list.length + ((h-sum).to_f/max).ceil
end

