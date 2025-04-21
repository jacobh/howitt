import { Hono } from "hono";
import { sql } from "bun";

const app = new Hono();

const exampleQuery = sql`
    select count(*) from osm_features;
`;

app.get("/", async (c) => {
  const res = await exampleQuery.execute();

  return c.json(res);
});

export default {
  port: 3001,
  fetch: app.fetch,
};
