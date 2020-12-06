import           Data.List.Split
import qualified Data.Set        as Set

part1 :: [String] -> Int
part1 groups = sum $ map length sets
  where
    sets = map (Set.fromList . (filter (/= '\n'))) groups

part2 :: [String] -> Int
part2 groups = sum $ map allAnswered groups
  where
    allAnswered s =
      length $
      foldl1 Set.intersection $ filter (\x -> (length x) > 0) individuals
      where
        individuals = map Set.fromList $ splitOn "\n" s

main = do
  content <- readFile "6.txt"
  let groups = splitOn "\n\n" content
  putStrLn $ show . part1 $ groups
  putStrLn $ show . part2 $ groups
