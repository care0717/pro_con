N, K  = gets.split.map(&:to_i)
as = gets.split.map(&:to_i)
index = as.index(1)
diff_0 = index
diff_1 = N-1 - index
mod_right = diff_0 % (K-1)
mod_left = diff_1 % (K-1)
sum =  (diff_0/(K-1).to_f).ceil + (diff_1/(K-1).to_f).ceil
if mod_left+mod_right <= K-1 && mod_right >= 1 && mod_left >= 1
  sum -= 1
end
puts sum