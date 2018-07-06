N = gets.to_i
primeNumbers = []
(1..1555).each do |i|
  next if i == 1

  if i == 2
    primeNumbers.push(i)
    next
  end

  judge = true
  primeNumbers.each do |number|
      if i % number == 0
        judge = false;
        break;
      end
  end
  primeNumbers.push(i) if judge
end
primeNumbers =  primeNumbers.select{|i| i%5 == 1}
puts primeNumbers[0..N-1].join(" ")

