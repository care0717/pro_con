N,W= gets.split(" ").map(&:to_i)
input_list = []
N.times{
  input_list  << gets.split(" ")
}
$w,$v = input_list.transpose

$dp = Array.new(N).map{Array.new(W,-1)}
def solve_dp(i, j)
  if ($dp[i][j] != -1) then
    return $dp[i][j]
  end
  if (i == N) then
    res = 0
  elsif (j < $w[i]) then
    res = solve_dp(i + 1, j)
  else
    res = [solve_dp(i + 1, j),
        solve_dp(i + 1, j - $w[i]) + $v[i]].max
  end
  return $dp[i][j] = res
end
puts(solve_dp(0,1))
