import           Data.Map   (Map)
import qualified Data.Map   as Map
import           Data.Maybe (catMaybes)

part1 :: [Integer] -> Integer
part1 nums = match * (2020 - match)
  where
    lookup = Map.fromList $ map (\x -> (x, x)) nums
    match = head $ catMaybes $ map (\x -> Map.lookup (2020 - x) lookup) nums

part2 :: [Integer] -> Integer
part2 nums = a * b * (2020 - a - b)
  where
    lookup =
      Map.fromList $
      map (\([x, y]) -> ((2020 - x - y, (x, y)))) $ sequence [nums, nums]
    (a, b) = head $ catMaybes $ map (\x -> Map.lookup x lookup) nums

main = do
  content <- readFile "1.txt"
  let inputLines = lines content
  let nums = map (\x -> read x :: Integer) inputLines
  putStrLn $ "Part 1: " ++ show (part1 nums)
  putStrLn $ "Part 2: " ++ show (part2 nums)
