/* eslint-disable */
import * as types from "./graphql";
import { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core";

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
const documents = {
  "\n  fragment elevationPath on ElevationPath {\n    elevationPointsJson\n    distancePointsJson\n  }\n":
    types.ElevationPathFragmentDoc,
  "\n    fragment viewerInfo on Viewer {\n        id\n        profile {\n          username\n        }\n    }\n  ":
    types.ViewerInfoFragmentDoc,
  "\n    fragment rideItem on Ride {\n        id\n        date\n        distance\n        startedAt\n        finishedAt\n        user {\n            username\n        }\n    }\n":
    types.RideItemFragmentDoc,
  "\n  fragment rideSummary on Ride {\n    id\n    name\n    distance\n    startedAt\n    finishedAt\n  }\n":
    types.RideSummaryFragmentDoc,
  "\n    fragment routeItem on Route {\n        id\n        name\n        slug\n        distance\n        elevationAscentM\n        elevationDescentM\n        isMetaComplete\n    }\n":
    types.RouteItemFragmentDoc,
  "\n  fragment tripMedia on Trip {\n    id\n    media {\n      id\n      path\n      createdAt\n      imageSizes {\n        fill600 {\n          webpUrl\n        }\n      }\n    }\n  }\n":
    types.TripMediaFragmentDoc,
  "\n  fragment tripRides on Trip {\n    id\n    user {\n        username\n    }\n    rides {\n      id\n      name\n      startedAt\n      finishedAt\n      distance\n    }\n  }\n":
    types.TripRidesFragmentDoc,
  "\n    query AllRides($username: String!) {\n      userWithUsername(username: $username) {\n        rides {\n          id\n          name\n          startedAt\n          finishedAt\n          distance\n        }\n      }\n    }\n  ":
    types.AllRidesDocument,
  "\n    fragment editTrip on Trip {\n    id\n    name \n    description\n    ...tripRides\n    ...tripMedia\n    media {\n      id\n    }\n    temporalContentBlocks {\n      __typename\n      contentAt\n      ... on Note {\n        text\n      }\n      ... on Media {\n        mediaId: id\n        imageSizes {\n          fit1200 {\n            webpUrl\n          }\n        }\n      }\n      ... on Ride {\n        rideId: id\n        name\n      }\n    }\n  }\n":
    types.EditTripFragmentDoc,
  "\n  mutation UpdateTrip($input: UpdateTripInput!) {\n    updateTrip(input: $input) {\n      trip {\n        id\n        name\n        description\n      }\n    }\n  }\n":
    types.UpdateTripDocument,
  "\n  mutation UpdateTripMedia($input: UpdateTripMediaInput!) {\n    updateTripMedia(input: $input) {\n      trip {\n        id\n      }\n    }\n  }\n":
    types.UpdateTripMediaDocument,
  "\n        fragment tripItem on Trip {\n        id\n        name\n        year\n        slug\n        user {\n          username\n        }\n    }\n":
    types.TripItemFragmentDoc,
  "\n  query LoginViewerInfo {\n    viewer {\n      id\n      profile {\n        username\n      }\n    ...viewerInfo\n    }\n  }  \n":
    types.LoginViewerInfoDocument,
  "\n  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {\n    viewer {\n      ...viewerInfo\n    }\n    userWithUsername(username: $username) {\n      username\n      ridesWithDate(date: $date) {\n        id\n        date\n        pointsJson(pointsPerKm: $pointsPerKm)\n        ...rideSummary\n        ...elevationPath\n      }\n    }\n  }\n":
    types.RidesWithDateDocument,
  "\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n        trips {\n          id\n          name\n          ...tripItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.UserProfileQueryDocument,
  "\n  fragment contentBlock on TemporalContentBlock {\n    __typename\n    contentAt\n    ... on Ride {\n      rideId: id\n      ...rideItem\n    }\n    ... on Media {\n      mediaId: id\n      capturedAt\n      imageSizes {\n        fit1600 {\n          webpUrl\n        }\n      }\n      rides {\n        id\n      }\n    }\n    ... on Note {\n      text\n      ride {\n        id\n      }\n    }\n  }\n":
    types.ContentBlockFragmentDoc,
  "\n  query TripQuery($username: String!, $slug: String!, $pointsPerKm: Int!) {\n    viewer {\n      id\n      ...viewerInfo\n    }\n\n    userWithUsername(username: $username) {\n      username\n      tripWithSlug(slug: $slug) {\n        id\n        name\n        ...editTrip\n        user {\n          id\n        }\n        media {\n          id\n          point\n        }\n        legs {\n          ...elevationPath\n          rides {\n            id\n            ...elevationPath\n            pointsJson(pointsPerKm: $pointsPerKm)\n          }\n        }\n        temporalContentBlocks {\n          ...contentBlock\n        }\n      }\n    }\n  }\n":
    types.TripQueryDocument,
  "\n  query publicUsers {\n    publicUsers {\n        id\n        ...userItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.PublicUsersDocument,
  "\n    fragment userItem on UserProfile {\n        id\n        username\n    }\n  ":
    types.UserItemFragmentDoc,
  "\n  fragment nearbyRoutesInfo on Terminus {\n    bearing\n    nearbyRoutes {\n      delta {\n        distance\n        bearing\n      }\n      closestTerminus {\n        bearing\n        route {\n          id\n          ...routeItem\n        }\n      }\n    }\n  }\n":
    types.NearbyRoutesInfoFragmentDoc,
  "\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n":
    types.RouteQueryDocument,
  "\n  query homeQuery($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      samplePoints\n      ...routeItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.HomeQueryDocument,
  "\n  query homeQueryPointOnly($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      pointsJson\n    }\n  }\n":
    types.HomeQueryPointOnlyDocument,
  "\n  query settings {\n    viewer {\n      ...viewerInfo\n        profile {\n            id\n            username\n            email\n        }\n        rwgpsConnection {\n            id\n            rwgpsUserId\n            createdAt\n            updatedAt\n        }\n        rwgpsAuthRequestUrl\n    }\n  }\n":
    types.SettingsDocument,
  "\n  mutation initiateRwgpsHistorySync {\n    initiateRwgpsHistorySync {\n      ...viewerInfo\n    }\n  }\n":
    types.InitiateRwgpsHistorySyncDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment elevationPath on ElevationPath {\n    elevationPointsJson\n    distancePointsJson\n  }\n",
): (typeof documents)["\n  fragment elevationPath on ElevationPath {\n    elevationPointsJson\n    distancePointsJson\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    fragment viewerInfo on Viewer {\n        id\n        profile {\n          username\n        }\n    }\n  ",
): (typeof documents)["\n    fragment viewerInfo on Viewer {\n        id\n        profile {\n          username\n        }\n    }\n  "];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    fragment rideItem on Ride {\n        id\n        date\n        distance\n        startedAt\n        finishedAt\n        user {\n            username\n        }\n    }\n",
): (typeof documents)["\n    fragment rideItem on Ride {\n        id\n        date\n        distance\n        startedAt\n        finishedAt\n        user {\n            username\n        }\n    }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment rideSummary on Ride {\n    id\n    name\n    distance\n    startedAt\n    finishedAt\n  }\n",
): (typeof documents)["\n  fragment rideSummary on Ride {\n    id\n    name\n    distance\n    startedAt\n    finishedAt\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    fragment routeItem on Route {\n        id\n        name\n        slug\n        distance\n        elevationAscentM\n        elevationDescentM\n        isMetaComplete\n    }\n",
): (typeof documents)["\n    fragment routeItem on Route {\n        id\n        name\n        slug\n        distance\n        elevationAscentM\n        elevationDescentM\n        isMetaComplete\n    }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment tripMedia on Trip {\n    id\n    media {\n      id\n      path\n      createdAt\n      imageSizes {\n        fill600 {\n          webpUrl\n        }\n      }\n    }\n  }\n",
): (typeof documents)["\n  fragment tripMedia on Trip {\n    id\n    media {\n      id\n      path\n      createdAt\n      imageSizes {\n        fill600 {\n          webpUrl\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment tripRides on Trip {\n    id\n    user {\n        username\n    }\n    rides {\n      id\n      name\n      startedAt\n      finishedAt\n      distance\n    }\n  }\n",
): (typeof documents)["\n  fragment tripRides on Trip {\n    id\n    user {\n        username\n    }\n    rides {\n      id\n      name\n      startedAt\n      finishedAt\n      distance\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    query AllRides($username: String!) {\n      userWithUsername(username: $username) {\n        rides {\n          id\n          name\n          startedAt\n          finishedAt\n          distance\n        }\n      }\n    }\n  ",
): (typeof documents)["\n    query AllRides($username: String!) {\n      userWithUsername(username: $username) {\n        rides {\n          id\n          name\n          startedAt\n          finishedAt\n          distance\n        }\n      }\n    }\n  "];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    fragment editTrip on Trip {\n    id\n    name \n    description\n    ...tripRides\n    ...tripMedia\n    media {\n      id\n    }\n    temporalContentBlocks {\n      __typename\n      contentAt\n      ... on Note {\n        text\n      }\n      ... on Media {\n        mediaId: id\n        imageSizes {\n          fit1200 {\n            webpUrl\n          }\n        }\n      }\n      ... on Ride {\n        rideId: id\n        name\n      }\n    }\n  }\n",
): (typeof documents)["\n    fragment editTrip on Trip {\n    id\n    name \n    description\n    ...tripRides\n    ...tripMedia\n    media {\n      id\n    }\n    temporalContentBlocks {\n      __typename\n      contentAt\n      ... on Note {\n        text\n      }\n      ... on Media {\n        mediaId: id\n        imageSizes {\n          fit1200 {\n            webpUrl\n          }\n        }\n      }\n      ... on Ride {\n        rideId: id\n        name\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  mutation UpdateTrip($input: UpdateTripInput!) {\n    updateTrip(input: $input) {\n      trip {\n        id\n        name\n        description\n      }\n    }\n  }\n",
): (typeof documents)["\n  mutation UpdateTrip($input: UpdateTripInput!) {\n    updateTrip(input: $input) {\n      trip {\n        id\n        name\n        description\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  mutation UpdateTripMedia($input: UpdateTripMediaInput!) {\n    updateTripMedia(input: $input) {\n      trip {\n        id\n      }\n    }\n  }\n",
): (typeof documents)["\n  mutation UpdateTripMedia($input: UpdateTripMediaInput!) {\n    updateTripMedia(input: $input) {\n      trip {\n        id\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n        fragment tripItem on Trip {\n        id\n        name\n        year\n        slug\n        user {\n          username\n        }\n    }\n",
): (typeof documents)["\n        fragment tripItem on Trip {\n        id\n        name\n        year\n        slug\n        user {\n          username\n        }\n    }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query LoginViewerInfo {\n    viewer {\n      id\n      profile {\n        username\n      }\n    ...viewerInfo\n    }\n  }  \n",
): (typeof documents)["\n  query LoginViewerInfo {\n    viewer {\n      id\n      profile {\n        username\n      }\n    ...viewerInfo\n    }\n  }  \n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {\n    viewer {\n      ...viewerInfo\n    }\n    userWithUsername(username: $username) {\n      username\n      ridesWithDate(date: $date) {\n        id\n        date\n        pointsJson(pointsPerKm: $pointsPerKm)\n        ...rideSummary\n        ...elevationPath\n      }\n    }\n  }\n",
): (typeof documents)["\n  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {\n    viewer {\n      ...viewerInfo\n    }\n    userWithUsername(username: $username) {\n      username\n      ridesWithDate(date: $date) {\n        id\n        date\n        pointsJson(pointsPerKm: $pointsPerKm)\n        ...rideSummary\n        ...elevationPath\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n        trips {\n          id\n          name\n          ...tripItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n",
): (typeof documents)["\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n        trips {\n          id\n          name\n          ...tripItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment contentBlock on TemporalContentBlock {\n    __typename\n    contentAt\n    ... on Ride {\n      rideId: id\n      ...rideItem\n    }\n    ... on Media {\n      mediaId: id\n      capturedAt\n      imageSizes {\n        fit1600 {\n          webpUrl\n        }\n      }\n      rides {\n        id\n      }\n    }\n    ... on Note {\n      text\n      ride {\n        id\n      }\n    }\n  }\n",
): (typeof documents)["\n  fragment contentBlock on TemporalContentBlock {\n    __typename\n    contentAt\n    ... on Ride {\n      rideId: id\n      ...rideItem\n    }\n    ... on Media {\n      mediaId: id\n      capturedAt\n      imageSizes {\n        fit1600 {\n          webpUrl\n        }\n      }\n      rides {\n        id\n      }\n    }\n    ... on Note {\n      text\n      ride {\n        id\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query TripQuery($username: String!, $slug: String!, $pointsPerKm: Int!) {\n    viewer {\n      id\n      ...viewerInfo\n    }\n\n    userWithUsername(username: $username) {\n      username\n      tripWithSlug(slug: $slug) {\n        id\n        name\n        ...editTrip\n        user {\n          id\n        }\n        media {\n          id\n          point\n        }\n        legs {\n          ...elevationPath\n          rides {\n            id\n            ...elevationPath\n            pointsJson(pointsPerKm: $pointsPerKm)\n          }\n        }\n        temporalContentBlocks {\n          ...contentBlock\n        }\n      }\n    }\n  }\n",
): (typeof documents)["\n  query TripQuery($username: String!, $slug: String!, $pointsPerKm: Int!) {\n    viewer {\n      id\n      ...viewerInfo\n    }\n\n    userWithUsername(username: $username) {\n      username\n      tripWithSlug(slug: $slug) {\n        id\n        name\n        ...editTrip\n        user {\n          id\n        }\n        media {\n          id\n          point\n        }\n        legs {\n          ...elevationPath\n          rides {\n            id\n            ...elevationPath\n            pointsJson(pointsPerKm: $pointsPerKm)\n          }\n        }\n        temporalContentBlocks {\n          ...contentBlock\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query publicUsers {\n    publicUsers {\n        id\n        ...userItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n",
): (typeof documents)["\n  query publicUsers {\n    publicUsers {\n        id\n        ...userItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n    fragment userItem on UserProfile {\n        id\n        username\n    }\n  ",
): (typeof documents)["\n    fragment userItem on UserProfile {\n        id\n        username\n    }\n  "];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  fragment nearbyRoutesInfo on Terminus {\n    bearing\n    nearbyRoutes {\n      delta {\n        distance\n        bearing\n      }\n      closestTerminus {\n        bearing\n        route {\n          id\n          ...routeItem\n        }\n      }\n    }\n  }\n",
): (typeof documents)["\n  fragment nearbyRoutesInfo on Terminus {\n    bearing\n    nearbyRoutes {\n      delta {\n        distance\n        bearing\n      }\n      closestTerminus {\n        bearing\n        route {\n          id\n          ...routeItem\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n",
): (typeof documents)["\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query homeQuery($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      samplePoints\n      ...routeItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n",
): (typeof documents)["\n  query homeQuery($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      samplePoints\n      ...routeItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query homeQueryPointOnly($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      pointsJson\n    }\n  }\n",
): (typeof documents)["\n  query homeQueryPointOnly($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      pointsJson\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  query settings {\n    viewer {\n      ...viewerInfo\n        profile {\n            id\n            username\n            email\n        }\n        rwgpsConnection {\n            id\n            rwgpsUserId\n            createdAt\n            updatedAt\n        }\n        rwgpsAuthRequestUrl\n    }\n  }\n",
): (typeof documents)["\n  query settings {\n    viewer {\n      ...viewerInfo\n        profile {\n            id\n            username\n            email\n        }\n        rwgpsConnection {\n            id\n            rwgpsUserId\n            createdAt\n            updatedAt\n        }\n        rwgpsAuthRequestUrl\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(
  source: "\n  mutation initiateRwgpsHistorySync {\n    initiateRwgpsHistorySync {\n      ...viewerInfo\n    }\n  }\n",
): (typeof documents)["\n  mutation initiateRwgpsHistorySync {\n    initiateRwgpsHistorySync {\n      ...viewerInfo\n    }\n  }\n"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> =
  TDocumentNode extends DocumentNode<infer TType, any> ? TType : never;
