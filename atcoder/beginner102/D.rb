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

=begin
def nibunibutan(as, left_num)
  left = 0
  index = 0
  right = as.length-1-2
  middle = 0
  while left+1 != right && left != right
    middle = (left+right)/2
    index1 = nibutan(as[(middle+1)..-1], as[middle])
    first = ((as[middle]-left_num)-(as[middle+1+index1] -as[middle])).abs
    index2 = nibutan(as[(middle+2)..-1], as[middle+1])
    second = ((as[middle+1]-left_num)-(as[middle+2+index2] -as[middle+1])).abs
    if first > second
      left = middle
    else
      right = middle
    end
  end
  left_index = nibutan(as[(left+1)..-1], as[left])
  right_index = nibutan(as[(right+1)..-1], as[right])
  res = if ((as[left]-left_num)-(as[left+1+left_index] -as[left])).abs < ((as[right]-left_num)-(as[right+1+right_index] -as[right])).abs
                [left, left_index]
  else
                [right, right_index]
        end
  res
end
=end

n = gets.to_i
as = gets.split.map(&:to_i)
sum = [0]
n.times{|i|
  sum << as[i]+sum[i]
}
sum = sum[1..-1]
result = 100000000000
(n-3).times{|i|
  temp = sum[(i+1)..-1].clone
  res_index, index =nibunibutan(temp, sum[i])
  #p [as[0..i], as[i+1..(res_index+1+i)], as[(res_index+1+i+1)..(res_index+1+index+1+i)], as[(res_index+1+index+1+i+1)..-1]]
  res = [sum[i], sum[res_index+1+i]-sum[i], sum[res_index+1+index+1+i] - sum[res_index+1+i], sum[-1]- sum[res_index+1+index+1+i]]
  result = (res.max - res.min) < result ?  (res.max - res.min) : result
}
puts result