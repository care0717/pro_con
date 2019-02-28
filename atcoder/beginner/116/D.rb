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

class Point
  attr_accessor :data, :list, :interim, :point
  def initialize(data,list, k)
    @list = list[k..-1]
    @interim = list[0..(k-1)]
    @data = data
    @interim_hash = {}
    @interim.each {|t,d|
      @data[t].delete_at(0)
      if @interim_hash[t] == nil 
        @interim_hash[t] = [d]
      else
        @interim_hash[t].push(d)
      end
    }
    @point = calc_point()
  end
  def result()
    t0 = @interim[-1][0]
    if @interim_hash[t0].length > 1 
      @interim_hash[t0].delete_at(-1)
      while isNewKey 
        t1, d1 =  @list[0]
        if @interim_hash[t1] == nil
          @interim_hash[t1] = [d1]
          isNewKey = Try
        else

        end
      end
    else
    end

  end

  def calc_point()
    @interim_hash.length ** 2 + @interim_hash.values.flatten.sum
  end
end

n, k = li()
data = {}
list = []
n.times{
  t,d =  li()
  if data[t] == nil
    data[t] =  [d]
  else
    data[t].push(d)
    data[t].sort!.reverse!
  end
  list << [t,d]
}

list.sort_by!{|t,d| -d}
point = Point.new(data,list,k)
p point
