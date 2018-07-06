N, K, Q = gets.split.map(&:to_i)
A = gets.split.map(&:to_i).to_a
for i in 0..(N-1) do
  if (i == 0) then 
    if (A[i] > A[i+1]) then
      next
    else 
      target = A[i]
    end
  elsif (i == N-1)then
    if (A[i-1] < A[i])then
      next
    else
      target = A[i]
    end
  else 
    if (A[i-1] < A[i] && A[i] > A[i+1]) then
      next
    else 
      target = A[i]
    end
  end
  p target
  p [*0..N-1]-A.each_with_index.select{|e, i| e <= target}.map{|e| e[1]} 
end
