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
  /** URL is a String implementing the [URL Standard](http://url.spec.whatwg.org/) */
  Url: any;
};

export type BikeSpec = {
  __typename?: "BikeSpec";
  frontSuspension: Array<Scalars["Float"]>;
  rearSuspension: Array<Scalars["Float"]>;
  tyreWidth: Array<Scalars["Float"]>;
};

export enum CardinalDirection {
  East = "EAST",
  North = "NORTH",
  South = "SOUTH",
  West = "WEST",
}

export type Cue = {
  __typename?: "Cue";
  destination: Scalars["String"];
  distanceMeters: Scalars["Float"];
  elevationAscentMeters?: Maybe<Scalars["Float"]>;
  elevationDescentMeters?: Maybe<Scalars["Float"]>;
  origin: Scalars["String"];
};

export enum DifficultyRating {
  Black = "BLACK",
  Blue = "BLUE",
  DoubleBlack = "DOUBLE_BLACK",
  Green = "GREEN",
}

export enum Direction {
  Either = "EITHER",
  OnlyAsRouted = "ONLY_AS_ROUTED",
  PrimarlityAsRouted = "PRIMARLITY_AS_ROUTED",
}

export type ExternalRef = {
  __typename?: "ExternalRef";
  canonicalUrl: Scalars["Url"];
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
  description?: Maybe<Scalars["String"]>;
  direction?: Maybe<Direction>;
  distance: Scalars["Float"];
  elevationAscentM?: Maybe<Scalars["Float"]>;
  elevationDescentM?: Maybe<Scalars["Float"]>;
  /** @deprecated use external_ref instead */
  externalCanonicalUrl?: Maybe<Scalars["Url"]>;
  externalRef?: Maybe<ExternalRef>;
  geojson: Scalars["String"];
  id: Scalars["RouteId"];
  idealBike?: Maybe<BikeSpec>;
  minimumBike?: Maybe<BikeSpec>;
  name: Scalars["String"];
  physicalDifficulty?: Maybe<DifficultyRating>;
  points: Array<Array<Scalars["Float"]>>;
  polyline: Scalars["String"];
  scouted?: Maybe<Scouted>;
  technicalDifficulty?: Maybe<DifficultyRating>;
  termini: Array<Terminus>;
};

export enum Scouted {
  No = "NO",
  Partially = "PARTIALLY",
  Yes = "YES",
}

export enum SlopeEnd {
  Downhill = "DOWNHILL",
  Flat = "FLAT",
  Uphill = "UPHILL",
}

export type Terminus = {
  __typename?: "Terminus";
  direction: CardinalDirection;
  distanceFromStart: Scalars["Float"];
  elevationGainFromStart?: Maybe<Scalars["Float"]>;
  point: Array<Scalars["Float"]>;
  slopeEnd?: Maybe<SlopeEnd>;
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
    description?: string | null;
    technicalDifficulty?: DifficultyRating | null;
    physicalDifficulty?: DifficultyRating | null;
    scouted?: Scouted | null;
    direction?: Direction | null;
    externalRef?: { __typename?: "ExternalRef"; canonicalUrl: any } | null;
    minimumBike?: {
      __typename?: "BikeSpec";
      tyreWidth: Array<number>;
      frontSuspension: Array<number>;
      rearSuspension: Array<number>;
    } | null;
    idealBike?: {
      __typename?: "BikeSpec";
      tyreWidth: Array<number>;
      frontSuspension: Array<number>;
      rearSuspension: Array<number>;
    } | null;
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "externalRef" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "canonicalUrl" },
                      },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
                { kind: "Field", name: { kind: "Name", value: "points" } },
                { kind: "Field", name: { kind: "Name", value: "description" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "technicalDifficulty" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "physicalDifficulty" },
                },
                { kind: "Field", name: { kind: "Name", value: "scouted" } },
                { kind: "Field", name: { kind: "Name", value: "direction" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "minimumBike" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "tyreWidth" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "frontSuspension" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rearSuspension" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "idealBike" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "tyreWidth" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "frontSuspension" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rearSuspension" },
                      },
                    ],
                  },
                },
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
