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

calcRes : Either FileError String -> (String -> Maybe Int) -> IO ()
calcRes (Left err) f = putStrLn "file error"
calcRes (Right str) f =
  let combine: String -> Maybe Int -> Maybe Int = \elem, acc => do
    a <- acc
    e <- f elem
    pure (a + e) in
  let res: Maybe Int = foldr combine (Just 0) (lines str) in
  case res of
    (Just res) => putStrLn(show res)
    Nothing => putStrLn("illegal format!")

calibrationValue1 : String -> Maybe Int
calibrationValue1 str =
  let arr = filter isDigit (unpack str) in
  case (head' arr, last' arr) of
    (Just x, Just y) => do
        i1 <- toDigit x
        i2 <- (toDigit y)
        pure (i1 + i2 * 10)
    (_, _) => Nothing

digitsStrs : (List (List Char, Int))
digitsStrs =
  let init = ("one", 1) :: ("two", 2) :: ("three", 3) :: ("four", 4) :: ("five", 5) :: ("six", 6) :: ("seven", 7) :: ("eight", 8) :: ("nine", 9) :: Nil in
  map (\(x, y) => (unpack x, y)) init

digitsStrsR : (List (List Char, Int))
digitsStrsR =
  map (\(x, y) => (reverse x, y)) digitsStrs

check : List (Char) -> (List (List Char, Int)) -> Maybe Int
check (h :: tail) voc =
  case (toDigit h) of
    (Just n) => Just n
    Nothing =>
      let res = find (\(x, _) => isPrefixOf x (h :: tail)) voc in
      case res of
        (Just (_, res)) => Just(res)
        Nothing => check tail voc

check _ _ = Nothing

calibrationValue2 : String -> Maybe Int
calibrationValue2 str =
  let u = unpack str in
  case (check u digitsStrs, check (reverse u) digitsStrsR) of
    (Just x, Just y) => Just (x * 10 + y)
    _ => Nothing

main : IO ()
main = do
  f <- readFile "input"
  calcRes f calibrationValue2
