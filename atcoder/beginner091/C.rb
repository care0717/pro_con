N = gets.to_i
reds = []
N.times{
  reds << gets.split.map(&:to_i)
}
reds.sort_by!{|r| -r[1]}
blues = []
N.times{
  blues << gets.split.map(&:to_i)
}
blues.sort_by!{|b| b[0]}
count = 0
blues.each{|b|
  pair =  reds.select{|r| r[0] < b[0] && r[1] < b[1] }[0]
  if pair != nil
    reds.delete(pair)
    count += 1
  end
}
puts count

