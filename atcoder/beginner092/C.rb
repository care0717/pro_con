N = gets.to_i
as = gets.split.map(&:to_i)
max = (as[0]).abs
(N-1).times{|i|
  max += (as[i+1]-as[i]).abs
}
max += as[N-1].abs
as.push(0)
as.unshift(0)
1.upto(N){|i|
  if as[i] < 0
    if as[i+1] <= as[i]
      puts max
    else
      puts (max- 2*[(as[i]-as[i-1]).abs, (as[i+1]-as[i]).abs].min)
    end
  else
    if as[i+1] >= as[i]
      puts max
    else
      puts (max- 2*[(as[i]-as[i-1]).abs, (as[i+1]-as[i]).abs].min)
    end
  end
}