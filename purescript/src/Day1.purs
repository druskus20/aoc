module Day1 where

import Data.List
import Data.Tuple
import Prelude

import Data.Filterable (filterMap)
import Data.List (fromFoldable, findMap, List, elemIndex, index)
import Data.Maybe (Maybe(..), fromMaybe)
import Effect (Effect)
import Effect.Console (log)
import Util.Input (readInputLines)
import Util.Parse (parseInt10)

main :: Effect Unit
main = do
  entries <- fromFoldable <$> filterMap parseInt10 <$> readInputLines "input/2020/day1.txt"
  let year = 2020

  -- Part 1
  case findPair year entries of 
      Just (a:b:Nil) -> log $ "First number: " <> show a <> ", Second number: " <> show b <> ", Product: " <> show (a * b)
      _ -> log $ "There's been a terrible mistake"

  -- Part 2 
  case findTrio year entries of 
      _ -> log $ "There's been a terrible mistake"


findPair :: Int -> List Int -> Maybe (List Int)
findPair _ Nil = Nothing
findPair sum (first:cons) = 
 case (elemIndex (sum-first) cons) of 
     Just ind -> Just $ fromMaybe 0 (index cons ind) : first : Nil
     Nothing -> findPair sum cons


findTrio :: Int -> List Int -> Maybe Int
findTrio _ Nil = Nothing
findTrio sum (first:cons) = Nothing

solve_simple :: Int -> List Int -> Maybe Int
solve_simple sum list = find (\a -> a  == sum) list

solve :: Int -> Array Int -> Maybe (Array Int)
solve sum list = find (\(Tuple a b) -> a + b  == sum) (cartesianProduct2 list)

cartesianProduct2 :: Array Int -> Array (Tuple Int Int)
cartesianProduct2 list = list >>= (\x -> list >>= (\y -> (Tuple x y)))
