import type { SVGProps } from "react";

export function LocalSearchLogo(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 64 44" {...props}>
      <path
        clipRule="evenodd"
        d="M4 1h54l-2 7 7 5-4 9 4 7-7 14H5l2-7-6-5 4-9-4-7L4 1Zm7 33h6l11-24h-6L11 34Zm19-24h6v19h4v5H30V10Zm13 0h14v6h-8v3h8v15H43v-6h8v-3h-8V10Z"
        fill="currentColor"
        fillRule="evenodd"
      />
    </svg>
  );
}
