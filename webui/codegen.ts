import { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: process.env.GRAPHQL_URL ?? "https://api.howittplains.net/",
  documents: ["app/**/*.tsx"],
  generates: {
    "./app/__generated__/schema.graphql": {
      plugins: ["schema-ast"],
    },
    "./app/__generated__/": {
      preset: "client",
      plugins: [],
      presetConfig: {
        gqlTagName: "gql",
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
