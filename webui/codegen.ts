import { CodegenConfig } from "@graphql-codegen/cli";

const scalars = {
  DateTime: "string",
  IsoDate: "string",
  MediaId: "string",
  PointOfInterestId: "string",
  RideId: "string",
  RouteId: "string",
  TripId: "string",
  UUID: "string",
  Url: "string",
  UserId: "string",
};

const config: CodegenConfig = {
  schema: process.env.GRAPHQL_URL ?? "https://api.howittplains.net/",
  documents: ["app/**/*.tsx"],
  generates: {
    "./app/__generated__/schema.graphql": {
      plugins: ["schema-ast"],
    },
    "./app/__generated__/schema-types.ts": {
      plugins: ["typescript"],
      config: { scalars },
    },
    "./app/__generated__/": {
      preset: "client",
      plugins: [],
      presetConfig: {
        gqlTagName: "gql",
      },
      config: {
        enumType: "native",
        scalars,
      },
    },
    "./app/__generated__/fragment-types.json": {
      plugins: ["fragment-matcher"],
      config: {
        module: "json",
        apolloClientVersion: 3,
      },
    },
  },
  ignoreNoDocuments: true,
};

export default config;
