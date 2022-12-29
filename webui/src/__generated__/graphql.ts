/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>;
};
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Checkpoint = {
  __typename?: "Checkpoint";
  id: Scalars["Int"];
  name: Scalars["String"];
  point: Point;
};

export type Point = {
  __typename?: "Point";
  lat: Scalars["Float"];
  lng: Scalars["Float"];
};

export type Query = {
  __typename?: "Query";
  checkpoint?: Maybe<Checkpoint>;
  checkpoints: Array<Checkpoint>;
  route?: Maybe<Route>;
  routes: Array<Route>;
  starredRoutes: Array<Route>;
};

export type QueryCheckpointArgs = {
  id: Scalars["Int"];
};

export type QueryRouteArgs = {
  id: Scalars["Int"];
};

export type Route = {
  __typename?: "Route";
  distance: Scalars["Float"];
  geojson: Scalars["String"];
  id: Scalars["Int"];
  name: Scalars["String"];
  points: Array<Array<Scalars["Float"]>>;
};

export type StarredRoutesQueryVariables = Exact<{ [key: string]: never }>;

export type StarredRoutesQuery = {
  __typename?: "Query";
  starredRoutes: Array<{
    __typename?: "Route";
    id: number;
    name: string;
    distance: number;
    points: Array<Array<number>>;
  }>;
};

export const StarredRoutesDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "starredRoutes" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "starredRoutes" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
                { kind: "Field", name: { kind: "Name", value: "points" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<StarredRoutesQuery, StarredRoutesQueryVariables>;
