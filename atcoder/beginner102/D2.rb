def nibutan(as, left_num)
  len  = as.length-1
  left = 0
  right = len
  while left+1 != right
    middle = (left+right)/2
    first = ((as[middle]-left_num)-(as[len] -as[middle])).abs
    second =((as[middle+1]-left_num)-(as[len] -as[middle+1])).abs
    if first > second
      left = middle
    else
      right = middle
    end
  end
  if ((as[left]-left_num)-(as[len] -as[left])).abs < ((as[right]-left_num)-(as[len] -as[right])).abs
    left
  else
    right
  end
end

n = gets.to_i
as = gets.split.map(&:to_i)
sum = [0]
n.times{|i|
  sum.push(as[i]+sum[i])
}
sum = sum[1..-1]
result = 100000000000

left_i_first = 0
right_i_first = 0
left_list = sum[0..-2]
right_list = sum[-1..-1]
2.upto(n-2){|i|
  temp = left_list.pop
  right_list.unshift(temp)
  left_i = nibutan(left_list, 0)
  index = n-i
  right_i = nibutan(right_list, sum[index-1])

  res = [sum[left_i], sum[index-1]-sum[left_i], sum[index+right_i]-sum[index-1], sum[-1]-sum[index+right_i]]
  max = res.max
  min = res.min
  result = (max - min) < result ?  (max - min) : result
}
puts result