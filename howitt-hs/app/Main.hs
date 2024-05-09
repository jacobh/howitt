module Main where

import Amazonka.DynamoDB as DynamoDB
import qualified Data.Text as T

-- testQuery :: Text -> Query


tableName :: T.Text
tableName = T.pack "howitt"

testQuery :: Query
testQuery = newQuery tableName

-- queryString :: BufferCodec from to state -> c
-- queryString = testQuery . toJSON . encode

main :: IO ()
main = putStrLn "hello"
