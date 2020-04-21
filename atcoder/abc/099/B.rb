a, b = gets.split.map(&:to_i)

c = b-a
for i in 1..1000 do
  j = i+1
    if j*(j+1)/2 - i*(i+1)/2 == c && i*(i+1)/2 > a && j*(j+1)/2 > b then
      puts i*(i+1)/2 - a
    end

end
