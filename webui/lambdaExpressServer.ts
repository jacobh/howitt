import serverlessExpress from "@vendia/serverless-express";
import express from "express";
import { createRequestHandler } from "@remix-run/express";

const app = express();

app.disable("x-powered-by");

app.use(
  "/build",
  express.static("public/build", { immutable: true, maxAge: "1y" }),
);

app.use(express.static("public", { maxAge: "1h" }));

// needs to handle all verbs (GET, POST, etc.)
app.all(
  /.*/,
  createRequestHandler({
    // `remix build` and `remix dev` output files to a build directory, you need
    // to pass that build to the request handler
    build: require("./build"),
  }),
);

export const handler = serverlessExpress({ app });
