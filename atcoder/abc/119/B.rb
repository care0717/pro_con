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

n= i()
a = []
n.times{
  a<<gets.split()
}
sum = 0
a.each{|x, u|
  if u == "JPY"
    sum += x.to_f
  else
    sum += x.to_f*380000.0
  end
}

puts sum



if t.length == 1 && s.length == 1
    
else


if x <= s[0] && x <= t[0]
  ans << ([s[0], t[0]].max-x)
elsif x >= s[a-1] && x >= t[b-1] 
  ans << (x-[s[a-1], t[b-1]].min)
elsif x >= s[a-1]
  jin = s[a-1]
  t_index = bin_search(t, x)
  if jin < t[t_index]
    ans << x-jin
  else
    ans << [(x-t[t_index]), min_len(jin, t[t_index+1], x)].min 
  end
elsif x >= t[b-1]
  tera = s[a-1]
  s_index = bin_search(s, x)
  if tera < s[s_index]
    ans << x-tera
  else
    ans << [(x-s[s_index]), min_len(tera, s[s_index+1], x)].min 
  end
elsif x <= t[0]
   tera = t[0]
   index = bin_search(s, x)
   if tera > s[index]
    ans << tera-x
   else
    ans << [s[index+1]-x, min_len(tera, s[index], x)].min 
   end
elsif x <= s[0]
    jin = s[0]
   index = bin_search(t, x)
   if jin > t[index]
    ans << jin-x
   else
    ans << [t[index+1]-x, min_len(jin, t[index], x)].min 
   end
 else
    t_i = bin_search(t, x)
    s_i = bin_search(s, x)
    if (s[s_i] < t[t_i] && s[s_i+1] > t[t_i+1]) || (s[s_i] > t[t_i] && s[s_i+1] < t[t_i+1])
      mi = [s[s_i], t[t_i]].min
      ma = [s[s_i+1], t[t_i+1]].max
      ans << [x-min, max-x].min
    else
      one =  [s[s_i], t[t_i]].min
      two =   [s[s_i], t[t_i]].max
      three =  [s[s_i+1], t[t_i+1]].min
      four  =   [s[s_i+1], t[t_i+1]].max
      ans << [x-one, four-x, min_len(two, three, x)].min
    end
  end
