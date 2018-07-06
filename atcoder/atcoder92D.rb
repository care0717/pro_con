a = [4,6,0,0,3,3]
b = [0, 5, 6, 5,0, 3]
N = 6
max = [a.max, b.max].max.to_s(2).length
bin_a=a.map{|i| i.to_s(2).reverse}
bin_b=b.map{|i| i.to_s(2).reverse}
res = ""

kurikoshi = 0
max.times{|j|
  count_a = 0
  count_b = 0
  N.times{|i|
    if bin_a[i][j] == "1" then
      count_a+=1
    end
    if bin_b[i][j] == "1" then
      count_b+=1
    end
  }
  puts count_a
  puts count_b
  puts kurikoshi
  res += (((count_a+count_b)*N+kurikoshi)%2).to_s
  kurikoshi = count_a*count_b
}
puts res.reverse.to_i(2)
