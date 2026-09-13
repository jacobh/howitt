export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
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
  DateTime: { input: string; output: string };
  IsoDate: { input: string; output: string };
  MediaId: { input: string; output: string };
  PointOfInterestId: { input: string; output: string };
  RideId: { input: string; output: string };
  RouteId: { input: string; output: string };
  TripId: { input: string; output: string };
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
  UUID: { input: string; output: string };
  /** URL is a String implementing the [URL Standard](http://url.spec.whatwg.org/) */
  Url: { input: string; output: string };
  UserId: { input: string; output: string };
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

export enum Poicondition {
  AllGood = "ALL_GOOD",
  Issue = "ISSUE",
}

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
  condition?: Maybe<Poicondition>;
  confirmation: VisitConfirmation;
  media: Array<Media>;
  pointOfInterest: PointOfInterest;
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

export enum VisitConfirmation {
  Confirmed = "CONFIRMED",
  Pending = "PENDING",
  Rejected = "REJECTED",
}
