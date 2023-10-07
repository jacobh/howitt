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
export type MakeEmpty<
  T extends { [key: string]: unknown },
  K extends keyof T
> = { [_ in K]?: never };
export type Incremental<T> =
  | T
  | {
      [P in keyof T]?: P extends " $fragmentName" | "__typename" ? T[P] : never;
    };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string };
  String: { input: string; output: string };
  Boolean: { input: boolean; output: boolean };
  Int: { input: number; output: number };
  Float: { input: number; output: number };
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: any; output: any };
  PhotoId: { input: any; output: any };
  PointOfInterestId: { input: any; output: any };
  RideId: { input: any; output: any };
  RouteId: { input: any; output: any };
  /** URL is a String implementing the [URL Standard](http://url.spec.whatwg.org/) */
  Url: { input: any; output: any };
};

export type BikeSpec = {
  __typename?: "BikeSpec";
  frontSuspension: Array<Scalars["Float"]["output"]>;
  rearSuspension: Array<Scalars["Float"]["output"]>;
  tyreWidth: Array<Scalars["Float"]["output"]>;
};

export enum CardinalDirection {
  East = "EAST",
  North = "NORTH",
  South = "SOUTH",
  West = "WEST",
}

export type Cue = {
  __typename?: "Cue";
  destination: Scalars["String"]["output"];
  distanceMeters: Scalars["Float"]["output"];
  elevationAscentMeters?: Maybe<Scalars["Float"]["output"]>;
  elevationDescentMeters?: Maybe<Scalars["Float"]["output"]>;
  origin: Scalars["String"]["output"];
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
  canonicalUrl: Scalars["Url"]["output"];
};

export type NearbyRoute = {
  __typename?: "NearbyRoute";
  delta: PointDelta;
  terminus: Terminus;
};

export type Photo = {
  __typename?: "Photo";
  caption?: Maybe<Scalars["String"]["output"]>;
  id: Scalars["PhotoId"]["output"];
  url: Scalars["Url"]["output"];
};

export type PointDelta = {
  __typename?: "PointDelta";
  bearing: Scalars["Float"]["output"];
  distance: Scalars["Float"]["output"];
  elevationGain?: Maybe<Scalars["Float"]["output"]>;
};

export type PointOfInterest = {
  __typename?: "PointOfInterest";
  id: Scalars["PointOfInterestId"]["output"];
  name: Scalars["String"]["output"];
  point: Array<Scalars["Float"]["output"]>;
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
  publishedRoutes: Array<Route>;
  rides: Array<Ride>;
  route?: Maybe<Route>;
  routes: Array<Route>;
  starredRoutes: Array<Route>;
  viewer: Viewer;
};

export type QueryPointOfInterestArgs = {
  id: Scalars["Int"]["input"];
};

export type QueryRouteArgs = {
  id: Scalars["RouteId"]["input"];
};

export type Ride = {
  __typename?: "Ride";
  distance: Scalars["Float"]["output"];
  finishedAt: Scalars["DateTime"]["output"];
  id: Scalars["RideId"]["output"];
  name: Scalars["String"]["output"];
  points: Array<Array<Scalars["Float"]["output"]>>;
  startedAt: Scalars["DateTime"]["output"];
};

export enum Role {
  Public = "PUBLIC",
  SuperUser = "SUPER_USER",
}

export type Route = {
  __typename?: "Route";
  cues: Array<Cue>;
  description?: Maybe<Scalars["String"]["output"]>;
  direction?: Maybe<Direction>;
  distance: Scalars["Float"]["output"];
  distancePoints: Array<Scalars["Float"]["output"]>;
  elevationAscentM?: Maybe<Scalars["Float"]["output"]>;
  elevationDescentM?: Maybe<Scalars["Float"]["output"]>;
  elevationPoints: Array<Scalars["Float"]["output"]>;
  externalRef?: Maybe<ExternalRef>;
  id: Scalars["RouteId"]["output"];
  idealBike?: Maybe<BikeSpec>;
  minimumBike?: Maybe<BikeSpec>;
  name: Scalars["String"]["output"];
  photos: Array<Photo>;
  physicalDifficulty?: Maybe<DifficultyRating>;
  points: Array<Array<Scalars["Float"]["output"]>>;
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
  distanceFromStart: Scalars["Float"]["output"];
  elevationGainFromStart?: Maybe<Scalars["Float"]["output"]>;
  nearbyRoutes: Array<NearbyRoute>;
  point: Array<Scalars["Float"]["output"]>;
  route: Route;
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
  routeId: Scalars["RouteId"]["input"];
}>;

export type RouteQueryQuery = {
  __typename?: "Query";
  route?: {
    __typename?: "Route";
    id: any;
    name: string;
    distance: number;
    points: Array<Array<number>>;
    elevationPoints: Array<number>;
    distancePoints: Array<number>;
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
    photos: Array<{
      __typename?: "Photo";
      id: any;
      url: any;
      caption?: string | null;
    }>;
    termini: Array<{
      __typename?: "Terminus";
      direction: CardinalDirection;
      nearbyRoutes: Array<{
        __typename?: "NearbyRoute";
        delta: {
          __typename?: "PointDelta";
          distance: number;
          bearing: number;
          elevationGain?: number | null;
        };
        terminus: {
          __typename?: "Terminus";
          direction: CardinalDirection;
          route: {
            __typename?: "Route";
            id: any;
            name: string;
            points: Array<Array<number>>;
          };
        };
      }>;
    }>;
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "elevationPoints" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "distancePoints" },
                },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "photos" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "url" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "caption" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "termini" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "direction" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "nearbyRoutes" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "delta" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "distance" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "bearing" },
                                  },
                                  {
                                    kind: "Field",
                                    name: {
                                      kind: "Name",
                                      value: "elevationGain",
                                    },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "terminus" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "direction" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "route" },
                                    selectionSet: {
                                      kind: "SelectionSet",
                                      selections: [
                                        {
                                          kind: "Field",
                                          name: { kind: "Name", value: "id" },
                                        },
                                        {
                                          kind: "Field",
                                          name: { kind: "Name", value: "name" },
                                        },
                                        {
                                          kind: "Field",
                                          name: {
                                            kind: "Name",
                                            value: "points",
                                          },
                                        },
                                      ],
                                    },
                                  },
                                ],
                              },
                            },
                          ],
                        },
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
