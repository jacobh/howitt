export function SidebarContainer({
  children,
}: {
  children?: React.ReactNode;
}): JSX.Element {
  return (
    <div css={{ height: "100vh", overflowY: "scroll" }}>
      <div
        css={{
          overflowY: "scroll",
          padding: "20px 50px",
        }}
      >
        {children}
      </div>
    </div>
  );
}
