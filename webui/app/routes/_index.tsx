import { redirect, TypedResponse } from "@remix-run/node";

export async function loader(): Promise<TypedResponse<never>> {
  return redirect("/routes");
}

export default function Index(): React.ReactElement {
  return <></>;
}
