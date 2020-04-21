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

n = i()
as = []
n.times{
  as << s()
}
res = [0,0,0,0,0]
n.times{|i|
  if (as[i][0] == "M")
    res[0] += 1
  elsif (as[i][0] == "A")
    res[1] += 1
  elsif (as[i][0] == "R")
    res[2] += 1
  elsif (as[i][0] == "C")
    res[3] += 1
  elsif (as[i][0] == "H")
    res[4] += 1
  end
}
sum = 0
3.times{|i|
  (i+1).upto(3){|j|
    (j+1).upto(4){|k|
      sum += res[i]*res[j]*res[k]
    }
  }
}
puts sum