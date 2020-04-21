require "time"
def s()
  gets().chomp
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
class Array
  def sum
    total = 0
    self.each do |item|
      total += item
    end
    total
  end
end


a = s()
t = Time.parse(a)
if t > Time.parse("2019/4/30") 
  puts "TBD"
else
  puts "Heisei"
end
