A, B, C = gets.split.map(&:to_i).sort
even_group = [A, B, C].select{|i| i%2==0}
odd_group = [A, B, C].select{|i| i%2==1}
if even_group.length == 1
  odd_group[0] += 1
  odd_group[1] += 1
  a, b, c = even_group.concat(odd_group).sort
  puts (c-a)/2 + (c-b)/2 + 1
elsif even_group.length == 2
  even_group[0] += 1
  even_group[1] += 1
  a, b, c = even_group.concat(odd_group).sort
  puts (c-a)/2 + (c-b)/2 + 1
else
  puts (C-A)/2 + (C-B)/2
end