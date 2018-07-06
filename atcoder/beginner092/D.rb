A, B =gets.split.map(&:to_i)
res = []
K = 40
W = 90
K.times{
  res << [*1..W].map{|i| "#"}
}
K.times{
  res << [*1..W].map{|i| "."}
}
index_black = [*0..(B-2)].map{|i| i*2}
index_white = [*0..(A-2)].map{|i| i*2}
index_white.each{|i|
  y = i/W*2
  x = i%W
  res[y][x] = "."
}

index_black.each{|i|
  y = i/W*2 + K+1
  x = i%W + y%2
  res[y][x] = "#"
}
puts (K*2).to_s+" "+W.to_s
res.each{|r|
  puts r.join("")
}
