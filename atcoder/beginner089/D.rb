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

H, W, D = li()
as = []
H.times{
  as << li()
}
Q= i()
lr = []
Q.times{
  lr << li()
}

list = Array.new(D, 0)
current = []
D.times{
  current << []
}

H.times{|i|
  W.times{|j|
    index = as[i][j]%D
    current[index] << [as[i][j],i,j]
  }
}
D.times{|i|
  current[i].sort_by!{|c| c[0]}
}
res = []
current.each{|c|
  temp = [0]
  (c.length-1).times{|i|
    temp << temp[i] + (c[i+1][1]-c[i][1]).abs + (c[i+1][2]-c[i][2]).abs
  }
  res << temp
}

lr.each{|l|
  if (l[0]%D == 0)
    puts res[l[0]%D][l[1]/D-1] - res[l[0]%D][l[0]/D-1]
  else
    puts res[l[0]%D][l[1]/D] - res[l[0]%D][l[0]/D]
  end
}