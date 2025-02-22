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
  K extends keyof T,
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
  IsoDate: { input: any; output: any };
  MediaId: { input: any; output: any };
  PointOfInterestId: { input: any; output: any };
  RideId: { input: any; output: any };
  RouteId: { input: any; output: any };
  TripId: { input: any; output: any };
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any };
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

export type CreatePointOfInterestInput = {
  description?: InputMaybe<Scalars["String"]["input"]>;
  name: Scalars["String"]["input"];
  point: Array<Scalars["Float"]["input"]>;
  pointOfInterestType: PointOfInterestType;
};

export type CreatePointOfInterestOutput = {
  __typename?: "CreatePointOfInterestOutput";
  pointOfInterest: PointOfInterest;
};

export type CreateTripInput = {
  description?: InputMaybe<Scalars["String"]["input"]>;
  name: Scalars["String"]["input"];
  rideIds: Array<Scalars["RideId"]["input"]>;
};

export type CreateTripOutput = {
  __typename?: "CreateTripOutput";
  trip: Trip;
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

export type ElevationPath = {
  /** Array of distance points */
  distancePoints: Array<Scalars["Float"]["output"]>;
  /** Array of distance points */
  distancePointsJson: Scalars["String"]["output"];
  /** Array of elevation points */
  elevationPoints: Array<Scalars["Float"]["output"]>;
  /** Array of elevation points */
  elevationPointsJson: Scalars["String"]["output"];
};

export type ExternalRef = {
  __typename?: "ExternalRef";
  canonicalUrl: Scalars["Url"]["output"];
};

export enum ImageMode {
  Fill = "FILL",
  Fit = "FIT",
}

export type ImageSize = {
  __typename?: "ImageSize";
  height: Scalars["Int"]["output"];
  jpegUrl: Scalars["String"]["output"];
  mode: ImageMode;
  webpUrl: Scalars["String"]["output"];
  width: Scalars["Int"]["output"];
};

export type ImageSizes = {
  __typename?: "ImageSizes";
  fill300: ImageSize;
  fill600: ImageSize;
  fit800: ImageSize;
  fit1200: ImageSize;
  fit1600: ImageSize;
  fit2000: ImageSize;
  fit2400: ImageSize;
};

export type Media = TemporalContentBlock & {
  __typename?: "Media";
  capturedAt?: Maybe<Scalars["DateTime"]["output"]>;
  contentAt: Scalars["DateTime"]["output"];
  createdAt: Scalars["DateTime"]["output"];
  id: Scalars["MediaId"]["output"];
  imageSizes: ImageSizes;
  path: Scalars["String"]["output"];
  point?: Maybe<Array<Scalars["Float"]["output"]>>;
  rides: Array<Ride>;
  tz?: Maybe<Scalars["String"]["output"]>;
  user: UserProfile;
};

export type MediaTarget = {
  media: Array<Media>;
};

export type Mutation = {
  __typename?: "Mutation";
  clearRwgpsConnection: Viewer;
  createPointOfInterest: CreatePointOfInterestOutput;
  createTrip: CreateTripOutput;
  initiateRwgpsHistorySync: Viewer;
  updatePointOfInterest: UpdatePointOfInterestOutput;
  updateTrip: UpdateTripOutput;
  updateTripMedia: TripMediaOutput;
  updateTripRides: TripRidesOutput;
};

export type MutationCreatePointOfInterestArgs = {
  input: CreatePointOfInterestInput;
};

export type MutationCreateTripArgs = {
  input: CreateTripInput;
};

export type MutationUpdatePointOfInterestArgs = {
  input: UpdatePointOfInterestInput;
};

export type MutationUpdateTripArgs = {
  input: UpdateTripInput;
};

export type MutationUpdateTripMediaArgs = {
  input: UpdateTripMediaInput;
};

export type MutationUpdateTripRidesArgs = {
  input: UpdateTripRidesInput;
};

export type NearbyRoute = {
  __typename?: "NearbyRoute";
  closestTerminus: Terminus;
  closestTerminusDelta: PointDelta;
  delta: PointDelta;
};

export type Note = TemporalContentBlock & {
  __typename?: "Note";
  contentAt: Scalars["DateTime"]["output"];
  ride?: Maybe<Ride>;
  text: Scalars["String"]["output"];
};

export type PointDelta = {
  __typename?: "PointDelta";
  bearing: Scalars["Float"]["output"];
  distance: Scalars["Float"]["output"];
  elevationGain: Scalars["Float"]["output"];
};

export type PointOfInterest = MediaTarget & {
  __typename?: "PointOfInterest";
  description?: Maybe<Scalars["String"]["output"]>;
  id: Scalars["PointOfInterestId"]["output"];
  media: Array<Media>;
  name: Scalars["String"]["output"];
  point: Array<Scalars["Float"]["output"]>;
  pointOfInterestType: PointOfInterestType;
  slug: Scalars["String"]["output"];
  visits: Array<PointOfInterestVisit>;
};

export enum PointOfInterestType {
  Campsite = "CAMPSITE",
  Generic = "GENERIC",
  Hut = "HUT",
  PublicTransportStop = "PUBLIC_TRANSPORT_STOP",
  WaterSource = "WATER_SOURCE",
}

export type PointOfInterestVisit = {
  __typename?: "PointOfInterestVisit";
  comment?: Maybe<Scalars["String"]["output"]>;
  media: Array<Media>;
  pointOfInterest: PointOfInterest;
  status: VisitStatus;
  user: UserProfile;
  visitedAt: Scalars["DateTime"]["output"];
};

export enum PointsDetail {
  High = "HIGH",
  Low = "LOW",
  Medium = "MEDIUM",
}

export type Query = {
  __typename?: "Query";
  pointOfInterestWithSlug?: Maybe<PointOfInterest>;
  pointsOfInterest: Array<PointOfInterest>;
  publicUsers: Array<UserProfile>;
  publishedTrips: Array<Trip>;
  queryRoutes: Array<Route>;
  rides: Array<Ride>;
  route?: Maybe<Route>;
  routeWithSlug?: Maybe<Route>;
  routes: Array<Route>;
  starredRoutes: Array<Route>;
  trip?: Maybe<Trip>;
  trips: Array<Trip>;
  userWithUsername?: Maybe<UserProfile>;
  viewer?: Maybe<Viewer>;
};

export type QueryPointOfInterestWithSlugArgs = {
  slug: Scalars["String"]["input"];
};

export type QueryQueryRoutesArgs = {
  input: QueryRoutesInput;
};

export type QueryRouteArgs = {
  id: Scalars["RouteId"]["input"];
};

export type QueryRouteWithSlugArgs = {
  slug: Scalars["String"]["input"];
};

export type QueryTripArgs = {
  id: Scalars["TripId"]["input"];
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

export type Ride = ElevationPath &
  MediaTarget &
  TemporalContentBlock & {
    __typename?: "Ride";
    contentAt: Scalars["DateTime"]["output"];
    date: Scalars["IsoDate"]["output"];
    distance: Scalars["Float"]["output"];
    distancePoints: Array<Scalars["Float"]["output"]>;
    distancePointsJson: Scalars["String"]["output"];
    elevationPoints: Array<Scalars["Float"]["output"]>;
    elevationPointsJson: Scalars["String"]["output"];
    finishedAt: Scalars["DateTime"]["output"];
    id: Scalars["RideId"]["output"];
    media: Array<Media>;
    name: Scalars["String"]["output"];
    points: Array<Array<Scalars["Float"]["output"]>>;
    pointsJson: Scalars["String"]["output"];
    startedAt: Scalars["DateTime"]["output"];
    tz?: Maybe<Scalars["String"]["output"]>;
    user: UserProfile;
  };

export type RidePointsArgs = {
  detailLevel: PointsDetail;
};

export type RidePointsJsonArgs = {
  detailLevel: PointsDetail;
};

export type Route = ElevationPath &
  MediaTarget & {
    __typename?: "Route";
    cues: Array<Cue>;
    description?: Maybe<Scalars["String"]["output"]>;
    direction?: Maybe<Direction>;
    distance: Scalars["Float"]["output"];
    distancePoints: Array<Scalars["Float"]["output"]>;
    distancePointsJson: Scalars["String"]["output"];
    elevationAscentM: Scalars["Float"]["output"];
    elevationDescentM: Scalars["Float"]["output"];
    elevationPoints: Array<Scalars["Float"]["output"]>;
    elevationPointsJson: Scalars["String"]["output"];
    externalRef?: Maybe<ExternalRef>;
    id: Scalars["RouteId"]["output"];
    idealBike?: Maybe<BikeSpec>;
    isMetaComplete: Scalars["Boolean"]["output"];
    media: Array<Media>;
    minimumBike?: Maybe<BikeSpec>;
    name: Scalars["String"]["output"];
    physicalDifficulty?: Maybe<DifficultyRating>;
    points: Array<Array<Scalars["Float"]["output"]>>;
    pointsCount: Scalars["Int"]["output"];
    pointsJson: Scalars["String"]["output"];
    samplePoints: Array<Array<Scalars["Float"]["output"]>>;
    samplePointsCount: Scalars["Int"]["output"];
    scouted?: Maybe<Scouted>;
    slug: Scalars["String"]["output"];
    tags?: Maybe<Array<Scalars["String"]["output"]>>;
    technicalDifficulty?: Maybe<DifficultyRating>;
    termini: Array<Terminus>;
    user: UserProfile;
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

export type TemporalContentBlock = {
  /** Timestamp associated with this content */
  contentAt: Scalars["DateTime"]["output"];
};

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

export type Trip = MediaTarget & {
  __typename?: "Trip";
  description?: Maybe<Scalars["String"]["output"]>;
  id: Scalars["TripId"]["output"];
  isPublished: Scalars["Boolean"]["output"];
  legs: Array<TripLeg>;
  media: Array<Media>;
  name: Scalars["String"]["output"];
  notes: Array<Note>;
  rides: Array<Ride>;
  slug: Scalars["String"]["output"];
  temporalContentBlocks: Array<TemporalContentBlock>;
  tz?: Maybe<Scalars["String"]["output"]>;
  user: UserProfile;
  year: Scalars["Int"]["output"];
};

export type TripLeg = ElevationPath & {
  __typename?: "TripLeg";
  distancePoints: Array<Scalars["Float"]["output"]>;
  distancePointsJson: Scalars["String"]["output"];
  elevationPoints: Array<Scalars["Float"]["output"]>;
  elevationPointsJson: Scalars["String"]["output"];
  rides: Array<Ride>;
  tz?: Maybe<Scalars["String"]["output"]>;
};

export type TripMediaOutput = {
  __typename?: "TripMediaOutput";
  trip?: Maybe<Trip>;
};

export type TripNoteInput = {
  text: Scalars["String"]["input"];
  timestamp: Scalars["DateTime"]["input"];
};

export type TripRidesOutput = {
  __typename?: "TripRidesOutput";
  trip?: Maybe<Trip>;
};

export type UpdatePointOfInterestInput = {
  description?: InputMaybe<Scalars["String"]["input"]>;
  name: Scalars["String"]["input"];
  point: Array<Scalars["Float"]["input"]>;
  pointOfInterestId: Scalars["PointOfInterestId"]["input"];
  pointOfInterestType: PointOfInterestType;
};

export type UpdatePointOfInterestOutput = {
  __typename?: "UpdatePointOfInterestOutput";
  pointOfInterest?: Maybe<PointOfInterest>;
};

export type UpdateTripInput = {
  description?: InputMaybe<Scalars["String"]["input"]>;
  isPublished: Scalars["Boolean"]["input"];
  name: Scalars["String"]["input"];
  notes: Array<TripNoteInput>;
  tripId: Scalars["TripId"]["input"];
};

export type UpdateTripMediaInput = {
  mediaIds: Array<Scalars["MediaId"]["input"]>;
  tripId: Scalars["TripId"]["input"];
};

export type UpdateTripOutput = {
  __typename?: "UpdateTripOutput";
  trip?: Maybe<Trip>;
};

export type UpdateTripRidesInput = {
  rideIds: Array<Scalars["RideId"]["input"]>;
  tripId: Scalars["TripId"]["input"];
};

export type UserProfile = {
  __typename?: "UserProfile";
  email?: Maybe<Scalars["String"]["output"]>;
  id: Scalars["UserId"]["output"];
  pointsOfInterest: Array<PointOfInterest>;
  recentRides: Array<Ride>;
  rides: Array<Ride>;
  ridesWithDate: Array<Ride>;
  routes: Array<Route>;
  tripWithSlug?: Maybe<Trip>;
  trips: Array<Trip>;
  username: Scalars["String"]["output"];
};

export type UserProfileRidesWithDateArgs = {
  date: Scalars["IsoDate"]["input"];
};

export type UserProfileTripWithSlugArgs = {
  slug: Scalars["String"]["input"];
};

export type UserRwgpsConnection = {
  __typename?: "UserRwgpsConnection";
  createdAt: Scalars["DateTime"]["output"];
  id: Scalars["UUID"]["output"];
  rwgpsUserId: Scalars["Int"]["output"];
  updatedAt: Scalars["DateTime"]["output"];
};

export type Viewer = {
  __typename?: "Viewer";
  id: Scalars["String"]["output"];
  profile: UserProfile;
  rwgpsAuthRequestUrl: Scalars["String"]["output"];
  rwgpsConnection?: Maybe<UserRwgpsConnection>;
};

export enum VisitStatus {
  AllGood = "ALL_GOOD",
  Issue = "ISSUE",
}

type ElevationPath_Ride_Fragment = {
  __typename?: "Ride";
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_Ride_Fragment" };

type ElevationPath_Route_Fragment = {
  __typename?: "Route";
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_Route_Fragment" };

type ElevationPath_TripLeg_Fragment = {
  __typename?: "TripLeg";
  elevationPointsJson: string;
  distancePointsJson: string;
} & { " $fragmentName"?: "ElevationPath_TripLeg_Fragment" };

export type ElevationPathFragment =
  | ElevationPath_Ride_Fragment
  | ElevationPath_Route_Fragment
  | ElevationPath_TripLeg_Fragment;

export type ViewerInfoFragment = {
  __typename?: "Viewer";
  id: string;
  profile: { __typename?: "UserProfile"; username: string };
} & { " $fragmentName"?: "ViewerInfoFragment" };

export type CreatePointOfInterestMutationVariables = Exact<{
  input: CreatePointOfInterestInput;
}>;

export type CreatePointOfInterestMutation = {
  __typename?: "Mutation";
  createPointOfInterest: {
    __typename?: "CreatePointOfInterestOutput";
    pointOfInterest: {
      __typename?: "PointOfInterest";
      id: any;
      name: string;
      slug: string;
    };
  };
};

export type EditPoiFragment = {
  __typename?: "PointOfInterest";
  id: any;
  name: string;
  description?: string | null;
  point: Array<number>;
  pointOfInterestType: PointOfInterestType;
} & { " $fragmentName"?: "EditPoiFragment" };

export type UpdatePointOfInterestMutationVariables = Exact<{
  input: UpdatePointOfInterestInput;
}>;

export type UpdatePointOfInterestMutation = {
  __typename?: "Mutation";
  updatePointOfInterest: {
    __typename?: "UpdatePointOfInterestOutput";
    pointOfInterest?: {
      __typename?: "PointOfInterest";
      id: any;
      name: string;
      description?: string | null;
      point: Array<number>;
      pointOfInterestType: PointOfInterestType;
    } | null;
  };
};

export type RideItemFragment = {
  __typename?: "Ride";
  id: any;
  date: any;
  tz?: string | null;
  distance: number;
  startedAt: any;
  finishedAt: any;
  user: { __typename?: "UserProfile"; username: string };
} & { " $fragmentName"?: "RideItemFragment" };

export type RideSummaryFragment = {
  __typename?: "Ride";
  id: any;
  name: string;
  distance: number;
  startedAt: any;
  finishedAt: any;
  tz?: string | null;
} & { " $fragmentName"?: "RideSummaryFragment" };

export type RouteItemFragment = ({
  __typename?: "Route";
  id: any;
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
  __typename?: "Route";
  distance: number;
  elevationAscentM: number;
  elevationDescentM: number;
} & { " $fragmentName"?: "RouteVitalsFragment" };

export type AllPoIsQueryVariables = Exact<{
  username: Scalars["String"]["input"];
}>;

export type AllPoIsQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    pointsOfInterest: Array<{
      __typename?: "PointOfInterest";
      id: any;
      name: string;
      slug: string;
      pointOfInterestType: PointOfInterestType;
    }>;
  } | null;
};

export type SettingsRideListQueryVariables = Exact<{
  username: Scalars["String"]["input"];
}>;

export type SettingsRideListQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    rides: Array<{
      __typename?: "Ride";
      id: any;
      name: string;
      startedAt: any;
      finishedAt: any;
      distance: number;
      date: any;
    }>;
  } | null;
};

export type AllRoutesQueryVariables = Exact<{
  username: Scalars["String"]["input"];
}>;

export type AllRoutesQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    routes: Array<{
      __typename?: "Route";
      id: any;
      name: string;
      slug: string;
      distance: number;
      elevationAscentM: number;
      elevationDescentM: number;
    }>;
  } | null;
};

export type AllTripsQueryVariables = Exact<{
  username: Scalars["String"]["input"];
}>;

export type AllTripsQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    trips: Array<{
      __typename?: "Trip";
      id: any;
      name: string;
      year: number;
      isPublished: boolean;
      slug: string;
    }>;
  } | null;
};

export type AllRidesQueryVariables = Exact<{
  username: Scalars["String"]["input"];
}>;

export type AllRidesQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    rides: Array<{
      __typename?: "Ride";
      id: any;
      name: string;
      startedAt: any;
      finishedAt: any;
      distance: number;
    }>;
  } | null;
};

export type CreateTripMutationVariables = Exact<{
  input: CreateTripInput;
}>;

export type CreateTripMutation = {
  __typename?: "Mutation";
  createTrip: {
    __typename?: "CreateTripOutput";
    trip: {
      __typename?: "Trip";
      id: any;
      name: string;
      slug: string;
      year: number;
      user: { __typename?: "UserProfile"; username: string };
    };
  };
};

export type TripMediaFragment = {
  __typename?: "Trip";
  id: any;
  media: Array<{
    __typename?: "Media";
    id: any;
    path: string;
    createdAt: any;
    capturedAt?: any | null;
    imageSizes: {
      __typename?: "ImageSizes";
      fill600: { __typename?: "ImageSize"; webpUrl: string };
    };
  }>;
} & { " $fragmentName"?: "TripMediaFragment" };

export type TripRidesFragment = {
  __typename?: "Trip";
  id: any;
  user: { __typename?: "UserProfile"; username: string };
  rides: Array<{
    __typename?: "Ride";
    id: any;
    name: string;
    startedAt: any;
    finishedAt: any;
    distance: number;
  }>;
} & { " $fragmentName"?: "TripRidesFragment" };

export type UpdateTripRidesMutationVariables = Exact<{
  input: UpdateTripRidesInput;
}>;

export type UpdateTripRidesMutation = {
  __typename?: "Mutation";
  updateTripRides: {
    __typename?: "TripRidesOutput";
    trip?: {
      __typename?: "Trip";
      id: any;
      rides: Array<{ __typename?: "Ride"; id: any }>;
    } | null;
  };
};

export type EditTripFragment = ({
  __typename?: "Trip";
  id: any;
  name: string;
  description?: string | null;
  isPublished: boolean;
  media: Array<{ __typename?: "Media"; id: any }>;
  temporalContentBlocks: Array<
    | {
        __typename: "Media";
        contentAt: any;
        mediaId: any;
        imageSizes: {
          __typename?: "ImageSizes";
          fit1200: { __typename?: "ImageSize"; webpUrl: string };
        };
      }
    | { __typename: "Note"; text: string; contentAt: any }
    | { __typename: "Ride"; name: string; contentAt: any; rideId: any }
  >;
} & {
  " $fragmentRefs"?: {
    TripRidesFragment: TripRidesFragment;
    TripMediaFragment: TripMediaFragment;
  };
}) & { " $fragmentName"?: "EditTripFragment" };

export type UpdateTripMutationVariables = Exact<{
  input: UpdateTripInput;
}>;

export type UpdateTripMutation = {
  __typename?: "Mutation";
  updateTrip: {
    __typename?: "UpdateTripOutput";
    trip?: {
      __typename?: "Trip";
      id: any;
      name: string;
      description?: string | null;
    } | null;
  };
};

export type UpdateTripMediaMutationVariables = Exact<{
  input: UpdateTripMediaInput;
}>;

export type UpdateTripMediaMutation = {
  __typename?: "Mutation";
  updateTripMedia: {
    __typename?: "TripMediaOutput";
    trip?: { __typename?: "Trip"; id: any } | null;
  };
};

export type TripItemFragment = {
  __typename?: "Trip";
  id: any;
  name: string;
  year: number;
  slug: string;
  legs: Array<{
    __typename?: "TripLeg";
    rides: Array<{ __typename?: "Ride"; startedAt: any }>;
  }>;
  user: { __typename?: "UserProfile"; username: string };
} & { " $fragmentName"?: "TripItemFragment" };

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

export type PoiQueryQueryVariables = Exact<{
  slug: Scalars["String"]["input"];
}>;

export type PoiQueryQuery = {
  __typename?: "Query";
  pointOfInterestWithSlug?:
    | ({
        __typename?: "PointOfInterest";
        id: any;
        name: string;
        point: Array<number>;
        description?: string | null;
        pointOfInterestType: PointOfInterestType;
        media: Array<{
          __typename?: "Media";
          id: any;
          point?: Array<number> | null;
        }>;
      } & { " $fragmentRefs"?: { EditPoiFragment: EditPoiFragment } })
    | null;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type PoIsQueryQueryVariables = Exact<{ [key: string]: never }>;

export type PoIsQueryQuery = {
  __typename?: "Query";
  pointsOfInterest: Array<
    {
      __typename?: "PointOfInterest";
      id: any;
      name: string;
      point: Array<number>;
      pointOfInterestType: PointOfInterestType;
    } & { " $fragmentRefs"?: { PoiItemFragment: PoiItemFragment } }
  >;
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

export type PoiItemFragment = {
  __typename?: "PointOfInterest";
  id: any;
  name: string;
  point: Array<number>;
  slug: string;
  pointOfInterestType: PointOfInterestType;
} & { " $fragmentName"?: "PoiItemFragment" };

export type RidesWithDateQueryVariables = Exact<{
  username: Scalars["String"]["input"];
  date: Scalars["IsoDate"]["input"];
  detailLevel: PointsDetail;
}>;

export type RidesWithDateQuery = {
  __typename?: "Query";
  viewer?:
    | ({ __typename?: "Viewer" } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
  userWithUsername?: {
    __typename?: "UserProfile";
    username: string;
    ridesWithDate: Array<
      {
        __typename?: "Ride";
        id: any;
        date: any;
        tz?: string | null;
        pointsJson: string;
      } & {
        " $fragmentRefs"?: {
          RideSummaryFragment: RideSummaryFragment;
          ElevationPath_Ride_Fragment: ElevationPath_Ride_Fragment;
        };
      }
    >;
  } | null;
};

export type UserProfileQueryQueryVariables = Exact<{
  username: Scalars["String"]["input"];
  detailLevel: PointsDetail;
}>;

export type UserProfileQueryQuery = {
  __typename?: "Query";
  userWithUsername?: {
    __typename?: "UserProfile";
    id: any;
    username: string;
    recentRides: Array<
      { __typename?: "Ride"; id: any; date: any; pointsJson: string } & {
        " $fragmentRefs"?: { RideItemFragment: RideItemFragment };
      }
    >;
    trips: Array<
      {
        __typename?: "Trip";
        id: any;
        name: string;
        legs: Array<{
          __typename?: "TripLeg";
          rides: Array<{ __typename?: "Ride"; id: any; pointsJson: string }>;
        }>;
      } & { " $fragmentRefs"?: { TripItemFragment: TripItemFragment } }
    >;
  } | null;
  viewer?:
    | ({ __typename?: "Viewer"; id: string } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
};

type ContentBlock_Media_Fragment = {
  __typename: "Media";
  capturedAt?: any | null;
  tz?: string | null;
  contentAt: any;
  mediaId: any;
  imageSizes: {
    __typename?: "ImageSizes";
    fit1600: { __typename?: "ImageSize"; webpUrl: string };
  };
  rides: Array<{ __typename?: "Ride"; id: any }>;
} & { " $fragmentName"?: "ContentBlock_Media_Fragment" };

type ContentBlock_Note_Fragment = {
  __typename: "Note";
  text: string;
  contentAt: any;
  ride?: { __typename?: "Ride"; id: any } | null;
} & { " $fragmentName"?: "ContentBlock_Note_Fragment" };

type ContentBlock_Ride_Fragment = ({
  __typename: "Ride";
  tz?: string | null;
  contentAt: any;
  rideId: any;
} & { " $fragmentRefs"?: { RideItemFragment: RideItemFragment } }) & {
  " $fragmentName"?: "ContentBlock_Ride_Fragment";
};

export type ContentBlockFragment =
  | ContentBlock_Media_Fragment
  | ContentBlock_Note_Fragment
  | ContentBlock_Ride_Fragment;

export type TripQueryQueryVariables = Exact<{
  username: Scalars["String"]["input"];
  slug: Scalars["String"]["input"];
  detailLevel: PointsDetail;
}>;

export type TripQueryQuery = {
  __typename?: "Query";
  viewer?:
    | ({ __typename?: "Viewer"; id: string } & {
        " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment };
      })
    | null;
  userWithUsername?: {
    __typename?: "UserProfile";
    username: string;
    tripWithSlug?:
      | ({
          __typename?: "Trip";
          id: any;
          name: string;
          user: { __typename?: "UserProfile"; id: any };
          media: Array<{
            __typename?: "Media";
            id: any;
            point?: Array<number> | null;
          }>;
          legs: Array<
            {
              __typename?: "TripLeg";
              rides: Array<
                { __typename?: "Ride"; id: any; pointsJson: string } & {
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
            | ({ __typename?: "Media" } & {
                " $fragmentRefs"?: {
                  ContentBlock_Media_Fragment: ContentBlock_Media_Fragment;
                };
              })
            | ({ __typename?: "Note" } & {
                " $fragmentRefs"?: {
                  ContentBlock_Note_Fragment: ContentBlock_Note_Fragment;
                };
              })
            | ({ __typename?: "Ride" } & {
                " $fragmentRefs"?: {
                  ContentBlock_Ride_Fragment: ContentBlock_Ride_Fragment;
                };
              })
          >;
        } & { " $fragmentRefs"?: { EditTripFragment: EditTripFragment } })
      | null;
  } | null;
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

export type NearbyRoutesInfoFragment = {
  __typename?: "Terminus";
  bearing: number;
  nearbyRoutes: Array<{
    __typename?: "NearbyRoute";
    delta: { __typename?: "PointDelta"; distance: number; bearing: number };
    closestTerminus: {
      __typename?: "Terminus";
      bearing: number;
      route: { __typename?: "Route"; id: any } & {
        " $fragmentRefs"?: { RouteItemFragment: RouteItemFragment };
      };
    };
  }>;
} & { " $fragmentName"?: "NearbyRoutesInfoFragment" };

export type RouteQueryQueryVariables = Exact<{
  slug: Scalars["String"]["input"];
}>;

export type RouteQueryQuery = {
  __typename?: "Query";
  routeWithSlug?:
    | ({
        __typename?: "Route";
        id: any;
        name: string;
        slug: string;
        tags?: Array<string> | null;
        distance: number;
        elevationAscentM: number;
        elevationDescentM: number;
        pointsJson: string;
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
        termini: Array<
          {
            __typename?: "Terminus";
            bearing: number;
            nearbyRoutes: Array<{
              __typename?: "NearbyRoute";
              closestTerminus: {
                __typename?: "Terminus";
                route: { __typename?: "Route"; id: any; pointsJson: string };
              };
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
  queryRoutes: Array<
    { __typename?: "Route"; id: any; samplePoints: Array<Array<number>> } & {
      " $fragmentRefs"?: { RouteItemFragment: RouteItemFragment };
    }
  >;
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

export type TripsQueryQueryVariables = Exact<{ [key: string]: never }>;

export type TripsQueryQuery = {
  __typename?: "Query";
  publishedTrips: Array<
    {
      __typename?: "Trip";
      id: any;
      name: string;
      legs: Array<{
        __typename?: "TripLeg";
        rides: Array<{ __typename?: "Ride"; id: any; pointsJson: string }>;
      }>;
    } & { " $fragmentRefs"?: { TripItemFragment: TripItemFragment } }
  >;
  viewer?:
    | ({
        __typename?: "Viewer";
        profile: { __typename?: "UserProfile"; id: any; username: string };
      } & { " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment } })
    | null;
};

export type TripsQueryPointsQueryVariables = Exact<{ [key: string]: never }>;

export type TripsQueryPointsQuery = {
  __typename?: "Query";
  publishedTrips: Array<{
    __typename?: "Trip";
    id: any;
    legs: Array<{
      __typename?: "TripLeg";
      rides: Array<{ __typename?: "Ride"; id: any; pointsJson: string }>;
    }>;
  }>;
};

export type SettingsQueryVariables = Exact<{ [key: string]: never }>;

export type SettingsQuery = {
  __typename?: "Query";
  viewer?:
    | ({
        __typename?: "Viewer";
        rwgpsAuthRequestUrl: string;
        profile: {
          __typename?: "UserProfile";
          id: any;
          username: string;
          email?: string | null;
        };
        rwgpsConnection?: {
          __typename?: "UserRwgpsConnection";
          id: any;
          rwgpsUserId: number;
          createdAt: any;
          updatedAt: any;
        } | null;
      } & { " $fragmentRefs"?: { ViewerInfoFragment: ViewerInfoFragment } })
    | null;
};

export type InitiateRwgpsHistorySyncMutationVariables = Exact<{
  [key: string]: never;
}>;

export type InitiateRwgpsHistorySyncMutation = {
  __typename?: "Mutation";
  initiateRwgpsHistorySync: { __typename?: "Viewer" } & {
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
