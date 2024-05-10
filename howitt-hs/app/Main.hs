{-# LANGUAGE AllowAmbiguousTypes #-}
{-# LANGUAGE DataKinds #-}
{-# LANGUAGE DuplicateRecordFields #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE NoMonomorphismRestriction #-}

module Main where

-- import Amazonka
import Amazonka.DynamoDB as DynamoDB
import Data.Function
import Data.Generics.Internal.VL.Lens
import Data.Generics.Product
import Data.HashMap.Strict qualified as HashMap
import Data.Text qualified as T

queryPk :: T.Text -> Query
queryPk pk = newQuery (T.pack "howitt")
    & field @"keyConditionExpression" .~ Just (T.pack "#pk = :pk")
    & field @"expressionAttributeNames" .~ Just (HashMap.fromList [(T.pack "#pk", T.pack "pk")])
    & field @"expressionAttributeValues" .~ Just (HashMap.fromList [(T.pack ":pk", DynamoDB.S pk)])

main :: IO ()
main = putStrLn "hello"
