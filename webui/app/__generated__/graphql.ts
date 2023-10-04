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
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: any;
  PointOfInterestId: any;
  RideId: any;
  RouteId: any;
};

export type Cue = {
  __typename?: "Cue";
  destination: Scalars["String"];
  distanceMeters: Scalars["Float"];
  elevationAscentMeters?: Maybe<Scalars["Float"]>;
  elevationDescentMeters?: Maybe<Scalars["Float"]>;
  origin: Scalars["String"];
};

export type PointOfInterest = {
  __typename?: "PointOfInterest";
  id: Scalars["PointOfInterestId"];
  name: Scalars["String"];
  point: Array<Scalars["Float"]>;
  pointOfInterestType: PointOfInterestType;
};

export enum PointOfInterestType {
  Generic = "GENERIC",
  Hut = "HUT",
  Locality = "LOCALITY",
  RailwayStation = "RAILWAY_STATION",
}

export type Query = {
  __typename?: "Query";
  pointOfInterest?: Maybe<PointOfInterest>;
  pointsOfInterest: Array<PointOfInterest>;
  rides: Array<Ride>;
  route?: Maybe<Route>;
  routes: Array<Route>;
  starredRoutes: Array<Route>;
  viewer: Viewer;
};

export type QueryPointOfInterestArgs = {
  id: Scalars["Int"];
};

export type QueryRouteArgs = {
  id: Scalars["RouteId"];
};

export type Ride = {
  __typename?: "Ride";
  distance: Scalars["Float"];
  finishedAt: Scalars["DateTime"];
  geojson: Scalars["String"];
  id: Scalars["RideId"];
  name: Scalars["String"];
  points: Array<Array<Scalars["Float"]>>;
  startedAt: Scalars["DateTime"];
};

export enum Role {
  Public = "PUBLIC",
  SuperUser = "SUPER_USER",
}

export type Route = {
  __typename?: "Route";
  cues: Array<Cue>;
  distance: Scalars["Float"];
  geojson: Scalars["String"];
  id: Scalars["RouteId"];
  name: Scalars["String"];
  points: Array<Array<Scalars["Float"]>>;
  polyline: Scalars["String"];
};

export type Viewer = {
  __typename?: "Viewer";
  role: Role;
};

export type HomeQueryQueryVariables = Exact<{ [key: string]: never }>;

export type HomeQueryQuery = {
  __typename?: "Query";
  starredRoutes: Array<{
    __typename?: "Route";
    id: any;
    name: string;
    distance: number;
    points: Array<Array<number>>;
  }>;
};

export type RouteQueryQueryVariables = Exact<{
  routeId: Scalars["RouteId"];
}>;

export type RouteQueryQuery = {
  __typename?: "Query";
  route?: {
    __typename?: "Route";
    id: any;
    name: string;
    distance: number;
    points: Array<Array<number>>;
  } | null;
  viewer: { __typename?: "Viewer"; role: Role };
};

export const HomeQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "homeQuery" },
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
} as unknown as DocumentNode<HomeQueryQuery, HomeQueryQueryVariables>;
export const RouteQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "RouteQuery" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "routeId" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "RouteId" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "route" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "routeId" },
                },
              },
            ],
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
          {
            kind: "Field",
            name: { kind: "Name", value: "viewer" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "role" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RouteQueryQuery, RouteQueryQueryVariables>;
