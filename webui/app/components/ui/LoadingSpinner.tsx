import { css } from "@emotion/react";
import { useEffect, useState } from "react";

const spinnerCss = css`
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: conic-gradient(#0000 10%, rgb(158, 158, 158));
  -webkit-mask: radial-gradient(farthest-side, #0000 calc(100% - 9px), #000 0);
  animation: spinner-zp9dbg 1s infinite linear;

  @keyframes spinner-zp9dbg {
    to {
      transform: rotate(1turn);
    }
  }
`;

const spinnerWrapperCss = css`
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  margin: 100px 0;
`;

export function LoadingSpinner(): React.ReactElement {
  return <div css={spinnerCss} />;
}

export function LoadingSpinnerSidebarContent(): React.ReactElement {
  let [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 400);
    return () => clearTimeout(timer);
  }, []);

  return <div css={spinnerWrapperCss}>{isVisible && <LoadingSpinner />}</div>;
}
