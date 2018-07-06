N, C =gets.split.map(&:to_i)
xs = [[0, 0]]
ys = [[0, 0]]

right_res_list = []
left_res_list = []
right_res_max_list = []
left_res_max_list = []
res = 0
max = 0
N.times{|i|
  temp = gets.split.map(&:to_i)
  xs << temp
  ys << [C - temp[0], temp[1]]
  res += xs[i+1][1] - (xs[i+1][0] - xs[i][0])
  max = res > max ? res : max
  right_res_max_list << max
  right_res_list << res
}
ys.sort_by!{|i| i[0]}
res = 0
max = 0
N.times{|i|
  res += ys[i+1][1] - (ys[i+1][0] - ys[i][0])
  max = res > max ? res : max
  left_res_max_list << max
  left_res_list << res
}
max = 0
max = [max, right_res_list.max, left_res_list.max].max
(N-1).times{|i|
  temp1 = right_res_list[i] - xs[i+1][0] + left_res_max_list[(N-i-2)]
  temp2 = left_res_list[i] - ys[i+1][0] + right_res_max_list[(N-i-2)]
  max = temp1 > max ? temp1 :max
  max = temp2 > max ? temp2 :max
}
puts max


