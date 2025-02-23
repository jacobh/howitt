type Props = {
  svgData: string;
  className?: string;
};

export function SvgIcon({ svgData, className }: Props): React.ReactElement {
  // Remove the data:image/svg+xml;utf8, prefix to get pure SVG code
  const pureSvg = svgData.replace("data:image/svg+xml;utf8,", "");

  return (
    <div dangerouslySetInnerHTML={{ __html: pureSvg }} className={className} />
  );
}
