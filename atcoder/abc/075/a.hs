import Control.Applicative

oneOutOfThree (x:y:z:[])
  | x == y = z
  | x == z = y
  | otherwise = x

main = do
  n <- fmap (read :: String -> Int) . words <$> getLine
  print(oneOutOfThree n)
  

  
