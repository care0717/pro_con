def s()
  gets()
end
def i()
  gets.to_i
end
def li()
  gets.split.map(&:to_i)
end
def ls()
  gets.split
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end

H, W =li()

def getMovePos(pos)
  res = [[pos[0]+1, pos[1]], [pos[0]-1, pos[1]], [pos[0], pos[1]+1], [pos[0], pos[1]-1]]
  if pos[0] == 0
    res -= [[pos[0]-1, pos[1]]]
  end
  if pos[0] == H-1
    res -= [[pos[0]+1, pos[1]]]
  end
  if pos[1] == 0
    res -= [[pos[0], pos[1]-1]]
  end
  if pos[1] == W-1
    res -= [[pos[0], pos[1]+1]]
  end
  res
end


s = []
H.times{
  s << s()
}
que = [[0, 0]]
visited = []
dist = []
H.times{
  visited << Array.new(W, 0)
  dist << Array.new(W, 0)
}
white_count = 0
H.times{|i|
  W.times{|j|
    if s[i][j] == "."
      white_count += 1
    end
  }
}

visited[0][0] = 1

goal = false
while que.length > 0
  current = que.shift
  movePos = getMovePos(current)
  movePos.each{|m|
    if s[m[0]][m[1]] == "." && visited[m[0]][m[1]] == 0
      visited[m[0]][m[1]] = 1
      que.push(m)
      dist[m[0]][m[1]] = dist[current[0]][current[1]] + 1
    end
    if m[0] == H-1 && m[1] == W-1
      goal = true
      break
    end
  }
end
dist.each{|d|
  puts d.join(" ")
}
if goal && s[0][0] != "#" && s[H-1][W-1] != "#"
  puts white_count - dist[H-1][W-1] -1
else
  puts -1
end



