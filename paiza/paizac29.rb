class Array
  def sum_rain(date, days)
    self[self.index(date)..(self.index(date)+days-1)][1].reduce(:+)
  end
end


m,n = gets.split(" ").map(&:to_i)
input_lists = []
date_lists = []
rain_lists = []
m.times {
  date,rain=gets.split(" ").map(&:to_i)
  date_lists.push(date)
  rain_lists.push(rain)
  input_lists.push([date, rain])
}
res = []
for i in date_lists[0..-n] do
    puts input_lists.sum_rain(i, n)
end
#print input_lists[res.index(res.min)][0]," ",input_lists[res.index(res.min)+n-1][0]
