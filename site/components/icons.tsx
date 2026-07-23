import type { SVGProps } from "react";

type IconProps = SVGProps<SVGSVGElement> & { size?: number };
type BrandIconProps = { size?: number; className?: string };

function BrandAssetIcon({
  src,
  name,
  size = 18,
  className = "",
}: BrandIconProps & { src: string; name: string }) {
  return (
    <span
      aria-hidden="true"
      className={`brand-asset-icon brand-asset-icon--${name} ${className}`.trim()}
      style={{ backgroundImage: `url(${src})`, height: size, width: size }}
    />
  );
}

function Icon({ size = 18, children, ...props }: IconProps) {
  return (
    <svg
      aria-hidden="true"
      fill="none"
      height={size}
      viewBox="0 0 24 24"
      width={size}
      {...props}
    >
      {children}
    </svg>
  );
}

const stroke = {
  stroke: "currentColor",
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
  strokeWidth: 2,
};

export function ArrowUpRightIcon(props: IconProps) {
  return <Icon {...props}><path d="M7 17 17 7M7 7h10v10" {...stroke} /></Icon>;
}

export function CheckIcon(props: IconProps) {
  return <Icon {...props}><path d="m5 12 4 4L19 6" {...stroke} /></Icon>;
}

export function ChevronDownIcon(props: IconProps) {
  return <Icon {...props}><path d="m6 9 6 6 6-6" {...stroke} /></Icon>;
}

export function ChevronRightIcon(props: IconProps) {
  return <Icon {...props}><path d="m9 18 6-6-6-6" {...stroke} /></Icon>;
}

export function CopyIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <rect height="13" rx="2" width="13" x="9" y="9" {...stroke} />
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" {...stroke} />
    </Icon>
  );
}

export function GithubIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <path
        d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3.3-.4 6.8-1.6 6.8-7A5.4 5.4 0 0 0 19.3 4 5 5 0 0 0 19.2.5S18 0 15 2a13.4 13.4 0 0 0-7 0C5 .1 3.8.5 3.8.5A5 5 0 0 0 3.7 4a5.4 5.4 0 0 0-1.5 3.7c0 5.3 3.5 6.5 6.8 6.9A4.8 4.8 0 0 0 8 18v4m0-3c-3 .9-3-1.5-4-2"
        {...stroke}
      />
    </Icon>
  );
}

export function GlobeIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <circle cx="12" cy="12" r="9" {...stroke} />
      <path d="M3 12h18M12 3a15 15 0 0 1 0 18M12 3a15 15 0 0 0 0 18" {...stroke} />
    </Icon>
  );
}

export function PackageIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <path d="m12 2 9 5-9 5-9-5 9-5Z" {...stroke} />
      <path d="m3 7 9 5 9-5M3 12l9 5 9-5M3 17l9 5 9-5" {...stroke} />
    </Icon>
  );
}

export function GithubBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="github" src="/brand/github-mono.svg" />;
}

export function RustOfficialIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="rust" src="/brand/rust-mono.svg" />;
}

export function CargoBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="cargo" src="/brand/cargo.png" />;
}

export function CratesIoBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="crates-io" src="/brand/crates-io.png" />;
}

export function ClaudeBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="claude" src="/brand/claude.svg" />;
}

export function CodexBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="codex" src="/brand/codex.svg" />;
}

export function CursorBrandIcon(props: BrandIconProps) {
  return <BrandAssetIcon {...props} name="cursor" src="/brand/cursor-mono.svg" />;
}

export function PlayIcon(props: IconProps) {
  return <Icon {...props}><path d="m8 5 11 7-11 7V5Z" {...stroke} /></Icon>;
}

export function RotateIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <path d="M20 7v5h-5M4 17v-5h5" {...stroke} />
      <path d="M6.1 9a7 7 0 0 1 11.5-2L20 12M4 12l2.4 5a7 7 0 0 0 11.5-2" {...stroke} />
    </Icon>
  );
}

export function SearchIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <circle cx="11" cy="11" r="7" {...stroke} />
      <path d="m20 20-4-4" {...stroke} />
    </Icon>
  );
}

export function StopIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <circle cx="12" cy="12" r="9" {...stroke} />
      <rect fill="currentColor" height="7" rx="1" width="7" x="8.5" y="8.5" />
    </Icon>
  );
}

export function TerminalIcon(props: IconProps) {
  return (
    <Icon {...props}>
      <rect height="18" rx="2" width="20" x="2" y="3" {...stroke} />
      <path d="m7 8 3 3-3 3M13 14h4" {...stroke} />
    </Icon>
  );
}

export function RustBrandIcon(props: IconProps) {
  return (
    <svg aria-hidden="true" fill="currentColor" viewBox="0 0 24 24" {...props}>
      <path d="M23.8346 11.7033l-1.0073-.6236a13.7268 13.7268 0 0 0-.0283-.2936l.8656-.8069a.3483.3483 0 0 0-.1154-.578l-1.1066-.414a8.4958 8.4958 0 0 0-.087-.2856l.6904-.9587a.3462.3462 0 0 0-.2257-.5446l-1.1663-.1894a9.3574 9.3574 0 0 0-.1407-.2622l.49-1.0761a.3437.3437 0 0 0-.0274-.3361.3486.3486 0 0 0-.3006-.154l-1.1845.0416a6.7444 6.7444 0 0 0-.1873-.2268l.2723-1.153a.3472.3472 0 0 0-.417-.4172l-1.1532.2724a14.0183 14.0183 0 0 0-.2278-.1873l.0415-1.1845a.3442.3442 0 0 0-.49-.328l-1.076.491c-.0872-.0476-.1742-.0952-.2623-.1407l-.1903-1.1673A.3483.3483 0 0 0 16.256.955l-.9597.6905a8.4867 8.4867 0 0 0-.2855-.086l-.414-1.1066a.3483.3483 0 0 0-.5781-.1154l-.8069.8666a9.2936 9.2936 0 0 0-.2936-.0284L12.2946.1683a.3462.3462 0 0 0-.5892 0l-.6236 1.0073a13.7383 13.7383 0 0 0-.2936.0284L9.9803.3374a.3462.3462 0 0 0-.578.1154l-.4141 1.1065c-.0962.0274-.1903.0567-.2855.086L7.744.955a.3483.3483 0 0 0-.5447.2258L7.009 2.348a9.3574 9.3574 0 0 0-.2622.1407l-1.0762-.491a.3462.3462 0 0 0-.49.328l.0416 1.1845a7.9826 7.9826 0 0 0-.2278.1873L3.8413 3.425a.3472.3472 0 0 0-.4171.4171l.2713 1.1531c-.0628.075-.1255.1509-.1863.2268l-1.1845-.0415a.3462.3462 0 0 0-.328.49l.491 1.0761a9.167 9.167 0 0 0-.1407.2622l-1.1662.1894a.3483.3483 0 0 0-.2258.5446l.6904.9587a13.303 13.303 0 0 0-.087.2855l-1.1065.414a.3483.3483 0 0 0-.1155.5781l.8656.807a9.2936 9.2936 0 0 0-.0283.2935l-1.0073.6236a.3442.3442 0 0 0 0 .5892l1.0073.6236c.008.0982.0182.1964.0283.2936l-.8656.8079a.3462.3462 0 0 0 .1155.578l1.1065.4141c.0273.0962.0567.1914.087.2855l-.6904.9587a.3452.3452 0 0 0 .2268.5447l1.1662.1893c.0456.088.0922.1751.1408.2622l-.491 1.0762a.3462.3462 0 0 0 .328.49l1.1834-.0415c.0618.0769.1235.1528.1873.2277l-.2713 1.1541a.3462.3462 0 0 0 .4171.4161l1.153-.2713c.075.0638.151.1255.2279.1863l-.0415 1.1845a.3442.3442 0 0 0 .49.327l1.0761-.49c.087.0486.1741.0951.2622.1407l.1903 1.1662a.3483.3483 0 0 0 .5447.2268l.9587-.6904a9.299 9.299 0 0 0 .2855.087l.414 1.1066a.3452.3452 0 0 0 .5781.1154l.8079-.8656c.0972.0111.1954.0203.2936.0294l.6236 1.0073a.3472.3472 0 0 0 .5892 0l.6236-1.0073c.0982-.0091.1964-.0183.2936-.0294l.8069.8656a.3483.3483 0 0 0 .578-.1154l.4141-1.1066a8.4626 8.4626 0 0 0 .2855-.087l.9587.6904a.3452.3452 0 0 0 .5447-.2268l.1903-1.1662c.088-.0456.1751-.0931.2622-.1407l1.0762.49a.3472.3472 0 0 0 .49-.327l-.0415-1.1845a6.7267 6.7267 0 0 0 .2267-.1863l1.1531.2713a.3472.3472 0 0 0 .4171-.416l-.2713-1.1542c.0628-.0749.1255-.1508.1863-.2278l1.1845.0415a.3442.3442 0 0 0 .328-.49l-.49-1.076c.0475-.0872.0951-.1742.1407-.2623l1.1662-.1893a.3483.3483 0 0 0 .2258-.5447l-.6904-.9587.087-.2855 1.1066-.414a.3462.3462 0 0 0 .1154-.5781l-.8656-.8079c.0101-.0972.0202-.1954.0283-.2936l1.0073-.6236a.3442.3442 0 0 0 0-.5892zm-7.0835 6.0406a.649.649 0 0 0-.7715.5l-.3573 1.6685c-1.1035.501-2.3285.7795-3.6193.7795a8.7368 8.7368 0 0 1-3.6951-.814l-.3574-1.6684a.648.648 0 0 0-.7714-.499l-1.473.3158a8.7216 8.7216 0 0 1-.7613-.898h7.1676c.081 0 .1356-.0141.1356-.088v-2.536c0-.074-.0536-.0881-.1356-.0881h-2.0966v-1.6077h2.2677c.2065 0 1.1065.0587 1.394 1.2088.0901.3533.2875 1.5044.4232 1.8729.1346.413.6833 1.2381 1.2685 1.2381h3.5716a8.7874 8.7874 0 0 1-1.1379 1.8385zm-6.7343-7.3091V8.7836h2.9601c.153 0 1.0792.1772 1.0792.8697 0 .575-.7107.7815-1.2948.7815zM20.7742 11.921c0 .2187-.008.4363-.0243.651h-.9c-.09 0-.1265.0586-.1265.1477v.413c0 .973-.5487 1.1846-1.0296 1.2382-.4576.0517-.9648-.1913-1.0275-.4717-.2704-1.5186-.7198-1.8436-1.4305-2.4034.8817-.5599 1.799-1.386 1.799-2.4915 0-1.1936-.819-1.9458-1.3769-2.3153-.7825-.5163-1.6491-.6195-1.883-.6195H5.4682a8.7651 8.7651 0 0 1 15.2635 4.9805c.0283.2875.0425.577.0425.8717z" />
    </svg>
  );
}

export function ChromeBrandIcon(props: IconProps) {
  return (
    <svg aria-hidden="true" fill="currentColor" viewBox="0 0 24 24" {...props}>
      <path d="M12 0C8.21 0 4.831 1.757 2.632 4.501l3.953 6.848A5.454 5.454 0 0 1 12 6.545h10.691A12 12 0 0 0 12 0zM1.931 5.47A11.943 11.943 0 0 0 0 12c0 6.012 4.42 10.991 10.189 11.864l3.953-6.847a5.45 5.45 0 0 1-6.865-2.29zm13.342 2.166a5.446 5.446 0 0 1 1.45 7.09l.002.001h-.002l-5.344 9.257c.206.01.413.016.621.016 6.627 0 12-5.373 12-12 0-1.54-.29-3.011-.818-4.364zM12 16.364a4.364 4.364 0 1 1 0-8.728 4.364 4.364 0 0 1 0 8.728Z" />
    </svg>
  );
}

export function GoogleBrandIcon(props: IconProps) {
  return (
    <svg aria-hidden="true" fill="currentColor" viewBox="0 0 24 24" {...props}>
      <path d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z" />
    </svg>
  );
}

export function BingBrandIcon(props: IconProps) {
  return (
    <svg aria-hidden="true" fill="currentColor" viewBox="0 0 678 1024" {...props}>
      <path d="M0 778.3c14.6 123.8 223.8 143 236.8 79.9-.3-.4-.5-678.1-.5-678.1-3.6-46-26.2-72-61.6-96.5-33-22.7-74.4-50.4-96.9-66.4C14.2-28 .1 31.4 0 33.2c0 0 .3 746.4 0 745.1z" />
      <path d="M236.8 832.8c-96.2 72.5-217 42.7-234.4-44-.8-4.2-2.4-10.4-2.4-10.4s.9 8.5 2 16.6c1.2 8.5 3.7 20.8 6.3 31.3 30 117.8 132.1 186 230.4 196.6C373.3 1034.8 497.4 931 599 855.8c6.3-6.2 15.4-16.2 18.1-20.1 66.2-95-13.6-197-72.5-193a59154 59154 0 0 0-307.7 190.1Z" />
      <path d="M312.8 381c7.4 47 34.6 108.7 59.6 172.6 20.2 41.3 62 53.4 103 65.5 42.4 12.6 65.6 21 85.6 30.9 138.5 68.7 38.5 207.7 59.6 181.4 89-110.7 79.7-325.4-90-418.1-57.6-28.7-115.4-66.6-156.5-83.6-41-17-68.7 4.3-61.3 51.3z" />
    </svg>
  );
}

export function DuckDuckGoBrandIcon(props: IconProps) {
  return (
    <svg aria-hidden="true" fill="currentColor" viewBox="0 0 24 24" {...props}>
      <circle cx="12" cy="12" r="11" fill="none" stroke="currentColor" strokeWidth="1.5" />
      <path d="M8.1 14.6c-.8-2.8-1.3-5.7.3-7.2.8-.8 2-1 3-.6.8.3 1.3.9 1.6 1.8.6.1 1.4.3 2 .7.7.5.6 1.2-.1 1.7-.6.4-1.4.6-2.3.6-.8 0-1.1.4-.9 1 .3.7 1.3 1.1 2.9 1 .7 0 1.2.3.9.8-.5.8-2 1.2-3.6.9-1.5-.2-2.8-.7-3.8-.7zM9.3 8.2a.8.8 0 1 0 0 1.6.8.8 0 0 0 0-1.6zm4.2.5a.65.65 0 1 0 0 1.3.65.65 0 0 0 0-1.3z" />
      <path d="m9 16.2 3 1.5 3-1.5-.4 3-2.6-1.5-2.6 1.5z" />
    </svg>
  );
}
