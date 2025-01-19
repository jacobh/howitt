import { redirect } from "@remix-run/node";

export async function loader(): Promise<any> {
  return redirect("/routes");
}

export default function Index(): React.ReactElement {
  return <></>;
}
