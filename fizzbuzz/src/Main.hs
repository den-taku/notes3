module Main where

main :: IO ()
main = fizzbuzz

fizzbuzz :: IO ()
fizzbuzz = mapM_ (print . convert) $ take 20 [1..]

convert :: Int -> String
convert n | n `mod` 15 == 0 = "FizzBuzz"
          | n `mod` 5 == 0 = "Buzz"
          | n `mod` 3 == 0 = "Fizz"
          | otherwise = show n
