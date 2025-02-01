import { css } from "@emotion/react";
import { useRef } from "react";
import { makeMqs } from "~/styles/mediaQueries";

const modalStyles = makeMqs([
  css`
    padding: 5vw;
    border: 0;
    border-radius: 0.5rem;
    box-shadow: 0 0 0.5rem 0.25rem hsl(0 0% 0% / 10%);

    width: 90vw;

    &::backdrop {
      background: hsl(0 0% 0% / 50%);
    }
  `,
  css`
    padding: 4vw;
    min-width: 500px;
    max-width: 1000px;
  `,
  css`
    padding: 3vw;
  `,
  css`
    padding: 2vw;
  `,
]);

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

export function Modal({
  isOpen,
  onClose,
  children,
}: ModalProps): React.ReactElement {
  const dialogRef = useRef<HTMLDialogElement>(null);

  // Show/hide modal
  if (isOpen) {
    dialogRef.current?.showModal();
  } else {
    dialogRef.current?.close();
  }

  return (
    <dialog ref={dialogRef} css={modalStyles} onClose={onClose}>
      {children}
    </dialog>
  );
}
