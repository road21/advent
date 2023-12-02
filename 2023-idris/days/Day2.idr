module Day2

import System.File
import Data.List
import Data.Maybe
import Data.String.Parser

data Color = Red | Green | Blue

record Result where
  constructor MkResult
  red : Nat
  green : Nat
  blue : Nat

colorParser : Monad m => ParseT m Color
colorParser = 
  (map (\_ => Green) (string "green")) <|> 
  (map (\_ => Red) (string "red")) <|> 
  (map (\_ => Blue) (string "blue"))

cubesParser : Monad m => ParseT m (Nat, Color)
cubesParser = do
  spaces
  cnt <- natural
  spaces1
  color <- colorParser
  pure (cnt, color)

-- todo: ne list
validateColors : (Maybe Nat, Maybe Nat, Maybe Nat) -> List (Nat, Color) -> Maybe (Maybe Nat, Maybe Nat, Maybe Nat)
-- leads to endless compilation:
-- validateColors _ Nil = Nothing
-- validateColors (Just _, _, _) ((_, Red) :: tail) = Nothing
-- validateColors (Nothing, g, b) ((r, Red) :: tail) = validateColors (Just r, g, b) tail
-- validateColors (_, Just _, _) ((_, Green) :: tail) = Nothing
-- validateColors (r, Nothing, b) ((g, Green) :: tail) = validateColors (r, Just g, b) tail
-- validateColors (_, _, Just _) ((_, Blue) :: tail) = Nothing
-- validateColors (r, g, Nothing) ((b, Blue) :: tail) = validateColors (r, g, Just b) tail
-- so:
validateColors acc l = 
  case l of
    (h :: tail) => 
      case (acc, h) of
        ((Just _, _, _), (_, Red)) => Nothing
        ((_, Just _, _), (_, Green)) => Nothing
        ((_, _, Just _), (_, Blue)) => Nothing
        ((Nothing, g, b), (r, Red)) => validateColors (Just r, g, b) tail
        ((r, Nothing, b), (g, Green)) => validateColors (r, Just g, b) tail
        ((r, g, Nothing), (b, Blue)) => validateColors (r, g, Just b) tail
        _  => Nothing --wtf?
    [] => Just acc

-- todo lazy snd arg
getOrElse : Maybe a -> a -> a
getOrElse (Just a) _ = a
getOrElse Nothing a = a

maybeToList : Maybe a -> List a
maybeToList (Just a) = a :: []
maybeToList Nothing = []

grabParser : Monad m => ParseT m Result
grabParser = do
  cs <- many (cubesParser <* (char ','))
  last <- cubesParser 
  case validateColors (Nothing, Nothing, Nothing) (last :: cs) of
    Just (r, g, b) => pure (MkResult (getOrElse r 0) (getOrElse g 0) (getOrElse b 0))
    Nothing => fail "illegal format"

gameParser : Monad m => (Nat, Nat, Nat) -> ParseT m (Maybe Nat)
gameParser (rMax, gMax, bMax) = do
  _ <- string "Game "
  id <- natural
  _ <- char ':'
  results <- many (grabParser <* (char ';'))
  last <- (grabParser <* (optional (char '\n')))
  pure $ case find (\r => r.red > rMax || r.green > gMax || r.blue > bMax) (last :: results) of
    Just x => Nothing
    Nothing => Just id

gamesParser : (HasIO m, Monad m) => (Nat, Nat, Nat) -> ParseT m (Nat)
gamesParser (rMax, gMax, bMax) = do
  ids <- many (gameParser (rMax, gMax, bMax))
  l <- pure $ join $ map maybeToList ids
  pure (foldl (+) 0 l) 

main : IO ()
main = do
  f <- readFile "input"
  case f of
    (Left err) => putStrLn "file error"
    (Right str) => do
      parsed <- parseT (gamesParser (12, 13, 14)) str
      case parsed of
        (Left err) => putStrLn "parsing error: \{err}"
        (Right (res, n)) => putStrLn "result - \{show res}"
