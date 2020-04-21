n = gets.to_i

as = gets.split.map(&:to_i)
max = 0
0.upto(n-1){|i|
  (0).upto(n-1){|j|
    diff = (as[j]-as[i]).abs
    max = diff > max ? diff : max
  }
}
puts max