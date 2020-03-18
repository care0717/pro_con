require 'set'

def s()
  gets()
end
def i()
  gets.to_i
end
def li()
  gets.split.map(&:to_i)
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end
cs = []
3.times{
  cs << li()
}
cs_t = cs.transpose
res = true
2.times{|i|
  res &&= Set.new(cs[0].zip(cs[i+1]).map{|a, b| a-b}).size == 1
  res &&= Set.new(cs_t[0].zip(cs_t[i+1]).map{|a, b| a-b}).size == 1

}
if res
  puts "Yes"
else
  puts "No"
end