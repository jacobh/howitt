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
  UserId: { input: any; output: any };
};

export type BikeSpec = {
  __typename?: "BikeSpec";
  frontSuspension: Array<Scalars["Float"]["output"]>;
  rearSuspension: Array<Scalars["Float"]["output"]>;
  tyreWidth: Array<Scalars["Float"]["output"]>;
};

export type Cue = {
  __typename?: "Cue";
  destination: Scalars["String"]["output"];
  distanceMeters: Scalars["Float"]["output"];
  elevationAscentMeters: Scalars["Float"]["output"];
  elevationDescentMeters: Scalars["Float"]["output"];
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
  closestTerminus: Terminus;
  closestTerminusDelta: PointDelta;
  delta: PointDelta;
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
  elevationGain: Scalars["Float"]["output"];
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
  publicUsers: Array<UserProfile>;
  queryRoutes: Array<Route>;
  rides: Array<Ride>;
  route?: Maybe<Route>;
  routes: Array<Route>;
  starredRoutes: Array<Route>;
  userWithUsername?: Maybe<UserProfile>;
  viewer?: Maybe<Viewer>;
};

export type QueryPointOfInterestArgs = {
  id: Scalars["Int"]["input"];
};

export type QueryQueryRoutesArgs = {
  input: QueryRoutesInput;
};

export type QueryRouteArgs = {
  id: Scalars["RouteId"]["input"];
};

export type QueryUserWithUsernameArgs = {
  username: Scalars["String"]["input"];
};

export type QueryRouteFilters = {
  hasAllTags?: InputMaybe<Array<Scalars["String"]["input"]>>;
  hasSomeTags?: InputMaybe<Array<Scalars["String"]["input"]>>;
  isPublished?: InputMaybe<Scalars["Boolean"]["input"]>;
};

export type QueryRoutesInput = {
  filters: Array<QueryRouteFilters>;
};

export type Ride = {
  __typename?: "Ride";
  distance: Scalars["Float"]["output"];
  finishedAt: Scalars["DateTime"]["output"];
  id: Scalars["RideId"]["output"];
  name: Scalars["String"]["output"];
  points: Array<Array<Scalars["Float"]["output"]>>;
  pointsJson: Scalars["String"]["output"];
  startedAt: Scalars["DateTime"]["output"];
};

export type RidePointsArgs = {
  pointsPerKm: Scalars["Int"]["input"];
};

export type RidePointsJsonArgs = {
  pointsPerKm: Scalars["Int"]["input"];
};

export type Route = {
  __typename?: "Route";
  cues: Array<Cue>;
  description?: Maybe<Scalars["String"]["output"]>;
  direction?: Maybe<Direction>;
  distance: Scalars["Float"]["output"];
  distancePoints: Array<Scalars["Float"]["output"]>;
  elevationAscentM: Scalars["Float"]["output"];
  elevationDescentM: Scalars["Float"]["output"];
  elevationPoints: Array<Scalars["Float"]["output"]>;
  externalRef?: Maybe<ExternalRef>;
  id: Scalars["RouteId"]["output"];
  idealBike?: Maybe<BikeSpec>;
  isMetaComplete: Scalars["Boolean"]["output"];
  minimumBike?: Maybe<BikeSpec>;
  name: Scalars["String"]["output"];
  photos: Array<Photo>;
  physicalDifficulty?: Maybe<DifficultyRating>;
  points: Array<Array<Scalars["Float"]["output"]>>;
  pointsCount: Scalars["Int"]["output"];
  pointsJson: Scalars["String"]["output"];
  samplePoints: Array<Array<Scalars["Float"]["output"]>>;
  samplePointsCount: Scalars["Int"]["output"];
  scouted?: Maybe<Scouted>;
  tags?: Maybe<Array<Scalars["String"]["output"]>>;
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
  bearing: Scalars["Float"]["output"];
  distanceFromStart: Scalars["Float"]["output"];
  elevationGainFromStart: Scalars["Float"]["output"];
  end: TerminusEnd;
  nearbyRoutes: Array<NearbyRoute>;
  point: Array<Scalars["Float"]["output"]>;
  route: Route;
  slopeEnd: SlopeEnd;
};

export enum TerminusEnd {
  End = "END",
  Start = "START",
}

export type UserProfile = {
  __typename?: "UserProfile";
  id: Scalars["UserId"]["output"];
  recentRides: Array<Ride>;
  username: Scalars["String"]["output"];
};

export type Viewer = {
  __typename?: "Viewer";
  id: Scalars["String"]["output"];
  profile: UserProfile;
};

export type ViewerInfoFragment = {
  __typename?: "Viewer";
  id: string;
  profile: { __typename?: "UserProfile"; username: string };
} & { " $fragmentName"?: "ViewerInfoFragment" };

export type LoginViewerInfoQueryVariables = Exact<{ [key: string]: never }>;

export type LoginViewerInfoQuery = {
  __typename?: "Query";
  viewer?:
    | ({
        __typename?: "Viewer";
        id: string;
        profile: { __typename?: "UserProfile"; username: string };
      } & { " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment } })
    | null;
};

export type UserProfileQueryQueryVariables = Exact<{
  username: Scalars["String"]["input"];
  pointsPerKm: Scalars["Int"]["input"];
}>;

export type UserProfileQueryQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    id: any;
    username: string;
    recentRides: Array<{
      __typename?: "Ride";
      id: any;
      finishedAt: any;
      pointsJson: string;
    }>;
  } | null;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type PublicUsersQueryVariables = Exact<{ [key: string]: never }>;

export type PublicUsersQuery = {
  __typename?: "Query";
  publicUsers: Array<
    { __typename?: "UserProfile"; id: any } & {
      " $fragmentRefs"?: { UserItemFragment: UserItemFragment };
    }
  >;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type UserItemFragment = {
  __typename?: "UserProfile";
  id: any;
  username: string;
} & { " $fragmentName"?: "UserItemFragment" };

export type RouteQueryQueryVariables = Exact<{
  routeId: Scalars["RouteId"]["input"];
}>;

export type RouteQueryQuery = {
  __typename?: "Query";
  route?: {
    __typename?: "Route";
    id: any;
    name: string;
    tags?: Array<string> | null;
    distance: number;
    elevationAscentM: number;
    elevationDescentM: number;
    pointsJson: string;
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
      bearing: number;
      nearbyRoutes: Array<{
        __typename?: "NearbyRoute";
        delta: {
          __typename?: "PointDelta";
          distance: number;
          bearing: number;
          elevationGain: number;
        };
        closestTerminus: {
          __typename?: "Terminus";
          bearing: number;
          route: {
            __typename?: "Route";
            id: any;
            name: string;
            pointsJson: string;
            distance: number;
            elevationAscentM: number;
            elevationDescentM: number;
          };
        };
      }>;
    }>;
  } | null;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type HomeQueryQueryVariables = Exact<{
  input: QueryRoutesInput;
}>;

export type HomeQueryQuery = {
  __typename?: "Query";
  queryRoutes: Array<{
    __typename?: "Route";
    id: any;
    name: string;
    distance: number;
    isMetaComplete: boolean;
    elevationAscentM: number;
    elevationDescentM: number;
    samplePoints: Array<Array<number>>;
  }>;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type HomeQueryPointOnlyQueryVariables = Exact<{
  input: QueryRoutesInput;
}>;

export type HomeQueryPointOnlyQuery = {
  __typename?: "Query";
  queryRoutes: Array<{ __typename?: "Route"; id: any; pointsJson: string }>;
};

export const ViewerInfoFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<ViewerInfoFragment, unknown>;
export const UserItemFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "userItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "UserProfile" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "username" } },
        ],
      },
    },
  ],
} as unknown as DocumentNode<UserItemFragment, unknown>;
export const LoginViewerInfoDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "LoginViewerInfo" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "viewer" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "profile" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "username" },
                      },
                    ],
                  },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<
  LoginViewerInfoQuery,
  LoginViewerInfoQueryVariables
>;
export const UserProfileQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "UserProfileQuery" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "username" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "String" },
            },
          },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "pointsPerKm" },
          },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "Int" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "userWithUsername" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "username" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "username" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "username" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "recentRides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "finishedAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "pointsJson" },
                        arguments: [
                          {
                            kind: "Argument",
                            name: { kind: "Name", value: "pointsPerKm" },
                            value: {
                              kind: "Variable",
                              name: { kind: "Name", value: "pointsPerKm" },
                            },
                          },
                        ],
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
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<
  UserProfileQueryQuery,
  UserProfileQueryQueryVariables
>;
export const PublicUsersDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "publicUsers" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "publicUsers" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "userItem" },
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
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "userItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "UserProfile" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "username" } },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<PublicUsersQuery, PublicUsersQueryVariables>;
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
                { kind: "Field", name: { kind: "Name", value: "tags" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "elevationAscentM" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "elevationDescentM" },
                },
                { kind: "Field", name: { kind: "Name", value: "pointsJson" } },
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
                        name: { kind: "Name", value: "bearing" },
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
                              name: { kind: "Name", value: "closestTerminus" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "bearing" },
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
                                            value: "pointsJson",
                                          },
                                        },
                                        {
                                          kind: "Field",
                                          name: {
                                            kind: "Name",
                                            value: "distance",
                                          },
                                        },
                                        {
                                          kind: "Field",
                                          name: {
                                            kind: "Name",
                                            value: "elevationAscentM",
                                          },
                                        },
                                        {
                                          kind: "Field",
                                          name: {
                                            kind: "Name",
                                            value: "elevationDescentM",
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
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RouteQueryQuery, RouteQueryQueryVariables>;
export const HomeQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "homeQuery" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "QueryRoutesInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "queryRoutes" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "isMetaComplete" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "elevationAscentM" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "elevationDescentM" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "samplePoints" },
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
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "viewerInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Viewer" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "profile" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<HomeQueryQuery, HomeQueryQueryVariables>;
export const HomeQueryPointOnlyDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "homeQueryPointOnly" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "QueryRoutesInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "queryRoutes" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "pointsJson" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<
  HomeQueryPointOnlyQuery,
  HomeQueryPointOnlyQueryVariables
>;
