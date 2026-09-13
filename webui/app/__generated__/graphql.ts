/* eslint-disable */
/** Internal type. DO NOT USE DIRECTLY. */
type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
/** Internal type. DO NOT USE DIRECTLY. */
export type Incremental<T> =
  | T
  | {
      [P in keyof T]?: P extends " $fragmentName" | "__typename" ? T[P] : never;
    };
import { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core";
export type CreatePointOfInterestInput = {
  description?: string | null | undefined;
  name: string;
  point: Array<number>;
  pointOfInterestType: PointOfInterestType;
};

export type CreateTripInput = {
  description?: string | null | undefined;
  name: string;
  rideIds: Array<string>;
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

export enum PointOfInterestType {
  Campsite = "CAMPSITE",
  Generic = "GENERIC",
  Hut = "HUT",
  PublicTransportStop = "PUBLIC_TRANSPORT_STOP",
  WaterSource = "WATER_SOURCE",
}

export enum PointsDetail {
  High = "HIGH",
  Low = "LOW",
  Medium = "MEDIUM",
}

export type QueryRouteFilters = {
  hasAllTags?: Array<string> | null | undefined;
  hasSomeTags?: Array<string> | null | undefined;
  isPublished?: boolean | null | undefined;
};

export type QueryRoutesInput = {
  filters: Array<QueryRouteFilters>;
};

export enum Scouted {
  No = "NO",
  Partially = "PARTIALLY",
  Yes = "YES",
}

export type TripNoteInput = {
  text: string;
  timestamp: string;
};

export type UpdatePointOfInterestInput = {
  description?: string | null | undefined;
  name: string;
  point: Array<number>;
  pointOfInterestId: string;
  pointOfInterestType: PointOfInterestType;
};

export type UpdateTripInput = {
  description?: string | null | undefined;
  isPublished: boolean;
  name: string;
  notes: Array<TripNoteInput>;
  tripId: string;
};

export type UpdateTripMediaInput = {
  mediaIds: Array<string>;
  tripId: string;
};

export type UpdateTripRidesInput = {
  rideIds: Array<string>;
  tripId: string;
};

type ElevationPath_Ride_Fragment = {
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_Ride_Fragment" };

type ElevationPath_Route_Fragment = {
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_Route_Fragment" };

type ElevationPath_TripLeg_Fragment = {
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_TripLeg_Fragment" };

export type ElevationPathFragment =
  | ElevationPath_Ride_Fragment
  | ElevationPath_Route_Fragment
  | ElevationPath_TripLeg_Fragment;

export type ViewerInfoFragment = {
  id: string;
  profile: { username: string };
} & { " $fragmentName"?: "ViewerInfoFragment" };

export type CreatePointOfInterestMutationVariables = Exact<{
  input: CreatePointOfInterestInput;
}>;

export type CreatePointOfInterestMutation = {
  createPointOfInterest: {
    pointOfInterest: { id: string; name: string; slug: string };
  };
};

export type EditPoiFragment = {
  id: string;
  name: string;
  description: string | null;
  point: Array<number>;
  pointOfInterestType: PointOfInterestType;
} & { " $fragmentName"?: "EditPoiFragment" };

export type UpdatePointOfInterestMutationVariables = Exact<{
  input: UpdatePointOfInterestInput;
}>;

export type UpdatePointOfInterestMutation = {
  updatePointOfInterest: {
    pointOfInterest: {
      id: string;
      name: string;
      description: string | null;
      point: Array<number>;
      pointOfInterestType: PointOfInterestType;
    } | null;
  };
};

export type RideItemFragment = {
  id: string;
  date: string;
  tz: string | null;
  distance: number;
  startedAt: string;
  finishedAt: string;
  user: { username: string };
} & { " $fragmentName"?: "RideItemFragment" };

export type RideSummaryFragment = {
  id: string;
  name: string;
  distance: number;
  startedAt: string;
  finishedAt: string;
  tz: string | null;
} & { " $fragmentName"?: "RideSummaryFragment" };

export type RouteItemFragment = ({
  id: string;
  name: string;
  slug: string;
  distance: number;
  elevationAscentM: number;
  elevationDescentM: number;
  isMetaComplete: boolean;
} & { " $fragmentRefs"?: { RouteVitalsFragment: RouteVitalsFragment } }) & {
  " $fragmentName"?: "RouteItemFragment";
};

export type RouteVitalsFragment = {
  distance: number;
  elevationAscentM: number;
  elevationDescentM: number;
} & { " $fragmentName"?: "RouteVitalsFragment" };

export type AllPoIsQueryVariables = Exact<{
  username: string;
}>;

export type AllPoIsQuery = {
  userWithUsername: {
    pointsOfInterest: Array<{
      id: string;
      name: string;
      slug: string;
      pointOfInterestType: PointOfInterestType;
    }>;
  } | null;
};

export type SettingsRideListQueryVariables = Exact<{
  username: string;
}>;

export type SettingsRideListQuery = {
  userWithUsername: {
    rides: Array<{
      id: string;
      name: string;
      startedAt: string;
      finishedAt: string;
      distance: number;
      date: string;
    }>;
  } | null;
};

export type AllRoutesQueryVariables = Exact<{
  username: string;
}>;

export type AllRoutesQuery = {
  userWithUsername: {
    routes: Array<{
      id: string;
      name: string;
      slug: string;
      distance: number;
      elevationAscentM: number;
      elevationDescentM: number;
    }>;
  } | null;
};

export type AllTripsQueryVariables = Exact<{
  username: string;
}>;

export type AllTripsQuery = {
  userWithUsername: {
    trips: Array<{
      id: string;
      name: string;
      year: number;
      isPublished: boolean;
      slug: string;
    }>;
  } | null;
};

export type AllRidesQueryVariables = Exact<{
  username: string;
}>;

export type AllRidesQuery = {
  userWithUsername: {
    rides: Array<{
      id: string;
      name: string;
      startedAt: string;
      finishedAt: string;
      distance: number;
    }>;
  } | null;
};

export type CreateTripMutationVariables = Exact<{
  input: CreateTripInput;
}>;

export type CreateTripMutation = {
  createTrip: {
    trip: {
      id: string;
      name: string;
      slug: string;
      year: number;
      user: { username: string };
    };
  };
};

export type TripMediaFragment = {
  id: string;
  media: Array<{
    id: string;
    path: string;
    createdAt: string;
    capturedAt: string | null;
    imageSizes: { fill600: { webpUrl: string } };
  }>;
} & { " $fragmentName"?: "TripMediaFragment" };

export type TripPoisFragment = { id: string; user: { username: string } } & {
  " $fragmentName"?: "TripPoisFragment";
};

export type TripRidesForPoiQueryVariables = Exact<{
  tripId: string;
}>;

export type TripRidesForPoiQuery = {
  trip: {
    id: string;
    legs: Array<{
      rides: Array<{ id: string; name: string; pointsJson: string }>;
    }>;
  } | null;
};

export type CreateTripPointOfInterestMutationVariables = Exact<{
  input: CreatePointOfInterestInput;
}>;

export type CreateTripPointOfInterestMutation = {
  createPointOfInterest: {
    pointOfInterest: { id: string; name: string; slug: string };
  };
};

export type TripRidesFragment = {
  id: string;
  user: { username: string };
  rides: Array<{
    id: string;
    name: string;
    startedAt: string;
    finishedAt: string;
    distance: number;
  }>;
} & { " $fragmentName"?: "TripRidesFragment" };

export type UpdateTripRidesMutationVariables = Exact<{
  input: UpdateTripRidesInput;
}>;

export type UpdateTripRidesMutation = {
  updateTripRides: {
    trip: { id: string; rides: Array<{ id: string }> } | null;
  };
};

export type EditTripFragment = ({
  id: string;
  name: string;
  description: string | null;
  isPublished: boolean;
  media: Array<{ id: string }>;
  temporalContentBlocks: Array<
    | {
        __typename: "Media";
        contentAt: string;
        mediaId: string;
        imageSizes: { fit1200: { webpUrl: string } };
      }
    | { __typename: "Note"; text: string; contentAt: string }
    | { __typename: "Ride"; name: string; contentAt: string; rideId: string }
  >;
} & {
  " $fragmentRefs"?: {
    TripRidesFragment: TripRidesFragment;
    TripMediaFragment: TripMediaFragment;
    TripPoisFragment: TripPoisFragment;
  };
}) & { " $fragmentName"?: "EditTripFragment" };

export type UpdateTripMutationVariables = Exact<{
  input: UpdateTripInput;
}>;

export type UpdateTripMutation = {
  updateTrip: {
    trip: { id: string; name: string; description: string | null } | null;
  };
};

export type UpdateTripMediaMutationVariables = Exact<{
  input: UpdateTripMediaInput;
}>;

export type UpdateTripMediaMutation = {
  updateTripMedia: { trip: { id: string } | null };
};

export type TripItemFragment = {
  id: string;
  name: string;
  year: number;
  slug: string;
  legs: Array<{ rides: Array<{ startedAt: string }> }>;
  user: { username: string };
} & { " $fragmentName"?: "TripItemFragment" };

export type LoginViewerInfoQueryVariables = Exact<{ [key: string]: never }>;

export type LoginViewerInfoQuery = {
  viewer:
    | ({ id: string; profile: { username: string } } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type PoiQueryQueryVariables = Exact<{
  slug: string;
}>;

export type PoiQueryQuery = {
  pointOfInterestWithSlug:
    | ({
        id: string;
        name: string;
        point: Array<number>;
        description: string | null;
        pointOfInterestType: PointOfInterestType;
        media: Array<{ id: string; point: Array<number> | null }>;
      } & { " $fragmentRefs"?: { EditPoiFragment: EditPoiFragment } })
    | null;
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type PoIsQueryQueryVariables = Exact<{ [key: string]: never }>;

export type PoIsQueryQuery = {
  pointsOfInterest: Array<
    {
      id: string;
      name: string;
      point: Array<number>;
      pointOfInterestType: PointOfInterestType;
    } & { " $fragmentRefs"?: { PoiItemFragment: PoiItemFragment } }
  >;
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type PoiItemFragment = {
  id: string;
  name: string;
  point: Array<number>;
  slug: string;
  pointOfInterestType: PointOfInterestType;
} & { " $fragmentName"?: "PoiItemFragment" };

export type RidesWithDateQueryVariables = Exact<{
  username: string;
  date: string;
  detailLevel: PointsDetail;
}>;

export type RidesWithDateQuery = {
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
  userWithUsername: {
    username: string;
    ridesWithDate: Array<
      { id: string; date: string; tz: string | null; pointsJson: string } & {
        " $fragmentRefs"?: {
          RideSummaryFragment: RideSummaryFragment;
          ElevationPath_Ride_Fragment: ElevationPath_Ride_Fragment;
        };
      }
    >;
  } | null;
};

export type UserProfileQueryQueryVariables = Exact<{
  username: string;
  detailLevel: PointsDetail;
}>;

export type UserProfileQueryQuery = {
  userWithUsername: {
    id: string;
    username: string;
    recentRides: Array<
      { id: string; date: string; pointsJson: string } & {
        " $fragmentRefs"?: { RideItemFragment: RideItemFragment };
      }
    >;
    trips: Array<
      {
        id: string;
        name: string;
        legs: Array<{ rides: Array<{ id: string; pointsJson: string }> }>;
      } & { " $fragmentRefs"?: { TripItemFragment: TripItemFragment } }
    >;
  } | null;
  viewer:
    | ({ id: string } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

type ContentBlock_Media_Fragment = {
  __typename: "Media";
  capturedAt: string | null;
  tz: string | null;
  contentAt: string;
  mediaId: string;
  imageSizes: { fit1600: { webpUrl: string } };
  rides: Array<{ id: string }>;
} & { " $fragmentName"?: "ContentBlock_Media_Fragment" };

type ContentBlock_Note_Fragment = {
  __typename: "Note";
  text: string;
  contentAt: string;
  ride: { id: string } | null;
} & { " $fragmentName"?: "ContentBlock_Note_Fragment" };

type ContentBlock_Ride_Fragment = ({
  __typename: "Ride";
  tz: string | null;
  contentAt: string;
  rideId: string;
} & { " $fragmentRefs"?: { RideItemFragment: RideItemFragment } }) & {
  " $fragmentName"?: "ContentBlock_Ride_Fragment";
};

export type ContentBlockFragment =
  | ContentBlock_Media_Fragment
  | ContentBlock_Note_Fragment
  | ContentBlock_Ride_Fragment;

export type TripQueryQueryVariables = Exact<{
  username: string;
  slug: string;
  detailLevel: PointsDetail;
}>;

export type TripQueryQuery = {
  viewer:
    | ({ id: string } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
  userWithUsername: {
    username: string;
    tripWithSlug:
      | ({
          id: string;
          name: string;
          user: { id: string };
          media: Array<{ id: string; point: Array<number> | null }>;
          legs: Array<
            {
              rides: Array<
                { id: string; pointsJson: string } & {
                  " $fragmentRefs"?: {
                    ElevationPath_Ride_Fragment: ElevationPath_Ride_Fragment;
                  };
                }
              >;
            } & {
              " $fragmentRefs"?: {
                ElevationPath_TripLeg_Fragment: ElevationPath_TripLeg_Fragment;
              };
            }
          >;
          temporalContentBlocks: Array<
            | {
                " $fragmentRefs"?: {
                  ContentBlock_Media_Fragment: ContentBlock_Media_Fragment;
                };
              }
            | {
                " $fragmentRefs"?: {
                  ContentBlock_Note_Fragment: ContentBlock_Note_Fragment;
                };
              }
            | {
                " $fragmentRefs"?: {
                  ContentBlock_Ride_Fragment: ContentBlock_Ride_Fragment;
                };
              }
          >;
        } & { " $fragmentRefs"?: { EditTripFragment: EditTripFragment } })
      | null;
  } | null;
};

export type PublicUsersQueryVariables = Exact<{ [key: string]: never }>;

export type PublicUsersQuery = {
  publicUsers: Array<
    { id: string } & {
      " $fragmentRefs"?: { UserItemFragment: UserItemFragment };
    }
  >;
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type UserItemFragment = { id: string; username: string } & {
  " $fragmentName"?: "UserItemFragment";
};

export type NearbyRoutesInfoFragment = {
  bearing: number;
  nearbyRoutes: Array<{
    delta: { distance: number; bearing: number };
    closestTerminus: {
      bearing: number;
      route: { id: string } & {
        " $fragmentRefs"?: { RouteItemFragment: RouteItemFragment };
      };
    };
  }>;
} & { " $fragmentName"?: "NearbyRoutesInfoFragment" };

export type RouteQueryQueryVariables = Exact<{
  slug: string;
}>;

export type RouteQueryQuery = {
  routeWithSlug:
    | ({
        id: string;
        name: string;
        slug: string;
        tags: Array<string> | null;
        distance: number;
        elevationAscentM: number;
        elevationDescentM: number;
        pointsJson: string;
        description: string | null;
        technicalDifficulty: DifficultyRating | null;
        physicalDifficulty: DifficultyRating | null;
        scouted: Scouted | null;
        direction: Direction | null;
        externalRef: { canonicalUrl: string } | null;
        minimumBike: {
          tyreWidth: Array<number>;
          frontSuspension: Array<number>;
          rearSuspension: Array<number>;
        } | null;
        idealBike: {
          tyreWidth: Array<number>;
          frontSuspension: Array<number>;
          rearSuspension: Array<number>;
        } | null;
        termini: Array<
          {
            bearing: number;
            nearbyRoutes: Array<{
              closestTerminus: { route: { id: string; pointsJson: string } };
            }>;
          } & {
            " $fragmentRefs"?: {
              NearbyRoutesInfoFragment: NearbyRoutesInfoFragment;
            };
          }
        >;
      } & {
        " $fragmentRefs"?: {
          ElevationPath_Route_Fragment: ElevationPath_Route_Fragment;
          RouteVitalsFragment: RouteVitalsFragment;
        };
      })
    | null;
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type HomeQueryQueryVariables = Exact<{
  input: QueryRoutesInput;
}>;

export type HomeQueryQuery = {
  queryRoutes: Array<
    { id: string; samplePoints: Array<Array<number>> } & {
      " $fragmentRefs"?: { RouteItemFragment: RouteItemFragment };
    }
  >;
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type HomeQueryPointOnlyQueryVariables = Exact<{
  input: QueryRoutesInput;
}>;

export type HomeQueryPointOnlyQuery = {
  queryRoutes: Array<{ id: string; pointsJson: string }>;
};

export type TripsQueryQueryVariables = Exact<{ [key: string]: never }>;

export type TripsQueryQuery = {
  publishedTrips: Array<
    {
      id: string;
      name: string;
      legs: Array<{ rides: Array<{ id: string; pointsJson: string }> }>;
    } & { " $fragmentRefs"?: { TripItemFragment: TripItemFragment } }
  >;
  viewer:
    | ({ profile: { id: string; username: string } } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type TripsQueryPointsQueryVariables = Exact<{ [key: string]: never }>;

export type TripsQueryPointsQuery = {
  publishedTrips: Array<{
    id: string;
    legs: Array<{ rides: Array<{ id: string; pointsJson: string }> }>;
  }>;
};

export type ViewerQueryQueryVariables = Exact<{ [key: string]: never }>;

export type ViewerQueryQuery = {
  viewer: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  } | null;
};

export type SettingsQueryVariables = Exact<{ [key: string]: never }>;

export type SettingsQuery = {
  viewer:
    | ({
        rwgpsAuthRequestUrl: string;
        profile: { id: string; username: string; email: string | null };
        rwgpsConnection: {
          id: string;
          rwgpsUserId: number;
          createdAt: string;
          updatedAt: string;
        } | null;
      } & { " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment } })
    | null;
};

export type InitiateRwgpsHistorySyncMutationVariables = Exact<{
  [key: string]: never;
}>;

export type InitiateRwgpsHistorySyncMutation = {
  initiateRwgpsHistorySync: {
    " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
  };
};

export const ElevationPathFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "elevationPath" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "ElevationPath" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "elevationPointsJson" },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "distancePointsJson" },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<ElevationPathFragment, unknown>;
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
export const EditPoiFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "editPOI" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "PointOfInterest" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "description" } },
          { kind: "Field", name: { kind: "Name", value: "point" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "pointOfInterestType" },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<EditPoiFragment, unknown>;
export const RideSummaryFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "rideSummary" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RideSummaryFragment, unknown>;
export const TripRidesFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripRides" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "rides" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "startedAt" } },
                { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<TripRidesFragment, unknown>;
export const TripMediaFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripMedia" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "media" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "path" } },
                { kind: "Field", name: { kind: "Name", value: "createdAt" } },
                { kind: "Field", name: { kind: "Name", value: "capturedAt" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "imageSizes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fill600" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "webpUrl" },
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
} as unknown as DocumentNode<TripMediaFragment, unknown>;
export const TripPoisFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripPois" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<TripPoisFragment, unknown>;
export const EditTripFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "editTrip" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "description" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "tripRides" },
          },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "tripMedia" },
          },
          { kind: "FragmentSpread", name: { kind: "Name", value: "tripPois" } },
          { kind: "Field", name: { kind: "Name", value: "isPublished" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "media" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "temporalContentBlocks" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "__typename" } },
                { kind: "Field", name: { kind: "Name", value: "contentAt" } },
                {
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Note" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "text" } },
                    ],
                  },
                },
                {
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Media" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        alias: { kind: "Name", value: "mediaId" },
                        name: { kind: "Name", value: "id" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "imageSizes" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fit1200" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "webpUrl" },
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
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Ride" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        alias: { kind: "Name", value: "rideId" },
                        name: { kind: "Name", value: "id" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripRides" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "rides" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "startedAt" } },
                { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripMedia" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "media" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "path" } },
                { kind: "Field", name: { kind: "Name", value: "createdAt" } },
                { kind: "Field", name: { kind: "Name", value: "capturedAt" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "imageSizes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fill600" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "webpUrl" },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripPois" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<EditTripFragment, unknown>;
export const TripItemFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "year" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "legs" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "startedAt" },
                      },
                    ],
                  },
                },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<TripItemFragment, unknown>;
export const PoiItemFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "poiItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "PointOfInterest" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "point" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "pointOfInterestType" },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<PoiItemFragment, unknown>;
export const RideItemFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "rideItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "date" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<RideItemFragment, unknown>;
export const ContentBlockFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "contentBlock" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "TemporalContentBlock" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "__typename" } },
          { kind: "Field", name: { kind: "Name", value: "contentAt" } },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Ride" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "rideId" },
                  name: { kind: "Name", value: "id" },
                },
                { kind: "Field", name: { kind: "Name", value: "tz" } },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "rideItem" },
                },
              ],
            },
          },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Media" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "mediaId" },
                  name: { kind: "Name", value: "id" },
                },
                { kind: "Field", name: { kind: "Name", value: "capturedAt" } },
                { kind: "Field", name: { kind: "Name", value: "tz" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "imageSizes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fit1600" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "webpUrl" },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
              ],
            },
          },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Note" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "text" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ride" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "rideItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "date" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<ContentBlockFragment, unknown>;
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
export const RouteVitalsFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeVitals" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RouteVitalsFragment, unknown>;
export const RouteItemFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
          { kind: "Field", name: { kind: "Name", value: "isMetaComplete" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "routeVitals" },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeVitals" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RouteItemFragment, unknown>;
export const NearbyRoutesInfoFragmentDoc = {
  kind: "Document",
  definitions: [
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "nearbyRoutesInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Terminus" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "bearing" } },
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
                              kind: "FragmentSpread",
                              name: { kind: "Name", value: "routeItem" },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeVitals" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
          { kind: "Field", name: { kind: "Name", value: "isMetaComplete" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "routeVitals" },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<NearbyRoutesInfoFragment, unknown>;
export const CreatePointOfInterestDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "CreatePointOfInterest" },
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
              name: { kind: "Name", value: "CreatePointOfInterestInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "createPointOfInterest" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointOfInterest" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
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
} as unknown as DocumentNode<
  CreatePointOfInterestMutation,
  CreatePointOfInterestMutationVariables
>;
export const UpdatePointOfInterestDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "UpdatePointOfInterest" },
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
              name: { kind: "Name", value: "UpdatePointOfInterestInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "updatePointOfInterest" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointOfInterest" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "description" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "point" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "pointOfInterestType" },
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
} as unknown as DocumentNode<
  UpdatePointOfInterestMutation,
  UpdatePointOfInterestMutationVariables
>;
export const AllPoIsDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "AllPOIs" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointsOfInterest" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "pointOfInterestType" },
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
} as unknown as DocumentNode<AllPoIsQuery, AllPoIsQueryVariables>;
export const SettingsRideListDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "SettingsRideList" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "startedAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "finishedAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "distance" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "date" } },
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
} as unknown as DocumentNode<
  SettingsRideListQuery,
  SettingsRideListQueryVariables
>;
export const AllRoutesDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "AllRoutes" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "routes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "distance" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "elevationAscentM" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "elevationDescentM" },
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
} as unknown as DocumentNode<AllRoutesQuery, AllRoutesQueryVariables>;
export const AllTripsDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "AllTrips" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trips" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "year" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "isPublished" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
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
} as unknown as DocumentNode<AllTripsQuery, AllTripsQueryVariables>;
export const AllRidesDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "AllRides" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "startedAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "finishedAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "distance" },
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
} as unknown as DocumentNode<AllRidesQuery, AllRidesQueryVariables>;
export const CreateTripDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "CreateTrip" },
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
              name: { kind: "Name", value: "CreateTripInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "createTrip" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trip" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
                      { kind: "Field", name: { kind: "Name", value: "year" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<CreateTripMutation, CreateTripMutationVariables>;
export const TripRidesForPoiDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TripRidesForPOI" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "tripId" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "TripId" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "trip" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "tripId" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "legs" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rides" },
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
                              name: { kind: "Name", value: "pointsJson" },
                              arguments: [
                                {
                                  kind: "Argument",
                                  name: { kind: "Name", value: "detailLevel" },
                                  value: { kind: "EnumValue", value: "HIGH" },
                                },
                              ],
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
} as unknown as DocumentNode<
  TripRidesForPoiQuery,
  TripRidesForPoiQueryVariables
>;
export const CreateTripPointOfInterestDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "CreateTripPointOfInterest" },
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
              name: { kind: "Name", value: "CreatePointOfInterestInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "createPointOfInterest" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointOfInterest" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      { kind: "Field", name: { kind: "Name", value: "slug" } },
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
} as unknown as DocumentNode<
  CreateTripPointOfInterestMutation,
  CreateTripPointOfInterestMutationVariables
>;
export const UpdateTripRidesDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "UpdateTripRides" },
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
              name: { kind: "Name", value: "UpdateTripRidesInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "updateTripRides" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trip" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rides" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
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
} as unknown as DocumentNode<
  UpdateTripRidesMutation,
  UpdateTripRidesMutationVariables
>;
export const UpdateTripDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "UpdateTrip" },
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
              name: { kind: "Name", value: "UpdateTripInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "updateTrip" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trip" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "description" },
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
} as unknown as DocumentNode<UpdateTripMutation, UpdateTripMutationVariables>;
export const UpdateTripMediaDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "UpdateTripMedia" },
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
              name: { kind: "Name", value: "UpdateTripMediaInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "updateTripMedia" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trip" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
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
} as unknown as DocumentNode<
  UpdateTripMediaMutation,
  UpdateTripMediaMutationVariables
>;
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
export const PoiQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "POIQuery" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "slug" } },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "String" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "pointOfInterestWithSlug" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "slug" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "slug" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "point" } },
                { kind: "Field", name: { kind: "Name", value: "description" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointOfInterestType" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "media" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "point" } },
                    ],
                  },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "editPOI" },
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
      name: { kind: "Name", value: "editPOI" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "PointOfInterest" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "description" } },
          { kind: "Field", name: { kind: "Name", value: "point" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "pointOfInterestType" },
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
} as unknown as DocumentNode<PoiQueryQuery, PoiQueryQueryVariables>;
export const PoIsQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "POIsQuery" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "pointsOfInterest" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "point" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "pointOfInterestType" },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "poiItem" },
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
      name: { kind: "Name", value: "poiItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "PointOfInterest" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "point" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "pointOfInterestType" },
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
} as unknown as DocumentNode<PoIsQueryQuery, PoIsQueryQueryVariables>;
export const RidesWithDateDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "ridesWithDate" },
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
          variable: { kind: "Variable", name: { kind: "Name", value: "date" } },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "IsoDate" },
            },
          },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "detailLevel" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "PointsDetail" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
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
                { kind: "Field", name: { kind: "Name", value: "username" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ridesWithDate" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "date" },
                      value: {
                        kind: "Variable",
                        name: { kind: "Name", value: "date" },
                      },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "date" } },
                      { kind: "Field", name: { kind: "Name", value: "tz" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "pointsJson" },
                        arguments: [
                          {
                            kind: "Argument",
                            name: { kind: "Name", value: "detailLevel" },
                            value: {
                              kind: "Variable",
                              name: { kind: "Name", value: "detailLevel" },
                            },
                          },
                        ],
                      },
                      {
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "rideSummary" },
                      },
                      {
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "elevationPath" },
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
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "rideSummary" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "elevationPath" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "ElevationPath" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "elevationPointsJson" },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "distancePointsJson" },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<RidesWithDateQuery, RidesWithDateQueryVariables>;
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
            name: { kind: "Name", value: "detailLevel" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "PointsDetail" },
            },
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
                      { kind: "Field", name: { kind: "Name", value: "date" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "pointsJson" },
                        arguments: [
                          {
                            kind: "Argument",
                            name: { kind: "Name", value: "detailLevel" },
                            value: {
                              kind: "Variable",
                              name: { kind: "Name", value: "detailLevel" },
                            },
                          },
                        ],
                      },
                      {
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "rideItem" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "trips" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "legs" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "rides" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "pointsJson" },
                                    arguments: [
                                      {
                                        kind: "Argument",
                                        name: {
                                          kind: "Name",
                                          value: "detailLevel",
                                        },
                                        value: {
                                          kind: "Variable",
                                          name: {
                                            kind: "Name",
                                            value: "detailLevel",
                                          },
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
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "tripItem" },
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
                { kind: "Field", name: { kind: "Name", value: "id" } },
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
      name: { kind: "Name", value: "rideItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "date" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "year" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "legs" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "startedAt" },
                      },
                    ],
                  },
                },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
export const TripQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TripQuery" },
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
          variable: { kind: "Variable", name: { kind: "Name", value: "slug" } },
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
            name: { kind: "Name", value: "detailLevel" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "PointsDetail" },
            },
          },
        },
      ],
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
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "viewerInfo" },
                },
              ],
            },
          },
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
                { kind: "Field", name: { kind: "Name", value: "username" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "tripWithSlug" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "slug" },
                      value: {
                        kind: "Variable",
                        name: { kind: "Name", value: "slug" },
                      },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "editTrip" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "user" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "media" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "point" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "legs" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "FragmentSpread",
                              name: { kind: "Name", value: "elevationPath" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "rides" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                  {
                                    kind: "FragmentSpread",
                                    name: {
                                      kind: "Name",
                                      value: "elevationPath",
                                    },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "pointsJson" },
                                    arguments: [
                                      {
                                        kind: "Argument",
                                        name: {
                                          kind: "Name",
                                          value: "detailLevel",
                                        },
                                        value: {
                                          kind: "Variable",
                                          name: {
                                            kind: "Name",
                                            value: "detailLevel",
                                          },
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
                        name: { kind: "Name", value: "temporalContentBlocks" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "FragmentSpread",
                              name: { kind: "Name", value: "contentBlock" },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripRides" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "username" } },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "rides" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "startedAt" } },
                { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
                { kind: "Field", name: { kind: "Name", value: "distance" } },
              ],
            },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripMedia" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "media" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "path" } },
                { kind: "Field", name: { kind: "Name", value: "createdAt" } },
                { kind: "Field", name: { kind: "Name", value: "capturedAt" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "imageSizes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fill600" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "webpUrl" },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "tripPois" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "rideItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Ride" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "date" } },
          { kind: "Field", name: { kind: "Name", value: "tz" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "startedAt" } },
          { kind: "Field", name: { kind: "Name", value: "finishedAt" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "editTrip" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "description" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "tripRides" },
          },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "tripMedia" },
          },
          { kind: "FragmentSpread", name: { kind: "Name", value: "tripPois" } },
          { kind: "Field", name: { kind: "Name", value: "isPublished" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "media" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "temporalContentBlocks" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "__typename" } },
                { kind: "Field", name: { kind: "Name", value: "contentAt" } },
                {
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Note" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "text" } },
                    ],
                  },
                },
                {
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Media" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        alias: { kind: "Name", value: "mediaId" },
                        name: { kind: "Name", value: "id" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "imageSizes" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fit1200" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "webpUrl" },
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
                  kind: "InlineFragment",
                  typeCondition: {
                    kind: "NamedType",
                    name: { kind: "Name", value: "Ride" },
                  },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        alias: { kind: "Name", value: "rideId" },
                        name: { kind: "Name", value: "id" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "elevationPath" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "ElevationPath" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "elevationPointsJson" },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "distancePointsJson" },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "contentBlock" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "TemporalContentBlock" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "__typename" } },
          { kind: "Field", name: { kind: "Name", value: "contentAt" } },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Ride" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "rideId" },
                  name: { kind: "Name", value: "id" },
                },
                { kind: "Field", name: { kind: "Name", value: "tz" } },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "rideItem" },
                },
              ],
            },
          },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Media" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "mediaId" },
                  name: { kind: "Name", value: "id" },
                },
                { kind: "Field", name: { kind: "Name", value: "capturedAt" } },
                { kind: "Field", name: { kind: "Name", value: "tz" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "imageSizes" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fit1600" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "webpUrl" },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
              ],
            },
          },
          {
            kind: "InlineFragment",
            typeCondition: {
              kind: "NamedType",
              name: { kind: "Name", value: "Note" },
            },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "text" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ride" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
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
} as unknown as DocumentNode<TripQueryQuery, TripQueryQueryVariables>;
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
          variable: { kind: "Variable", name: { kind: "Name", value: "slug" } },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "String" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "routeWithSlug" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "slug" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "slug" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                { kind: "Field", name: { kind: "Name", value: "slug" } },
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
                              name: { kind: "Name", value: "closestTerminus" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
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
                                          name: {
                                            kind: "Name",
                                            value: "pointsJson",
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
                        kind: "FragmentSpread",
                        name: { kind: "Name", value: "nearbyRoutesInfo" },
                      },
                    ],
                  },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "elevationPath" },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "routeVitals" },
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
      name: { kind: "Name", value: "routeVitals" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
          { kind: "Field", name: { kind: "Name", value: "isMetaComplete" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "routeVitals" },
          },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "nearbyRoutesInfo" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Terminus" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "bearing" } },
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
                              kind: "FragmentSpread",
                              name: { kind: "Name", value: "routeItem" },
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
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "elevationPath" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "ElevationPath" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "elevationPointsJson" },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "distancePointsJson" },
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "samplePoints" },
                },
                {
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "routeItem" },
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
      name: { kind: "Name", value: "routeVitals" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
        ],
      },
    },
    {
      kind: "FragmentDefinition",
      name: { kind: "Name", value: "routeItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Route" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          { kind: "Field", name: { kind: "Name", value: "distance" } },
          { kind: "Field", name: { kind: "Name", value: "elevationAscentM" } },
          { kind: "Field", name: { kind: "Name", value: "elevationDescentM" } },
          { kind: "Field", name: { kind: "Name", value: "isMetaComplete" } },
          {
            kind: "FragmentSpread",
            name: { kind: "Name", value: "routeVitals" },
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
export const TripsQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TripsQuery" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "publishedTrips" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "legs" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rides" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "pointsJson" },
                              arguments: [
                                {
                                  kind: "Argument",
                                  name: { kind: "Name", value: "detailLevel" },
                                  value: { kind: "EnumValue", value: "LOW" },
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
                  kind: "FragmentSpread",
                  name: { kind: "Name", value: "tripItem" },
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
                  kind: "Field",
                  name: { kind: "Name", value: "profile" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
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
      name: { kind: "Name", value: "tripItem" },
      typeCondition: {
        kind: "NamedType",
        name: { kind: "Name", value: "Trip" },
      },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          { kind: "Field", name: { kind: "Name", value: "id" } },
          { kind: "Field", name: { kind: "Name", value: "name" } },
          { kind: "Field", name: { kind: "Name", value: "year" } },
          { kind: "Field", name: { kind: "Name", value: "slug" } },
          {
            kind: "Field",
            name: { kind: "Name", value: "legs" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rides" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "startedAt" },
                      },
                    ],
                  },
                },
              ],
            },
          },
          {
            kind: "Field",
            name: { kind: "Name", value: "user" },
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
} as unknown as DocumentNode<TripsQueryQuery, TripsQueryQueryVariables>;
export const TripsQueryPointsDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TripsQueryPoints" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "publishedTrips" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "legs" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rides" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "pointsJson" },
                              arguments: [
                                {
                                  kind: "Argument",
                                  name: { kind: "Name", value: "detailLevel" },
                                  value: { kind: "EnumValue", value: "MEDIUM" },
                                },
                              ],
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
} as unknown as DocumentNode<
  TripsQueryPointsQuery,
  TripsQueryPointsQueryVariables
>;
export const ViewerQueryDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "viewerQuery" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
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
} as unknown as DocumentNode<ViewerQueryQuery, ViewerQueryQueryVariables>;
export const SettingsDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "settings" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
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
                {
                  kind: "Field",
                  name: { kind: "Name", value: "profile" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "username" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "email" } },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rwgpsConnection" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rwgpsUserId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "createdAt" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "updatedAt" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rwgpsAuthRequestUrl" },
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
} as unknown as DocumentNode<SettingsQuery, SettingsQueryVariables>;
export const InitiateRwgpsHistorySyncDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "initiateRwgpsHistorySync" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "initiateRwgpsHistorySync" },
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
  InitiateRwgpsHistorySyncMutation,
  InitiateRwgpsHistorySyncMutationVariables
>;
