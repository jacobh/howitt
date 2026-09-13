import { redirect } from "react-router";

export async function loader(): Promise<Response> {
  return redirect("/routes");
}

export default function Index(): React.ReactElement {
  return <></>;
}
