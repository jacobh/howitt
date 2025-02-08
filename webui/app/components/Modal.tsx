import * as Dialog from "@radix-ui/react-dialog";
import { css } from "@emotion/react";
import { makeMqs } from "~/styles/mediaQueries";

const modalStyles = makeMqs([
  css`
    padding: 5vw;
    border: 0;
    border-radius: 0.5rem;
    box-shadow: 0 0 0.5rem 0.25rem hsl(0 0% 0% / 10%);
    width: 90vw;
    background: white;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 2;
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

const overlayStyles = css`
  background: hsl(0 0% 0% / 50%);
  position: fixed;
  inset: 0;
  z-index: 1;
`;

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
  return (
    <Dialog.Root open={isOpen} onOpenChange={onClose}>
      <Dialog.Portal>
        <Dialog.Overlay css={overlayStyles} />
        <Dialog.Content css={modalStyles}>{children}</Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
