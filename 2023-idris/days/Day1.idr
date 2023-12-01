module Day1

import System.File
import Data.List
import Data.String

toDigit : Char -> Maybe Int
toDigit '0' = Just 0
toDigit '1' = Just 1
toDigit '2' = Just 2
toDigit '3' = Just 3
toDigit '4' = Just 4
toDigit '5' = Just 5
toDigit '6' = Just 6
toDigit '7' = Just 7
toDigit '8' = Just 8
toDigit '9' = Just 9
toDigit _ = Nothing

calibrationValue : String -> Maybe Int
calibrationValue str = 
  let arr = filter isDigit (unpack str) in
  case (head' arr, last' arr) of 
    (Just x, Just y) => do 
        i1 <- toDigit x
        i2 <- (toDigit y)
        pure (i1 + i2 * 10)
    (_, _) => Nothing   

calcRes : Either FileError String -> IO ()
calcRes (Left f) = putStrLn "file error"
calcRes (Right str) =
  let combine: String -> Maybe Int -> Maybe Int = \elem, acc => do
    a <- acc
    e <- calibrationValue elem
    pure (a + e) in
  let res: Maybe Int = foldr1By combine calibrationValue ("00" :: (lines str)) in
  case res of 
    (Just res) => putStrLn(show res)
    Nothing => putStrLn("illegal format!")

task1 : IO ()
task1 = do
  putStrLn (show $ filter isDigit (unpack "00"))
  f <- readFile "input"
  calcRes f

main : IO ()
main = task1
