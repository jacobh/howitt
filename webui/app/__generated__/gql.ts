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
  "\n  fragment elevationPath on ElevationPath {\n    elevationPoints\n    distancePoints\n  }\n":
    types.ElevationPathFragmentDoc,
  "\n    fragment viewerInfo on Viewer {\n        id\n        profile {\n          username\n        }\n    }\n  ":
    types.ViewerInfoFragmentDoc,
  "\n    fragment rideItem on Ride {\n        id\n        date\n        distance\n        startedAt\n        finishedAt\n        user {\n            username\n        }\n    }\n":
    types.RideItemFragmentDoc,
  "\n  fragment rideSummary on Ride {\n    id\n    name\n    distance\n    startedAt\n    finishedAt\n  }\n":
    types.RideSummaryFragmentDoc,
  "\n    fragment routeItem on Route {\n        id\n        name\n        slug\n        distance\n        elevationAscentM\n        elevationDescentM\n        isMetaComplete\n    }\n":
    types.RouteItemFragmentDoc,
  "\n  query LoginViewerInfo {\n    viewer {\n      id\n      profile {\n        username\n      }\n    ...viewerInfo\n    }\n  }  \n":
    types.LoginViewerInfoDocument,
  "\n  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {\n    viewer {\n      ...viewerInfo\n    }\n    userWithUsername(username: $username) {\n      username\n      ridesWithDate(date: $date) {\n        id\n        date\n        pointsJson(pointsPerKm: $pointsPerKm)\n        ...rideSummary\n        ...elevationPath\n      }\n    }\n  }\n":
    types.RidesWithDateDocument,
  "\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.UserProfileQueryDocument,
  "\n  query publicUsers {\n    publicUsers {\n        id\n        ...userItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.PublicUsersDocument,
  "\n    fragment userItem on UserProfile {\n        id\n        username\n    }\n  ":
    types.UserItemFragmentDoc,
  "\n  fragment nearbyRoutesInfo on Terminus {\n    bearing\n    nearbyRoutes {\n      delta {\n        distance\n        bearing\n      }\n      closestTerminus {\n        bearing\n        route {\n          id\n          ...routeItem\n        }\n      }\n    }\n  }\n":
    types.NearbyRoutesInfoFragmentDoc,
  "\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    photos {\n      id\n      url\n      caption\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n":
    types.RouteQueryDocument,
  "\n  query homeQuery($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      samplePoints\n      ...routeItem\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n":
    types.HomeQueryDocument,
  "\n  query homeQueryPointOnly($input: QueryRoutesInput!) {\n    queryRoutes(input: $input) {\n      id\n      pointsJson\n    }\n  }\n":
    types.HomeQueryPointOnlyDocument,
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
  source: "\n  fragment elevationPath on ElevationPath {\n    elevationPoints\n    distancePoints\n  }\n",
): (typeof documents)["\n  fragment elevationPath on ElevationPath {\n    elevationPoints\n    distancePoints\n  }\n"];
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
  source: "\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n",
): (typeof documents)["\n  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {\n    userWithUsername(username: $username) {\n        id\n        username\n        recentRides {\n          id\n          date\n          pointsJson(pointsPerKm: $pointsPerKm)\n          ...rideItem\n        }\n    }\n    viewer {\n      ...viewerInfo\n    }\n  }\n"];
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
  source: "\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    photos {\n      id\n      url\n      caption\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n",
): (typeof documents)["\nquery RouteQuery($slug: String!) {\n  routeWithSlug(slug: $slug) {\n    id\n    name\n    slug\n    externalRef {\n      canonicalUrl\n    }\n    tags\n    distance\n    elevationAscentM\n    elevationDescentM\n    pointsJson\n    description\n    technicalDifficulty\n    physicalDifficulty\n    scouted\n    direction\n    minimumBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    idealBike {\n      tyreWidth\n      frontSuspension\n      rearSuspension\n    }\n    photos {\n      id\n      url\n      caption\n    }\n    termini {\n      bearing\n\n      nearbyRoutes {\n        closestTerminus {\n          route {\n            id\n            pointsJson\n          }\n        }\n      }\n\n      ...nearbyRoutesInfo\n    }\n\n    ...elevationPath\n  }\n  viewer {\n    ...viewerInfo\n  }\n}\n"];
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

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> =
  TDocumentNode extends DocumentNode<infer TType, any> ? TType : never;
