N, K = gets.split.map(&:to_i)
sum = 0
if K==0
  sum = N**2
else
  (K+1).upto(N){|b|
    sum += ((N-K)/b+1)*(b-K)
    over = ((N-K)/b)*b+K+(b-K-1)-N
    if over >= 0
      sum -= over
    end
  }
end
puts sum