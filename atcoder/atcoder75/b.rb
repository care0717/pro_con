def calc(arr, i, j)
  holi = [-1,0,1]
  vert = [-1,0,1]
  sum = 0
  holi.each{|h|
    vert.each{|v|
      if i+h >= 0 && j+v >= 0 && i+h < H
        sum +=1 if arr[i+h][j+v] == "#"
      end
    }
  }
  sum.to_s
end


H, W = gets.split.map(&:to_i)
arr = []
H.times{arr <<  gets[0..(W-1)] }
H.times{|i|
  W.times{ |j|
    arr[i][j] = calc(arr, i, j) if arr[i][j] == '.'
  }
}
puts arr

