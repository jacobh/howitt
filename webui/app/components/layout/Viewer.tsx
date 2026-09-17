import { useQuery } from "@apollo/client/react";
import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  type PropsWithChildren,
} from "react";

import { gql } from "~/__generated__";
import type { ViewerQueryQuery } from "~/__generated__/graphql";

export const ViewerQuery = gql(`
  query viewerQuery {
    viewer {
      id
      profile {
        username
      }
    }
  }
`);

export type ViewerState =
  | { status: "loading" }
  | { status: "anonymous" }
  | {
      status: "authenticated";
      viewer: NonNullable<ViewerQueryQuery["viewer"]>;
    };

export function resolveViewerState(
  data: ViewerQueryQuery | undefined,
): ViewerState {
  if (data === undefined) {
    return { status: "loading" };
  }

  if (data.viewer === null) {
    return { status: "anonymous" };
  }

  return { status: "authenticated", viewer: data.viewer };
}

type ViewerContextValue = ViewerState & {
  refresh: () => Promise<void>;
};

const ViewerContext = createContext<ViewerContextValue | undefined>(undefined);

export function ViewerProvider({
  children,
}: PropsWithChildren): React.ReactNode {
  const { data, refetch } = useQuery(ViewerQuery);
  const state = resolveViewerState(data);
  const refresh = useCallback(async (): Promise<void> => {
    await refetch();
  }, [refetch]);
  const value = useMemo(() => ({ ...state, refresh }), [refresh, state]);

  return (
    <ViewerContext.Provider value={value}>{children}</ViewerContext.Provider>
  );
}

export function useViewer(): ViewerContextValue {
  const context = useContext(ViewerContext);

  if (context === undefined) {
    throw new Error("useViewer must be used within ViewerProvider");
  }

  return context;
}
