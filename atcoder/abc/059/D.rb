X, Y=gets.split(" ").map(&:to_i)
@dp = Array.new(X).map{Array.new(Y,"")}
@dp[0][0]="op"
@dp[0][1]="op"
@dp[1][0]="op"
@dp[1][1]="op"
@dp[2][0]="sa"
@dp[0][2]="sa"
X.times{|i|
  Y.times{|j|
    if (@dp[i][j] == "") then
      @dp[i-1][j] == ""
    end
  }
}
